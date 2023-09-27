use fltk::{prelude::*, *};
use fltk_theme::{cmap, ColorMap, ColorTheme};

fn main() {
    let mut map = vec![];
    map.push(cmap!(32, 0, 0, 0));
    let mut c = 33;
    let mut r = 114;
    let mut g = 100;
    let mut b = 46;
    loop {
        if c == 55 {
            break;
        }
        map.push(cmap!(c, r, g, b));
        c += 1;
        r += 6;
        g += 6;
        b += 9;
    }
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(&map);
    theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    button::Button::new(160, 200, 80, 40, "Hello");
    win.end();
    win.show();
    a.run().unwrap();
}
