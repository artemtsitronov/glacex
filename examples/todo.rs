use glacex::{
    Alignment, AnyWidget, App, Button, Checkbox, Column, Divider, Label, Row, ScrollView, TabItem,
    Tabs, TextInput, Ui, Widget, column, row,
};
use winit::keyboard::{Key, NamedKey};

struct Item {
    id: u64,
    text: String,
    done: bool,
}

struct Todo {
    items: Vec<Item>,
    next_id: u64,
}

impl Todo {
    fn new() -> Self {
        let mut todo = Todo {
            items: Vec::new(),
            next_id: 0,
        };
        todo.add("buy milk");
        todo.add("fix the bike");
        todo.add("call mom");
        todo
    }

    fn add(&mut self, text: &str) {
        self.items.push(Item {
            id: self.next_id,
            text: text.to_string(),
            done: false,
        });
        self.next_id += 1;
    }
}

impl Widget for Todo {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = ui.window_size();

        let mut input = TextInput::new()
            .id("new")
            .placeholder("what needs doing?")
            .width(300.0);
        let mut add = Button::new("Add").id("add").primary();

        let mut filter = Tabs::new(vec![
            TabItem::new("All").id("all"),
            TabItem::new("Open").id("open"),
            TabItem::new("Done").id("done"),
        ])
        .id("filter");
        let shown = filter.selected(ui).unwrap_or("all".into());

        let visible: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, it)| match shown.as_str() {
                "open" => !it.done,
                "done" => it.done,
                _ => true,
            })
            .map(|(i, _)| i)
            .collect();

        let mut checks: Vec<Checkbox> = visible
            .iter()
            .map(|&i| {
                let it = &self.items[i];
                Checkbox::new()
                    .id(format!("check-{}", it.id))
                    .default_checked(it.done)
            })
            .collect();
        let mut texts: Vec<Label> = visible
            .iter()
            .map(|&i| {
                let it = &self.items[i];
                let label = Label::new(&it.text).width(250.0);
                if it.done { label.muted() } else { label }
            })
            .collect();
        let mut removes: Vec<Button> = visible
            .iter()
            .map(|&i| {
                Button::new("x")
                    .id(format!("rm-{}", self.items[i].id))
                    .ghost()
            })
            .collect();

        let left = self.items.iter().filter(|it| !it.done).count();
        let mut count = Label::new(match left {
            0 => "nothing left".to_string(),
            1 => "1 thing left".to_string(),
            n => format!("{n} things left"),
        })
        .caption();
        let mut clear = Button::new("Clear done").id("clear").ghost();

        {
            let mut rows: Vec<Row> = checks
                .iter_mut()
                .zip(texts.iter_mut())
                .zip(removes.iter_mut())
                .map(|((c, t), r)| row![c, t, r].spacing(8.0).align(Alignment::Center))
                .collect();
            let mut empty = Label::new("empty").muted();
            let mut list = if rows.is_empty() {
                column![&mut empty]
            } else {
                Column::new(
                    rows.iter_mut()
                        .map(|r| Box::new(r) as Box<dyn AnyWidget>)
                        .collect(),
                )
                .spacing(4.0)
            };

            column![
                &mut Label::new("Todo").title(),
                &mut row![&mut input, &mut add].spacing(8.0),
                &mut filter,
                &mut ScrollView::new(&mut list).id("list").size([370.0, 320.0]),
                &mut Divider::horizontal(370.0).faint(),
                &mut row![&mut count, &mut clear]
                    .spacing(150.0)
                    .align(Alignment::Center),
            ]
            .spacing(12.0)
            .padding([40.0, 40.0])
            .size(size)
            .align(Alignment::Start)
            .arrange_at([0.0, 0.0], ui);
        }

        for (k, &i) in visible.iter().enumerate() {
            self.items[i].done = checks[k].is_checked(ui);
        }

        let removed: Vec<u64> = visible
            .iter()
            .zip(removes.iter())
            .filter(|(_, b)| b.clicked())
            .map(|(&i, _)| self.items[i].id)
            .collect();
        self.items.retain(|it| !removed.contains(&it.id));

        if clear.clicked() {
            self.items.retain(|it| !it.done);
        }

        let enter = input.focused(ui) && ui.key_pressed(Key::Named(NamedKey::Enter));
        if add.clicked() || enter {
            let text = input.text(ui);
            if !text.trim().is_empty() {
                self.add(text.trim());
                input.set_text(ui, String::new());
            }
        }
    }
}

fn main() {
    App::new(Todo::new())
        .title("Todo")
        .window_size(450, 600)
        .run();
}
