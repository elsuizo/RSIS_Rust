#![allow(unused_assignments)]

extern crate sfml;

use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Font, Transformable,
                     RectangleShape, Text};
use sfml::window::{Key, VideoMode, window_style, event, MouseButton};
use sfml::system::{Clock, Time, Vector2f};
use std::f32::consts::PI;
use std::collections::VecDeque;

pub struct Rosette<'a> {
    pub color: Color,
    pub radius: f32,
    pub shape: CircleShape<'a>,
    pub position: Vector2f,
    pub velocity: Vector2f,
    pub aceleration: Vector2f,
}

impl<'a> Rosette<'a> {
    fn new(color: Color, radius: f32) -> Self {
        let mut s = CircleShape::new().unwrap();
        let mut v = Vector2f::new(1.0, 1.5);
        let mut p = Vector2f::new(0.0, 0.0);
        let mut a = Vector2f::new(0.0, 0.0);
        s.set_radius(radius);
        s.set_outline_thickness(2.0);
        s.set_outline_color(&Color::red());
        s.set_fill_color(&color);
        Rosette {
            color: color,
            radius: radius,
            shape: s,
            position: p,
            velocity: v,
            aceleration: a,
        }
    }

    pub fn update(&mut self, t: Time) -> () {

        let p = update_position(t.as_seconds(), 3, 5);
        self.position = p;
        self.shape.move_(&self.position);
    }

}

pub fn update_position(time: f32, f1: i32, f2: i32) -> Vector2f {
        let x = (2.0 * PI * time * f1 as f32).cos() + (2.0 * PI * time * f2 as f32).cos();
        let y = (2.0 * PI * time * f1 as f32).sin() - (2.0 * PI * time * f2 as f32).sin();
        Vector2f::new(x * 10.0, y * 10.0)
}

fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: (f64, f64),
                  lower_rigth: (f64, f64))
    -> (f64, f64)
{
    // It might be nicer to find the position of the *middle* of the pixel,
    // instead of its upper left corner, but this is easier to write test for.
    let (width, height) = (lower_rigth.0 - upper_left.0,
                           upper_left.1 - lower_rigth.1);
    // return tuple
    (upper_left.0 + pixel.0 as f64 * width / bounds.0 as f64,
     upper_left.1 - pixel.1 as f64 * height / bounds.1 as f64)
}

fn main() {
    // Define some constants
    let side: u32 = 700;
    let width: u32 = side;
    let height: u32 = side;
    // Create the window of the application
    let fg_color = &Color::new_rgb(255, 255, 255);
    let mut window = RenderWindow::new(VideoMode::new_init(width, height, 32),
                                       "Rosette",
                                       window_style::CLOSE,
                                       &Default::default()).unwrap();
    window.set_vertical_sync_enabled(true);
    let k = side as f32 / 320E+9;
    let radius = 20.0;
    let dt = 1E-4;
    let font = Font::new_from_file("src/res/sansation.ttf").unwrap();
    let mut counter:i32 = 0;
    let rosette_color = Color::black();
    let mut rosette = Rosette::new(rosette_color, 10.0);
    rosette.shape.set_position(&Vector2f::new(height as f32 / 2.0, width as f32 / 2.0));
    let bg_color = &Color::new_rgb(0, 0, 0);
    let mut clock = Clock::new();
    let mut clock_general = Clock::new();
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
        }
        let time = clock.restart();
        let t = clock_general.get_elapsed_time();
        println!("{:?}", t.as_seconds());
        if time.as_seconds() < 0.7 {
            rosette.update(t);
        }
        // rosette.shape.move2f(350.0 + pos.0, 350.0 + pos.1);
        window.clear(bg_color);

        window.draw(&rosette.shape);
        window.draw(&msg);
        // window.draw(&obj);
        window.display();
    }
}

fn scale(v: &Vector2f, c: f32) -> Vector2f {
    return Vector2f::new(c * v.x, c * v.y);
}
