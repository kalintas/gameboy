pub mod cpu;
mod instructions;
pub mod memory_map;
mod registers;
pub mod ppu;

use std::path::Path;

use cpu::Cpu;
use ppu::Ppu;
use memory_map::MemoryMap;

use self::memory_map::Io;

use std::ops::BitOr;

const CPU_CLOCK_RATE: u32 = 4_194_304;
const PPU_CLOCK_RATE: u32 = 1_048_576;

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
            joypad_keys: JoypadKeys::NONE
        }
    }

    pub fn after_boot() -> Self {
        Self {
            cpu: Cpu::after_boot(),
            ppu: Ppu::new(),
            memory_map: MemoryMap::after_boot(),

            base_clock: 0,
            joypad_keys: JoypadKeys::NONE
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
                self.memory_map.set_io(Io::TIMA, self.memory_map.get_io(Io::TMA));
                self.memory_map.set_io(Io::IF, self.memory_map.get_io(Io::IF) | 0x4);
            }
        }
    }   

    pub fn cycle(&mut self, elapsed_time: f32, on_change: Option<impl Fn(u16) -> bool>) {

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
            _ => unreachable!()
        };

        let base_clock_cycles = (CPU_CLOCK_RATE as f32 * elapsed_time) as u32;

        self.base_clock = self.base_clock % CPU_CLOCK_RATE;
        self.ppu.clock_cycles = self.ppu.clock_cycles % (114 * 154);

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

            if self.base_clock % (CPU_CLOCK_RATE / PPU_CLOCK_RATE) == 0 { // TODO: Remove?
                self.ppu.cycle(&mut self.memory_map);
            }

            // We check each time if the cpu clock is lower than base clock.
            // This is done because Cpu instructions have different instruction latencies.
            if self.cpu.clock_cycles - start_cpu_clock_cycles < self.base_clock - start_base_clock {
                self.cpu.cycle(&mut self.memory_map);

                if let Some(func) = &on_change {
                    if func(self.cpu.pc) {
                        return;
                    }
                }
            }

            self.base_clock += 4;
        }
    }

    // TODO: TIMA and DIV timers are not updated.
    pub fn cycle_once(&mut self) {

        self.cpu.cycle(&mut self.memory_map);
        self.ppu.cycle(&mut self.memory_map);
    }

    pub fn update_joypad_keys(&mut self, keys: JoypadKeys) {
        self.joypad_keys = keys;
    }

    pub fn update_joypad(&mut self) {

        let joyp = self.memory_map.get_io(Io::JOYP);

        let keys = if joyp & 0x30 == 0x30 {
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

        self.memory_map.set_io(Io::JOYP, joyp | keys);

        // Handle interrupt
        // let new_joyp = self.memory_map.get_io(Io::JOYP);

        // if joyp != new_joyp {
        //     self.memory_map.set_io(Io::IF, self.memory_map.get_io(Io::IF) | 0x8);
        // }
    }
}
