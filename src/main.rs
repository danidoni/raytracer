use glam::{Vec2, Vec3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

const INF: f32 = f32::MAX;

const BACKGROUND_COLOR: Color = Color::WHITE;

struct Sphere {
    radius: f32,
    center: Vec3,
    color: Color,
}

struct Scene {
    spheres: Vec<Sphere>,
}

fn canvas_2_viewport(
    x: i32,
    y: i32,
    canvas_w: i32,
    canvas_h: i32,
    viewport: Vec3,
) -> Vec3 {
    return Vec3::new(
        x as f32 * (viewport.x as f32 / canvas_w as f32),
        y as f32 * (viewport.y as f32 / canvas_h as f32),
        viewport.z as f32,
    );
}

fn intersect_ray_sphere(origin: Vec3, distance: Vec3, sphere: &Sphere) -> (f32, f32) {
    let r = sphere.radius;
    let co = origin - sphere.center;

    let a = distance.dot(distance);
    let b = 2.0 * co.dot(distance);
    let c = co.dot(co) - r * r;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (INF, INF);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

    return (t1, t2);
}

fn trace_ray(origin: Vec3, distance: Vec3, min_t: f32, max_t: f32, scene: &Scene) -> Color {
    let mut closest_t = INF;
    let mut closest_sphere = None;

    for sphere in &scene.spheres {
        let ts = intersect_ray_sphere(origin, distance, &sphere);
        let t1 = ts.0;
        let t2 = ts.1;
        if min_t < t1 && t1 < max_t && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if min_t < t2 && t2 < max_t && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    return match closest_sphere {
        None => BACKGROUND_COLOR,
        Some(sphere) => sphere.color,
    };
}

fn main() {
    let cw: i32 = 800;
    let ch: i32 = 600;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Raytracer", cw as u32, ch as u32)
        .build()
        .unwrap();
    let mut sdl_canvas = window.into_canvas().present_vsync().build().unwrap();
    let scene = Scene {
        spheres: vec![
            Sphere {
                center: Vec3::new(0.0, -1.0, 3.0),
                radius: 1.0,
                color: Color::RGB(255, 0, 0),
            },
            Sphere {
                center: Vec3::new(2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color::RGB(0, 0, 255),
            },
            Sphere {
                center: Vec3::new(-2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color::RGB(0, 255, 0),
            },
        ],
    };

    // This is the camera origin
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let viewport = Vec3::new(
        // Viewport size or Frame size
        1.0, 1.0, 
        // Frame distance
        1.0);

    // For each point in the canvas...
    for cx in (-cw / 2)..(cw / 2) {
        for cy in (-ch / 2)..(ch / 2) {
            // Get the direction of the casted ray, from O and passing through V, that would go into the canvas point
            let direction = canvas_2_viewport(cx, cy, cw, ch, viewport);

            // See if the ray hits something, and if so, get the color of the object we hit
            let color = trace_ray(origin, direction, 1.0, INF, &scene);

            sdl_canvas.set_draw_color(color);

            let sx = (cw / 2) + cx;
            let sy = (ch / 2) - cy;
            sdl_canvas
                .draw_point(Point::new(sx as i32, sy as i32))
                .unwrap();
        }
    }

    sdl_canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
