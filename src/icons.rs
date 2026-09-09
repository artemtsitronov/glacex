//! Common Nerd Font glyphs for compact controls and product navigation.

include!("nerd_icons_generated.rs");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NerdGlyph {
    pub name: &'static str,
    pub character: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NerdIcon {
    ArrowLeft,
    ArrowRight,
    Check,
    ChevronDown,
    Close,
    Download,
    ExternalLink,
    Github,
    Home,
    Menu,
    Plus,
    Search,
    Settings,
    Star,
    Trash,
    User,
}

impl NerdIcon {
    pub const fn glyph(self) -> &'static str {
        match self {
            Self::ArrowLeft => "󰁍",
            Self::ArrowRight => "󰁔",
            Self::Check => "󰄬",
            Self::ChevronDown => "󰅀",
            Self::Close => "󰅖",
            Self::Download => "󰇚",
            Self::ExternalLink => "󰏌",
            Self::Github => "󰊤",
            Self::Home => "󰋜",
            Self::Menu => "󰍜",
            Self::Plus => "󰐕",
            Self::Search => "󰍉",
            Self::Settings => "󰒓",
            Self::Star => "󰓎",
            Self::Trash => "󰆴",
            Self::User => "󰀄",
        }
    }

    pub const fn mono(self) -> &'static str {
        self.glyph()
    }

    pub fn named(name: &str) -> Option<NerdGlyph> {
        NERD_GLYPHS
            .binary_search_by_key(&name, |(name, _)| *name)
            .ok()
            .map(|index| {
                let (name, character) = NERD_GLYPHS[index];
                NerdGlyph { name, character }
            })
    }

    pub const fn all() -> &'static [(&'static str, char)] {
        NERD_GLYPHS
    }
}

#[cfg(test)]
mod tests {
    use super::NerdIcon;

    #[test]
    fn generated_catalog_contains_the_full_nerd_font_sets() {
        assert!(NerdIcon::all().len() > 10_000);
        assert_eq!(
            NerdIcon::named("md-home").map(|icon| icon.character),
            Some('\u{f02dc}')
        );
        assert!(NerdIcon::named("does-not-exist").is_none());
    }
}
