use std::ops::Range;

pub struct Primitives {}

impl Primitives {

    pub fn sizeof_u8() -> usize {
        size_of::<u8>()
    }

    pub fn get_u8(buffer: &[u8], r: &mut Range<usize>) -> u8 {

        r.end = r.start + Self::sizeof_u8();
        let value = &buffer[r.clone()];
        r.start = r.end;

        value[0]
    }

    pub fn sizeof_u16() -> usize {
        size_of::<u16>()
    }

    pub fn get_u16(buffer: &[u8], r: &mut Range<usize>) -> u16 {

        r.end = r.start + Self::sizeof_u16();
        let value = &buffer[r.clone()];
        r.start = r.end;

        value[0] as u16 | ((value[1] as u16) << 8)
    }

    pub fn sizeof_u32() -> usize {
        size_of::<u32>()
    }

    pub fn get_u32(buffer: &[u8], r: &mut Range<usize>) -> u32 {
        
        r.end = r.start + Self::sizeof_u32();
        let value = &buffer[r.clone()];
        r.start = r.end;

        value[0] as u32 | ((value[1] as u32) << 8) | ((value[2] as u32) << 16) | ((value[3] as u32) << 24)
    }
}
