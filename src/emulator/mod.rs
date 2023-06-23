mod cpu;
pub mod instructions;
mod memory_map;
mod registers;

use std::path::Path;

use cpu::Cpu;
use memory_map::MemoryMap;

pub struct Emulator {
    pub cpu: Cpu,

    pub memory_map: MemoryMap,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory_map: MemoryMap::new(),
        }
    }

    pub fn load_cartidge(&mut self, path: impl AsRef<Path>) {
        self.memory_map.load_rom(path);
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.memory_map);
    }
}
