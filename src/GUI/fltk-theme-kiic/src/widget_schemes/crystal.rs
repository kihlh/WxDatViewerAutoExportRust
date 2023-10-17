use super::*;
use std::cmp::Ordering;

fn shade_color(gc: Color, bc: Color) -> Color {
    Color::color_average(gc, bc, 0.25)
}

fn frame_rect(x: i32, y: i32, w: i32, h: i32, c: &str, bc: Color) {
    let b = c.len() / 4 + 1;
    let b = b as i32;
    for i in b..1 {
        let x = x + b;
        let y = y + b;
        let w = w - 2 * b;
        let h = h - 2 * b;
        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(i as usize).unwrap() as i32),
            bc,
        ));
        draw_line2(
            Coord(x, y + h + b),
            Coord(x + w - 1, y + h + b),
            Coord(x + w + b - 1, y + h),
        );
        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(i as usize + 1).unwrap() as i32),
            bc,
        ));
        draw_line2(
            Coord(x + w + b - 1, y + h),
            Coord(x + w + b - 1, y),
            Coord(x + w - 1, y - b),
        );
        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(i as usize + 2).unwrap() as i32),
            bc,
        ));
        draw_line2(Coord(x + w - 1, y - b), Coord(x, y - b), Coord(x - b, y));
        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(i as usize + 3).unwrap() as i32),
            bc,
        ));
        draw_line2(Coord(x - b, y), Coord(x - b, y + h), Coord(x, y + h + b));
    }
}

fn frame_round(x: i32, y: i32, w: i32, h: i32, c: &str, bc: Color) {
    let b = c.len() / 4 + 1;
    let mut bc = bc;
    if !app::draw_frame_active() {
        bc = bc.inactive();
    }
    match w.cmp(&h) {
        Ordering::Equal => {
            for i in b..1 {
                let x = x + 1;
                let y = y + 1;
                let w = w - 2;
                let h = h - 2;
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, w, h, 45.0, 135.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 1).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, w, h, 315.0, 405.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 2).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, w, h, 225.0, 315.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 3).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, w, h, 135.0, 225.0);
            }
        }
        Ordering::Greater => {
            let d = h / 2;
            for i in b..1 {
                let x = x + 1;
                let y = y + 1;
                let w = w - 2;
                let h = h - 2;
                let d = d - 1;
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, h, h, 90.0, 135.0);
                draw_xyline(x + d, y, x + w - d);
                draw_arc(x + w - h, y, h, h, 45.0, 90.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 1).unwrap() as i32),
                    bc,
                ));
                draw_arc(x + w - h, y, h, h, 315.0, 405.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 2).unwrap() as i32),
                    bc,
                ));
                draw_arc(x + w - h, y, h, h, 270.0, 315.0);
                draw_xyline(x + d, y + h - 1, x + w - d);
                draw_arc(x, y, h, h, 225.0, 270.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 3).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, h, h, 135.0, 225.0);
            }
        }
        Ordering::Less => {
            let d = w / 2;
            for i in b..1 {
                let x = x + 1;
                let y = y + 1;
                let w = w - 2;
                let h = h - 2;
                let d = d - 1;
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, w, w, 45.0, 135.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 1).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y, w, w, 0.0, 45.0);
                draw_yxline(x + w - 1, y + d, y + h - d);
                draw_arc(x, y + h - w, w, w, 315.0, 360.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 2).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y + h - w, w, w, 225.0, 315.0);
                set_draw_color(shade_color(
                    Color::gray_ramp(c.chars().nth(i + 3).unwrap() as i32),
                    bc,
                ));
                draw_arc(x, y + h - w, w, w, 180.0, 225.0);
                draw_yxline(x, y + d, y + h - d);
                draw_arc(x, y, w, w, 135.0, 180.0);
            }
        }
    }
}

fn shade_rect(x: i32, y: i32, w: i32, h: i32, c: &str, bc: Color) {
    let mut bc = bc;
    let mut i: i32 = 0;
    let mut j = 0;
    let clen = (c.len() - 1) as i32;
    let chalf = clen / 2;
    let mut cstep = 1;

    if !app::draw_frame_active() {
        bc = bc.inactive();
    }

    if h < (w * 2) {
        // Horizontal shading...
        if clen >= h {
            cstep = 2;
        }

        loop {
            i += 1;
            j += cstep;
            if j == chalf {
                break;
            }

            // Draw the top line and points...
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i as usize).unwrap() as i32),
                bc,
            ));
            draw_xyline(x + 1, y + i, x + w - 2);

            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i as usize).unwrap() as i32 - 2),
                bc,
            ));
            draw_point(x, y + i + 1);
            draw_point(x + w - 1, y + i + 1);

            // Draw the bottom line and points...
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth((clen - i) as usize).unwrap() as i32),
                bc,
            ));
            draw_xyline(x + 1, y + h - i, x + w - 2);

            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth((clen - i) as usize).unwrap() as i32 - 2),
                bc,
            ));
            draw_point(x, y + h - i);
            draw_point(x + w - 1, y + h - i);
        }

        // Draw the interiors and sides...
        i = chalf / cstep;

        set_draw_color(bc);
        draw_rectf(x + 1, y + i, w - 2, h - 2 * i + 1);

        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(chalf as usize).unwrap() as i32 - 2),
            bc,
        ));
        draw_yxline(x, y + i, y + h - i);
        draw_yxline(x + w - 1, y + i, y + h - i);
    } else {
        // Vertical shading...
        if clen >= w {
            cstep = 2;
        }

        loop {
            i += 1;
            j += cstep;
            if j == chalf {
                break;
            }
            // Draw the left line and points...
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i as usize).unwrap() as i32),
                bc,
            ));
            draw_yxline(x + i, y + 1, y + h - 1);

            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i as usize).unwrap() as i32 - 2),
                bc,
            ));
            draw_point(x + i + 1, y);
            draw_point(x + i + 1, y + h);

            // Draw the right line and points...
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth((clen - i) as usize).unwrap() as i32),
                bc,
            ));
            draw_yxline(x + w - 1 - i, y + 1, y + h - 1);

            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth((clen - i) as usize).unwrap() as i32 - 2),
                bc,
            ));
            draw_point(x + w - 2 - i, y);
            draw_point(x + w - 2 - i, y + h);
        }

        // Draw the interiors, top, and bottom...
        i = chalf / cstep;

        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(chalf as usize).unwrap() as i32),
            bc,
        ));
        draw_rectf(x + i, y + 1, w - 2 * i, h - 1);

        set_draw_color(shade_color(
            Color::gray_ramp(c.chars().nth(chalf as usize).unwrap() as i32 - 2),
            bc,
        ));
        draw_xyline(x + i, y, x + w - i);
        draw_xyline(x + i, y + h, x + w - i);
    }
}

fn shade_round(x: i32, y: i32, w: i32, h: i32, c: &str, bc: Color) {
    let i = 0;
    let clen = c.len() - 1;
    let chalf = clen / 2;

    if w > h {
        let d = h / 2;
        let na = 8;
        for i in 0..chalf {
            let d = d - 1;
            let x = x + 1;
            let y = y + 1;
            let w = w - 2;
            let h = h - 2;
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i).unwrap() as i32),
                bc,
            ));
            draw_pie(x, y, h, h, 90.0, 135.0 + (i * na) as f64);
            draw_xyline(x + d, y, x + w - d);
            draw_pie(x + w - h, y, h, h, 45.0 + (i * na) as f64, 90.0);
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i).unwrap() as i32 - 2),
                bc,
            ));
            draw_pie(
                x + w - h,
                y,
                h,
                h,
                315.0 + (i * na) as f64,
                405.0 + (i * na) as f64,
            );
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(clen - i).unwrap() as i32),
                bc,
            ));
            draw_pie(x + w - h, y, h, h, 270.0, 315.0 + (i * na) as f64);
            draw_xyline(x + d, y + h - 1, x + w - d);
            draw_pie(x, y, h, h, 225.0 + (i * na) as f64, 270.0);
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(clen - i).unwrap() as i32 - 2),
                bc,
            ));
            draw_pie(x, y, h, h, 135.0 + (i * na) as f64, 225.0 + (i * na) as f64);
        }
        set_draw_color(bc);
        draw_rectf(x + d, y, w - h + 1, h + 1);
        draw_pie(x, y, h, h, 90.0, 270.0);
        draw_pie(x + w - h, y, h, h, 270.0, 90.0);
    } else {
        let d = w / 2;
        let na = 8;
        for i in 0..chalf {
            let d = d - 1;
            let x = x + 1;
            let y = y + 1;
            let w = w - 2;
            let h = h - 2;
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i).unwrap() as i32),
                bc,
            ));
            draw_pie(x, y, w, w, 45.0 + (i * na) as f64, 135.0 + (i * na) as f64);
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(i).unwrap() as i32 - 2),
                bc,
            ));
            draw_pie(x, y, w, w, 0.0, 45.0 + (i * na) as f64);
            draw_yxline(x + w - 1, y + d, y + h - d);
            draw_pie(x, y + h - w, w, w, 315.0 + (i * na) as f64, 360.0);
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(clen - i).unwrap() as i32),
                bc,
            ));
            draw_pie(
                x,
                y + h - w,
                w,
                w,
                225.0 + (i * na) as f64,
                315.0 + (i * na) as f64,
            );
            set_draw_color(shade_color(
                Color::gray_ramp(c.chars().nth(clen - i).unwrap() as i32 - 2),
                bc,
            ));
            draw_pie(x, y + h - w, w, w, 180.0, 225.0 + (i * na) as f64);
            draw_yxline(x, y + d, y + h - d);
            draw_pie(x, y, w, w, 135.0 + (i * na) as f64, 180.0);
        }
        set_draw_color(bc);
        draw_rectf(x, y + d, w + 1, h - w + 1);
        draw_pie(x, y, w, w, 0.0, 180.0);
        draw_pie(x, y + h - w, w, w, 180.0, 360.0);
    }
}

fn up_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    frame_rect(x, y, w, h - 1, "KLDIIJLM", c);
}

fn narrow_thin_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if h <= 0 || w <= 0 {
        return;
    }
    set_draw_color(shade_color(Color::gray_ramp('R' as i32), c));
    draw_rectf(x + 1, y + 1, w - 2, h - 2);
    set_draw_color(shade_color(Color::gray_ramp('I' as i32), c));
    if w > 1 {
        draw_xyline(x + 1, y, x + w - 2);
        draw_xyline(x + 1, y + h - 1, x + w - 2);
    }
    if h > 1 {
        draw_yxline(x, y + 1, y + h - 2);
        draw_yxline(x + w - 1, y + 1, y + h - 2);
    }
}

fn thin_up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w > 4 && h > 4 {
        shade_rect(x + 1, y + 1, w - 2, h - 3, "RQOQSUWQ", c);
        frame_rect(x, y, w, h - 1, "IJLM", c);
    } else {
        narrow_thin_box(x, y, w, h, c);
    }
}

fn up_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w > 8 && h > 8 {
        shade_rect(x + 1, y + 1, w - 2, h - 3, "RVQNOPQRSTUVWVQ", c);

        frame_rect(x, y, w, h - 1, "IJLM", c);
    } else {
        thin_up_box(x, y, w, h, c);
    }
}

fn up_round(x: i32, y: i32, w: i32, h: i32, c: Color) {
    shade_round(x, y, w, h, "RVQNOPQRSTUVWVQ", c);
    frame_round(x, y, w, h, "IJLM", c);
}

fn down_frame(x: i32, y: i32, w: i32, h: i32, c: Color) {
    frame_rect(x, y, w, h - 1, "LLLLTTRR", c);
}

fn down_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    if w > 6 && h > 6 {
        draw_rect_fill(x + 2, y + 2, w - 4, h - 5, c.darker());
        down_frame(x, y, w, h, c);
    } else {
        narrow_thin_box(x, y, w, h, c);
    }
}

fn down_round(x: i32, y: i32, w: i32, h: i32, c: Color) {
    shade_round(x, y, w, h, "STUVWWWVT", c);
    frame_round(x, y, w, h, "IJLM", c);
}

fn border_box(x: i32, y: i32, w: i32, h: i32, c: Color) {
    set_draw_color(c);
    draw_rectf(x, y, w, h);
    set_draw_color(c.darker());
    draw_rect(x, y, w, h);
}

pub(crate) fn use_crystal_scheme() {
    use fltk::enums::FrameType::*;
    app::reload_scheme().ok();
    app::set_scheme(app::Scheme::Base);
    app::set_frame_type_cb(UpBox, up_box, 4, 4, 8, 8);
    app::set_frame_type_cb(DownBox, down_box, 2, 2, 4, 4);
    app::set_frame_type_cb(UpFrame, up_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(DownFrame, down_frame, 2, 2, 4, 4);
    app::set_frame_type_cb(ThinUpBox, thin_up_box, 1, 1, 2, 2);
    app::set_frame_type_cb(ThinDownBox, down_box, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundUpBox, up_round, 1, 1, 2, 2);
    app::set_frame_type_cb(RoundDownBox, down_round, 1, 1, 2, 2);
    app::set_frame_type_cb(BorderBox, border_box, 1, 1, 2, 2);
}
