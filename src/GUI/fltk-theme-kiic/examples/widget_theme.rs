use fltk::{prelude::*, *};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};

fn main() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::AquaClassic);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Classic|Aero|Metro|AquaClassic|Greybird|Blue|HighContrast|Dark");
    choice.set_value(3);
    choice.set_frame(widget_themes::OS_PANEL_THIN_UP_BOX);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut btn = button::Button::new(160, 220, 80, 30, "Hello");
    btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
    win.end();
    win.show();
    choice.set_callback(|c| {
        let theme = match c.value() {
            0 => WidgetTheme::new(ThemeType::Classic),
            1 => WidgetTheme::new(ThemeType::Aero),
            2 => WidgetTheme::new(ThemeType::Metro),
            3 => WidgetTheme::new(ThemeType::AquaClassic),
            4 => WidgetTheme::new(ThemeType::Greybird),
            5 => WidgetTheme::new(ThemeType::Blue),
            6 => WidgetTheme::new(ThemeType::HighContrast),
            7 => WidgetTheme::new(ThemeType::Dark),
            _ => WidgetTheme::new(ThemeType::Classic),
        };
        theme.apply();
    });

    a.run().unwrap();
}
