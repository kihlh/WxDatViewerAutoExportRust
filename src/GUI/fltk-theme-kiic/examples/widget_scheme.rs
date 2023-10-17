use fltk::{prelude::*, *};
use fltk_theme::{SchemeType, WidgetScheme};

fn main() {
    let a = app::App::default();
    let scheme = WidgetScheme::new(SchemeType::Clean);
    scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Clean|Crystal|Gleam");
    choice.set_value(3);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "Check");
    check.set_value(true);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "Round");
    round.set_value(true);
    let mut _btn = button::Button::new(160, 220, 80, 30, "Hello");
    win.end();
    win.show();
    choice.set_callback(|c| {
        let scheme = match c.value() {
            0 => WidgetScheme::new(SchemeType::Clean),
            1 => WidgetScheme::new(SchemeType::Crystal),
            2 => WidgetScheme::new(SchemeType::Gleam),
            _ => unimplemented!(),
        };
        scheme.apply();
    });
    a.run().unwrap();
}
