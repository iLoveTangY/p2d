use std::collections::VecDeque;
use std::f32::consts::PI;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::UNIX_EPOCH;

use p2d::world::World;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use softbuffer::{Context, Surface};
use winit::dpi::PhysicalSize;
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

fn draw_ball(dt: &mut DrawTarget, pos: (f64, f64)) {
    let mut pb = PathBuilder::new();
    pb.arc(pos.0 as f32, pos.1 as f32, 10., 0., 2. * PI);
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)),
        &DrawOptions::new(),
    );
}

fn render(dt: &mut DrawTarget, balls: &Vec<Ball>) {
    dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));

    // draw balls
    for ball in balls {
        draw_ball(dt, (ball.0, ball.1));
    }
}

struct Ball(f64, f64);

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn main() {
    let mut frames = VecDeque::with_capacity(100);
    let mut last_frame_timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut balls = Vec::new();

    let world = World::new(, iterations, gravity_scale)

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(
        WindowBuilder::new()
            .with_title("Winit - Mouse Left Button Up Event")
            .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
            .build(&event_loop)
            .unwrap(),
    );
    let context = Context::new(window.clone()).unwrap();
    let mut surface = Surface::new(&context, window.clone()).unwrap();

    let mut dt = DrawTarget::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    event_loop.set_control_flow(ControlFlow::Poll);
    let mut mouse_position = None;

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                elwt.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => match event.physical_key {
                PhysicalKey::Code(KeyCode::Escape) => {
                    elwt.exit();
                }
                _ => {}
            },
            WindowEvent::CursorMoved { position, .. } => {
                mouse_position = Some((position.x, position.y));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if button == MouseButton::Left && state == winit::event::ElementState::Released {
                    let pos = mouse_position.unwrap();
                    let ball = Ball(pos.0, pos.1);
                    balls.push(ball);
                    println!("Left mouse button released");
                }
            }
            WindowEvent::RedrawRequested => {
                let now = std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let delta = now - last_frame_timestamp;
                // println!("Delta: {}", delta);
                last_frame_timestamp = now;
                let fps = (1.0 / delta as f64) * 1000.0;
                frames.push_back(fps);
                if frames.len() > 100 {
                    frames.pop_front();
                }
                let avg_fps =
                    frames.iter().fold(0.0, |prev, value| prev + value) / (frames.len() as f64);
                println!("FPS: {}", avg_fps);

                // prepare surface and buffer
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                let mut buffer = surface.buffer_mut().unwrap();

                // make some painting
                render(&mut dt, &balls);

                // present buffer
                buffer.copy_from_slice(dt.get_data());
                buffer.present().unwrap();
            }
            _ => (),
        },
        Event::AboutToWait => {
            window.request_redraw();
        }
        _ => (),
    });
}
