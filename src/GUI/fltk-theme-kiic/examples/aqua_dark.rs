use fltk::{enums::*, prelude::*, *};
use fltk_theme::widget_schemes::aqua::frames::*;
use fltk_theme::{SchemeType, WidgetScheme};

use fltk_theme::colors::aqua::dark::*; // get all the dark aqua colors

// use fltk_theme::colors::aqua::light::*; // get all the light aqua colors

// use fltk_theme::colors::aqua::sys::*; // get all the system aqua colors, requires MacOS

fn main() {
    let a = app::App::default();
    let bg = windowBackgroundColor.to_rgb();
    app::background(bg.0, bg.1, bg.2);
    let ctrl = controlAccentColor.to_rgb();
    app::background2(ctrl.0, ctrl.1, ctrl.2);
    let lbl = labelColor.to_rgb();
    app::foreground(lbl.0, lbl.1, lbl.2);
    app::set_color(Color::Selection, 255, 255, 255);
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 50, 200, 30, None);
    choice.set_color(*controlColor);
    choice.add_choice("Opt1|Opt2|Opt3");
    choice.set_value(2);
    let mut inp = input::Input::new(50, 100, 300, 30, None);
    inp.set_color(*controlColor);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(enums::FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(enums::FrameType::FlatBox);
    let mut btn = button::Button::new(160, 230, 80, 30, "Hello");
    btn.set_color(*controlColor);
    btn.set_selection_color(*controlAccentColor);
    btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}
