pub struct Line {
    p1: (f32, f32),
    p2: (f32, f32),
}

pub struct Strip {
    lines: Vec<Line>,
    max: (f32, f32),
    size: (f32, f32),
}

impl Strip {
    pub fn new(width: f32, height: f32) -> Self {
        Strip {
            lines: vec![],
            max: (0.0, 0.0),
            size: (width, height),
        }
    }

    pub fn add_line(&mut self, p: (f32, f32)) {
        let p1 = match self.lines.last_mut() {
            Some(line) => line.last_point(),
            None => (0.0, 0.0),
        };

        self.lines.push(Line::new(p1, p));

        if self.max.0 < p.0 {
            self.max.0 = p.0;
        }
        if self.max.1 < p.1 {
            self.max.1 = p.1;
        }
    }

    pub fn draw(&self) {
        for line in &self.lines {
            line.draw(self.max, self.size);
        }
    }
}

impl Line {
    pub fn new(p1: (f32, f32), p2: (f32, f32)) -> Self {
        Line { p1, p2 }
    }

    pub fn draw(&self, max: (f32, f32), size: (f32, f32)) {
        let p1 = (self.p1.0 / max.0 * size.0, self.p1.1 / max.1 * size.1);
        let p2 = (self.p2.0 / max.0 * size.0, self.p2.1 / max.1 * size.1);

        unsafe {
            glu_sys::glBegin(glu_sys::GL_LINES);
            glu_sys::glVertex2f(p1.0, p1.1);
            glu_sys::glVertex2f(p2.0, p2.1);
            glu_sys::glEnd();
        }
    }

    pub fn last_point(&self) -> (f32, f32) {
        self.p2
    }
}
