pub struct Grid {
    width: f32,
    height: f32,
    cell_size: f32,
}

impl Grid {
    pub fn new(width: f32, height: f32, cell_size: f32) -> Self {
        Grid {
            width,
            height,
            cell_size,
        }
    }

    pub fn draw(&self) {
        unsafe {
            glu_sys::glColor3f(0.5, 0.5, 0.5);
            glu_sys::glBegin(glu_sys::GL_LINES);

            for x in (0..=(self.width as i32)).step_by(self.cell_size as usize) {
                glu_sys::glVertex2f(x as f32, 0.0);
                glu_sys::glVertex2f(x as f32, self.height);
            }

            for y in (0..=(self.height as i32)).step_by(self.cell_size as usize) {
                glu_sys::glVertex2f(0.0, y as f32);
                glu_sys::glVertex2f(self.width, y as f32);
            }

            glu_sys::glEnd();
        }
    }
}
