pub mod cpu;
mod instructions;
pub mod memory_map;
pub mod ppu;
mod registers;

use std::path::Path;

use cpu::Cpu;
use memory_map::MemoryMap;
use ppu::Ppu;

use self::memory_map::Io;

use std::ops::BitOr;

/*
    TODO:
    https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html
*/

const CPU_CLOCK_RATE: u32 = 4_194_304;

const DIV_REGISTER_CLOCK_RATE: u32 = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoypadKeys(u8);

impl JoypadKeys {
    pub const NONE: Self = Self(0x0);

    pub const BUTTONA: Self = Self(0x1);
    pub const BUTTONB: Self = Self(0x2);
    pub const SELECT: Self = Self(0x4);
    pub const START: Self = Self(0x8);

    pub const RIGHT: Self = Self(0x10);
    pub const LEFT: Self = Self(0x20);
    pub const UP: Self = Self(0x40);
    pub const DOWN: Self = Self(0x80);
}

impl BitOr for JoypadKeys {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { 0: self.0 | rhs.0 }
    }
}

impl AsRef<str> for JoypadKeys {
    fn as_ref(&self) -> &str {
        match *self {
            Self::BUTTONA => "Button A",
            Self::BUTTONB => "Button B",
            Self::SELECT => "Select",
            Self::START => "Start",
            Self::RIGHT => "Right",
            Self::LEFT => "Left",
            Self::UP => "Up",
            Self::DOWN => "Down",
            _ => "",
        }
    }
}

pub struct Emulator {
    pub cpu: Cpu,
    pub ppu: Ppu,

    pub memory_map: MemoryMap,

    base_clock: u32,
    dma_transfer_start: Option<u32>,

    joypad_keys: JoypadKeys,
}

impl Emulator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),

            memory_map: MemoryMap::new(),

            base_clock: 0,
            dma_transfer_start: None,

            joypad_keys: JoypadKeys::NONE,
        }
    }

    pub fn after_boot() -> Self {
        Self {
            cpu: Cpu::after_boot(),
            ppu: Ppu::new(),

            memory_map: MemoryMap::after_boot(),

            base_clock: 0,
            dma_transfer_start: None,

            joypad_keys: JoypadKeys::NONE,
        }
    }

    pub fn load_cartidge(&mut self, path: impl AsRef<Path>) {
        self.memory_map.load_rom(path);
    }

    fn increment_tima(&mut self) {
        let tac = self.memory_map.get_io(Io::TAC);

        // Check if the timer is stopped.
        if tac & 0x4 != 0 {
            // Calculate the new timer value.
            let tima = self.memory_map.get_io(Io::TIMA);

            if tima < 0xFF {
                self.memory_map.set_io(Io::TIMA, tima + 1);
            } else {
                /*
                    Note:
                    If a TMA write is executed on the same cycle as the content of TMA is transferred to TIMA due to a timer overflow,
                    the old value is transferred to TIMA.
                */

                // TIMA overflow occured. Set the TIMA register to TMA and request a Timer interrupt.
                self.memory_map
                    .set_io(Io::TIMA, self.memory_map.get_io(Io::TMA));
                self.memory_map
                    .set_io(Io::IF, self.memory_map.get_io(Io::IF) | 0x4);
            }
        }
    }

    /// Parameter base_clock_cycles represents how much base cycles this function will take.
    /// Function tries to do a complete emulation such as running Cpu, Ppu and counting timers such as DIV and TIMA.
    fn cycle_impl<T: Fn(u16) -> bool>(&mut self, base_clock_cycles: u32, on_change: Option<T>) {
        // Handle timers.
        /*
            FF07 - TAC - Timer Control (R/W)
            Bit 2    - Timer Stop  (0=Stop, 1=Start)
            Bits 1-0 - Input Clock Select
                00:   4096 Hz    (~4194 Hz SGB)
                01: 262144 Hz  (~268400 Hz SGB)
                10:  65536 Hz   (~67110 Hz SGB)
                11:  16384 Hz   (~16780 Hz SGB)
        */

        let tac = self.memory_map.get_io(Io::TAC);
        let timer_clock = match tac & 0x3 {
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16384,
            _ => unreachable!(),
        };

        self.base_clock = self.base_clock % CPU_CLOCK_RATE;
        self.ppu.clock_cycles = self.ppu.clock_cycles % ppu::PPU_ONE_FRAME;

        let start_base_clock = self.base_clock;
        let start_cpu_clock_cycles = self.cpu.clock_cycles;

        while self.base_clock - start_base_clock < base_clock_cycles {
            self.update_joypad();

            if self.base_clock % (CPU_CLOCK_RATE / timer_clock) == 0 {
                self.increment_tima();
            }

            if self.base_clock % (CPU_CLOCK_RATE / DIV_REGISTER_CLOCK_RATE) == 0 {
                self.memory_map.increment_div();
            }

            self.ppu.cycle(&mut self.memory_map, (ppu::PPU_CLOCK_RATE * 4) / CPU_CLOCK_RATE);

            // We check each time if the cpu clock is lower than base clock.
            // This is done because Cpu instructions have different instruction latencies.
            if self.cpu.clock_cycles - start_cpu_clock_cycles <= self.base_clock - start_base_clock {
                
                // Check if the DMA transfer is over.
                if let Some(dma_start) = self.dma_transfer_start {
                    
                    let diff = if dma_start > self.base_clock {
                        (CPU_CLOCK_RATE + self.base_clock) - dma_start
                    } else {
                        self.base_clock - dma_start
                    };

                    // TODO: 160?
                    if diff >= 160 {
                        self.dma_transfer_start = None;
                        self.memory_map.on_dma_transfer = false;
                    } else {
                        // Set DMA transfer true for restricting CPU memory access. 
                        self.memory_map.on_dma_transfer = true;
                    }
                }

                self.cpu.cycle(&mut self.memory_map);
                
                // Check if the DMA transfer is started.
                if self.memory_map.on_dma_transfer && self.dma_transfer_start.is_none() {
                    self.dma_transfer_start = Some(self.base_clock);
                }
                // Reset DMA transfer so PPU can access memory.
                self.memory_map.on_dma_transfer = false;

                if let Some(func) = &on_change {
                    if func(self.cpu.pc) {
                        return;
                    }
                }
            }

            self.base_clock += 4;
        }
    }


    pub fn cycle<T: Fn(u16) -> bool>(&mut self, elapsed_time: f32, on_change: Option<T>) {
        
        let base_clock_cycles = (CPU_CLOCK_RATE as f32 * elapsed_time) as u32;
    
        self.cycle_impl(base_clock_cycles, on_change)
    }

    pub fn cycle_once(&mut self) {
        // Call the implementation function with the base clock's step cycle count.
        // This way emulator exactly does the smallest unit of emulation.
        // Rust requires a type to be passed on. So we pass a dummy function pointer. 
        self.cycle_impl::<fn (u16) -> bool>(4, None) 
    }

    pub fn update_joypad_keys(&mut self, keys: JoypadKeys) {
        self.joypad_keys = keys;
    }

    pub fn update_joypad(&mut self) {
        let joyp = self.memory_map.get_io(Io::JOYP);

        let keys = if joyp & 0x30 == 0x30 {
            // Reset joypad.
            self.memory_map.set_io(Io::JOYP, 0xFF);
            return;
        } else if joyp & 0x10 != 0 {
            // Button keys
            !self.joypad_keys.0 & 0xF
        } else if joyp & 0x20 != 0 {
            // Direction keys
            !(self.joypad_keys.0 >> 4) & 0xF
        } else {
            return;
        };
        
        self.memory_map.set_io(Io::JOYP, (joyp & 0xF0) | keys);

        // Handle interrupt
        if keys != 0xF  {
            self.memory_map.set_io(Io::IF, self.memory_map.get_io(Io::IF) | 0x10);
        }
    }
}
