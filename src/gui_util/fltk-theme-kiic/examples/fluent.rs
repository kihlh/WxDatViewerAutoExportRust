use fltk::{enums::*, prelude::*, *};
use fltk_theme::widget_schemes::fluent::colors::*;
use fltk_theme::widget_schemes::fluent::frames::*;
use fltk_theme::{SchemeType, WidgetScheme};

fn main() {
    let a = app::App::default();
    app::background(0xFC, 0xFC, 0xFC);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(
        Color::Selection,
        SELECTION_COLOR.0,
        SELECTION_COLOR.1,
        SELECTION_COLOR.2,
    );
    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();
    let mut win = window::SingleWindow::default().with_size(400, 300);
    let mut choice = menu::Choice::new(100, 100, 200, 30, None);
    choice.add_choice("Fluent");
    choice.set_value(0);
    choice.set_frame(FrameType::FlatBox);
    let mut check = button::CheckButton::new(160, 150, 80, 30, "  Check");
    check.set_value(true);
    check.set_frame(FrameType::FlatBox);
    let mut round = button::RoundButton::new(160, 180, 80, 30, "  Round");
    round.set_value(true);
    round.set_frame(FrameType::FlatBox);
    let mut toggle = button::ToggleButton::new(100, 220, 80, 30, "Toggle");
    toggle.set_color(Color::from_rgba_tuple(ACCENT_COLOR));
    toggle.set_label_color(Color::White);
    toggle.set_selection_color(toggle.color().darker());
    let mut btn = button::Button::new(220, 220, 80, 30, "Hello");
    btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    btn.set_down_frame(OS_DEFAULT_DEPRESSED_DOWN_BOX);
    // handle hover
    btn.handle(|b, ev| match ev {
        Event::Enter => {
            b.set_frame(OS_HOVERED_UP_BOX);
            b.redraw();
            true
        }
        Event::Leave => {
            b.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
            b.redraw();
            true
        }
        _ => false,
    });
    win.end();
    win.make_resizable(true);
    win.show();
    a.run().unwrap();
}
