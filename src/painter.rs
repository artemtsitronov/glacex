use crate::shapes::{QUAD_VERTICES, QuadVertex, RectInstance};
use glyphon::{
    Attrs, Cache, FontSystem, Metrics, Resolution, Shaping, SwashCache, TextArea, TextAtlas,
    TextBounds, TextRenderer, Viewport,
};
use std::borrow::Cow;
use std::iter;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferUsages, Color,
    ColorTargetState, ColorWrites, CommandEncoderDescriptor, CurrentSurfaceTexture, Device,
    DeviceDescriptor, FragmentState, Instance, LoadOp, MultisampleState, Operations,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface,
    SurfaceConfiguration, TextureViewDescriptor, VertexState,
};
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct WindowSize {
    width: f32,
    height: f32,
}

/// Owns every GPU and font-rendering detail. Knows nothing about buttons,
/// labels, hit-testing, or layout — its entire job is "draw a rectangle" /
/// "draw some text", queued per frame, uploaded and submitted once.
pub struct Painter {
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,

    window_size_buffer: Buffer,
    bind_group: BindGroup,

    quad_vertex_buffer: Buffer,
    rect_instance_buffer: Buffer,
    rect_instance_capacity: usize,
    pending_rects: Vec<([f32; 4], RectInstance)>,

    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    text_atlas: TextAtlas,
    text_renderer: TextRenderer,
    font_metrics: Metrics,
    pending_labels: Vec<(glyphon::Buffer, [f32; 2], [f32; 4])>,

    bgcolor: Color,
}

impl Painter {
    pub async fn new(window: Arc<Window>) -> Painter {
        let instance = Instance::default();

        let surface = instance.create_surface(Arc::clone(&window)).unwrap();
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &surface_config);

        let window_size_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[WindowSize {
                width: size.width as f32,
                height: size.height as f32,
            }]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                count: None,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            }],
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: window_size_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &[Some(QuadVertex::LAYOUT), Some(RectInstance::LAYOUT)],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let quad_vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let rect_instance_capacity: usize = 16;
        let rect_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (rect_instance_capacity * std::mem::size_of::<RectInstance>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut text_atlas = TextAtlas::new(&device, &queue, &cache, surface_config.format);
        let text_renderer =
            TextRenderer::new(&mut text_atlas, &device, MultisampleState::default(), None);

        let font_metrics = Metrics {
            font_size: 16.0,
            line_height: 20.0,
        };

        Painter {
            surface,
            surface_config,
            device,
            queue,
            render_pipeline,
            window_size_buffer,
            bind_group,
            quad_vertex_buffer,
            rect_instance_buffer,
            rect_instance_capacity,
            pending_rects: vec![],
            font_system,
            swash_cache,
            viewport,
            text_atlas,
            text_renderer,
            font_metrics,
            pending_labels: vec![],
            bgcolor: Color::BLACK,
        }
    }

    pub fn begin_frame(&mut self) {
        self.pending_rects.clear();
        self.pending_labels.clear();
    }

    pub fn window_size(&self) -> [f32; 2] {
        [
            self.surface_config.width as f32,
            self.surface_config.height as f32,
        ]
    }

    pub fn line_height(&self) -> f32 {
        self.font_metrics.line_height
    }

    pub fn measure_text(&mut self, text: &str) -> f32 {
        let mut buffer = glyphon::Buffer::new(&mut self.font_system, self.font_metrics);
        buffer.set_size(Some(1000.0), Some(1000.0));
        buffer.set_text(text, &Attrs::new(), Shaping::Basic, None);
        buffer.shape_until_scroll(&mut self.font_system, false);

        buffer
            .layout_runs()
            .map(|run| run.line_w)
            .fold(0.0, f32::max)
    }

    pub fn set_bgcolor(&mut self, color: [f32; 4]) {
        self.bgcolor = Color {
            r: color[0] as f64,
            g: color[1] as f64,
            b: color[2] as f64,
            a: color[3] as f64,
        };
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_rect(
        &mut self,
        position: [f32; 2],
        size: [f32; 2],
        color: [f32; 4],
        corner_radius: f32,
        border_width: f32,
        border_color: [f32; 4],
        blur_radius: f32,
        sharp: f32,
        clip: [f32; 4],
    ) {
        self.pending_rects.push((
            clip,
            RectInstance {
                position,
                size,
                color,
                corner_radius,
                border_width,
                border_color,
                blur_radius,
                sharp,
            },
        ));
    }

    pub fn draw_text(&mut self, text: &str, position: [f32; 2], bounds: [f32; 4]) {
        let mut buffer = glyphon::Buffer::new(&mut self.font_system, self.font_metrics);
        buffer.set_size(Some(1000.0), Some(1000.0));
        buffer.set_text(text, &Attrs::new(), Shaping::Basic, None);
        buffer.shape_until_scroll(&mut self.font_system, false);

        self.pending_labels.push((buffer, position, bounds));
    }

    pub fn present(&mut self) {
        if self.pending_rects.len() > self.rect_instance_capacity {
            self.rect_instance_capacity = self.pending_rects.len() * 2;
            let placeholder = RectInstance {
                position: [0.0; 2],
                size: [0.0; 2],
                color: [0.0; 4],
                corner_radius: 8.0,
                border_width: 5.0,
                border_color: [255.0; 4],
                blur_radius: 5.0,
                sharp: 0.0,
            };
            self.rect_instance_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vec![placeholder; self.rect_instance_capacity]),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            });
        }

        // Upload just the instance data — clip rects stay CPU-side, used only
        // to decide scissor rects/draw ranges below, never sent to the GPU.
        let instances: Vec<RectInstance> = self.pending_rects.iter().map(|(_, r)| *r).collect();
        self.queue.write_buffer(
            &self.rect_instance_buffer,
            0,
            bytemuck::cast_slice(&instances),
        );

        let surface_texture = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture) => texture,
            _ => return,
        };

        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        self.viewport.update(
            &self.queue,
            Resolution {
                width: self.surface_config.width,
                height: self.surface_config.height,
            },
        );

        let text_areas = self
            .pending_labels
            .iter()
            .map(|(buffer, position, bounds)| TextArea {
                buffer,
                left: position[0],
                top: position[1],
                scale: 1.0,
                bounds: TextBounds {
                    left: bounds[0] as i32,
                    top: bounds[1] as i32,
                    right: bounds[2] as i32,
                    bottom: bounds[3] as i32,
                },
                default_color: glyphon::Color::rgb(255, 255, 255),
                custom_glyphs: &[],
            });

        self.text_renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.text_atlas,
                &self.viewport,
                text_areas,
                &mut self.swash_cache,
            )
            .expect("failed to prepare text");

        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(self.bgcolor),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.quad_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.rect_instance_buffer.slice(..));
            render_pass.set_bind_group(0, &self.bind_group, &[]);

            // Group consecutive rects sharing the same clip rect into one
            // draw call each, setting the scissor rect before every group.
            // Submission order is preserved — only the *batching* changes,
            // never the paint order.
            let surface_w = self.surface_config.width;
            let surface_h = self.surface_config.height;

            let mut range_start = 0usize;
            while range_start < self.pending_rects.len() {
                let clip = self.pending_rects[range_start].0;
                let mut range_end = range_start + 1;
                while range_end < self.pending_rects.len()
                    && self.pending_rects[range_end].0 == clip
                {
                    range_end += 1;
                }

                // Clip rect -> scissor rect, clamped into the surface bounds.
                // wgpu panics on a scissor rect that extends past the render
                // target or has zero/negative size, so both are guarded here.
                let x = clip[0].max(0.0) as u32;
                let y = clip[1].max(0.0) as u32;
                let right = (clip[2].max(0.0) as u32).min(surface_w);
                let bottom = (clip[3].max(0.0) as u32).min(surface_h);

                if right > x && bottom > y {
                    render_pass.set_scissor_rect(x, y, right - x, bottom - y);
                    render_pass.draw(
                        0..QUAD_VERTICES.len() as u32,
                        range_start as u32..range_end as u32,
                    );
                }
                // else: this group's clip rect is fully offscreen/degenerate
                // (e.g. scrolled entirely out of view) — correctly skipped,
                // not drawn at all.

                range_start = range_end;
            }

            render_pass.set_scissor_rect(0, 0, surface_w, surface_h);

            self.text_renderer
                .render(&self.text_atlas, &self.viewport, &mut render_pass)
                .expect("failed to render text");
        }

        self.queue.submit(iter::once(command_encoder.finish()));
        self.queue.present(surface_texture);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);

        self.queue.write_buffer(
            &self.window_size_buffer,
            0,
            bytemuck::cast_slice(&[WindowSize {
                width: width as f32,
                height: height as f32,
            }]),
        );
    }
}
