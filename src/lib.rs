#![no_std]


#[repr(C)]
pub struct MemoryArea {
    base_address: usize,
    length: usize,
    typ: u32,
    reserved: u32,
}
impl MemoryArea {
    pub fn new(
        base_address: usize,
        length: usize,
        typ: u32,
        reserved: u32
    ) -> Self {
        MemoryArea {
            base_address: base_address,
            length: length,
            typ: typ,
            reserved: reserved,
        }
    }

    pub fn get_base_address(&self) -> usize { self.base_address }
    pub fn get_length(&self) -> usize { self.length }
    pub fn get_type(&self) -> u32 { self.typ }
    pub fn get_reserved(&self) -> u32 { self.reserved }
}

#[derive(Clone)]
pub struct MemoryAreaIter {
    current_area: *const MemoryArea,
    last_area: *const MemoryArea,
    entry_size: u32,
}
impl MemoryAreaIter {
    pub fn new(
        current_area: *const MemoryArea,
        last_area: *const MemoryArea,
        entry_size: u32,
    ) -> Self {
        MemoryAreaIter {
            current_area: current_area,
            last_area: last_area,
            entry_size: entry_size,
        }
    }
}

impl Iterator for MemoryAreaIter {
    type Item = &'static MemoryArea;
    fn next(&mut self) -> Option<&'static MemoryArea> {
        if self.current_area > self.last_area {
            None
        } else {
            let area = unsafe {
                &*self.current_area
            };

            self.current_area = ((self.current_area as u32) + self.entry_size) as *const MemoryArea;

            if area.get_type() == 1 {
                Some(area)
            } else {
                self.next()
            }
        }
    }
}
