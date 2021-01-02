extern crate euclid;
extern crate rustybuzz;
extern crate webrender;
extern crate webrender_api;
// use font_kit::font;

use std::str::FromStr;
use crate::fragment::{Point, Rect, Size, TextFragment};
use crate::glyph::{GlyphData, GlyphStore};

mod fragment;
mod glyph;

pub fn layout_text(width: i32, text: String) -> Vec<TextFragment> {
    let point_size = 16.0;
    let line_height = 18;
    let font = font_kit::source::SystemSource::new()
        .select_by_postscript_name("monospace")
        .unwrap()
        .load()
        .unwrap();
    let data = font.copy_font_data().expect("Failed to load font data");
    let mut face = rustybuzz::Face::from_slice(&data, 0).unwrap();
    face.set_points_per_em(Some(point_size));
    let buzz_features = [rustybuzz::Feature::from_str("kern[0]").unwrap()];
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(&text);
    let glyph_buffer = rustybuzz::shape(&face, &buzz_features, buffer);

    let mut fragments = Vec::<TextFragment>::new();
    let mut store = GlyphStore::new();
    let mut cur_block = 0;

    for (info, position) in glyph_buffer.glyph_infos().iter().zip(glyph_buffer.glyph_positions()) {
        let data = GlyphData::new(info.codepoint, position.x_advance, position.x_offset, position.y_offset);
        if store.length() + position.x_advance <= width {
            store.add_glyph(data);
        } else {
            let size = Size::new(store.length(), line_height);
            let origin = Point::new(0, cur_block);
            let rect = Rect::new(origin, size);
            let fragment = TextFragment::new(store, rect);
            fragments.push(fragment);
            cur_block += line_height;
            store = GlyphStore::new();
        }
    }

    fragments
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
