use std::{collections::VecDeque, time::UNIX_EPOCH, f32::consts::PI};

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use minifb::{Key, MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};
const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn draw_ball(dt: &mut DrawTarget, pos: (f32, f32), _timestamp: u128) {
    let mut pb = PathBuilder::new();
    pb.arc(pos.0, pos.1, 10., 0., 2. * PI);
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)),
        &DrawOptions::new(),
    );
}

fn run() {
    let mut window = Window::new(
        "TestCircleCollision",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    // let mut frames = VecDeque::with_capacity(100);
    // let mut last_frame_timestamp = std::time::SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_millis();

    let size = window.get_size();
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));
        if let Some(pos) = window.get_mouse_pos(MouseMode::Clamp) {
            draw_ball(&mut dt, pos, now);

            let pos_string = format!("{:?}", pos);
            dt.draw_text(
                &font,
                36.,
                &pos_string,
                Point::new(0., 100.),
                &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0)),
                &DrawOptions::new(),
            );
            // let delta = now - last_frame_timestamp;
            // // println!("Delta: {}", delta);
            // last_frame_timestamp = now;
            // let fps = (1.0 / delta as f64) * 1000.0;
            // frames.push_back(fps);
            // if frames.len() > 100 {
            //     frames.pop_front();
            // }
            // let avg_fps =
            //     frames.iter().fold(0.0, |prev, value| prev + value) / (frames.len() as f64);
            // println!("FPS: {}", avg_fps);

            window
                .update_with_buffer(dt.get_data(), size.0, size.1)
                .unwrap();
        }
    }
}

fn main() {
    run();
}
