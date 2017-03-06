

#[repr(C)]
pub struct MemoryArea {
    base_address: u32,
    length: u32,
    kind: u32,
    reserved: u32,
}
impl MemoryArea {
    #[inline(always)]
    pub fn new(
        base_address: u32,
        length: u32,
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

    #[inline(always)]
    pub fn base_address(&self) -> u32 { self.base_address }
    #[inline(always)]
    pub fn length(&self) -> u32 { self.length }
    #[inline(always)]
    pub fn kind(&self) -> u32 { self.kind }
    #[inline(always)]
    pub fn reserved(&self) -> u32 { self.reserved }
}
