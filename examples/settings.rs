use glacex::{
    Accordion, AccordionItem, Alignment, App, Button, Label, RadioButton, ScrollView, SelectBox,
    SelectOption, Slider, Switch, TextInput, Theme, Ui, Widget, column, row,
};

#[derive(Clone, PartialEq)]
struct Prefs {
    theme: Option<String>,
    tab_width: f32,
    spaces: bool,
    line_numbers: bool,
    wrap: bool,
    format_on_save: bool,
    name: String,
    email: String,
}

struct Settings {
    saved: Prefs,
    status: String,
}

impl Widget for Settings {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let size = ui.window_size();

        let mut theme = SelectBox::new(
            Theme::all()
                .iter()
                .map(|t| SelectOption::new(t.name, t.name.replace('-', " ")))
                .collect(),
        )
        .id("theme")
        .placeholder("pick one")
        .searchable()
        .width(220.0);
        let picked_theme = theme.selected(ui);
        if let Some(t) = picked_theme.as_deref().and_then(Theme::from_name) {
            ui.set_theme(t);
        }
        ui.set_bgcolor(ui.theme().bg_canvas);

        let mut tab_width = Slider::new(2.0, 8.0).id("tab-width").default_value(4.0);
        let tab_width_value = tab_width.value(ui).round();
        let mut tab_width_label = Label::new(format!("Tab width  {tab_width_value}"));

        let mut spaces = RadioButton::new("indent").id("spaces");
        let mut tabs = RadioButton::new("indent").id("tabs");
        if ui.selected_option("indent").is_none() {
            ui.select("indent", "spaces");
        }

        let mut line_numbers = Switch::new().id("line-numbers").default_enabled(true);
        let mut wrap = Switch::new().id("wrap");
        let mut format_on_save = Switch::new().id("fmt").default_enabled(true);

        let mut name = TextInput::new()
            .id("name")
            .placeholder("your name")
            .width(260.0);
        let mut email = TextInput::new()
            .id("email")
            .placeholder("you@example.com")
            .width(260.0);
        let mut password = TextInput::new()
            .id("password")
            .placeholder("new password")
            .secure(true)
            .width(260.0);

        let current = Prefs {
            theme: picked_theme,
            tab_width: tab_width_value,
            spaces: ui.selected_option("indent") == Some("spaces"),
            line_numbers: line_numbers.enabled(ui),
            wrap: wrap.enabled(ui),
            format_on_save: format_on_save.enabled(ui),
            name: name.text(ui),
            email: email.text(ui),
        };
        let changed = current != self.saved || !password.text(ui).is_empty();

        let mut save = Button::new("Save").id("save").primary();
        let mut revert = Button::new("Revert").id("revert").ghost();
        let mut status = Label::new(if changed {
            "unsaved changes"
        } else {
            self.status.as_str()
        })
        .caption();

        {
            let mut theme_label = Label::new("Theme");
            let mut appearance = column![&mut theme_label, &mut theme]
                .spacing(8.0)
                .align(Alignment::Start);

            let mut spaces_label = Label::new("Spaces");
            let mut tabs_label = Label::new("Tabs");
            let mut indent = row![&mut spaces, &mut spaces_label, &mut tabs, &mut tabs_label]
                .spacing(8.0)
                .align(Alignment::Center);
            let mut ln_label = Label::new("Line numbers");
            let mut wrap_label = Label::new("Wrap long lines");
            let mut fmt_label = Label::new("Format on save");
            let mut ln_row = row![&mut line_numbers, &mut ln_label]
                .spacing(10.0)
                .align(Alignment::Center);
            let mut wrap_row = row![&mut wrap, &mut wrap_label]
                .spacing(10.0)
                .align(Alignment::Center);
            let mut fmt_row = row![&mut format_on_save, &mut fmt_label]
                .spacing(10.0)
                .align(Alignment::Center);
            let mut editor = column![
                &mut tab_width_label,
                &mut tab_width,
                &mut indent,
                &mut ln_row,
                &mut wrap_row,
                &mut fmt_row,
            ]
            .spacing(10.0)
            .align(Alignment::Start);

            let mut account = column![&mut name, &mut email, &mut password]
                .spacing(10.0)
                .align(Alignment::Start);

            let mut sections = Accordion::new(vec![
                AccordionItem::new("Appearance", &mut appearance).default_open(true),
                AccordionItem::new("Editor", &mut editor),
                AccordionItem::new("Account", &mut account),
            ])
            .id("sections")
            .multiple()
            .width(400.0);

            let mut footer = row![&mut save, &mut revert, &mut status]
                .spacing(10.0)
                .align(Alignment::Center);

            let mut title = Label::new("Settings").title();
            let mut page = column![&mut title, &mut sections, &mut footer,]
                .spacing(16.0)
                .padding([36.0, 28.0])
                .align(Alignment::Start);

            ScrollView::new(&mut page)
                .id("page")
                .size(size)
                .arrange_at([0.0, 0.0], ui);
        }

        if save.clicked() {
            self.status = if password.text(ui).is_empty() {
                "saved".into()
            } else {
                "saved, password changed".into()
            };
            password.set_text(ui, String::new());
            self.saved = current;
        } else if revert.clicked() {
            let p = self.saved.clone();
            theme.set_selected(ui, p.theme);
            tab_width.set_value(ui, p.tab_width);
            ui.select("indent", if p.spaces { "spaces" } else { "tabs" });
            line_numbers.set_enabled(ui, p.line_numbers);
            wrap.set_enabled(ui, p.wrap);
            format_on_save.set_enabled(ui, p.format_on_save);
            name.set_text(ui, p.name);
            email.set_text(ui, p.email);
            password.set_text(ui, String::new());
            self.status = "reverted".into();
        }
    }
}

fn main() {
    let saved = Prefs {
        theme: None,
        tab_width: 4.0,
        spaces: true,
        line_numbers: true,
        wrap: false,
        format_on_save: true,
        name: String::new(),
        email: String::new(),
    };
    App::new(Settings {
        saved,
        status: String::new(),
    })
    .title("Settings")
    .window_size(480, 640)
    .run();
}
