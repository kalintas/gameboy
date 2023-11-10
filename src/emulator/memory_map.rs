/*
Memory map of the gameboy(from pandocs: http://bgb.bircd.org/pandocs.htm):
    0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
    4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
    8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
    A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)
    C000-CFFF   4KB Work RAM Bank 0 (WRAM)
    D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
    E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)
    FE00-FE9F   Sprite Attribute Table (OAM)
    FEA0-FEFF   Not Usable
    FF00-FF7F   I/O Ports
    FF80-FFFE   High RAM (HRAM)
    FFFF        Interrupt Enable Register
*/
use memoffset::offset_of;
use std::{
    cell::{UnsafeCell, RefCell}, collections::HashMap, marker::PhantomData, path::Path,
    ptr::copy_nonoverlapping,
};
use strum_macros::{AsRefStr, EnumIter};

use super::{
    mbc::{self, Mbc},
    Emulator,
};

pub trait SyncMem {
    fn sync(&mut self);
}

pub struct MemSyncer<T: SyncMem> {
    phantom: PhantomData<T>,
    offset_address: Option<usize>,
    is_syncing: UnsafeCell<bool>,
    old_is_syncing: UnsafeCell<bool>,
}

impl<T: SyncMem> MemSyncer<T> {
    // Creates a MemSyncer with given offset_address.
    // Parameter must be the offset address of the caller struct and its MemoryMap.
    pub fn new(offset_address: usize) -> Self {
        Self {
            phantom: PhantomData::default(),
            offset_address: Some(offset_address),
            is_syncing: UnsafeCell::new(false),
            old_is_syncing: UnsafeCell::new(false),
        }
    }

    pub fn open_sync(&mut self) {
        *self.is_syncing.get_mut() = true;
    }

    pub fn close_sync(&mut self) {
        *self.is_syncing.get_mut() = false;
    }

    fn sync_start(&self) -> bool {
        // TODO: Safety
        unsafe {
            if !*self.is_syncing.get() {
                return false;
            }

            *self.old_is_syncing.get() = *self.is_syncing.get();
            *self.is_syncing.get() = false;

            if let Some(offset_address) = self.offset_address {
                (*((self as *const Self as *const u8)
                    .sub(offset_of!(MemoryMap, mem_syncer) + offset_address)
                    as *mut T))
                    .sync();
            }

            true
        }
    }

    fn sync_end(&self) {
        unsafe {
            *self.is_syncing.get() = *self.old_is_syncing.get();
        }
    }
}

impl<T: SyncMem> Default for MemSyncer<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
            offset_address: None,
            is_syncing: UnsafeCell::new(false),
            old_is_syncing: UnsafeCell::new(false),
        }
    }
}

impl<T: SyncMem> Clone for MemSyncer<T> {
    fn clone(&self) -> Self {
        Self {
            phantom: PhantomData::default(),
            offset_address: self.offset_address,
            is_syncing: UnsafeCell::new(unsafe { *self.is_syncing.get() }),
            old_is_syncing: UnsafeCell::new(unsafe { *self.old_is_syncing.get() }),
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OamCorruption {
    Read,
    Write,
    IncDecRead,
}

#[repr(u16)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, EnumIter, AsRefStr, PartialEq)]
pub enum Io {
    LCDC = 0xFF40,  // LCD Control (R/W)
    STAT = 0xFF41,  // LCDC Status (R/W)
    SCY = 0xFF42,   // Scroll Y (R/W)
    SCX = 0xFF43,   // Scroll X (R/W)
    LY = 0xFF44,    // LCDC Y-Coordinate (R)
    LYC = 0xFF45,   // LY Compare (R/W)
    WY = 0xFF4A,    // Window Y Position (R/W)
    WX = 0xFF4B,    // Window X Position minus 7 (R/W)
    BGP = 0xFF47,   // BG Palette Data (R/W) - Non CGB Mode Only
    OBP0 = 0xFF48,  // Object Palette 0 Data (R/W) - Non CGB Mode Only
    OBP1 = 0xFF49,  // Object Palette 1 Data (R/W) - Non CGB Mode Only
    BGPI = 0xFF68,  // CGB Mode Only - Background Palette Index
    BGPD = 0xFF69,  // CGB Mode Only - Background Palette Data
    OBPI = 0xFF6A,  // CGB Mode Only - Sprite Palette Index
    OBPD = 0xFF6B,  // CGB Mode Only - Sprite Palette Data
    VBK = 0xFF4F,   // CGB Mode Only - VRAM Bank
    DMA = 0xFF46,   // DMA Transfer and Start Address (W)
    HDMA1 = 0xFF51, // CGB Mode Only - New DMA Source, High
    HDMA2 = 0xFF52, // CGB Mode Only - New DMA Source, Low
    HDMA3 = 0xFF53, // CGB Mode Only - New DMA Destination, High
    HDMA4 = 0xFF54, // CGB Mode Only - New DMA Destination, Low
    HDMA5 = 0xFF55, // CGB Mode Only - New DMA Length/Mode/Start
    NR10 = 0xFF10,  // Channel 1 Sweep register (R/W)
    NR11 = 0xFF11,  // Channel 1 Sound length/Wave pattern duty (R/W)
    NR12 = 0xFF12,  // Channel 1 Volume Envelope (R/W)
    NR13 = 0xFF13,  // Channel 1 Frequency lo (Write Only)
    NR14 = 0xFF14,  // Channel 1 Frequency hi (R/W)
    NR21 = 0xFF16,  // Channel 2 Sound Length/Wave Pattern Duty (R/W)
    NR22 = 0xFF17,  // Channel 2 Volume Envelope (R/W)
    NR23 = 0xFF18,  // Channel 2 Frequency lo data (W)
    NR24 = 0xFF19,  // Channel 2 Frequency hi data (R/W)
    NR30 = 0xFF1A,  // Channel 3 Sound on/off (R/W)
    NR31 = 0xFF1B,  // Channel 3 Sound Length
    NR32 = 0xFF1C,  // Channel 3 Select output level (R/W)
    NR33 = 0xFF1D,  // Channel 3 Frequency's lower data (W)
    NR34 = 0xFF1E,  // Channel 3 Frequency's higher data (R/W)
    NR41 = 0xFF20,  // Channel 4 Sound Length (R/W)
    NR42 = 0xFF21,  // Channel 4 Volume Envelope (R/W)
    NR43 = 0xFF22,  // Channel 4 Polynomial Counter (R/W)
    NR44 = 0xFF23,  // Channel 4 Counter/consecutive; Inital (R/W)
    NR50 = 0xFF24,  // Channel control / ON-OFF / Volume (R/W)
    NR51 = 0xFF25,  // Selection of Sound output terminal (R/W)
    NR52 = 0xFF26,  // Sound on/off
    JOYP = 0xFF00,  // Joypad (R/W)
    SB = 0xFF01,    // Serial transfer data (R/W)
    SC = 0xFF02,    // Serial Transfer Control (R/W)
    DIV = 0xFF04,   // Divider Register (R/W)
    TIMA = 0xFF05,  // Timer counter (R/W)
    TMA = 0xFF06,   // Timer Modulo (R/W)
    TAC = 0xFF07,   // Timer Control (R/W)
    IE = 0xFFFF,    // Interrupt Enable (R/W)
    IF = 0xFF0F,    // Interrupt Flag (R/W)
    KEY1 = 0xFF4D,  // CGB Mode Only - Prepare Speed Switch
    RP = 0xFF56,    // CGB Mode Only - Infrared Communications Port
    SVBK = 0xFF70,  // CGB Mode Only - WRAM Bank
}

#[repr(C)]
#[derive(Clone)]
pub struct MemoryMap {
    rom_banks: Vec<[u8; 0x4000]>,    // 16KB rom banks that is 'in the cartridge'.
    vrams: Vec<[u8; 0x2000]>,        // 8KB video rams(VRAM)
    external_ram: Vec<[u8; 0x2000]>, // 8KB external ram that is'in the cartridge'.
    wrams: Vec<[u8; 0x1000]>,        // 4KB work rams(WRAM)
    oam: RefCell<[u8; 0x100]>,    // Sprite Attribute Table(OAM)
    io_ports: [u8; 0x80],
    high_ram: [u8; 0x7F],
    ier: u8, // Interrupt Enable Register

    mbc: Box<dyn Mbc>,

    pub mem_syncer: MemSyncer<Emulator>,

    pub current_oam_row: Option<u16>, // Current row of the OAM in PPU.
    oam_corruption_enabled: bool, // When set to true OAM corruptions will be disabled in cpu_get and cpu_set functions.

    pub boot_rom: Vec<u8>,

    pub changes: HashMap<u16, u8>,

    pub memory_watches: Vec<(u16, Option<u8>)>,
    pub triggered_watch: Option<usize>,

    pub on_dma_transfer: bool,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            rom_banks: Vec::new(),
            vrams: Vec::new(),
            external_ram: Vec::new(),
            wrams: Vec::new(),
            oam: RefCell::new([0u8; 0x100]),
            io_ports: [0u8; 0x80],
            high_ram: [0u8; 0x7F],
            ier: 0u8,

            mbc: Box::new(mbc::NoMbc) as Box<dyn Mbc>,

            mem_syncer: MemSyncer::default(),
            current_oam_row: None,
            oam_corruption_enabled: true,
            
            boot_rom: Vec::new(),

            memory_watches: Vec::new(),
            triggered_watch: None,

            changes: HashMap::new(),
            on_dma_transfer: false,
        }
    }

    pub fn after_boot() -> Self {
        let mut memory = Self::new();

        memory.cpu_set_io(Io::TIMA, 0x00);
        memory.cpu_set_io(Io::TMA, 0x00);
        memory.cpu_set_io(Io::TAC, 0x00);
        memory.cpu_set_io(Io::NR10, 0x80);
        memory.cpu_set_io(Io::NR11, 0xBF);
        memory.cpu_set_io(Io::NR12, 0xF3);
        memory.cpu_set_io(Io::NR14, 0xBF);
        memory.cpu_set_io(Io::NR21, 0x3F);
        memory.cpu_set_io(Io::NR22, 0x00);
        memory.cpu_set_io(Io::NR24, 0xBF);
        memory.cpu_set_io(Io::NR30, 0x7F);
        memory.cpu_set_io(Io::NR31, 0xFF);
        memory.cpu_set_io(Io::NR32, 0x9F);
        memory.cpu_set_io(Io::NR33, 0xBF);
        memory.cpu_set_io(Io::NR41, 0xFF);
        memory.cpu_set_io(Io::NR42, 0x00);
        memory.cpu_set_io(Io::NR43, 0x00);
        memory.cpu_set_io(Io::NR30, 0xBF);
        memory.cpu_set_io(Io::NR50, 0x77);
        memory.cpu_set_io(Io::NR51, 0xF3);
        memory.cpu_set_io(Io::NR52, 0xF1);
        memory.cpu_set_io(Io::LCDC, 0x91);
        memory.cpu_set_io(Io::SCY, 0x00);
        memory.cpu_set_io(Io::SCX, 0x00);
        memory.cpu_set_io(Io::LYC, 0x00);
        memory.cpu_set_io(Io::BGP, 0xFC);
        memory.cpu_set_io(Io::OBP0, 0xFF);
        memory.cpu_set_io(Io::OBP1, 0xFF);
        memory.cpu_set_io(Io::WY, 0x00);
        memory.cpu_set_io(Io::WX, 0x00);
        memory.cpu_set_io(Io::IE, 0x00);

        memory
    }

    pub fn load_rom<T: AsRef<Path>>(&mut self, path: T) {
        let rom = std::fs::read(path).unwrap();

        let cartridge_type = rom[0x147];
        let rom_bank_count = 1 << (rom[0x148] + 1);

        let ram_bank_count = match rom[0x149] {
            0 => 0,
            0x2 => 1,
            0x3 => 4,
            0x4 => 16,
            0x5 => 8,
            0x1 | _ => panic!("ram size(0x149) is not valid"),
        };

        self.mbc = match cartridge_type {
            0x00 => Box::new(mbc::NoMbc),
            0x01 | 0x2 | 0x3 => Box::new(mbc::Mbc1::new(rom_bank_count, ram_bank_count)),
            // 0x05 | 0x06 => MBC2,
            // 0x0B | 0x0C | 0x0D => MMM01,
            // 0x0F | 0x10 | 0x11 | 0x12 | 0x13 => MBC3,
            // 0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E => MBC5,
            // 0x20 => MBC6,
            // 0x22 => MBC7
            _ => unimplemented!(),
        };

        self.rom_banks.resize(rom_bank_count, [0u8; 0x4000]);
        self.external_ram.resize(ram_bank_count, [0u8; 0x2000]);
        self.vrams.resize(1, [0u8; 0x2000]);
        self.wrams.resize(2, [0u8; 0x1000]);

        unsafe {
            // TODO: SAFETY
            copy_nonoverlapping(rom.as_ptr(), self.rom_banks.as_mut_ptr() as _, rom.len());
        }

        self.changes.reserve(rom.len());

        for i in self.boot_rom.len() as u16..0x8000 {
            self.changes.insert(i, self.cpu_get(i));
        }
    }

    pub fn load_boot_rom<T: AsRef<Path>>(&mut self, path: T) {
        self.boot_rom = std::fs::read(path).unwrap();

        self.changes.reserve(self.boot_rom.len());

        for i in 0..self.boot_rom.len() {
            self.changes.insert(i as u16, self.boot_rom[i]);
        }
    }

    pub fn clear_boot_rom(&mut self) {
        let boot_rom_length = self.boot_rom.len();
        self.boot_rom.clear();

        for i in 0..boot_rom_length as u16 {
            self.changes.insert(i, self.cpu_get(i));
        }
    }

    // General I/O. Can be used to get memory without any restriction.
    pub fn get(&self, address: u16) -> u8 {
        
        let address = address as usize;
        
        if address < self.boot_rom.len() {
            return self.boot_rom[address];
        } else if address < 0x4000 {
            // 0000-3FFF   16KB ROM Bank 00
            return self.rom_banks[0][address];
        }
        if address < 0x8000 {
            // 4000-7FFF   16KB ROM Bank 01..NN
            return self.rom_banks[self.mbc.get_rom_bank()][address - 0x4000];
        }
        if address < 0xA000 {
            // 8000-9FFF   8KB Video RAM (VRAM)
            return self.vrams[0][address - 0x8000];
        }
        if address < 0xC000 {
            // A000-BFFF   8KB External RAM
            if let Some(ram_bank) = self.mbc.get_ram_bank() {
                if !self.external_ram.is_empty() {
                    return self.external_ram[ram_bank][address - 0xA000];
                }
            }
            return 0xFF;
        }
        if address < 0xD000 {
            // C000-CFFF   4KB Work RAM Bank 0 (WRAM)
            return self.wrams[0][address - 0xC000];
        }
        if address < 0xE000 {
            // D000-DFFF   4KB Work RAM Bank 1
            return self.wrams[1][address - 0xD000];
        }
        if address < 0xFE00 {
            // E000-FDFF   Same as C000-DDFF (ECHO)
            return self.get(address as u16 - 0x2000);
        }
        if address < 0xFEA0 {
            // FE00-FE9F   Sprite Attribute Table (OAM)
            return self.oam.borrow()[address - 0xFE00];
        }
        if address < 0xFF00 {
            // FEA0-FEFF   Not Usable
            return 0;
        }
        if address < 0xFF80 {
            // FF00-FF7F   I/O Ports
            return self.io_ports[address - 0xFF00];
        }
        if address < 0xFFFF {
            return self.high_ram[address - 0xFF80];
        }

        return self.ier;
    }

    pub fn set(&mut self, address: u16, value: u8) {

        let address = address as usize;

        if address < 0x8000 {
            // 0000-3FFF   16KB ROM Bank 00
            // 4000-7FFF   16KB ROM Bank 01..NN
        } else if address < 0xA000 {
            // 8000-9FFF   8KB Video RAM (VRAM)
            self.vrams[0][address - 0x8000] = value;
        } else if address < 0xC000 {
            // A000-BFFF   8KB External RAM
            if let Some(ram_bank) = self.mbc.get_ram_bank() {
                if !self.external_ram.is_empty() {
                    self.external_ram[ram_bank][address - 0xA000] = value;
                }
            }
        } else if address < 0xD000 {
            // C000-CFFF   4KB Work RAM Bank 0 (WRAM)
            self.wrams[0][address - 0xC000] = value;
        } else if address < 0xE000 {
            // D000-DFFF   4KB Work RAM Bank 1
            self.wrams[1][address - 0xD000] = value;
        } else if address < 0xFE00 {
            // E000-FDFF   Same as C000-DDFF (ECHO)
            return self.set(address as u16 - 0x2000, value);
        } else if address < 0xFEA0 {
            // FE00-FE9F   Sprite Attribute Table (OAM)
            self.oam.borrow_mut()[address - 0xFE00] = value;
        } else if address < 0xFF00 {
            // FEA0-FEFF   Not Usable
        } else if address < 0xFF80 {
            // FF00-FF7F   I/O Ports
            self.io_ports[address - 0xFF00] = value;
        } else if address < 0xFFFF {
            self.high_ram[address - 0xFF80] = value;
        } else {
            self.ier = value;
        }
    }

    pub fn get_io(&self, address: Io) -> u8 {
        self.get(address as u16)
    }
    
    pub fn set_io(&mut self, address: Io, value: u8) {
        self.set(address as u16, value)
    }

    #[allow(dead_code)]
    pub fn get_u16(&self, address: u16) -> u16 {
        let lsb = self.get(address) as u16; // Get the least significant byte
        let msb = self.get(address + 1) as u16; // Get the most significant byte

        (msb << 8) | lsb
    }

    #[allow(dead_code)]
    pub fn set_u16(&mut self, address: u16, value: u16) {
        let lsb = (value & 0xFF) as u8; // Get the least significant byte
        let msb = ((value >> 8) & 0xFF) as u8; // Get the most significant byte

        self.set(address, lsb);
        self.set(address + 1, msb);
    }

    // CPU I/O.
    pub fn cpu_set(&mut self, address: u16, mut value: u8) {
        
        let sync_start = self.mem_syncer.sync_start();

        let lcd_disabled = (self.cpu_get_io(Io::LCDC) & 0x80) == 0;

        if self.oam_corruption_enabled {
            self.try_corrupt_oam(address, OamCorruption::Write);
        }

        let can_set = if self.on_dma_transfer && (address < 0xFF80 || address == 0xFFFF) {
            // CPU can access only HRAM (memory at FF80-FFFE) during DMA transfer.
            false
        } else if address >= 0x8000 && address < 0xA000 { // VRAM
            // Ppu is in pixel transfer mode.
            lcd_disabled || self.cpu_get_io(Io::STAT) & 0x3 != 0x3
        } else if address >= 0xFE00 && address < 0xFEA0 { // OAM
            // Ppu is in pixel transfer or OAM search mode.
            lcd_disabled || self.cpu_get_io(Io::STAT) & 0x2 == 0
        } else if address == Io::DMA as _ {
            // DMA: Writing to this register launches a DMA transfer
            // It takes 640 cpu clock cycles for the DMA transfer to be complete.
            self.dma_transfer(value);
            false
        } else {
            true
        };

        if address == Io::DIV as _ {
            // DIV: Divider register, writing any value to this register resets it to 0x00
            value = 0;
        }

        if can_set {
            self.set(address, value);
            self.changes.insert(address as u16, value);
        
            self.triggered_watch =
                self.memory_watches
                    .iter()
                    .position(|&(watch_address, watch_value)| {
                        watch_address == address as u16
                            && watch_value.map_or(true, |watch_value| watch_value == value)
                    });
        }

        self.mbc.set(address as u16, value);

        if sync_start {
            self.mem_syncer.sync_end();
        }
    }

    pub fn cpu_get(&self, address: u16) -> u8 {
        
        let sync_start = self.mem_syncer.sync_start();

        if self.oam_corruption_enabled {
            self.try_corrupt_oam(address, OamCorruption::Read);
        }

        let mut value = self.get(address);

        if self.on_dma_transfer && (address < 0xFF80 || address == 0xFFFF) {
            // CPU can access only HRAM (memory at FF80-FFFE) during DMA transfer.
            value = 0xFF;
        }

        if address >= 0x8000 && address < 0xA000 { // VRAM
            if self.cpu_get_io(Io::STAT) & 0x3 == 0x3 {
                // Ppu is in pixel transfer mode.
                value = 0xFF;
            }
        } else if address >= 0xFE00 && address < 0xFEA0 { // OAM
            if self.cpu_get_io(Io::STAT) & 0x2 != 0 {
                // Ppu is in pixel transfer or OAM search mode.
                value = 0xFF;
            }
        }

        if sync_start {
            self.mem_syncer.sync_end();
        }

        value
    }

    pub fn cpu_get_u16(&self, address: u16) -> u16 {
        let lsb = self.cpu_get(address) as u16; // Get the least significant byte
        let msb = self.cpu_get(address + 1) as u16; // Get the most significant byte

        (msb << 8) | lsb
    }

    pub fn cpu_set_u16(&mut self, address: u16, value: u16) {
        let lsb = (value & 0xFF) as u8; // Get the least significant byte
        let msb = ((value >> 8) & 0xFF) as u8; // Get the most significant byte

        self.cpu_set(address, lsb);
        self.cpu_set(address + 1, msb);
    }

    pub fn enable_oam_corruption(&mut self) {
        self.oam_corruption_enabled = true;
    }

    pub fn disable_oam_corruption(&mut self) {
        self.oam_corruption_enabled = false;
    }
    
    pub fn cpu_get_io(&self, address: Io) -> u8 {
        self.cpu_get(address as u16)
    }

    pub fn cpu_set_io(&mut self, address: Io, value: u8) {
        self.cpu_set(address as u16, value)
    }

    // PPU I/O.
    pub fn ppu_get_vram(&self, address: u16) -> u8 {
        /*
            At various times during PPU operation read access to VRAM is blocked and the value read is $FF:
            LCD turning off
            At scanline 0 on CGB when not in double speed mode
            When switching from mode 3 to mode 0
            On CGB when searching OAM and index 37 is reached
        */

        let lcdc = self.cpu_get_io(Io::LCDC);
        let stat = self.cpu_get_io(Io::STAT);

        if lcdc & 0x80 == 0 {
            // LCDC off
            0xFF
        } else if stat & 0x3 != 0x3 {
            // Mode is not 3(pixel transfer)
            0xFF
        } else {
            self.vrams[0][address as usize - 0x8000]
        }
    }

    pub fn ppu_get_oam(&self, address: u16) -> u8 {
        self.oam.borrow()[address as usize - 0xFE00]
    }

    pub unsafe fn get_vram(&self, address: u16) -> u8 {
        self.vrams[0][address as usize - 0x8000]
    }

    // DIV register is a special I/O port that only cpu can write.
    // When programmer tries to write to this register it automatically set to 0.
    pub fn increment_div(&mut self) {
        self.io_ports[Io::DIV as usize - 0xFF00] =
            self.io_ports[Io::DIV as usize - 0xFF00].wrapping_add(1);
    }

    pub fn dma_transfer(&mut self, source: u8) {
        // TODO: XX(source) = $00 to $DF
        if source > 0xDF {
            return;
        }

        let source = (source as u16) << 8;

        for i in 0..0xA0 {
            self.cpu_set(0xFE00 + i, self.cpu_get(source + i));
        }

        self.on_dma_transfer = true;
    }

    /*
        There is a bug in the gameboy hardware called OAM Corruption Bug.
        Any read or write to the OAM address space in OamSearch mode(Mode 2) will corrupt the OAM in a specific pattern.
        Besides read/write of the memory, increment or decrement of a 16 bit integer will also corrupt the OAM 
            if the value is in the OAM address space
        More info is available in the pandocs:
        // https://gbdev.io/pandocs/OAM_Corruption_Bug.html
    */
    pub fn try_corrupt_oam(&self, value: u16, mut corruption: OamCorruption) {        
    
        if !(value >= 0xFE00 && value < 0xFF00) {
            // Not in OAM address spoace.
            return;
        }

        let set_oam_u16 = |address: u16, value: u16| {

            let address = address as usize - 0xFE00;

            self.oam.borrow_mut()[address] = (value & 0xFF) as u8;
            self.oam.borrow_mut()[address + 1] = (value >> 8)as u8;
        };

        if let Some(current_oam_row) = self.current_oam_row {

            if corruption == OamCorruption::IncDecRead {

                if current_oam_row < 20 {
                    let start = 0xFE00 + (current_oam_row - 1) as u16 * 8;
        
                    let a = self.get_u16(start - 8); // First word two rows before the currently accessed row.
                    let b = self.get_u16(start);     // First word in the preceding row (the word being corrupted).
                    let c = self.get_u16(start + 8); // First word in the currently accessed row.
                    let d = self.get_u16(start + 4); // Third word in the preceding row.
        
                    let result = (b & (a | c | d)) | (a & c & d);
        
                    set_oam_u16(start, result);

                    for i in (0..8).step_by(2) {
                        set_oam_u16(start - 8 + i, self.get_u16(start + i));
                        set_oam_u16(start + 8 + i, self.get_u16(start + i));
                    }
                }

                // Regardless of whether the previous corruption occurred or not, 
                // a normal read corruption is then applied.
                corruption = OamCorruption::Read;
            }

            if 0 < current_oam_row && current_oam_row < 20 {
                let start = 0xFE00 + current_oam_row as u16 * 8;

                let a = self.get_u16(start);     // Original value of that word
                let b = self.get_u16(start - 8); // First word in the preceding row
                let c = self.get_u16(start - 4); // Third word in the preceding row

                let result = if corruption == OamCorruption::Read {
                    b | (a & c)
                } else {
                    ((a ^ c) & (b ^ c)) ^ c
                };

                set_oam_u16(start, result);
                set_oam_u16(start + 2, self.get_u16(start - 6));
                set_oam_u16(start + 4, self.get_u16(start - 4));
                set_oam_u16(start + 6, self.get_u16(start - 2));
            }
        }
    }
}
