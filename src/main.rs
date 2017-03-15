#![allow(unused_assignments)]

extern crate sfml;

use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Font, Transformable,
                     RectangleShape, Text};
use sfml::window::{Key, VideoMode, window_style, event, MouseButton};
use sfml::system::Vector2f;
use std::f32::consts::PI;
use std::collections::VecDeque;

struct Body {
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
    let sun_radius: f32 = 11.;
    let earth_radius: f32 = 8.;

    let mut euler = false;
    let mut run = true;
    let mut stepmode = false;

    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(width, height, 32),
                                       "SFML Pong",
                                       window_style::CLOSE,
                                       &Default::default())
                         .unwrap();
    window.set_vertical_sync_enabled(true);

    let font = Font::new_from_file("src/res/sansation.ttf").unwrap();

    let earth_color = &Color::new_rgb(0, 102, 255);
    let sun_color = &Color::new_rgb(255, 102, 0);
    let bg_color = &Color::new_rgb(0, 0, 0);
    let fg_color = &Color::new_rgb(255, 255, 255);

    let mut sun = createObj(sun_radius, sun_color);
    let mut earth = createObj(earth_radius, earth_color);
    let mut aphec = createObj(earth_radius, earth_color);
    let mut peric = createObj(earth_radius, earth_color);

    let mut earthvec = RectangleShape::new_init(&Vector2f::new(80., 3.)).unwrap();

    let mut msg = Text::new_init("asdfasdf", &font, 20).unwrap();
    msg.set_position(&(Vector2f::new(0., 0.)));
    msg.set_color(fg_color);

    // SI, of course
    // -------------------------------------------------------------
    let dt = 1E-4;
    let dist = 149.6E+9;
    let mut ntrace = 25;
    let mut trace_skip = 5;
    let mut trace = VecDeque::new();
    let mut t_tmp = CircleShape::new().unwrap();

    for i in 0..ntrace {
        t_tmp = createObj(2., earth_color);
        window.draw(&t_tmp);
        trace.push_back(t_tmp);
    }
    window.display();

    // Scaling constant, for distances
    let k = side as f32 / 320E+9;
    let (ms, me) = (1.989E+30, 5.972E+24);
    let (rads, rade) = (6371E+3, 696342E+3);
    let mut dre = Vector2f::new(0., 0.);
    let mut drs = Vector2f::new(0., 0.);
    let mut ve = Vector2f::new(89.78E+11, 0.);
    let mut vs = Vector2f::new(0., 0.);
    let mut acce = Vector2f::new(0., 0.);
    let mut acce_ = Vector2f::new(0., 0.);
    let mut accs = Vector2f::new(0., 0.);
    let mut accs_ = Vector2f::new(0., 0.);
    let mut tmp = Vector2f::new(0., 0.);

    sun.set_position(&Vector2f::new(width as f32 / 2., height as f32 / 2.));
    earth.set_position(&sun.get_position());
    earth.move_(&scale(&Vector2f::new(0., dist), k));

    let mut peri = earth.get_position();
    let mut aphe = earth.get_position();
    let mut distance = dist * dist;
    let mut maxdistance = 0.;
    let mut mindistance = dist * dist;

    earthvec.set_fill_color(&Color::new_rgb(0, 51, 127));

    let mut maxv: f32 = 0.;

    let mut method = "Verlet";
    let mut iter: u32 = 0;
    let mut trace_iter: u32 = 0;
    let mut ntrfilled = 0;

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => return,
                        Key::Space => {
                            run = !run;
                        }
                        Key::E => {
                            euler = !euler;
                            if method == "Euler" {
                                method = "Verlet";
                            } else {
                                method = "Euler";
                            }
                        }
                        Key::S => {
                            stepmode = !stepmode;
                        }
                        Key::Num3 => {
                            if ntrace > 1 {
                                ntrace -= 1;
                                trace.pop_front();
                            }
                        }
                        Key::Num4 => {
                            ntrace += 1;
                            t_tmp = createObj(2., earth_color);
                            trace.push_back(t_tmp);
                        }
                        Key::Num5 => {
                            if trace_skip > 1 {
                                trace_skip -= 1;
                            }
                        }
                        Key::Num6 => {
                            trace_skip += 1;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if run {
            if euler {
                // Euler
                acce = accel(&earth.get_position(), &sun.get_position(), ms);
                accs = accel(&sun.get_position(), &earth.get_position(), me);

                earth.move_(&scale(&ve, k * dt));
                sun.move_(&scale(&vs, k * dt));

                ve = add(&ve, &scale(&acce, dt));
                vs = add(&vs, &scale(&accs, dt));
            } else {
                // Verlet
                acce = accel(&earth.get_position(), &sun.get_position(), ms);
                accs = accel(&sun.get_position(), &earth.get_position(), me);

                // position += timestep * (velocity + timestep * acceleration / 2);
                tmp = add(&ve, &scale(&acce, dt / 2.));
                dre = scale(&tmp, dt);
                earth.move_(&scale(&dre, k));

                tmp = add(&vs, &scale(&accs, dt / 2.));
                drs = scale(&tmp, dt);
                sun.move_(&scale(&drs, k));

                // newAcceleration
                acce_ = accel(&earth.get_position(), &sun.get_position(), ms);
                accs_ = accel(&sun.get_position(), &earth.get_position(), me);

                // velocity += timestep * (acceleration + newAcceleration) / 2;
                ve = add(&ve, &scale(&(acce + acce_), dt / 2.));
                vs = add(&vs, &scale(&(accs + accs_), dt / 2.));
            }

            if len(&vs) > maxv {
                maxv = len(&vs);
            }

            distance = sqdist(&earth.get_position(), &sun.get_position());
            if (distance > maxdistance) {
                maxdistance = distance;
                aphe = earth.get_position();
                aphec.set_position(&aphe);
            }
            if (distance < mindistance) {
                mindistance = distance;
                peri = earth.get_position();
                peric.set_position(&peri);
            }

            if iter % trace_skip == 0 {
                let a = trace.pop_front().unwrap();
                trace.push_back(a);
                trace.back_mut().unwrap().set_position(&earth.get_position());
            }
            msg.set_string(&format!("Rendering with {}\niter #{}\nrel pos {} {}\nvel {} {}\nmax \
                                     vel {}\nstep {}\nmax {} min {}\ntraces: {}, skipping {}",
                                    method,
                                    iter,
                                    earth.get_position().x - ((width as f32) / 2.),
                                    earth.get_position().y - ((height as f32) / 2.),
                                    ve.x / 1E+6,
                                    ve.y / 1E+6,
                                    maxv,
                                    stepmode,
                                    maxdistance,
                                    mindistance,
                                    ntrace,
                                    trace_skip));


            earthvec.set_position(&earth.get_position());
            earthvec.set_scale(&Vector2f::new(len(&ve) / (4E+13), 1.));
            earthvec.set_rotation(180. * (ve.y.atan2(ve.x)) / PI);

            window.clear(bg_color);

            for o in trace.iter() {
                window.draw(o);
            }

            window.draw(&aphec);
            window.draw(&peric);
            window.draw(&sun);
            window.draw(&earthvec);
            window.draw(&earth);
            window.draw(&msg);
            window.display();

            iter = iter + 1;
            if stepmode {
                run = false;
            }
        }
    }
}

fn createObj(radius: f32, fill: &Color) -> CircleShape {
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
