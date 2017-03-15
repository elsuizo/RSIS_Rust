#![allow(unused_assignments)]

extern crate sfml;

use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Font, Transformable,
                     RectangleShape, Text};
use sfml::window::{Key, VideoMode, window_style, event, MouseButton};
use sfml::system::Vector2f;
use std::f32::consts::PI;
use std::collections::VecDeque;

struct Rosette {
    color: Color,
    radius: f32,
    pos: Vector2f,
    vel: Vector2f,
    acc: Vector2f,
}

fn main() {
    // Define some constants
    let side: u32 = 700;
    let width: u32 = side;
    let height: u32 = side;
    let rosette_radius: f32 = 11.;

    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(width, height, 32),
                                       "SFML Pong",
                                       window_style::CLOSE,
                                       &Default::default())
                                        .unwrap();
    window.set_vertical_sync_enabled(true);

    let font = Font::new_from_file("src/res/sansation.ttf").unwrap();

    let rosette_color = &Color::new_rgb(0, 102, 255);
    let bg_color = &Color::new_rgb(0, 0, 0);
    let mut rosette = create_circle(rosette_radius, rosette_color);
    rosette.set_position(&Vector2f::new(width as f32 / 2., height as f32 / 2.));

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => return,
                        _ => {}
                    }
                }
                _ => {}
            }

            window.clear(bg_color);
            window.draw(&rosette);
            window.display();
        }
 }
}

fn create_circle(radius: f32, fill: &Color) -> CircleShape {
    let mut obj = CircleShape::new().unwrap();
    obj.set_radius(radius as f32);
    obj.set_outline_thickness(0.);
    obj.set_outline_color(&Color::black());
    obj.set_fill_color(fill);
    obj.set_origin(&Vector2f::new(radius / 2., radius / 2.));

    return obj;
}

fn scale(v: &Vector2f, c: f32) -> Vector2f {
    return Vector2f::new(c * v.x, c * v.y);
}

fn dist(v1: &Vector2f, v2: &Vector2f) -> f32 {
    return ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2)).sqrt();
}

fn len(v: &Vector2f) -> f32 {
    return (v.x * v.x + v.y * v.y).sqrt();
}

fn sqdist(v1: &Vector2f, v2: &Vector2f) -> f32 {
    return ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2));
}

fn add(a: &Vector2f, b: &Vector2f) -> Vector2f {
    return Vector2f::new(a.x + b.x, a.y + b.y);
}

fn sub(a: &Vector2f, b: &Vector2f) -> Vector2f {
    return Vector2f::new(a.x - b.x, a.y - b.y);
}

fn accel(r: &Vector2f, s: &Vector2f, ms: f32) -> Vector2f {
    let G = 6.672E-11;
    let epsilon = 1.;
    let v = -G * ms / (epsilon * epsilon + sqdist(r, s)).powf(1.5);
    return scale(&sub(r, s), v);
}
