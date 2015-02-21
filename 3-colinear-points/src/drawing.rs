use graphics::{self, Context, Ellipse, Line, rectangle, Rectangle, RelativeTransform};
use std::cell::RefCell;
use opengl_graphics::{Gl, OpenGL};
use piston::window::WindowSettings;
use piston::event::{events, RenderEvent, UpdateEvent};
use sdl2_window::Sdl2Window as Window;
use point::Point;
use std::iter::IteratorExt;
use std::sync::mpsc::Receiver;
use std::cmp;

pub fn display(points: &[Point], incoming_lines: Receiver<Option<[i32; 4]>>) {
    if points.len() == 0 {
        return;
    }

    // Create an SDL window.
    let window = Window::new(
        OpenGL::_2_1,
        WindowSettings {
            title: "Colinear points".to_string(),
            size: [1280, 1024],
            samples: 0,
            fullscreen: false,
            exit_on_esc: true,
        }
    );
    let window = RefCell::new(window);

    let bounds = [
        points.iter().map(|p| p.x).min().unwrap(), points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(), points.iter().map(|p| p.y).max().unwrap()
    ];
    println!("bounds = {:?}", bounds);

    let gl = &mut Gl::new(OpenGL::_2_1);
    let mut lines = Vec::new();
    let mut complete = false;

    for e in events(&window) {
        if let Some(args) = e.render_args() {
            graphics::clear(if complete { [0.0, 0.0, 0.0, 1.0] } else { [0.1, 0.1, 0.1, 1.0] }, gl);

            let min_x = bounds[0] as f64;
            let max_x = bounds[1] as f64;
            let scale_x = args.width as f64 / (max_x - min_x);
            let min_y = bounds[2] as f64;
            let max_y = bounds[3] as f64;
            let scale_y = args.height as f64 / (max_y - min_y);

            let context = &Context::abs(args.width as f64, args.height as f64)
                            .scale(scale_x, scale_y)
                            .flip_v()
                            .trans(-min_x, -min_y - max_y); // also do "- max_y" because we flip_v'd earlier

            let dot_sx = 3f64 / scale_x;
            let dot_sy = 3f64 / scale_y;

            let blue_dot = Ellipse::new([0.0, 0.0, 1.0, 1.0]);
            for p in points {
                blue_dot.draw(rectangle::centered([p.x as f64, p.y as f64, dot_sx, dot_sy]), context, gl);
            }

            let red_line = Line::new([1.0, 0.0, 0.0, 1.0], 1f64 / cmp::partial_min(scale_x, scale_y).unwrap_or(scale_x));
            for line in &lines {
                red_line.draw(*line, context, gl);
            }

            let green_rect = Rectangle::new([0.0, 1.0, 0.0, 1.0]);
            green_rect.draw(rectangle::centered([min_x, min_y, dot_sx, dot_sy]), context, gl);
            green_rect.draw(rectangle::centered([min_x, max_y, dot_sx, dot_sy]), context, gl);
            green_rect.draw(rectangle::centered([max_x, min_y, dot_sx, dot_sy]), context, gl);
            green_rect.draw(rectangle::centered([max_x, max_y, dot_sx, dot_sy]), context, gl);
        }

        if let Some(_) = e.update_args() {
            loop {
                match incoming_lines.try_recv() {
                    Ok(Some([x1, y1, x2, y2])) => lines.push([x1 as f64, y1 as f64, x2 as f64, y2 as f64]),
                    Ok(None) => complete = true, // line finder is done
                    Err(_) => break, // no new lines at this time
                };
            }
        }
    }
}
