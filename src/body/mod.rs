use std::sync::atomic::{AtomicU16, Ordering};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use crate::util::Vector2;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;

#[derive(Clone)]
pub struct Body
{
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub id: u16,
}

pub struct World
{
    pub initial_bodies: Vec<Body>,
    pub bodies: Vec<Body>,
}

impl World
{
    pub fn new() -> World
    {
        World {
            initial_bodies: Vec::new(),
            bodies: Vec::new(),
        }
    }

    pub fn save_state(&mut self)
    {
        self.initial_bodies = self.bodies.clone();
    }

    pub fn update(&mut self) {
        let mut accelerations = vec![Vector2::new(0.0, 0.0); self.bodies.len()];
        for (i, body) in self.bodies.iter().enumerate() {
            let mut accel_acc = Vector2::new(0.0, 0.0);
            for (j, second_body) in self.bodies.iter().enumerate() {
                if i == j {
                    continue;
                }
                let r_vector = vec![body.position.x - second_body.position.x, body.position.y - second_body.position.y];
                let r_mag = r_vector[0] * r_vector[0] + r_vector[1] * r_vector[1];
                let r_mag = if r_mag < (body.radius + second_body.radius) * (body.radius + second_body.radius) {
                    body.radius + second_body.radius
                } else {
                    r_mag.sqrt()
                };
                let accel = -1.0 * physical_constants::STANDARD_ACCELERATION_OF_GRAVITY * second_body.mass as f64 / r_mag.powf(2.0) as f64;
                let r_vector_unit = [r_vector[0] / r_mag, r_vector[1] / r_mag];
                let tmp = vec![accel * r_vector_unit[0] as f64, accel * r_vector_unit[1] as f64];
                accel_acc.x += tmp[0] as f32;
                accel_acc.y += tmp[1] as f32;
            }
            accelerations[i] = accel_acc;
        }
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let accel = accelerations[i];
            body.velocity.x += accel.x;
            body.velocity.y += accel.y;
            body.position.x += body.velocity.x;
            body.position.y += body.velocity.y;
        }
    }

    fn render_text(str: &str, font: &mut sdl2::ttf::Font, texture_creator: &sdl2::render::TextureCreator<WindowContext>, target: Rect, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>)
    {
        let surface = font.render(str).blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        canvas.copy(&texture, None, Some(target)).unwrap();
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &mut sdl2::ttf::Font, selected_body: Option<u16>)
    {
        for body in &self.bodies {
            if let Some(id) = selected_body {
                if id == body.id {
                    canvas.filled_circle(body.position.x as i16, body.position.y as i16, body.radius as i16, Color::RGB(255, 0, 0)).unwrap();
                    continue;
                }
            }
            canvas.filled_circle(body.position.x as i16, body.position.y as i16, body.radius as i16, Color::RGB(255, (255.0 * (100.0 - body.mass) / 100.0) as u8, (255.0 * (100.0 - body.mass) / 100.0) as u8)).unwrap();
        }
        if let Some(..) = selected_body {
            let id = selected_body.unwrap() as usize;
            let body = &self.bodies[id];
            let pos = body.position;
            let vel = body.velocity;
            let rad = body.radius;
            let mass = body.mass;
            let texture_creator = canvas.texture_creator();
            let target = Rect::new(0, 0, 100, 50);
            Self::render_text(&format!("Body #{id}"), font, &texture_creator, target, canvas);
            let target = Rect::new(0, 50, 300, 50);
            Self::render_text(&format!("pos: ({:.2}; {:.2})", pos.x, pos.y), font, &texture_creator, target, canvas);
            let target = Rect::new(0, 100, 300, 50);
            Self::render_text(&format!("vel: ({:.2}; {:.2})", vel.x, vel.y), font, &texture_creator, target, canvas);
            let target = Rect::new(0, 150, 100, 50);
            Self::render_text(&format!("rad: {rad}"), font, &texture_creator, target, canvas);
            let target = Rect::new(0, 200, 100, 50);
            Self::render_text(&format!("mass: {mass}"), font, &texture_creator, target, canvas);
        }
    }

    pub fn add_body(&mut self, x: i32, y: i32, mass: f32)
    {
        static COUNTER: AtomicU16 = AtomicU16::new(0);
        self.bodies.push(Body {
            velocity: Vector2::new(0.0, 0.0),
            position: Vector2::new(x as f32, y as f32),
            mass,
            radius: 2.0,
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        });
    }
}