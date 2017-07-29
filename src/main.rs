#![allow(unused_assignments)]

extern crate sfml;

use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Font, Transformable,
                     RectangleShape, Text};
use sfml::window::{Key, VideoMode, window_style, event, MouseButton};
use sfml::system::{Clock, Time, Vector2f};
use std::f32::consts::PI;
use std::collections::VecDeque;

struct Rosette<'a> {
    color: Color,
    radius: f32,
    shape: CircleShape<'a>,
    position: Vector2f,
    velocity: Vector2f,
    aceleration: Vector2f,
}

impl<'a> Rosette<'a> {
    fn new(color: Color, radius: f32) -> Self {
        Rosette {
            color: color,
            radius: radius,
            shape: CircleShape::new().unwrap(),
            position: Vector2f::new(0.0, 0.0),
            velocity: Vector2f::new(0.0, 0.0),
            aceleration: Vector2f::new(0.0, 0.0),
        }
    }
}

fn main() {
    // Define some constants
    let side: u32 = 700;
    let width: u32 = side;
    let height: u32 = side;
    // Create the window of the application
    let rosette_color = Color::new_rgb(0, 0, 0);
    let fg_color = &Color::new_rgb(255, 255, 255);
    let r = Rosette::new(rosette_color, 37.0);
    let mut window = RenderWindow::new(VideoMode::new_init(width, height, 32),
                                       "Rosette",
                                       window_style::CLOSE,
                                       &Default::default()).unwrap();
    window.set_vertical_sync_enabled(true);

    let font = Font::new_from_file("src/res/sansation.ttf").unwrap();
    let mut counter = 0;
    let rosette_color = Color::new_rgb(0, 102, 255);
    let rosette = Rosette::new(rosette_color, 3.0);
    let bg_color = &Color::new_rgb(0, 0, 0);
    let mut clock = Clock::new();
    let mut msg = Text::new_init("asdfasdf", &font, 20).unwrap();
    msg.set_position(&(Vector2f::new(0., 0.)));
    msg.set_color(fg_color);
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
            msg.set_string(&format!("counter: {}", counter));
            counter += 1;
            window.clear(bg_color);

            window.draw(&msg);
            window.display();
        }
    }
}

