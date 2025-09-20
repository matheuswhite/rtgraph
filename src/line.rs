use crate::grid::Grid;

pub struct Line {
    p1: (f32, f32),
    p2: (f32, f32),
}

pub struct Strip {
    color: (f32, f32, f32),
    lines: Vec<Line>,
}

impl Strip {
    pub fn new(color: (f32, f32, f32)) -> Self {
        Strip {
            color,
            lines: vec![],
        }
    }

    pub fn add_line(&mut self, p: (f32, f32), grid: &mut Grid) {
        let p1 = match self.lines.last_mut() {
            Some(line) => line.last_point(),
            None => (0.0, 0.0),
        };

        self.lines.push(Line::new(p1, p));

        grid.update_bounds(p);
    }

    pub fn draw(&self, grid: &Grid) {
        unsafe {
            glu_sys::glColor3f(self.color.0, self.color.1, self.color.2);
        }
        for line in &self.lines {
            line.draw(grid);
        }
    }
}

impl Line {
    pub fn new(p1: (f32, f32), p2: (f32, f32)) -> Self {
        Line { p1, p2 }
    }

    pub fn draw(&self, grid: &Grid) {
        let p1 = grid.to_screen(self.p1);
        let p2 = grid.to_screen(self.p2);

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
