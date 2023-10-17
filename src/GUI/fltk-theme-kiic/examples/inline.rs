use fltk::{prelude::*, *};
use fltk_theme::{cmap, ColorMap, ColorTheme};

const MAP: &[ColorMap] = &[
    cmap!(0, 180, 180, 180),
    cmap!(1, 162, 48, 48),
    cmap!(2, 90, 180, 0),
    cmap!(3, 255, 255, 0),
    cmap!(4, 88, 120, 152),
    cmap!(5, 180, 0, 180),
    cmap!(6, 0, 180, 180),
    cmap!(7, 80, 80, 80),
    cmap!(8, 100, 100, 100),
    cmap!(9, 119, 71, 71),
    cmap!(10, 80, 130, 80),
    cmap!(11, 150, 150, 90),
    cmap!(12, 110, 123, 145),
    cmap!(13, 150, 90, 150),
    cmap!(14, 90, 150, 150),
    cmap!(15, 200, 50, 50),
    cmap!(32, 3, 3, 3),
    cmap!(33, 5, 5, 5),
    cmap!(34, 8, 8, 8),
    cmap!(35, 10, 10, 10),
    cmap!(36, 14, 14, 14),
    cmap!(37, 16, 16, 16),
    cmap!(38, 21, 21, 21),
    cmap!(39, 26, 26, 26),
    cmap!(40, 32, 32, 32),
    cmap!(41, 37, 37, 37),
    cmap!(42, 42, 42, 42),
    cmap!(43, 48, 48, 48),
    cmap!(44, 53, 53, 53),
    cmap!(45, 58, 58, 58),
    cmap!(46, 64, 64, 64),
    cmap!(47, 69, 69, 69),
    cmap!(48, 74, 74, 74),
    cmap!(49, 80, 80, 80),
    cmap!(50, 85, 85, 85),
    cmap!(51, 90, 90, 90),
    cmap!(52, 96, 96, 96),
    cmap!(53, 101, 101, 101),
    cmap!(54, 106, 106, 106),
    cmap!(55, 110, 110, 110),
    cmap!(56, 150, 150, 150),
    cmap!(59, 80, 150, 80),
    cmap!(63, 0, 180, 0),
    cmap!(71, 0, 180, 0),
    cmap!(88, 180, 0, 0),
    cmap!(90, 180, 80, 40),
    cmap!(91, 180, 120, 0),
    cmap!(94, 150, 110, 20),
    cmap!(95, 120, 120, 36),
    cmap!(124, 107, 92, 57),
    cmap!(254, 60, 70, 70),
    cmap!(255, 50, 50, 50),
];

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(MAP);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 40, "Hello");
    btn.set_color(btn.color().lighter());
    win.end();
    win.show();
    a.run().unwrap();
}
