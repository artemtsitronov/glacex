use glacex::{
    Alignment, App, Expanded, Language, TabItem, Tabs, TextArea, TextEditState, Theme, Ui, Widget,
    column,
};
use std::fs::read_to_string;
use std::path::Path;

struct EditorFile {
    filename: String,
    filecontent: String,
    cursor: usize,
    selection_anchor: Option<usize>,
}

impl EditorFile {
    fn new(path: &Path) -> Self {
        EditorFile {
            filename: path.file_name().unwrap().to_string_lossy().into_owned(),
            filecontent: read_to_string(path).expect("Problems!"),
            cursor: 0,
            selection_anchor: None,
        }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn filecontent(&self) -> String {
        self.filecontent.clone()
    }

    pub fn set_filecontent(&mut self, filecontent: String) {
        self.filecontent = filecontent;
    }

    pub fn cursor(&self) -> usize {
        self.cursor.clone()
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    pub fn selection_anchor(&self) -> Option<usize> {
        self.selection_anchor.clone()
    }

    pub fn set_selection_anchor(&mut self, selection_anchor: Option<usize>) {
        self.selection_anchor = selection_anchor;
    }
}

struct CodeEditor {
    editor_files: Vec<EditorFile>,
    loaded_idx: Option<usize>,
}

impl CodeEditor {
    fn new() -> Self {
        CodeEditor {
            editor_files: vec![
                EditorFile::new(Path::new("examples/demo.rs")),
                EditorFile::new(Path::new("src/lib.rs")),
                EditorFile::new(Path::new("src/painter.rs")),
            ],
            loaded_idx: None,
        }
    }
}

impl Widget for CodeEditor {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_theme(Theme::DARK);

        let tab_items = self
            .editor_files
            .iter()
            .enumerate()
            .map(|(i, x)| TabItem::new(x.filename()).id(i.to_string()))
            .collect();
        let mut tabs = Tabs::new(tab_items).id("tabs");

        let active_idx = tabs.selected(ui).and_then(|id| id.parse::<usize>().ok());

        let (active_editor_content, active_editor_cursor, active_editor_selection_anchor) =
            active_idx
                .and_then(|i| self.editor_files.get(i))
                .map(|f| (f.filecontent(), f.cursor(), f.selection_anchor()))
                .unwrap_or_default();

        let mut code_area = TextArea::new().language(Language::Rust).id("code-area");

        if self.loaded_idx != active_idx {
            let code_area_state = ui.widget_state::<TextEditState>("code-area");
            code_area_state.set_text(active_editor_content);
            code_area_state.set_cursor(active_editor_cursor);
            code_area_state.set_selection_anchor(active_editor_selection_anchor);
            self.loaded_idx = active_idx;
        }

        let mut editor = Expanded::new(&mut code_area);

        column![&mut tabs, &mut editor]
            .size(ui.window_size())
            .spacing(10.0)
            .padding([10.0; 2])
            .align(Alignment::Stretch)
            .arrange_at([0.0; 2], ui);

        if let Some(f) = active_idx.and_then(|i| self.editor_files.get_mut(i)) {
            let code_area_state = ui.widget_state::<TextEditState>("code-area");
            f.set_filecontent(code_area_state.text().to_string());
            f.set_cursor(code_area_state.cursor());
            f.set_selection_anchor(code_area_state.selection_anchor());
        }
    }
}

fn main() {
    App::new(CodeEditor::new()).run()
}
