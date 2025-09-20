use crate::font::{Font, Text};

type ScreenF32 = f32;
type SignalF32 = f32;

pub struct Grid {
    width: ScreenF32,
    height: ScreenF32,
    x_step: SignalF32,
    y_step: SignalF32,
    max: (SignalF32, SignalF32),
    min: (SignalF32, SignalF32),
    font: Font,
    texts: Vec<(f32, f32, Text)>,
}

impl Grid {
    pub fn new(width: ScreenF32, height: ScreenF32, x_step: SignalF32, y_step: SignalF32) -> Self {
        Grid {
            width,
            height,
            x_step,
            y_step,
            max: (x_step * 0.5, y_step * 0.5),
            min: (-x_step * 0.5, -y_step * 0.5),
            font: Font::new(16.0, (1.0, 1.0, 1.0)),
            texts: vec![],
        }
    }

    pub fn to_screen(&self, point: (SignalF32, SignalF32)) -> (ScreenF32, ScreenF32) {
        (
            (point.0 - self.min.0) / (self.max.0 - self.min.0) * self.width,
            (point.1 - self.min.1) / (self.max.1 - self.min.1) * self.height,
        )
    }

    pub fn update_bounds(&mut self, point: (SignalF32, SignalF32)) {
        if self.max.0 < point.0 + self.x_step {
            self.max.0 = point.0 + self.x_step;
        }
        if self.max.1 <= point.1 + self.y_step * 0.5 {
            self.max.1 = point.1 + self.y_step * 0.5;
        }
        if self.min.0 >= point.0 - self.x_step {
            self.min.0 = point.0 - self.x_step;
        }
        if self.min.1 >= point.1 - self.y_step * 0.5 {
            self.min.1 = point.1 - self.y_step * 0.5;
        }
    }

    pub fn build_texts(&mut self) {
        self.texts.clear();

        let mut x = 0.0;
        while x <= self.max.0 {
            let text = self.font.build_text(&format!("{:.2}", x));
            self.texts.push((
                (x - self.min.0) / (self.max.0 - self.min.0) * self.width,
                0.0,
                text,
            ));
            x += self.x_step;
        }
        x = 0.0;
        while x >= self.min.0 {
            let text = self.font.build_text(&format!("{:.2}", x));
            self.texts.push((
                (x - self.min.0) / (self.max.0 - self.min.0) * self.width,
                0.0,
                text,
            ));
            x -= self.x_step;
        }

        let mut y = 0.0;
        while y <= self.max.1 {
            let text = self.font.build_text(&format!("{:.2}", y));
            self.texts.push((
                0.0,
                (y - self.min.1) / (self.max.1 - self.min.1) * self.height,
                text,
            ));
            y += self.y_step;
        }
        y = 0.0;
        while y >= self.min.1 {
            let text = self.font.build_text(&format!("{:.2}", y));
            self.texts.push((
                0.0,
                (y - self.min.1) / (self.max.1 - self.min.1) * self.height,
                text,
            ));
            y -= self.y_step;
        }
    }

    pub fn draw(&self) {
        unsafe {
            glu_sys::glColor3f(0.5, 0.5, 0.5);
            glu_sys::glBegin(glu_sys::GL_LINES);

            let mut x = 0.0;
            while x <= self.max.0 {
                glu_sys::glVertex2f(
                    (x - self.min.0) / (self.max.0 - self.min.0) * self.width,
                    0.0,
                );
                glu_sys::glVertex2f(
                    (x - self.min.0) / (self.max.0 - self.min.0) * self.width,
                    self.height,
                );
                x += self.x_step;
            }
            x = 0.0;
            while x >= self.min.0 {
                glu_sys::glVertex2f(
                    (x - self.min.0) / (self.max.0 - self.min.0) * self.width,
                    0.0,
                );
                glu_sys::glVertex2f(
                    (x - self.min.0) / (self.max.0 - self.min.0) * self.width,
                    self.height,
                );
                x -= self.x_step;
            }

            let mut y = 0.0;
            while y <= self.max.1 {
                glu_sys::glVertex2f(
                    0.0,
                    (y - self.min.1) / (self.max.1 - self.min.1) * self.height,
                );
                glu_sys::glVertex2f(
                    self.width,
                    (y - self.min.1) / (self.max.1 - self.min.1) * self.height,
                );
                y += self.y_step;
            }
            y = 0.0;
            while y >= self.min.1 {
                glu_sys::glVertex2f(
                    0.0,
                    (y - self.min.1) / (self.max.1 - self.min.1) * self.height,
                );
                glu_sys::glVertex2f(
                    self.width,
                    (y - self.min.1) / (self.max.1 - self.min.1) * self.height,
                );
                y -= self.y_step;
            }

            glu_sys::glEnd();
        }

        for (x, y, text) in &self.texts {
            text.draw(*x, *y);
        }
    }
}
