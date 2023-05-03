use sdl2::event::Event;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use crate::body::World;

mod body;
mod util;

fn running_events(appstate: &mut AppState, event: &Event) -> bool
{
    match event {
        Event::Quit { .. } => {
            return true;
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => {
            appstate.simulation_running = false;
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => {
            appstate.update_ms /= 2;
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => {
            appstate.update_ms *= 2;
        }
        Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
            let mut found = false;
            for body in appstate.world.bodies.iter() {
                if (*x as f32 - body.position.x).powf(2.) + (*y as f32 - body.position.y).powf(2.) < body.radius.powf(2.) {
                    appstate.selected_body = Some(body.id);
                    found = true;
                }
            }
            if !found {
                appstate.selected_body = None;
            }
        }
        _ => {}
    }
    false
}

fn not_running_events(appstate: &mut AppState, event: &Event) -> bool
{
    match event {
        Event::Quit { .. } => {
            return true;
        }
        Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
            appstate.world.add_body(*x, *y, 1.0);
        }
        Event::MouseButtonDown { mouse_btn: MouseButton::Right, x, y, .. } => {
            appstate.world.add_body(*x, *y, 10.0);
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => {
            appstate.world.save_state();
            appstate.simulation_running = true;
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => {
            appstate.world.bodies = appstate.world.initial_bodies.clone();
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => {
            appstate.world.bodies.clear();
            appstate.world.initial_bodies.clear();
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => {
            let x = appstate.mouse_state.unwrap().x();
            let y = appstate.mouse_state.unwrap().y();
            for body in appstate.world.bodies.iter_mut() {
                if body.position.x == x as f32 && body.position.y == y as f32 {
                    body.mass += 1.0;
                }
            }
        }
        Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => {
            let x = appstate.mouse_state.unwrap().x();
            let y = appstate.mouse_state.unwrap().y();
            for body in appstate.world.bodies.iter_mut() {
                if body.position.x == x as f32 && body.position.y == y as f32 {
                    body.radius += 1.0;
                }
            }
        }
        _ => {}
    }
    false
}

struct AppState {
    pub world: World,
    pub simulation_running: bool,
    pub mouse_state: Option<MouseState>,
    pub update_ms: u16,
    pub selected_body: Option<u16>,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let window = video_subsystem.window("N-Body Simulation", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("res/fira.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut state = AppState {
        world: World::new(),
        simulation_running: false,
        mouse_state: None,
        update_ms: 16,
        selected_body: None,
    };
    'running: loop {
        for event in event_pump.poll_iter() {
            if !state.simulation_running {
                if not_running_events(&mut state, &event) {
                    break 'running;
                }
            } else if state.simulation_running && running_events(&mut state, &event) {
                break 'running;
            }
        }
        if state.simulation_running {
            state.world.update();
        }
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        state.world.draw(&mut canvas, &mut font, state.selected_body);
        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(state.update_ms as u64));
    }
    Ok(())
}
