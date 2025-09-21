extern crate glfw;
extern crate glu_sys;

use crate::{grid::Grid, line::Strip};
use core::str;
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use std::{
    io,
    sync::mpsc::{Receiver, Sender, channel},
};

mod font;
mod grid;
mod line;

struct App {
    glfw: Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    strips: Vec<Strip>,
    line_rx: Receiver<(f32, Vec<f32>)>,
    command_rx: Receiver<Command>,
    grid: Grid,
    color_pool: Vec<(f32, f32, f32)>,
}

enum Command {
    Save(String),
}

impl App {
    const WIDTH: f32 = 500.0;
    const HEIGHT: f32 = 250.0;

    fn new(
        line_rx: Receiver<(f32, Vec<f32>)>,
        command_rx: Receiver<Command>,
        title: String,
        x_step: f32,
        y_step: f32,
    ) -> Self {
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
                &title,
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
            strips: vec![],
            line_rx,
            command_rx,
            grid: Grid::new(Self::WIDTH, Self::HEIGHT, x_step, y_step),
            color_pool: vec![
                (1.0, 1.0, 0.0),
                (0.0, 1.0, 0.0),
                (0.0, 0.0, 1.0),
                (1.0, 1.0, 0.0),
                (1.0, 0.0, 1.0),
                (0.0, 1.0, 1.0),
                (1.0, 0.5, 0.0),
                (0.5, 1.0, 0.0),
                (0.5, 0.5, 1.0),
            ],
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

            for strip in &self.strips {
                strip.draw(&self.grid);
            }
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
            while let Some((time, values)) = self.line_rx.try_recv().ok() {
                for (i, value) in values.iter().enumerate() {
                    let strip = if i < self.strips.len() {
                        &mut self.strips[i]
                    } else {
                        self.strips
                            .push(Strip::new(self.color_pool[i % self.color_pool.len()]));
                        self.strips.last_mut().unwrap()
                    };
                    strip.add_line((time, *value), &mut self.grid);
                }
            }

            self.grid.build_texts();

            self.window.swap_buffers();
            self.process_events();
            self.render();

            while let Some(command) = self.command_rx.try_recv().ok() {
                match command {
                    Command::Save(filepath) => {
                        let (w, h) = self.window.get_framebuffer_size();
                        let w = w as u32;
                        let h = h as u32;
                        let frame_buffer = read_frame_buffer(w, h);

                        if let Err(e) = image::save_buffer(
                            &filepath,
                            &frame_buffer,
                            w,
                            h,
                            image::ColorType::Rgba8,
                        ) {
                            eprintln!("Failed to save {}: {}", filepath, e);
                        } else {
                            println!("Screenshot saved to {}", filepath);
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let (line_tx, line_rx) = channel();
    let (command_tx, command_rx) = channel();

    let mut x_step = 1.0;
    let mut y_step = 5.0;
    let mut title = String::from("RTGraph");

    let mut args = std::env::args();
    /* skip program name */
    args.next();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: rtgraph [x_step] [y_step] [-t|--title <title>]");
                return;
            }
            "-t" | "--title" => {
                let Some(t) = args.next() else {
                    panic!("Title not provided after -t/--title");
                };

                title = t;
            }
            "-xs" | "--xstep" => {
                let Some(xs) = args.next() else {
                    panic!("X step not provided after -x/--xstep");
                };

                x_step = xs.parse::<f32>().unwrap_or(1.0);
            }
            "-ys" | "--ystep" => {
                let Some(ys) = args.next() else {
                    panic!("Y step not provided after -y/--ystep");
                };

                y_step = ys.parse::<f32>().unwrap_or(5.0);
            }
            _ => {
                panic!("Unknown argument: {}", arg);
            }
        }
    }

    std::thread::spawn(move || {
        /* Read lines from stdin */
        for line in io::stdin().lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(",").collect();

            handle_save_command(&parts, &command_tx).or_else(|| handle_line_plot(&parts, &line_tx));
        }
    });

    let mut app = App::new(line_rx, command_rx, title, x_step, y_step);

    app.run();
}

fn read_frame_buffer(fb_width: u32, fb_height: u32) -> Vec<u8> {
    let fb_width = fb_width as u32;
    let fb_height = fb_height as u32;

    // Allocate correct buffer size: width * height * 4 (RGBA)
    let mut frame_buffer = vec![0u8; (fb_width * fb_height * 4) as usize];

    unsafe {
        glu_sys::glReadPixels(
            0,
            0,
            fb_width as i32,
            fb_height as i32,
            glu_sys::GL_RGBA,
            glu_sys::GL_UNSIGNED_BYTE,
            frame_buffer.as_mut_ptr() as *mut std::ffi::c_void,
        );
    }

    // Flip vertically (OpenGL origin is bottom-left, images are top-left)
    let mut flipped_pixels = vec![0u8; frame_buffer.len()];
    let bytes_per_row = fb_width as usize * 4;
    for i in 0..fb_height as usize {
        for j in 0..bytes_per_row {
            flipped_pixels[(fb_height as usize - 1 - i) * bytes_per_row + j] =
                frame_buffer[i * bytes_per_row + j];
        }
    }

    flipped_pixels
}

fn handle_save_command(line: &[&str], command_tx: &Sender<Command>) -> Option<()> {
    if let ["!save", filepath] = line {
        command_tx
            .send(Command::Save(filepath.to_string()))
            .unwrap();

        return Some(());
    }

    None
}

fn handle_line_plot(line: &[&str], line_tx: &Sender<(f32, Vec<f32>)>) -> Option<()> {
    if line.len() < 2 {
        return None;
    }

    if let (Ok(x), y) = (
        line[0].parse::<f32>(),
        line[1..]
            .iter()
            .filter_map(|v| v.parse::<f32>().ok())
            .collect(),
    ) {
        line_tx.send((x, y)).unwrap();
        return Some(());
    }

    None
}
