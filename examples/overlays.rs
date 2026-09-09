use glacex::*;

struct Overlays;

impl Widget for Overlays {
    type Output = ();

    fn ui(&mut self, ui: &mut Ui) {
        let mut title =
            Typography::new("Overlays and navigation").variant(TypographyVariant::Title);
        let mut body = Typography::new("Menus, dialogs, sheets, and contextual actions.");
        let mut commands = CommandPalette::new(
            "commands",
            [
                Command::new("settings", "Open settings").hint("⌘ ,"),
                Command::new("search", "Search").hint("⌘ K"),
            ],
        );
        let mut menu = Menu::new(
            "menu",
            "Actions",
            [
                MenuItem::new("new", "New project"),
                MenuItem::new("archive", "Archive"),
            ],
        )
        .position([210.0, 124.0])
        .render_trigger(false);
        let mut dialog = Dialog::new("dialog", "Delete project", "This action cannot be undone.");
        let mut sheet = Sheet::new("sheet", "Activity", "Recent workspace activity");
        let mut popover = Popover::new("popover", "Details", "More information about this item.");
        let mut command_button = Button::new("commands-trigger", "Command palette")
            .outline()
            .tooltip("Open the command palette");
        let mut menu_button = Button::new("menu-trigger", "Menu").outline();
        let mut dialog_button = Button::new("dialog-trigger", "Dialog").outline();
        let mut sheet_button = Button::new("sheet-trigger", "Sheet").outline();
        let mut popover_button = Button::new("popover-trigger", "Popover").outline();
        let mut actions = glacex::row![
            &mut command_button,
            &mut menu_button,
            &mut dialog_button,
            &mut sheet_button,
            &mut popover_button,
        ]
        .spacing(12.0)
        .align(Alignment::Center);
        let mut layout = glacex::column![&mut title, &mut body, &mut actions]
            .spacing(18.0)
            .align(Alignment::Start);
        layout.arrange_at([48.0, 40.0], ui);
        drop(layout);
        drop(actions);

        if dialog_button.clicked() {
            let open = dialog.open(ui);
            dialog.set_open(ui, !open);
        }
        if sheet_button.clicked() {
            let open = sheet.open(ui);
            sheet.set_open(ui, !open);
        }
        if popover_button.clicked() {
            let open = popover.open(ui);
            popover.set_open(ui, !open);
        }

        commands.ui(ui);
        if command_button.clicked() {
            let open = commands.open(ui);
            commands.set_open(ui, !open);
        }
        let menu_clicked = menu_button.clicked();
        let menu_was_open = menu.open(ui);
        menu.ui(ui);
        if menu_clicked {
            menu.set_open(ui, !menu_was_open);
        }
        dialog.ui(ui);
        sheet.ui(ui);
        popover.ui(ui);
    }
}

fn main() {
    App::new(Overlays)
        .title("Glacex — Overlays")
        .window_size(1000, 500)
        .run();
}
