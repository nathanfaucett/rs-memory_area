use memory_area::MemoryArea;


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
