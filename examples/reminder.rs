//! reminder -- a real todo app, and a lesson in writing your own widget.
//!
//! what you will learn here:
//!   1. how to build a custom widget (`Todo`: `Measurable` + `Widget`)
//!   2. how to show a dynamic list (`Vec` → `Column::new` → `ScrollView` → `Card`)
//!   3. the borrow dance: layout borrows your widgets, so mutate `self` AFTER layout
//!   4. `TextInput::text(ui)` / `set_text()` for reading and clearing a field
//!
//! run it: `cargo run --example reminder`

use glacex::*;

// ---------------------------------------------------------------------------
// part 1: the custom widget.
// a todo row = a label + a delete button. it composes built-in widgets
// instead of drawing anything itself -- most custom widgets work this way.
// ---------------------------------------------------------------------------

// the answer a Todo gives every frame: "was i deleted?"
struct TodoResponse {
    _deleted: bool,
}

struct Todo {
    text: String,
    deleted: bool, // recorded during arrange, read by the parent afterwards
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
    // every layout asks its children "how big are you?" -- answer honestly.
    fn measure(&mut self, _ui: &mut Ui) -> [f32; 2] {
        [300.0, 40.0]
    }

    // ...then tells them "draw yourself at THIS position, THIS size."
    fn arrange(&mut self, pos: [f32; 2], size: [f32; 2], ui: &mut Ui) -> TodoResponse {
        // children are rebuilt every frame. their STATE persists in Ui
        // under their ids -- building is cheap, remembering is Ui's job.
        // NOTE: ids must be unique per todo, or two rows share one memory.
        // (here we key on the text; a real app would use a unique todo id.)
        let mut button = Button::new("Delete").id(format!("delete-{}", self.text));
        let mut label = Label::new(&self.text);
        let mut content = row![&mut label, &mut button]
            .size(size)
            .align(Alignment::Center);
        content.arrange_at(pos, ui);
        drop(content); // end the layout's borrows before touching our own fields

        let clicked = button.clicked();
        self.deleted = clicked;

        TodoResponse { _deleted: clicked }
    }
}

impl Widget for Todo {
    type Output = TodoResponse;

    // standalone drawing (used if someone calls todo.ui(ui) directly).
    fn ui(&mut self, ui: &mut Ui) -> TodoResponse {
        let size = self.measure(ui);
        self.arrange([0.0, 0.0], size, ui)
    }
}

// ---------------------------------------------------------------------------
// part 2: the app. owns the todo list (YOUR data) and lays out everything.
// ---------------------------------------------------------------------------

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
            Label::new("Add something. Delete something. Live a little.").caption();

        // the input row: a text field plus a chunky "+" button.
        // stable ids, as always, because both remember things.
        let mut text_input = TextInput::new().id("text_input").size([300.0, 40.0]);
        let mut add_button = Button::new("+")
            .id("add_button")
            .primary()
            .size([40.0, 40.0]);

        let mut header = row![&mut text_input, &mut add_button,];

        // -- the dynamic list ------------------------------------------------
        // rebuild one Todo per string, every frame. then box them as trait
        // objects, because a Column holding N widgets of one type needs
        // `Vec<Box<dyn AnyWidget>>` -- the "dynamic list" incantation.
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
                    // a transparent 20px divider: the classy way to add blank space
                    &mut Divider::horizontal(0.0)
                        .height(20.0)
                        .color(Color::TRANSPARENT),
                    &mut header,
                    // list-with-scrolling, the standard nesting:
                    // Card > ScrollView > Column > your rows
                    &mut Card::new(
                        &mut ScrollView::new(&mut Column::new(todos))
                            .size([310.0, 400.0])
                            .id("scroll_view") // scroll offset is state → needs an id
                    )
                ],]
                .align(Alignment::Center)
                .height(window_size[1])
            ]
            .align(Alignment::Center)
            .size(window_size) // claim the whole window, then center inside it
            .arrange_at([0.0, 0.0], ui);
        }

        drop(header); // release the layout's borrows so we may mutate self below.
        // (without this, `self.todos.retain` would fight an outstanding &mut.)

        // -- react: AFTER layout, mutate your own data -----------------------
        // which rows got deleted? their buttons recorded it during arrange.
        for todo in &todo_widgets {
            if todo.deleted {
                self.todos.retain(|t| t != &todo.text);
            }
        }

        // add button: take the input's text, clear the field, push the todo.
        if add_button.clicked() {
            let text = text_input.text(ui);
            text_input.set_text(ui, String::new());
            if !text.is_empty() {
                self.todos.push(text);
            }
        }
    }
}

fn main() {
    App::new(ReminderApp::default()).run();
}
