
#[cfg(feature = "std")]
use std::ptr::read_unaligned;

#[cfg(not(feature = "std"))]
use core::ptr::read_unaligned;

#[cfg(feature = "std")]
use std::mem::size_of;

#[cfg(not(feature = "std"))]
use core::mem::size_of;


use crate::Vec;
use crate::Map;
use crate::tables::cmap::CmapTable;
use crate::tables::glyf::Glyph;
use crate::tables::head::HeadTable;
use crate::tables::hhea::HheaTable;
use crate::tables::hmtx::HmtxTable;
use crate::tables::loca::LocaTable;
use crate::tables::maxp::MaxpTable;

#[derive(Copy, Clone, Debug)]
pub struct OffsetTable {
    pub _scaler_type: u32,
    pub num_tables: u16,
    pub _search_range: u16,
    pub _entry_selector: u16,
    pub _range_shift: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct TableRecord {
    pub table_tag: [u8; 4],
    pub check_sum: u32,
    pub offset: u32,
    pub length: u32,
}

impl TableRecord {
    pub fn new() -> TableRecord {
        TableRecord {
            table_tag: [0; 4],
            check_sum: 0,
            offset: 0,
            length: 0,
        }
    }
}


pub struct TrueTypeFont {
    pub offset_table: OffsetTable,
    pub tables: Vec<TableRecord>,
    pub cmap: CmapTable,
    pub head: HeadTable,
    pub loca: LocaTable,
    pub maxp: MaxpTable,
    pub glyf: TableRecord,
    pub hhea: HheaTable,
    pub hmtx: HmtxTable,

    pub glyph_data_table: Map<u32, Glyph>,
    pub glyph_id_table: Map<char, u32>,
    pub kern_table: Map<(u32, u32), i16>,

    pub cache: crate::cache::Cache,
    pub can_cache: bool,
    
    pub winding_buffer: Vec<f32>,
    pub bitmap_buffer: Vec<u8>,
}



impl OffsetTable {
    pub fn new() -> OffsetTable {
        OffsetTable {
            _scaler_type: 0,
            num_tables: 0,
            _search_range: 0,
            _entry_selector: 0,
            _range_shift: 0,
        }
    }
}

impl TrueTypeFont {
    pub fn new() -> Self {
        TrueTypeFont {
            offset_table: OffsetTable::new(),
            tables: Vec::new(),
            cmap: CmapTable::new(),
            head: HeadTable::new(),
            loca: LocaTable::Short(Vec::new()),
            maxp: MaxpTable::new(),
            glyf: TableRecord::new(),
            hhea: HheaTable::new(),
            hmtx: HmtxTable::new(),

            glyph_data_table: Map::new(),
            glyph_id_table: Map::new(),
            kern_table: Map::new(),

            cache: crate::cache::Cache::new(),
            can_cache: true,

            winding_buffer: Vec::new(),
            bitmap_buffer: Vec::new(),
        }
    }

    pub fn load_offset_table(&mut self, font_bytes: &[u8]) {
        if font_bytes.len() >= 12 {
            self.offset_table = OffsetTable {
                _scaler_type: get_u32_be(font_bytes.as_ptr(), 0),
                num_tables: get_u16_be(font_bytes.as_ptr(), 4),
                _search_range: get_u16_be(font_bytes.as_ptr(), 6),
                _entry_selector: get_u16_be(font_bytes.as_ptr(), 8),
                _range_shift: get_u16_be(font_bytes.as_ptr(), 10),
            }
        } else {
            panic!("Invalid font file");
        }
    }

    pub fn load_tables(&mut self, font_bytes: &[u8]) {
        let mut offset = size_of::<OffsetTable>();

        for _i in 0..self.offset_table.num_tables {
            let table = TableRecord {
                table_tag: [font_bytes[offset], font_bytes[offset + 1], font_bytes[offset + 2], font_bytes[offset + 3]],
                check_sum: get_u32_be(font_bytes.as_ptr(), offset as isize + 4),
                offset: get_u32_be(font_bytes.as_ptr(), offset as isize + 8),
                length: get_u32_be(font_bytes.as_ptr(), offset as isize + 12),
            };

            self.tables.push(table);
            offset += size_of::<TableRecord>();
        }
    }

    pub fn load_font(font_bytes: &[u8]) -> Self {
        let mut font = Self::new();


        font.load_offset_table(&font_bytes);
        font.load_tables(&font_bytes);

        font.load_cmap(&font_bytes);
        font.load_cmap_encodings(&font_bytes);
        font.load_cmap_subtable_formats(&font_bytes);
        font.load_cmap_subtables(&font_bytes);
        font.load_hhea(&font_bytes);

        font.load_head(&font_bytes);
        font.load_maxp(&font_bytes);
        font.load_loca(&font_bytes);
        font.load_glyf();
        font.load_hmtx(&font_bytes);

        font.cache_all_glyphs(&font_bytes);
        font.load_kerning_pairs(&font_bytes);

        font
    }
}

#[inline]
pub fn get_u32_be(base: *const u8, offset: isize) -> u32 {
    unsafe { u32::from_be(read_unaligned(base.offset(offset) as *const u32)) }
}

#[inline]
pub fn get_u16_be(base: *const u8, offset: isize) -> u16 {
    unsafe { u16::from_be(read_unaligned(base.offset(offset) as *const u16)) }
}

#[inline]
pub fn get_i16_be(base: *const u8, offset: isize) -> i16 {
    unsafe { i16::from_be(read_unaligned(base.offset(offset) as *const i16)) }
}

#[inline]
pub fn get_i64_be(base: *const u8, offset: isize) -> i64 {
    unsafe { i64::from_be(read_unaligned(base.offset(offset) as *const i64)) }
}