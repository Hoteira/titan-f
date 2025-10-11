use crate::font::{get_u16_be, get_u32_be, TrueTypeFont};

#[derive(Debug, Copy, Clone)]
pub struct MaxpTable {
    pub version: u32,
    pub num_glyphs: u16,
    
    pub max_points: u16,
    pub max_contours: u16,
    pub max_composite_points: u16,
    pub max_composite_contours: u16,
    pub max_zones: u16,
    pub max_twilight_points: u16,
    pub max_storage: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_stack_elements: u16,
    pub max_size_of_instructions: u16,
    pub max_component_elements: u16,
    pub max_component_depth: u16,
}

impl MaxpTable {
    pub fn new() -> Self {
        MaxpTable {
            version: 0,
            num_glyphs: 0,

            max_points: 0,
            max_contours: 0,
            max_composite_points: 0,
            max_composite_contours: 0,
            max_zones: 0,
            max_twilight_points: 0,
            max_storage: 0,
            max_function_defs: 0,
            max_instruction_defs: 0,
            max_stack_elements: 0,
            max_size_of_instructions: 0,
            max_component_elements: 0,
            max_component_depth: 0,
        }
    }
}

impl TrueTypeFont {
    pub fn load_maxp(&mut self, font_bytes: &[u8]) {
        for table in &self.tables {
            if table.table_tag == "maxp".as_bytes() {
                let offset = table.offset as usize;

                self.maxp = MaxpTable {
                    version: get_u32_be(font_bytes.as_ptr(), offset as isize),
                    num_glyphs: get_u16_be(font_bytes.as_ptr(), offset as isize + 4),

                    max_points: get_u16_be(font_bytes.as_ptr(), offset as isize + 6),
                    max_contours: get_u16_be(font_bytes.as_ptr(), offset as isize + 8),
                    max_composite_points: get_u16_be(font_bytes.as_ptr(), offset as isize + 10),
                    max_composite_contours: get_u16_be(font_bytes.as_ptr(), offset as isize + 12),
                    max_zones: get_u16_be(font_bytes.as_ptr(), offset as isize + 14),
                    max_twilight_points: get_u16_be(font_bytes.as_ptr(), offset as isize + 16),
                    max_storage: get_u16_be(font_bytes.as_ptr(), offset as isize + 18),
                    max_function_defs: get_u16_be(font_bytes.as_ptr(), offset as isize + 20),
                    max_instruction_defs: get_u16_be(font_bytes.as_ptr(), offset as isize + 22),
                    max_stack_elements: get_u16_be(font_bytes.as_ptr(), offset as isize + 24),
                    max_size_of_instructions: get_u16_be(font_bytes.as_ptr(), offset as isize + 26),
                    max_component_elements: get_u16_be(font_bytes.as_ptr(), offset as isize + 28),
                    max_component_depth: get_u16_be(font_bytes.as_ptr(), offset as isize + 30),
                };

                return;
            }
        }

        panic!("MAXP table not found");
    }
}