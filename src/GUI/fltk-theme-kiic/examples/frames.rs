use fltk::{prelude::*, *};
use fltk_theme::{widget_themes::*, ThemeType, WidgetTheme};

fn main() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::HighContrast);
    theme.apply();
    let mut win = window::Window::default().with_size(800, 800);
    let mut choice = menu::Choice::new(300, 10, 200, 30, None);
    choice.add_choice("Classic|Aero|Metro|AquaClassic|Greybird|Blue|HighContrast|Dark");
    choice.set_value(6);
    choice.set_frame(OS_PANEL_THIN_UP_BOX);
    let mut vgrid = group::VGrid::new(50, 100, 700, 700, None);
    vgrid.set_params(6, 6, 5);
    let mut frame = frame::Frame::default().with_label(&"BUTTON_UP_BOX".to_lowercase());
    frame.set_frame(OS_BUTTON_UP_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"CHECK_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_CHECK_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"BUTTON_UP_FRAME".to_lowercase());
    frame.set_frame(OS_BUTTON_UP_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"CHECK_DOWN_FRAME".to_lowercase());
    frame.set_frame(OS_CHECK_DOWN_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"PANEL_THIN_UP_BOX".to_lowercase());
    frame.set_frame(OS_PANEL_THIN_UP_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"SPACER_THIN_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_SPACER_THIN_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"PANEL_THIN_UP_FRAME".to_lowercase());
    frame.set_frame(OS_PANEL_THIN_UP_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"SPACER_THIN_DOWN_FRAME".to_lowercase());
    frame.set_frame(OS_SPACER_THIN_DOWN_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"RADIO_ROUND_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_RADIO_ROUND_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"HOVERED_UP_BOX".to_lowercase());
    frame.set_frame(OS_HOVERED_UP_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"DEPRESSED_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_DEPRESSED_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"HOVERED_UP_FRAME".to_lowercase());
    frame.set_frame(OS_HOVERED_UP_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"DEPRESSED_DOWN_FRAME".to_lowercase());
    frame.set_frame(OS_DEPRESSED_DOWN_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"INPUT_THIN_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_INPUT_THIN_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"INPUT_THIN_DOWN_FRAME".to_lowercase());
    frame.set_frame(OS_INPUT_THIN_DOWN_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"MINI_BUTTON_UP_BOX".to_lowercase());
    frame.set_frame(OS_MINI_BUTTON_UP_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"MINI_DEPRESSED_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_MINI_DEPRESSED_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"MINI_BUTTON_UP_FRAME".to_lowercase());
    frame.set_frame(OS_MINI_BUTTON_UP_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"MINI_DEPRESSED_DOWN_FRAME".to_lowercase());
    frame.set_frame(OS_MINI_DEPRESSED_DOWN_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"DEFAULT_BUTTON_UP_BOX".to_lowercase());
    frame.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"DEFAULT_HOVERED_UP_BOX".to_lowercase());
    frame.set_frame(OS_DEFAULT_HOVERED_UP_BOX);
    frame.set_label_size(10);
    let mut frame =
        frame::Frame::default().with_label(&"DEFAULT_DEPRESSED_DOWN_BOX".to_lowercase());
    frame.set_frame(OS_DEFAULT_DEPRESSED_DOWN_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"TOOLBAR_BUTTON_HOVER_BOX".to_lowercase());
    frame.set_frame(OS_TOOLBAR_BUTTON_HOVER_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"TABS_BOX".to_lowercase());
    frame.set_frame(OS_TABS_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"SWATCH_BOX".to_lowercase());
    frame.set_frame(OS_SWATCH_BOX);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"SWATCH_FRAME".to_lowercase());
    frame.set_frame(OS_SWATCH_FRAME);
    frame.set_label_size(10);
    let mut frame = frame::Frame::default().with_label(&"BG_BOX".to_lowercase());
    frame.set_frame(OS_BG_BOX);
    frame.set_label_size(10);
    frame::Frame::default();
    frame::Frame::default();
    frame::Frame::default();
    vgrid.end();
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
            _ => unimplemented!(),
        };
        theme.apply();
    });
    a.run().unwrap();
}
