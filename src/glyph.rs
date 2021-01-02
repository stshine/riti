use euclid::default::Point2D;

#[derive(Copy, Clone)]
pub struct GlyphData {
    id: u32,
    advance: i32,
    offset: Point2D<i32>,
}

impl GlyphData {
    pub fn new(id: u32, advance: i32, offset_x:i32, offset_y: i32) -> Self {
        GlyphData {
            id,
            advance,
            offset: Point2D::new(offset_x, offset_y)
        }
    }
}

pub struct GlyphStore {
    length: i32,
    glyphs: Vec<GlyphData>,
}

impl GlyphStore {
    pub fn new() -> Self {
        GlyphStore {
            length: 0,
            glyphs: Vec::new(),
        }
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn add_glyph(&mut self, data: GlyphData) {
        self.length += data.advance;
        self.glyphs.push(data);
    }
}