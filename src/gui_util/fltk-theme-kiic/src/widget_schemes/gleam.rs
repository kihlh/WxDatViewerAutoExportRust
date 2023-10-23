use super::*;

fn frame_rect(x: i32, y: i32, w: i32, h: i32, bc: Color) {
    // Draw the outline around the perimeter of the box
    set_draw_color(Color::color_average(Color::Black, Color::Background, 0.1));
    draw_line(x, y, x + w, y);
    draw_line(x + w, y, x + w, y + h);
    draw_line(x + w, y + h, x, y + h);
    draw_line(x, y + h, x, y);
}

fn shade_rect_up(x: i32, y: i32, w: i32, h: i32, bc: Color) {
    // Draws the shiny
    let third = h as f32 / 3.;
    set_draw_color(activated_color(bc));
    draw_rectf(x, y, w, third as i32 + 1);

    let step_size = 0.10 / (h as f32 - third);
    let mut j = 0;

    let mut k = 1.0;
    loop {
        if k >= 0.90 {
            break;
        }
        j += 1;
        set_draw_color(activated_color(Color::color_average(bc, Color::White, k)));
        draw_line(
            x,
            y + j + third as i32 - 1,
            x + w - 1,
            y + j + third as i32 - 1,
        );
        k -= step_size;
    }
}

fn frame_rect_up(x: i32, y: i32, w: i32, h: i32, bc: Color) {
    // Draw the outline around the perimeter of the box
    set_draw_color(activated_color(bc));
    draw_line(x, y, x + w, y); //Go across.
    draw_line(x, y + (h / 2), x, y + 1); //Go to top
    draw_line(x + w, y + (h / 2), x + w, y + 1); //Go to top

    set_draw_color(activated_color(bc.darker()));
    draw_line(x, y + h, x + w, y + h); //Go across again!
    draw_line(x, y + (h / 2), x, y + h - 1); //Go to top
    draw_line(x + w, y + (h / 2), x + w, y + h - 1); //Go to top
}

fn frame_rect_down(x: i32, y: i32, w: i32, h: i32, bc: Color) {
    // Draw the outline around the perimeter of the box
    set_draw_color(activated_color(bc.darker()));
    draw_line(x, y, x + w, y); //Go across.
    draw_line(x, y + (h / 2), x, y + 1); //Go to top
    draw_line(x + w, y + (h / 2), x + w, y + 1); //Go to top

    //set_draw_color(activated_color(bc));
    draw_line(x, y + h, x + w, y + h); //Go across again!
    draw_line(x, y + (h / 2), x, y + h - 1); //Go to top
    draw_line(x + w, y + (h / 2), x + w, y + h - 1); //Go to top
}

fn shade_rect_down(x: i32, y: i32, w: i32, h: i32, bc: Color) {
    set_draw_color(activated_color(bc));
    let color = get_color();
    draw_rectf(x, y, w, h);
    set_draw_color(activated_color(Color::color_average(
        bc,
        color.darker(),
        0.65,
    )));
    draw_line(x, y + 1, x + w, y + 1);
    draw_line(x, y + 1, x, y + h - 2);
    set_draw_color(activated_color(Color::color_average(
        bc,
        color.darker(),
        0.85,
    )));
    draw_line(x + 1, y + 2, x + w, y + 2);
    draw_line(x + 1, y + 2, x + 1, y + h - 2);
}

fn up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    frame_rect_up(x, y, w - 1, h - 1, c.darker());
}

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    shade_rect_up(x + 1, y, w - 2, h - 1, c);
    frame_rect_up(x, y, w - 1, h - 1, c.darker());
    //draw the inner rect.
    frame_rect(
        x + 1,
        y + 1,
        w - 3,
        h - 3,
        Color::color_average(c, Color::White, 0.25),
    );
}

fn down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    frame_rect_down(x, y, w - 1, h - 1, c.darker());
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    shade_rect_down(x + 1, y, w - 2, h, c);
    down_frame(x, y, w, h, c.darker());
    //draw the inner rect.
    //frame_rect(x + 1, y + 1, w - 3, h - 3, Color::color_average(c, Color::Black, .65));
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(activated_color(c));
    draw_rectf(x, y, w, h);
    set_draw_color(activated_color(c.darker()));
    draw_rect(x + 1, y + 1, w - 2, h - 2);
}

pub(crate) fn use_gleam_scheme() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Gleam);
    app::set_visible_focus(false);
    app::set_frame_type_cb(UpBox, up_box, 2, 2, 4, 4);
    app::set_frame_type_cb(DownBox, down_box, 2, 2, 3, 3);
    app::set_frame_type_cb(ThinUpBox, up_box, 2, 2, 3, 3);
    app::set_frame_type_cb(ThinDownBox, down_box, 2, 2, 3, 3);
    app::set_frame_type_cb(UpFrame, up_frame, 2, 2, 3, 3);
    app::set_frame_type_cb(DownFrame, down_frame, 2, 2, 3, 3);
    app::set_frame_type_cb(RoundUpBox, up_box, 2, 2, 3, 3);
    app::set_frame_type_cb(RoundDownBox, down_box, 2, 2, 3, 3);
    app::set_frame_type_cb(BorderBox, border_box, 1, 1, 2, 2);
}
