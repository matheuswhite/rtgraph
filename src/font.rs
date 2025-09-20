use rusttype::{PositionedGlyph, Scale};

static FONT_DATA: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");

pub struct Font {
    color: (f32, f32, f32),
    font: rusttype::Font<'static>,
    scale: Scale,
}

impl Font {
    pub fn new(size: f32, color: (f32, f32, f32)) -> Self {
        let font = rusttype::Font::try_from_bytes(FONT_DATA).unwrap();
        let scale = Scale::uniform(size);

        Font { color, font, scale }
    }

    pub fn build_text(&self, text: &str) -> Text {
        let v_metrics = self.font.v_metrics(self.scale);
        let glyphs = self
            .font
            .layout(text, self.scale, rusttype::point(0.0, v_metrics.ascent))
            .collect::<Vec<_>>();

        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        Text {
            glyphs,
            glyphs_size: (glyphs_width, glyphs_height),
            color: self.color,
        }
    }
}

pub struct Text {
    glyphs: Vec<PositionedGlyph<'static>>,
    glyphs_size: (u32, u32),
    color: (f32, f32, f32),
}

impl Text {
    pub fn draw(&self, x: f32, y: f32) {
        for glyph in &self.glyphs {
            let Some(bb) = glyph.pixel_bounding_box() else {
                continue;
            };

            glyph.draw(|gx, gy, v| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;

                let px = x as i32 + gx;
                let py = y as i32 + (self.glyphs_size.1 as i32 - gy);

                unsafe {
                    glu_sys::glColor4f(v, v, v, v);
                    glu_sys::glBegin(glu_sys::GL_POINTS);
                    glu_sys::glVertex2i(px, py);
                    glu_sys::glEnd();
                }
            });
        }
    }
}
