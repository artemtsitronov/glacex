use crate::sdf::{bake_open_sdf, bake_sdf};
use kurbo::{BezPath, Rect, Shape};
use std::collections::HashMap;
use wgpu::{Device, Queue};

const ATLAS_SIZE: u32 = 2048;
const BAKE_MARGIN: f64 = 24.0;

fn resolution_for_bounds(bounds: Rect) -> (u32, u32) {
    let w = (bounds.width() as u32).clamp(32, 256);
    let h = (bounds.height() as u32).clamp(32, 256);
    (w, h)
}

fn f32_to_f16_bits(value: f32) -> u16 {
    let bits = value.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xff) as i32 - 127 + 15;
    let mantissa = bits & 0x7fffff;

    if exp <= 0 {
        sign
    } else if exp >= 0x1f {
        sign | 0x7c00
    } else {
        sign | ((exp as u16) << 10) | (mantissa >> 13) as u16
    }
}

#[derive(Default)]
pub struct ShelfPacker {
    cursor_x: u32,
    cursor_y: u32,
    shelf_height: u32,
}

impl ShelfPacker {
    pub fn alloc(&mut self, w: u32, h: u32) -> Option<(u32, u32)> {
        if self.cursor_x + w > ATLAS_SIZE {
            self.cursor_x = 0;
            self.cursor_y += self.shelf_height;
            self.shelf_height = 0;
        }
        if self.cursor_y + h > ATLAS_SIZE {
            return None;
        }
        let pos = (self.cursor_x, self.cursor_y);
        self.cursor_x += w;
        self.shelf_height = self.shelf_height.max(h);
        Some(pos)
    }
}

const NO_LENGTH: f32 = 1_000_000.0;

#[derive(Clone, Copy)]
pub struct PathSdfHandle {
    pub uv_rect: [f32; 4],
    pub origin: [f32; 2],
    pub size: [f32; 2],
    pub total_length: f32,
}

pub struct PathSdfAtlas {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    cache: HashMap<u64, PathSdfHandle>,
    shelf_packer: ShelfPacker,
}

impl PathSdfAtlas {
    pub fn new(device: &Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("path sdf atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rg16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let shelf_packer = ShelfPacker::default();

        PathSdfAtlas {
            texture,
            texture_view,
            sampler,
            shelf_packer,
            cache: HashMap::new(),
        }
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    fn upload_and_cache(
        &mut self,
        queue: &Queue,
        field: Vec<[f32; 2]>,
        bounds: Rect,
        w: u32,
        h: u32,
        key: u64,
        total_length: f32,
    ) -> PathSdfHandle {
        let field_f16: Vec<u16> = field
            .iter()
            .flat_map(|pair| [f32_to_f16_bits(pair[0]), f32_to_f16_bits(pair[1])])
            .collect();
        let (x, y) = self
            .shelf_packer
            .alloc(w, h)
            .expect("path sdf atlas full — add eviction");

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&field_f16),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );

        let handle = PathSdfHandle {
            uv_rect: [
                x as f32 / ATLAS_SIZE as f32,
                y as f32 / ATLAS_SIZE as f32,
                (x + w) as f32 / ATLAS_SIZE as f32,
                (y + h) as f32 / ATLAS_SIZE as f32,
            ],
            origin: [bounds.x0 as f32, bounds.y0 as f32],
            size: [bounds.width() as f32, bounds.height() as f32],
            total_length,
        };
        self.cache.insert(key, handle);
        handle
    }

    pub fn bake_cached(&mut self, queue: &Queue, path: &BezPath, key: u64) -> PathSdfHandle {
        if let Some(&handle) = self.cache.get(&key) {
            return handle;
        }

        let bounds = path.bounding_box().inset(BAKE_MARGIN);
        let (w, h) = resolution_for_bounds(bounds);
        let field: Vec<[f32; 2]> = bake_sdf(path, bounds, w as usize, h as usize)
            .into_iter()
            .map(|d| [d, 0.0])
            .collect();

        self.upload_and_cache(queue, field, bounds, w, h, key, NO_LENGTH)
    }

    pub fn bake_open_cached(
        &mut self,
        queue: &Queue,
        centerline: &[[f32; 2]],
        thickness: f32,
        bounds: Rect,
        key: u64,
    ) -> PathSdfHandle {
        if let Some(&handle) = self.cache.get(&key) {
            return handle;
        }

        let (w, h) = resolution_for_bounds(bounds);
        let (field, total_length) =
            bake_open_sdf(centerline, thickness, bounds, w as usize, h as usize);

        self.upload_and_cache(queue, field, bounds, w, h, key, total_length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_to_new_shelf_when_row_is_full() {
        let mut packer = ShelfPacker::default();
        let first = packer.alloc(1500, 100).unwrap();
        let second = packer.alloc(1500, 100).unwrap();
        assert_eq!(first, (0, 0));
        assert_eq!(second.1, 100);
    }
}
