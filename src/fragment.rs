use crate::glyph::GlyphStore;

pub struct Size {
    width: i32,
    height: i32
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Size {
            width,
            height,
        }
    }
}

pub struct Point {
    inline: i32,
    block: i32,
}

impl Point {
    pub fn new(inline: i32, block: i32) -> Self {
        Point {
            inline,
            block
        }
    }
}

pub struct Rect {
    origin: Point,
    size: Size
}

impl Rect {
    pub fn new(origin: Point, size: Size) -> Self {
        Rect {
            origin,
            size
        }
    }
}

pub struct TextFragment {
    glyphs: GlyphStore,
    rect: Rect,
}

impl TextFragment {
    pub fn new(store: GlyphStore, rect: Rect) -> Self {
        TextFragment {
            glyphs: store,
            rect,
        }
    }
}