use sdl2::event::Event;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use crate::body::World;

mod body;
mod util;

fn running_events(_world: &mut World, running: &mut bool, _mouse: MouseState, event: &Event) -> bool
{
    match event {
        Event::Quit { .. } => {
            return true;
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => {
            *running = false;
        }
        _ => {}
    }
    false
}

fn not_running_events(world: &mut World, running: &mut bool, mouse: MouseState, event: &Event) -> bool
{
    match event {
        Event::Quit { .. } => {
            return true;
        }
        Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
            world.add_body(*x, *y, 1.0);
        }
        Event::MouseButtonDown { mouse_btn: MouseButton::Right, x, y, .. } => {
            world.add_body(*x, *y, 10.0);
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => {
            world.save_state();
            *running = true;
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
            world.bodies = world.initial_bodies.clone();
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => {
            world.bodies.clear();
            world.initial_bodies.clear();
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => {
            let x = mouse.x();
            let y = mouse.y();
            for body in world.bodies.iter_mut() {
                if body.position.x == x as f32 && body.position.y == y as f32 {
                    body.mass += 1.0;
                }
            }
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => {
            let x = mouse.x();
            let y = mouse.y();
            for body in world.bodies.iter_mut() {
                if body.position.x == x as f32 && body.position.y == y as f32 {
                    body.radius += 1.0;
                }
            }
        }
        _ => {}
    }
    false
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("N-Body Simulation", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut world = World::new();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = false;
    'running: loop {
        let mouse = event_pump.mouse_state();
        for event in event_pump.poll_iter() {
            if !running {
                if not_running_events(&mut world, &mut running, mouse, &event) {
                    break 'running;
                }
            } else if running && running_events(&mut world, &mut running, mouse, &event) {
                break 'running;
            }
        }
        if running {
            world.update();
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        world.draw(&mut canvas);
        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
