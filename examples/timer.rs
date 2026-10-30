use glacex::{
    Alignment, AnyWidget, App, Button, Card, Column, Divider, Label, ProgressBar, Slider, Switch,
    Ui, Widget, column, row,
};

struct Timer {
    length: f32,
    left: f32,
    running: bool,
    finished: Vec<String>,
}

impl Timer {
    fn new() -> Self {
        Timer {
            length: 300.0,
            left: 300.0,
            running: false,
            finished: Vec::new(),
        }
    }
}

fn clock(seconds: f32) -> String {
    let s = seconds.ceil() as u32;
    format!("{:02}:{:02}", s / 60, s % 60)
}

impl Widget for Timer {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        if self.running {
            self.left -= ui.dt();
        }

        let mut repeat = Switch::new().id("repeat");
        if self.left <= 0.0 {
            self.finished
                .insert(0, format!("{} timer done", clock(self.length)));
            self.left = self.length;
            self.running = repeat.enabled(ui);
        }

        let mut minutes = Slider::new(1.0, 60.0)
            .id("minutes")
            .default_value(5.0)
            .width(280.0);
        let picked = minutes.value(ui).round() * 60.0;
        if !self.running && picked != self.length {
            self.length = picked;
            self.left = picked;
        }

        let mut tea = Button::new("Tea 3m").id("tea").outline();
        let mut eggs = Button::new("Eggs 7m").id("eggs").outline();
        let mut pasta = Button::new("Pasta 10m").id("pasta").outline();

        let mut time = Label::new(clock(self.left)).metric().mono();
        let mut length = Label::new(format!("of {}", clock(self.length))).muted();
        let mut bar = ProgressBar::new(1.0 - self.left / self.length)
            .id("bar")
            .width(280.0);

        let mut start = Button::new(if self.running { "Pause" } else { "Start" })
            .id("start")
            .primary()
            .width(136.0);
        let mut reset = Button::new("Reset").id("reset").width(136.0);

        let mut log: Vec<Label> = self
            .finished
            .iter()
            .take(5)
            .map(|s| Label::new(s).caption())
            .collect();
        let mut nothing = Label::new("nothing finished yet").caption();

        {
            let mut history = if log.is_empty() {
                column![&mut nothing]
            } else {
                Column::new(
                    log.iter_mut()
                        .map(|l| Box::new(l) as Box<dyn AnyWidget>)
                        .collect(),
                )
                .spacing(4.0)
            };

            let mut buttons = row![&mut start, &mut reset].spacing(8.0);
            let mut face = column![&mut time, &mut length, &mut bar, &mut buttons,]
                .spacing(10.0)
                .align(Alignment::Center);

            column![
                &mut Card::new(&mut face).padding([24.0, 24.0]),
                &mut Label::new("Minutes").secondary(),
                &mut minutes,
                &mut row![&mut tea, &mut eggs, &mut pasta].spacing(8.0),
                &mut row![&mut repeat, &mut Label::new("start again when it rings")]
                    .spacing(8.0)
                    .align(Alignment::Center),
                &mut Divider::horizontal(328.0).faint(),
                &mut history,
            ]
            .spacing(12.0)
            .align(Alignment::Start)
            .arrange_at([36.0, 36.0], ui);
        }

        if start.clicked() {
            self.running = !self.running;
        }
        if reset.clicked() {
            self.running = false;
            self.left = self.length;
        }
        for (button, m) in [(&tea, 3.0), (&eggs, 7.0), (&pasta, 10.0)] {
            if button.clicked() {
                minutes.set_value(ui, m);
                self.running = false;
                self.length = m * 60.0;
                self.left = self.length;
            }
        }
    }
}

fn main() {
    App::new(Timer::new())
        .title("Timer")
        .window_size(400, 640)
        .run();
}
