use glacex::*;

struct TodoResponse {
    delete_button_clicked: bool,
}

struct Todo {
    text: String,
    deleted: bool,
}

impl Todo {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            deleted: false,
        }
    }
}

impl Measurable for Todo {
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [300.0, 40.0]
    }

    fn arrange(&mut self, pos: [f32; 2], size: [f32; 2], ui: &mut Ui) -> TodoResponse {
        let mut button = Button::new("Delete").id(&self.text);
        let mut label = Label::new(&self.text);
        let mut content = row![&mut label, &mut button]
            .size(size)
            .align(Alignment::Center);
        content.arrange_at(pos, ui);
        drop(content);

        let clicked = button.clicked();
        self.deleted = clicked;

        TodoResponse {
            delete_button_clicked: clicked,
        }
    }
}

impl Widget for Todo {
    type Output = TodoResponse;

    fn ui(&mut self, ui: &mut Ui) -> TodoResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

struct ReminderApp {
    todos: Vec<String>,
}

impl Default for ReminderApp {
    fn default() -> Self {
        Self {
            todos: vec![String::from("First todo")],
        }
    }
}

impl Widget for ReminderApp {
    type Output = ();
    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(Theme::BG_CANVAS);

        let window_size = ui.window_size();

        let mut heading = Label::new("Simple Reminder app").heading();
        let mut subheading =
            Label::new("This app is made with Glacex, and it's meant for showcase").caption();

        let mut text_input = TextInput::new().id("text_input").size([300.0, 40.0]);
        let mut add_button = Button::new("+")
            .id("add_button")
            .primary()
            .size([40.0, 40.0]);

        let mut header = row![&mut text_input, &mut add_button,];

        let mut todo_widgets: Vec<Todo> = self.todos.iter().map(|t| Todo::new(t)).collect();
        let todos: Vec<Box<dyn AnyWidget + '_>> = todo_widgets
            .iter_mut()
            .map(|t| Box::new(t) as Box<dyn AnyWidget>)
            .collect();

        {
            glacex::column![
                &mut row![&mut glacex::column![
                    &mut heading,
                    &mut subheading,
                    &mut Divider::horizontal(0.0) // creates a larger gap this way
                        .height(20.0)
                        .color(Color::TRANSPARENT),
                    &mut header,
                    &mut Card::new(
                        &mut ScrollView::new(&mut Column::new(todos))
                            .size([310.0, 400.0])
                            .id("scroll_view")
                    )
                ],]
                .align(Alignment::Center)
                .height(window_size[1])
            ]
            .align(Alignment::Center)
            .size(window_size)
            .arrange_at([0.0, 0.0], ui);
        }

        drop(header);

        for todo in &todo_widgets {
            if todo.deleted {
                self.todos.retain(|t| t != &todo.text);
            }
        }

        if add_button.clicked() {
            let text = text_input.text(ui);
            text_input.set_text(ui, String::new());
            if !text.clone().is_empty() {
                self.todos.push(text);
            }
        }
    }
}

fn main() {
    App::new(ReminderApp::default()).run();
}
