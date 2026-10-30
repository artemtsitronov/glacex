use glacex::{
    Alignment, AnyWidget, App, Button, Column, Divider, Expanded, Label, ScrollView, TextArea,
    TextInput, Ui, Widget, column, row,
};

struct Note {
    title: String,
    body: String,
}

struct Notes {
    notes: Vec<Note>,
    open: usize,
    loaded: Option<usize>,
}

impl Notes {
    fn new() -> Self {
        Notes {
            notes: vec![
                Note {
                    title: "groceries".into(),
                    body: "eggs\nbread\ncoffee, the good one\nsomething for dinner".into(),
                },
                Note {
                    title: "book ideas".into(),
                    body: "a lighthouse keeper who forgets to turn the light on\n\nprobably a short story, not a book".into(),
                },
                Note {
                    title: "wifi password".into(),
                    body: "it's on the back of the router. stop asking.".into(),
                },
            ],
            open: 0,
            loaded: None,
        }
    }
}

impl Widget for Notes {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = ui.window_size();

        let mut search = TextInput::new()
            .id("search")
            .placeholder("search")
            .width(200.0);
        let query = search.text(ui).to_lowercase();
        let mut new = Button::new("New note").id("new").width(200.0);

        let matches: Vec<usize> = self
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| {
                query.is_empty()
                    || n.title.to_lowercase().contains(&query)
                    || n.body.to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect();

        let mut entries: Vec<Button> = matches
            .iter()
            .map(|&i| {
                let title = match self.notes[i].title.trim() {
                    "" => "untitled",
                    t => t,
                };
                let b = Button::new(title).id(format!("note-{i}")).width(200.0);
                if i == self.open {
                    b.outline()
                } else {
                    b.ghost()
                }
            })
            .collect();

        let mut title = TextInput::new().id("title").placeholder("title");
        let mut body = TextArea::new().id("body");

        if self.loaded != Some(self.open) {
            if let Some(n) = self.notes.get(self.open) {
                title.set_text(ui, n.title.clone());
                body.set_text(ui, n.body.clone());
            }
            self.loaded = Some(self.open);
        }

        let words = body.text(ui).split_whitespace().count();
        let mut info = Label::new(format!("{words} words")).caption();
        let mut delete = Button::new("Delete").id("delete").danger();

        {
            let mut list = Column::new(
                entries
                    .iter_mut()
                    .map(|b| Box::new(b) as Box<dyn AnyWidget>)
                    .collect(),
            )
            .spacing(2.0);
            let mut list_scroll = ScrollView::new(&mut list).id("list").width(200.0);

            let mut list_area = Expanded::new(&mut list_scroll);
            let mut sidebar = column![&mut search, &mut new, &mut list_area]
                .spacing(10.0)
                .width(200.0);

            let mut body_area = Expanded::new(&mut body);
            let mut footer = row![&mut info, &mut delete]
                .spacing(16.0)
                .align(Alignment::Center);

            let mut editor = column![&mut title, &mut body_area, &mut footer]
                .spacing(10.0)
                .align(Alignment::Stretch);

            row![
                &mut sidebar,
                &mut Divider::vertical(0.0).faint(),
                &mut Expanded::new(&mut editor),
            ]
            .spacing(16.0)
            .padding([16.0, 16.0])
            .size(size)
            .align(Alignment::Stretch)
            .arrange_at([0.0, 0.0], ui);
        }

        if let Some(n) = self.notes.get_mut(self.open) {
            n.title = title.text(ui);
            n.body = body.text(ui);
        }

        for (b, &i) in entries.iter().zip(matches.iter()) {
            if b.clicked() {
                self.open = i;
            }
        }

        if new.clicked() {
            self.notes.push(Note {
                title: String::new(),
                body: String::new(),
            });
            self.open = self.notes.len() - 1;
            search.set_text(ui, String::new());
        }

        if delete.clicked() && !self.notes.is_empty() {
            self.notes.remove(self.open);
            if self.notes.is_empty() {
                self.notes.push(Note {
                    title: String::new(),
                    body: String::new(),
                });
            }
            self.open = self.open.min(self.notes.len() - 1);
            self.loaded = None;
        }
    }
}

fn main() {
    App::new(Notes::new())
        .title("Notes")
        .window_size(820, 560)
        .run();
}
