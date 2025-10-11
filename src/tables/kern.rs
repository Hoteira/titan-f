use crate::font::{
    get_i16_be,
    get_u16_be,
    TrueTypeFont
};

impl TrueTypeFont {
    pub fn load_kerning_pairs(&mut self, font_bytes: &[u8]) {
        for table in &self.tables {
            if table.table_tag == "kern".as_bytes() {

                let offset = table.offset as usize;

                let version = get_u16_be(font_bytes.as_ptr(), offset as isize);
                let n_tables = get_u16_be(font_bytes.as_ptr(), (offset + 2) as isize);

                let mut subtable_offset = offset + 4;

                for _ in 0..n_tables {
                    let version = get_u16_be(font_bytes.as_ptr(), subtable_offset as isize);
                    let length = get_u16_be(font_bytes.as_ptr(), (subtable_offset + 2) as isize);
                    let coverage = get_u16_be(font_bytes.as_ptr(), (subtable_offset + 4) as isize);

                    let format = coverage >> 8;
                    let horizontal = (coverage & 0x01) != 0;

                    if format == 0 && horizontal {
                        let n_pairs = get_u16_be(font_bytes.as_ptr(), (subtable_offset + 6) as isize);
                        let mut pair_offset = subtable_offset + 14;

                        for _ in 0..n_pairs {
                            let left = get_u16_be(font_bytes.as_ptr(), pair_offset as isize);
                            let right = get_u16_be(font_bytes.as_ptr(), (pair_offset + 2) as isize);
                            let value = get_i16_be(font_bytes.as_ptr(), (pair_offset + 4) as isize);

                            self.kern_table.insert((left as u32, right as u32), value);
                            pair_offset += 6;
                        }
                    }

                    subtable_offset += length as usize;
                }

                return;
            }
        }
    }

    pub fn get_kerning_by_id(&self, left: u32, right: u32) -> Option<&i16> {
        self.kern_table.get(&(left, right))
    }

    pub fn get_kerning(&self, left: char, right: char) -> Option<&i16> {
        let left = self.glyph_id_table.get(&left).unwrap_or(&0);
        let right = self.glyph_id_table.get(&right).unwrap_or(&0);
        self.kern_table.get(&(*left, *right))
    }
}