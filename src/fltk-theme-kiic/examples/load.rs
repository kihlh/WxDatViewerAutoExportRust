use fltk::{prelude::*, *};
use fltk_theme::{ColorMap, ColorTheme};

fn load_colormap(file: &str) -> ColorTheme {
    let buf = std::fs::read_to_string(file).unwrap();
    let mut vec: Vec<ColorMap> = vec![];
    for line in buf.lines() {
        let line = line.trim_start();
        if line.starts_with("cmap") {
            let map: Vec<&str> = line.split_whitespace().collect();
            let cmap = ColorMap {
                index: map[1].parse().expect("Parse Error!"),
                r: map[2].parse().expect("Parse Error!"),
                g: map[3].parse().expect("Parse Error!"),
                b: map[4].parse().expect("Parse Error!"),
            };
            vec.push(cmap);
        }
    }
    ColorTheme(vec)
}

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let black_theme = load_colormap("examples/themes/black.map");
    black_theme.apply();
    let mut win = window::Window::default().with_size(400, 300);
    let mut btn = button::Button::new(160, 200, 80, 40, "Hello");
    btn.set_color(btn.color().lighter());
    win.end();
    win.show();
    a.run().unwrap();
}
