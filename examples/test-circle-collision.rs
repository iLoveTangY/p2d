use std::collections::VecDeque;
use std::f32::consts::PI;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::UNIX_EPOCH;

use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use p2d::body::Body;
use p2d::shape::{Circle, AABB};
use p2d::vec2::Vec2;
use p2d::world::World;
use raqote::{DrawOptions, DrawTarget, PathBuilder, Point, SolidSource, Source};

use softbuffer::{Context, Surface};
use winit::dpi::PhysicalSize;
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

fn draw_ball(dt: &mut DrawTarget, pos: Vec2, radius: f32) {
    let mut pb = PathBuilder::new();
    pb.arc(pos.x, pos.y, radius, 0., 2. * PI);
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0, 0xff, 0)),
        &DrawOptions::new(),
    );
}

fn draw_aabb(dt: &mut DrawTarget, min: Vec2, max: Vec2, pos: Vec2) {
    let mut pb = PathBuilder::new();
    let half_extend = (max - min) / 2.;
    let left_top = pos - half_extend;
    pb.rect(
        left_top.x,
        left_top.y,
        half_extend.x * 2.,
        half_extend.y * 2.,
    );
    let path = pb.finish();
    dt.fill(
        &path,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 110, 123, 108)),
        &DrawOptions::new(),
    );
}

fn render_fps(dt: &mut DrawTarget, fps: i32) {
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();
    let fps_string = format!("fps: {}", fps);
    dt.draw_text(
        &font,
        24.,
        &fps_string,
        Point::new(0., 30.),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
        &DrawOptions::new(),
    )
}

fn render(dt: &mut DrawTarget, world: &World) {
    dt.clear(SolidSource::from_unpremultiplied_argb(
        0xff, 0xff, 0xff, 0xff,
    ));

    let bodies = world.get_bodies();
    for body in bodies {
        let inner_body = body.as_ref().borrow();
        match inner_body.shape() {
            p2d::shape::ShapeType::Circle(ref circle) => {
                draw_ball(dt, inner_body.position(), circle.radius());
            }
            p2d::shape::ShapeType::AABB(ref aabb) => {
                draw_aabb(dt, aabb.min(), aabb.max(), inner_body.position());
            }
        }
    }
}


const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

fn render_loop(world: &mut World) {
    let mut frames = VecDeque::with_capacity(100);
    let mut last_frame_timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

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
                mouse_position = Some(Vec2::new(position.x as f32, position.y as f32));
            }
            WindowEvent::MouseInput { button, state, .. } => {
                if button == MouseButton::Left && state == winit::event::ElementState::Released {
                    let pos = mouse_position.unwrap();
                    world.add_body(Body::new_circle(Circle::new(30.), pos, 1.0));
                    println!("Left mouse button released");
                }
            }
            WindowEvent::RedrawRequested => {
                // compute fps
                let now = std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let delta = now - last_frame_timestamp;
                last_frame_timestamp = now;
                let fps = (1.0 / delta as f64) * 1000.0;
                frames.push_back(fps);
                if frames.len() > 100 {
                    frames.pop_front();
                }
                let avg_fps =
                    frames.iter().fold(0.0, |prev, value| prev + value) / (frames.len() as f64);

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
                world.step();
                render(&mut dt, world);
                render_fps(&mut dt, avg_fps as i32);

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

fn main() {
    let dt = 1. / 60.;
    let mut world = World::new(dt, 20, 1.0);
    // add ground
    let ground_height = 20;
    let ground_aabb = AABB::new(
        Vec2::new(0., (WINDOW_HEIGHT - ground_height) as f32),
        Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
    );
    let mut groud = Body::new_aabb(ground_aabb, ground_aabb.center(), 0.5);
    groud.make_static();
    world.add_body(groud);
    render_loop(&mut world);
}
