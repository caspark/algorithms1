use graphics::{self, Context, Rectangle, RelativeTransform};
use std::cell::RefCell;
use opengl_graphics::{Gl, OpenGL};
use piston::window::WindowSettings;
use piston::event::{events, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use sdl2_window::Sdl2Window as Window;
use point::Point;
use std::iter::IteratorExt;

struct App<'a> {
    gl: Gl,       // OpenGL drawing backend.
    points: &'a [Point],
    bounds: [i32; 4], // min x, max x, min y, max y
    rotation: f64 // Rotation for the square.
}

impl<'a> App<'a> {
    fn render(&mut self, _: &mut Window, args: &RenderArgs) {
        const COLOR_BLACK:    [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const COLOR_GREEN:  [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const COLOR_BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let min_x = self.bounds[0] as f64;
        let max_x = self.bounds[1] as f64;
        let scale_x = args.width as f64 / (max_x - min_x);
        let min_y = self.bounds[2] as f64;
        let max_y = self.bounds[3] as f64;
        let scale_y = args.height as f64 / (max_y - min_y);

        let dot_sx = 2f64 / scale_x;
        let dot_sy = 2f64 / scale_y;

        let context = &Context::abs(args.width as f64, args.height as f64)
                        .scale(scale_x, scale_y)
                        .trans(-min_x, -min_y);

        graphics::clear(COLOR_BLACK, &mut self.gl);
        for point in self.points {
            Rectangle::new(COLOR_BLUE).draw(graphics::rectangle::centered([point.x as f64, point.y as f64, dot_sx, dot_sy]), context, &mut self.gl);
        }

        Rectangle::new(COLOR_GREEN).draw(graphics::rectangle::centered([min_x, min_y, dot_sx, dot_sy]), context, &mut self.gl);
        Rectangle::new(COLOR_GREEN).draw(graphics::rectangle::centered([min_x, max_y, dot_sx, dot_sy]), context, &mut self.gl);
        Rectangle::new(COLOR_GREEN).draw(graphics::rectangle::centered([max_x, min_y, dot_sx, dot_sy]), context, &mut self.gl);
        Rectangle::new(COLOR_GREEN).draw(graphics::rectangle::centered([max_x, max_y, dot_sx, dot_sy]), context, &mut self.gl);
    }

    fn update(&mut self, _: &mut Window, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

pub fn display(points: &[Point]) {
    if points.len() == 0 {
        return;
    }

    // Create an SDL window.
    let window = Window::new(
        OpenGL::_2_1,
        WindowSettings {
            title: "Colinear points".to_string(),
            size: [1024, 768],
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

    // Create a new game and run it.
    let mut app = App {
        gl: Gl::new(OpenGL::_2_1),
        points: points,
        bounds: bounds,
        rotation: 0.0
    };

    for e in events(&window) {
        if let Some(r) = e.render_args() {
            app.render(&mut window.borrow_mut(), &r);
        }

        if let Some(u) = e.update_args() {
            app.update(&mut window.borrow_mut(), &u);
        }
    }
}
