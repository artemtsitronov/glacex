use glacex::{
    Alignment, App, Button, Card, Label, RadioButton, SelectBox, SelectOption, TextInput, Ui,
    Widget, column, row,
};

struct Unit {
    name: &'static str,
    to_base: fn(f64) -> f64,
    from_base: fn(f64) -> f64,
}

macro_rules! linear {
    ($name:expr, $factor:expr) => {
        Unit {
            name: $name,
            to_base: |v| v * $factor,
            from_base: |v| v / $factor,
        }
    };
}

fn units(kind: &str) -> Vec<Unit> {
    match kind {
        "weight" => vec![
            linear!("grams", 1.0),
            linear!("kilograms", 1000.0),
            linear!("pounds", 453.592),
            linear!("ounces", 28.3495),
        ],
        "temperature" => vec![
            Unit {
                name: "celsius",
                to_base: |v| v,
                from_base: |v| v,
            },
            Unit {
                name: "fahrenheit",
                to_base: |v| (v - 32.0) * 5.0 / 9.0,
                from_base: |v| v * 9.0 / 5.0 + 32.0,
            },
            Unit {
                name: "kelvin",
                to_base: |v| v - 273.15,
                from_base: |v| v + 273.15,
            },
        ],
        _ => vec![
            linear!("meters", 1.0),
            linear!("kilometers", 1000.0),
            linear!("centimeters", 0.01),
            linear!("feet", 0.3048),
            linear!("inches", 0.0254),
            linear!("miles", 1609.344),
        ],
    }
}

fn pretty(v: f64) -> String {
    let s = format!("{v:.4}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

struct Converter;

impl Widget for Converter {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        ui.set_bgcolor(ui.theme().bg_canvas);

        let kind = ui.selected_option("kind").unwrap_or("length").to_string();
        let list = units(&kind);

        let mut kinds: Vec<(RadioButton, Label)> = ["length", "weight", "temperature"]
            .iter()
            .map(|k| (RadioButton::new("kind").id(*k), Label::new(*k)))
            .collect();
        if ui.selected_option("kind").is_none() {
            ui.select("kind", "length");
        }

        let options = || -> Vec<SelectOption> {
            list.iter()
                .map(|u| SelectOption::new(u.name, u.name))
                .collect()
        };
        let mut from = SelectBox::new(options())
            .id(format!("from-{kind}"))
            .width(160.0)
            .placeholder(list[0].name);
        let mut to = SelectBox::new(options())
            .id(format!("to-{kind}"))
            .width(160.0)
            .placeholder(list[1].name);

        let from_name = from.selected(ui).unwrap_or(list[0].name.to_string());
        let to_name = to.selected(ui).unwrap_or(list[1].name.to_string());
        let find = |n: &str| list.iter().find(|u| u.name == n).unwrap_or(&list[0]);
        let (a, b) = (find(&from_name), find(&to_name));

        let mut amount = TextInput::new().id("amount").default_text("1").width(160.0);
        let raw = amount.text(ui);

        let mut result = match raw.trim().replace(',', ".").parse::<f64>() {
            Ok(v) => Label::new(format!(
                "{} {} is {} {}",
                pretty(v),
                a.name,
                pretty((b.from_base)((a.to_base)(v))),
                b.name
            ))
            .subheading(),
            Err(_) if raw.trim().is_empty() => Label::new("type a number").muted(),
            Err(_) => Label::new("that's not a number").error(),
        };

        let mut swap = Button::new("Swap").id("swap").outline();

        {
            let mut kind_rows: Vec<_> = kinds
                .iter_mut()
                .map(|(r, l)| row![r, l].spacing(6.0).align(Alignment::Center))
                .collect();
            let [k0, k1, k2] = &mut kind_rows[..] else {
                unreachable!()
            };
            let mut kind_row = row![k0, k1, k2].spacing(20.0);

            let mut to_label = Label::new("to").muted();
            let mut units_row = row![&mut from, &mut to_label, &mut to, &mut swap]
                .spacing(10.0)
                .align(Alignment::Center);
            let mut body = column![&mut amount, &mut units_row, &mut result]
                .spacing(14.0)
                .align(Alignment::Start);

            column![
                &mut Label::new("Convert").title(),
                &mut kind_row,
                &mut Card::new(&mut body).padding([20.0, 20.0]),
            ]
            .spacing(16.0)
            .align(Alignment::Start)
            .arrange_at([36.0, 36.0], ui);
        }

        if swap.clicked() {
            from.set_selected(ui, Some(to_name));
            to.set_selected(ui, Some(from_name));
        }
    }
}

fn main() {
    App::new(Converter)
        .title("Convert")
        .window_size(620, 360)
        .run();
}
