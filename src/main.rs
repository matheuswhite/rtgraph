extern crate glfw;
extern crate glu_sys;

use crate::{grid::Grid, line::Strip};
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use std::{io, sync::mpsc::Receiver};

mod grid;
mod line;

struct App {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    strip: Strip,
    line_rx: Receiver<(f32, f32)>,
    grid: Grid,
}

impl App {
    const WIDTH: f32 = 500.0;
    const HEIGHT: f32 = 250.0;

    fn new(line_rx: Receiver<(f32, f32)>) -> Self {
        let mut glfw = glfw::init_no_callbacks().unwrap();

        // Set OpenGL version hints BEFORE creating the window
        glfw.window_hint(glfw::WindowHint::ContextVersion(2, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Any,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(false));

        let (mut window, events) = glfw
            .create_window(
                Self::WIDTH as u32,
                Self::HEIGHT as u32,
                "RTGraph",
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        window.make_current();
        window.set_key_polling(true);
        window.show();

        Self::setup_opengl();

        Self {
            glfw,
            window,
            events,
            strip: Strip::new(Self::WIDTH, Self::HEIGHT),
            line_rx,
            grid: Grid::new(Self::WIDTH, Self::HEIGHT, 100.0),
        }
    }

    fn setup_opengl() {
        unsafe {
            glu_sys::glClearColor(0.0, 0.1, 0.1, 1.0);
            glu_sys::glMatrixMode(glu_sys::GL_PROJECTION);
            glu_sys::gluOrtho2D(0.0, Self::WIDTH as f64, 0.0, Self::HEIGHT as f64);
        }
    }

    fn render(&self) {
        unsafe {
            glu_sys::glClear(glu_sys::GL_COLOR_BUFFER_BIT);

            self.grid.draw();

            glu_sys::glColor3f(1.0, 1.0, 0.0);
            self.strip.draw();
        }
    }

    fn process_events(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }
    }

    fn run(&mut self) {
        while !self.window.should_close() {
            while let Some(point) = self.line_rx.try_recv().ok() {
                self.strip.add_line(point);
            }

            self.window.swap_buffers();
            self.process_events();
            self.render();
        }
    }
}

fn main() {
    let (line_tx, line_rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        // Read lines from stdin
        for line in io::stdin().lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() != 2 {
                continue;
            }

            if let (Ok(x), Ok(y)) = (parts[0].parse::<f32>(), parts[1].parse::<f32>()) {
                line_tx.send((x, y)).unwrap();
            }
        }
    });

    let mut app = App::new(line_rx);

    app.run();
}
