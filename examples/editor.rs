use glacex::{
    Alignment, AnyWidget, App, Button, Column, Divider, Expanded, FontFamily, Label, Language,
    ScrollView, SelectBox, SelectOption, Switch, TabItem, Tabs, TextArea, TextEditState, TextStyle,
    Theme, Ui, Widget, column, row,
};
use std::fs;
use std::path::PathBuf;
use winit::keyboard::{KeyCode, PhysicalKey};

struct File {
    path: PathBuf,
    text: String,
    saved: String,
    cursor: usize,
}

impl File {
    fn open(path: PathBuf) -> Option<Self> {
        let text = fs::read_to_string(&path).ok()?;
        Some(File {
            path,
            saved: text.clone(),
            text,
            cursor: 0,
        })
    }

    fn name(&self) -> String {
        self.path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    }

    fn dirty(&self) -> bool {
        self.text != self.saved
    }

    fn save(&mut self) -> bool {
        if fs::write(&self.path, &self.text).is_ok() {
            self.saved = self.text.clone();
            true
        } else {
            false
        }
    }
}

struct Editor {
    dir: PathBuf,
    tree: Vec<PathBuf>,
    open: Vec<File>,
    active: Option<usize>,
    loaded: Option<PathBuf>,
    message: String,
}

impl Editor {
    fn new(dir: PathBuf) -> Self {
        let mut tree: Vec<PathBuf> = fs::read_dir(&dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.is_file())
                    .collect()
            })
            .unwrap_or_default();
        tree.sort();
        Editor {
            dir,
            tree,
            open: Vec::new(),
            active: None,
            loaded: None,
            message: String::new(),
        }
    }

    fn open_file(&mut self, path: &PathBuf) {
        if let Some(i) = self.open.iter().position(|f| &f.path == path) {
            self.active = Some(i);
            return;
        }
        match File::open(path.clone()) {
            Some(f) => {
                self.open.push(f);
                self.active = Some(self.open.len() - 1);
            }
            None => self.message = format!("can't read {}", path.display()),
        }
    }
}

impl Widget for Editor {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = ui.window_size();

        let mut theme_pick = SelectBox::new(
            Theme::all()
                .iter()
                .map(|t| SelectOption::new(t.name, t.name))
                .collect(),
        )
        .id("theme")
        .placeholder("theme")
        .width(180.0);
        if let Some(name) = theme_pick.selected(ui)
            && let Some(theme) = Theme::from_name(&name)
        {
            ui.set_theme(theme);
        }

        let mut font_pick = SelectBox::new(
            [12, 13, 14, 16, 18]
                .iter()
                .map(|s| SelectOption::new(s.to_string(), format!("{s}px")))
                .collect(),
        )
        .id("font")
        .placeholder("14px")
        .width(90.0);
        let font_size: f32 = font_pick
            .selected(ui)
            .and_then(|s| s.parse().ok())
            .unwrap_or(14.0);

        let mut autosave = Switch::new().id("autosave");
        let autosave_on = autosave.enabled(ui);

        let mut folder = Label::new(self.dir.display().to_string()).caption();
        let mut entries: Vec<Button> = self
            .tree
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let name = p.file_name().unwrap_or_default().to_string_lossy();
                let b = Button::new(name).id(format!("tree-{i}")).width(180.0);
                let is_active = self
                    .active
                    .and_then(|a| self.open.get(a))
                    .is_some_and(|f| &f.path == p);
                if is_active { b.outline() } else { b.ghost() }
            })
            .collect();

        let mut tabs = Tabs::new(
            self.open
                .iter()
                .map(|f| {
                    let name = if f.dirty() {
                        format!("{} *", f.name())
                    } else {
                        f.name()
                    };
                    TabItem::new(name).id(f.path.display().to_string())
                })
                .collect(),
        )
        .id("tabs");
        if let Some(f) = self.active.and_then(|a| self.open.get(a)) {
            tabs.set_selected(ui, f.path.display().to_string());
        }
        let mut close = Button::new("Close").id("close").ghost();

        let mut code = TextArea::new()
            .id("code")
            .language(Language::Rust)
            .text_style(TextStyle {
                font_family: FontFamily::Mono,
                ..TextStyle::new().size(font_size, (font_size * 1.5).round())
            });

        let active_path = self
            .active
            .and_then(|a| self.open.get(a))
            .map(|f| f.path.clone());
        if self.loaded != active_path {
            let state = ui.widget_state::<TextEditState>("code");
            match self.active.and_then(|a| self.open.get(a)) {
                Some(f) => {
                    state.set_text(f.text.clone());
                    state.set_cursor(f.cursor);
                }
                None => state.set_text(String::new()),
            }
            state.set_selection_anchor(None);
            self.loaded = active_path.clone();
        }

        let status = {
            let state = ui.widget_state::<TextEditState>("code");
            let cursor = state.cursor();
            let line = state
                .text()
                .chars()
                .take(cursor)
                .filter(|&c| c == '\n')
                .count()
                + 1;
            let col = state.column_of(cursor) + 1;
            match &active_path {
                Some(_) => format!("Ln {line}, Col {col}    {} lines", state.line_count()),
                None => "no file open".to_string(),
            }
        };
        let mut status_label = Label::new(status).caption();
        let mut message = Label::new(&self.message).caption();
        let mut autosave_label = Label::new("autosave").caption();

        {
            let mut tree = Column::new(
                entries
                    .iter_mut()
                    .map(|b| Box::new(b) as Box<dyn AnyWidget>)
                    .collect(),
            )
            .spacing(2.0);
            let mut tree_scroll = ScrollView::new(&mut tree).id("tree").width(180.0);
            let mut tree_area = Expanded::new(&mut tree_scroll);
            let mut sidebar = column![&mut folder, &mut tree_area]
                .spacing(8.0)
                .width(180.0);

            let mut empty = Label::new("open something from the left").muted();
            let mut tab_bar = if self.open.is_empty() {
                row![&mut empty].align(Alignment::Center)
            } else {
                row![&mut tabs, &mut close]
                    .spacing(8.0)
                    .align(Alignment::Center)
            };
            let mut code_area = Expanded::new(&mut code);
            let mut message_area = Expanded::new(&mut message);
            let mut status_bar = row![
                &mut status_label,
                &mut message_area,
                &mut autosave_label,
                &mut autosave,
                &mut font_pick,
                &mut theme_pick,
            ]
            .spacing(12.0)
            .align(Alignment::Center);

            let mut main = column![&mut tab_bar, &mut code_area, &mut status_bar]
                .spacing(8.0)
                .align(Alignment::Stretch);

            row![
                &mut sidebar,
                &mut Divider::vertical(0.0).faint(),
                &mut Expanded::new(&mut main),
            ]
            .spacing(10.0)
            .padding([10.0, 10.0])
            .size(size)
            .align(Alignment::Stretch)
            .arrange_at([0.0, 0.0], ui);
        }

        if let Some(f) = self.active.and_then(|a| self.open.get_mut(a)) {
            let state = ui.widget_state::<TextEditState>("code");
            f.text = state.text().to_string();
            f.cursor = state.cursor();
        }

        if let Some(id) = tabs.selected(ui)
            && let Some(i) = self
                .open
                .iter()
                .position(|f| f.path.display().to_string() == id)
            && self.active != Some(i)
        {
            if autosave_on && let Some(f) = self.active.and_then(|a| self.open.get_mut(a)) {
                f.save();
            }
            self.active = Some(i);
        }

        let save = ui.ctrl_held() && ui.physical_key_pressed(PhysicalKey::Code(KeyCode::KeyS));
        if save && let Some(f) = self.active.and_then(|a| self.open.get_mut(a)) {
            self.message = if f.save() {
                format!("saved {}", f.name())
            } else {
                format!("couldn't save {}", f.name())
            };
        }

        if close.clicked()
            && let Some(a) = self.active
        {
            if autosave_on {
                self.open[a].save();
            }
            self.open.remove(a);
            self.active = if self.open.is_empty() {
                None
            } else {
                Some(a.min(self.open.len() - 1))
            };
        }

        let clicked: Option<PathBuf> = entries
            .iter()
            .zip(self.tree.iter())
            .find(|(b, _)| b.clicked())
            .map(|(_, p)| p.clone());
        if let Some(p) = clicked {
            if autosave_on && let Some(f) = self.active.and_then(|a| self.open.get_mut(a)) {
                f.save();
            }
            self.open_file(&p);
        }
    }
}

fn main() {
    let dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("examples"));
    App::new(Editor::new(dir))
        .title("editor")
        .window_size(1100, 720)
        .run();
}
