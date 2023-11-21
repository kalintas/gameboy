#![feature(const_mut_refs)]

pub mod cpu;
pub mod instructions;
mod mbc;
pub mod memory_map;
pub mod ppu;
mod registers;

use std::{error::Error, path::Path, time::Duration};

use cpu::Cpu;
use instructions::{Instruction, INSTRUCTIONS, PREFIX_CB_INSTRUCTIONS};
use memoffset::offset_of;
use memory_map::MemoryMap;
use ppu::Ppu;

use self::memory_map::{Io, MemSyncer, SyncMem};

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

#[repr(C)]
#[derive(Clone)]
pub struct Gameboy {
    pub cpu: Cpu,
    pub ppu: Ppu,

    pub memory_map: MemoryMap,

    base_clock: u32,
    // Cpu clock cycles are different in latency.
    // So we need a variable to store remainder cpu cycles after an emulator cycle.
    remainder_cpu_cycles: u32,

    dma_transfer_start: Option<u32>,

    joypad_keys: JoypadKeys,
}

impl Gameboy {
    /// Creates a new instance of Gameboy emulator that is ready to be booted.
    /// # Arguments
    /// * `boot_room_path` - Path to the valid boot room.
    #[allow(dead_code)]
    pub fn new(boot_rom_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut emulator = Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),

            memory_map: MemoryMap::new(),

            ..Self::after_boot()
        };

        emulator.memory_map.load_boot_rom(boot_rom_path)?;

        Ok(emulator)
    }

    /// Creates a new instance of Gameboy emulator that is finished booting.
    /// This also means there is no checksums to be checked and can run practically any rom.
    #[allow(dead_code)]
    pub fn after_boot() -> Self {
        Self {
            cpu: Cpu::after_boot(),
            ppu: Ppu::new(),

            memory_map: MemoryMap::after_boot(),

            base_clock: 0,
            remainder_cpu_cycles: 0,

            dma_transfer_start: None,

            joypad_keys: JoypadKeys::NONE,
        }
    }

    #[allow(dead_code)]
    pub fn load_cartidge(&mut self, path: impl AsRef<Path>) {
        self.memory_map.load_rom(path);
        self.memory_map.mem_syncer = MemSyncer::new(offset_of!(Gameboy, memory_map));
    }

    fn increment_tima(&mut self) {
        /*
            FF07 - TAC - Timer Control (R/W)
            Bit 2    - Timer Stop  (0=Stop, 1=Start)
            Bits 1-0 - Input Clock Select
                00:   4096 Hz    (~4194 Hz SGB)
                01: 262144 Hz  (~268400 Hz SGB)
                10:  65536 Hz   (~67110 Hz SGB)
                11:  16384 Hz   (~16780 Hz SGB)
        */
        let tac = self.memory_map.cpu_get_io(Io::TAC);

        let timer_clock = match tac & 0x3 {
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16384,
            _ => unreachable!(),
        };

        // Check if the timer is stopped.
        if tac & 0x4 != 0 {
            if self.base_clock % (CPU_CLOCK_RATE / timer_clock) != 0 {
                return;
            }

            // Calculate the new timer value.
            let tima = self.memory_map.cpu_get_io(Io::TIMA);

            if tima < 0xFF {
                self.memory_map.cpu_set_io(Io::TIMA, tima + 1);
            } else {
                /*
                    Note:
                    If a TMA write is executed on the same cycle as the content of TMA is transferred to TIMA due to a timer overflow,
                    the old value is transferred to TIMA.
                */

                // TIMA overflow occured. Set the TIMA register to TMA and request a Timer interrupt.
                self.memory_map
                    .cpu_set_io(Io::TIMA, self.memory_map.cpu_get_io(Io::TMA));
                self.memory_map
                    .cpu_set_io(Io::IF, self.memory_map.cpu_get_io(Io::IF) | 0x4);
            }
        }
    }

    fn update_peripherals(&mut self) {
        if self.cpu.is_stopped() {
            return;
        }

        self.update_joypad();

        self.increment_tima();

        if self.base_clock % (CPU_CLOCK_RATE / DIV_REGISTER_CLOCK_RATE) == 0 {
            self.memory_map.increment_div();
        }

        self.ppu.cycle(
            &mut self.memory_map,
            (ppu::PPU_CLOCK_RATE * 4) / CPU_CLOCK_RATE,
        );
    }

    /// Function for private implementation of the emulator cycle.
    /// Parameter base_clock_cycles represents how much base cycles this function will take.
    /// Function tries to do a complete emulation such as running Cpu, Ppu and counting timers such as DIV and TIMA.
    fn cycle_impl<T: FnMut(&Self) -> bool>(
        &mut self,
        base_clock_cycles: u32,
        mut on_cpu_cycle: Option<T>,
    ) {
        // Reset the triggered watch variable every cycle.
        self.memory_map.triggered_watch = None;

        self.base_clock = self.base_clock % CPU_CLOCK_RATE;
        self.ppu.clock_cycles = self.ppu.clock_cycles % ppu::PPU_ONE_FRAME;

        let start_base_clock = self.base_clock;
        let start_cpu_clock_cycles = self.cpu.clock_cycles;

        while self.base_clock - start_base_clock < base_clock_cycles {
            self.update_peripherals();

            // We check each time if the cpu clock is lower than base clock.
            // This is done because Cpu instructions have different instruction latencies.
            if self.cpu.clock_cycles - start_cpu_clock_cycles + self.remainder_cpu_cycles
                <= self.base_clock - start_base_clock
            {
                // Check if the DMA transfer is over.
                if let Some(dma_start) = self.dma_transfer_start {
                    let diff = if dma_start > self.base_clock {
                        self.base_clock - (dma_start % CPU_CLOCK_RATE)
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

                // Memory is triggered in user given condition. Stop execution.
                if self.memory_map.triggered_watch.is_some() {
                    self.base_clock += 4;
                    break;
                }

                // Check if the DMA transfer is started.
                if self.memory_map.on_dma_transfer && self.dma_transfer_start.is_none() {
                    self.dma_transfer_start = Some(self.base_clock);
                }
                // Reset DMA transfer so PPU can access memory.
                self.memory_map.on_dma_transfer = false;

                // Check if there is an boot rom in the memory map.
                // Boot rom hands over the control to cartridges rom in 0x100.
                if !self.memory_map.boot_rom.is_empty() && self.cpu.pc >= 0x100 {
                    // Clear the boot room.
                    self.memory_map.clear_boot_rom();
                }

                if let Some(func) = &mut on_cpu_cycle {
                    if func(self) {
                        self.base_clock += 4;
                        break;
                    }
                }
            }
            self.base_clock += 4;
        }

        /*
            A brief description of the below algorithm:
            Base clock: (Cycle start)  |  |  |  |  |  |  |  |  |  |        (End of cycle)
            Cpu clock:  (Cycle start)  ---|-----|--------|--|-----------|  (End of cycle)
                                       ^                           ^ Overworked cpu instruction in this emulator cycle.
                                       Remainder cpu instruction from last emulator cycle.
            Because that cpu instructions are wary in latency sometimes they overwork(run more than the base clock).
            And then we store these overworked cycles and don't run the cpu until same amount of cycles passes.
        */
        let total_cpu_cycles =
            self.cpu.clock_cycles - start_cpu_clock_cycles + self.remainder_cpu_cycles;
        let total_base_clock_cycles = self.base_clock - start_base_clock;

        if total_cpu_cycles > total_base_clock_cycles {
            self.remainder_cpu_cycles = total_cpu_cycles - total_base_clock_cycles;
        } else {
            self.remainder_cpu_cycles = 0;
        }
    }

    /// Cycles the emulator with given elapsed time parameter.
    /// # Arguments
    /// * `elapsed_time` - Duration to execute the emulator. Note that this is in emulator time not in local machine time.
    ///     For example if elapsed_time is 1 second function will effectively run 4 194 304 cpu cycles.
    ///   
    /// * `on_cpu_cycle` - A closure that will be called after running every cpu instruction.
    pub fn debug_cycle<T: FnMut(&Self) -> bool>(
        &mut self,
        elapsed_time: Duration,
        on_cpu_cycle: T,
    ) {
        let base_clock_cycles = (CPU_CLOCK_RATE as f32 * elapsed_time.as_secs_f32()) as u32;

        self.cycle_impl(base_clock_cycles, Some(on_cpu_cycle))
    }

    /// Cycles the emulator with given elapsed time parameter.
    /// # Arguments
    /// * `elapsed_time` - Duration to execute the emulator. Note that this is in emulator time not in local machine time.
    ///     For example if elapsed_time is 1 second function will effectively run 4 194 304 cpu cycles.
    ///   
    pub fn cycle(&mut self, elapsed_time: Duration) {
        let base_clock_cycles = (CPU_CLOCK_RATE as f32 * elapsed_time.as_secs_f32()) as u32;

        self.cycle_impl::<fn(&Self) -> bool>(base_clock_cycles, None);
    }

    /// Cycles the emulator in the smallest possible step.
    /// Function is guaranteed to run exactly 1 cpu instruction.
    pub fn cycle_once(&mut self) {
        // Rust requires a type to be passed on. So we pass a dummy function pointer.
        self.cycle_impl::<fn(&Self) -> bool>(self.remainder_cpu_cycles + 4, None)
    }

    pub fn update_joypad_keys(&mut self, keys: JoypadKeys) {
        self.joypad_keys = keys;
    }

    fn update_joypad(&mut self) {
        let joyp = self.memory_map.cpu_get_io(Io::JOYP);

        let keys = if joyp & 0x30 == 0x30 {
            // Reset joypad.
            self.memory_map.cpu_set_io(Io::JOYP, 0xFF);
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

        self.memory_map.cpu_set_io(Io::JOYP, (joyp & 0xF0) | keys);

        // Handle interrupt
        if keys != 0xF {
            self.memory_map
                .cpu_set_io(Io::IF, self.memory_map.cpu_get_io(Io::IF) | 0x10);
        }
    }

    /// A instruction fetch and decode without any side effects.
    /// # Arguments
    /// * `address` - Starting address of the instruction
    pub fn decode_instr(&self, address: u16) -> Instruction {
        let opcode = self.memory_map.get(address);

        if opcode == 0xCB {
            PREFIX_CB_INSTRUCTIONS[self.memory_map.get(address + 1) as usize]
        } else {
            INSTRUCTIONS[opcode as usize]
        }
    }
}

impl SyncMem for Gameboy {
    fn sync(&mut self) {
        self.base_clock += 4;
        self.update_peripherals();
    }
}
