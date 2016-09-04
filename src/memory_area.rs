

#[repr(C)]
pub struct MemoryArea {
    base_address: usize,
    length: usize,
    kind: u32,
    reserved: u32,
}
impl MemoryArea {
    pub fn new(
        base_address: usize,
        length: usize,
        kind: u32,
        reserved: u32
    ) -> Self {
        MemoryArea {
            base_address: base_address,
            length: length,
            kind: kind,
            reserved: reserved,
        }
    }

    pub fn get_base_address(&self) -> usize { self.base_address }
    pub fn get_length(&self) -> usize { self.length }
    pub fn get_type(&self) -> u32 { self.kind }
    pub fn get_kind(&self) -> u32 { self.kind }
    pub fn get_reserved(&self) -> u32 { self.reserved }
}
