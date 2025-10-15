use crate::font::{
    get_i16_be,
    get_i64_be,
    get_u16_be,
    get_u32_be,
    TrueTypeFont
};

#[derive(Debug, Copy, Clone)]
pub struct HeadTable {
    pub _major_version: u16,
    pub _minor_version: u16,
    pub _font_revision: u32,
    pub _checksum_adjustment: u32,
    pub _magic_number: u32, // 0x5F0F3CF5
    pub _flags: u16,
    pub units_per_em: u16,
    pub _created: i64,
    pub modified: i64,
    pub x_min: i16,
    pub y_min: i16,
    pub x_max: i16,
    pub y_max: i16,
    pub mac_style: u16,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: i16,
    pub index_to_loc_format: i16,
    pub glyph_data_format: i16,
}

impl HeadTable {
    pub fn new() -> Self {
        HeadTable {
            _major_version: 0,
            _minor_version: 0,
            _font_revision: 0,
            _checksum_adjustment: 0,
            _magic_number: 0,
            _flags: 0,
            units_per_em: 0,
            _created: 0,
            modified: 0,
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,
            mac_style: 0,
            lowest_rec_ppem: 0,
            font_direction_hint: 0,
            index_to_loc_format: 0,
            glyph_data_format: 0,
        }
    }
}

impl TrueTypeFont {
    pub fn load_head(&mut self, font_bytes: &[u8]) {
        for table in &self.tables {
            if table.table_tag == "head".as_bytes() {

                let offset = table.offset as usize;

                self.head = HeadTable {
                    _major_version: get_u16_be(font_bytes.as_ptr(), offset as isize),
                    _minor_version: get_u16_be(font_bytes.as_ptr(), offset as isize + 2),
                    _font_revision: get_u32_be(font_bytes.as_ptr(), offset as isize + 4),
                    _checksum_adjustment: get_u32_be(font_bytes.as_ptr(), offset as isize + 8),
                    _magic_number: get_u32_be(font_bytes.as_ptr(), offset as isize + 12),
                    _flags: get_u16_be(font_bytes.as_ptr(), offset as isize + 16),
                    units_per_em: get_u16_be(font_bytes.as_ptr(), offset as isize + 18),
                    _created: get_i64_be(font_bytes.as_ptr(), offset as isize + 20),
                    modified: get_i64_be(font_bytes.as_ptr(), offset as isize + 28),
                    x_min: get_i16_be(font_bytes.as_ptr(), offset as isize + 36),
                    y_min: get_i16_be(font_bytes.as_ptr(), offset as isize + 38),
                    x_max: get_i16_be(font_bytes.as_ptr(), offset as isize + 40),
                    y_max: get_i16_be(font_bytes.as_ptr(), offset as isize + 42),
                    mac_style: get_u16_be(font_bytes.as_ptr(), offset as isize + 44),
                    lowest_rec_ppem: get_u16_be(font_bytes.as_ptr(), offset as isize + 46),
                    font_direction_hint: get_i16_be(font_bytes.as_ptr(), offset as isize + 48),
                    index_to_loc_format: get_i16_be(font_bytes.as_ptr(), offset as isize + 50),
                    glyph_data_format: get_i16_be(font_bytes.as_ptr(), offset as isize + 52),
                };

                return;
            }
        }

        panic!("HEAD table not found!");
    }
}