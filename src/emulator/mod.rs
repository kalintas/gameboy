pub mod cpu;
mod instructions;
pub mod memory_map;
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

    pub fn after_boot() -> Self {
        Self {
            cpu: Cpu::after_boot(),
            memory_map: MemoryMap::after_boot(),
        }
    }

    pub fn load_cartidge(&mut self, path: impl AsRef<Path>) {
        self.memory_map.load_rom(path);
    }

    fn handle_interrupt(&mut self) -> Option<(u16, u8)> {
        /*
            From pandocs:
            Provided that IME and IE allow the execution of more than one of the requested interrupts, 
            then the interrupt with the highest priority becomes executed first. 
            The priorities are ordered as the bits in the IE and IF registers, 
            Bit 0 (V-Blank) having the highest priority, and Bit 4 (Joypad) having the lowest priority.
        */
        
        let ie_reg = self.memory_map.get(0xFFFF);
        let if_reg = self.memory_map.get(0xFF0F);
        let interrupt = ie_reg & if_reg;

        if interrupt & 0x1 != 0 {
            // V-Blank interrupt
            println!("VBLANK");
            return Some((0x40, if_reg & 0x1E));
        } else if interrupt & 0x2 != 0 {
            // LCD STAT interrupt 
            return Some((0x48, if_reg & 0xFD));
        } else if interrupt & 0x4 != 0 {
            // Timer interrupt
            return Some((0x50, if_reg & 0xFB));
        } else if interrupt & 0x8 != 0 {
            // Serial interrupt
            return Some((0x58, if_reg & 0xF7));
        } else if interrupt & 0x10 != 0 {
            // Joypad interrupt
            return Some((0x60, if_reg & 0xEF));
        }

        return None;
    }

    pub fn handle_interrupts(&mut self) -> bool {

        if !self.cpu.ime {
            return false;
        }

        if let Some((interrupt_address, new_if_reg)) = self.handle_interrupt() {

            self.cpu.ime = false;

            self.cpu.call(&mut self.memory_map, interrupt_address);
            
            // Replace the if flag.
            self.memory_map.set(0xFF0F, new_if_reg);

            // TODO: "The entire routine should last a total of 5 M-cycles." 5 M-cycles =? 20
            self.cpu.clock_cycles += 20;
            
            true
        } else {
            false
        }
    }

    pub fn cycle(&mut self) {

        if !self.handle_interrupts() {
            self.cpu.cycle(&mut self.memory_map);
        }
    }
}
