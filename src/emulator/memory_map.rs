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

use std::path::Path;

pub struct MemoryMap {
    rom_banks: [[u8; 0x4000]; 2], // 16KB rom banks that is 'in the cartridge'.
    vrams: [[u8; 0x2000]; 2],     // 8KB video rams(VRAM)
    external_ram: [u8; 0x2000],   // 8KB external ram that is'in the cartridge'.
    wrams: [[u8; 0x1000]; 8],     // 4KB work rams(WRAM)
    oam: [u8; 0x100],             // Sprite Attribute Table(OAM)
    io_ports: [u8; 0x80],
    high_ram: [u8; 0x7F],
    ier: u8, // Interrupt Enable Register

    pub changes: Vec<(usize, u8)>,
}

#[repr(u16)]
#[allow(dead_code)]
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

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            rom_banks: [[0u8; 0x4000]; 2],
            vrams: [[0u8; 0x2000]; 2],
            external_ram: [0u8; 0x2000],
            wrams: [[0u8; 0x1000]; 8],
            oam: [0u8; 0x100],
            io_ports: [0u8; 0x80],
            high_ram: [0u8; 0x7F],
            ier: 0u8,

            changes: Vec::new(),
        }
    }

    pub fn after_boot() -> Self {
        let mut memory = Self::new();
        
        memory.set_io(Io::TIMA, 0x00);
        memory.set_io(Io::TMA, 0x00);
        memory.set_io(Io::TAC, 0x00);
        memory.set_io(Io::NR10, 0x80);
        memory.set_io(Io::NR11, 0xBF);
        memory.set_io(Io::NR12, 0xF3);
        memory.set_io(Io::NR14, 0xBF);
        memory.set_io(Io::NR21, 0x3F);
        memory.set_io(Io::NR22, 0x00);
        memory.set_io(Io::NR24, 0xBF);
        memory.set_io(Io::NR30, 0x7F);
        memory.set_io(Io::NR31, 0xFF);
        memory.set_io(Io::NR32, 0x9F);
        memory.set_io(Io::NR33, 0xBF);
        memory.set_io(Io::NR41, 0xFF);
        memory.set_io(Io::NR42, 0x00);
        memory.set_io(Io::NR43, 0x00);
        memory.set_io(Io::NR30, 0xBF);
        memory.set_io(Io::NR50, 0x77);
        memory.set_io(Io::NR51, 0xF3);
        memory.set_io(Io::NR52, 0xF1);
        memory.set_io(Io::LCDC, 0x91);
        memory.set_io(Io::SCY, 0x00);
        memory.set_io(Io::SCX, 0x00);
        memory.set_io(Io::LYC, 0x00);
        memory.set_io(Io::BGP, 0xFC);
        memory.set_io(Io::OBP0, 0xFF);
        memory.set_io(Io::OBP1, 0xFF);
        memory.set_io(Io::WY, 0x00);
        memory.set_io(Io::WX, 0x00);
        memory.set_io(Io::IE, 0x00);

        memory
    }

    pub fn load_rom<T: AsRef<Path>>(&mut self, path: T) {
        let rom = std::fs::read(path).unwrap();

        self.rom_banks[0].copy_from_slice(&rom[..0x4000]);
        self.rom_banks[1].copy_from_slice(&rom[0x4000..0x8000]);

        for i in 0..rom.len() {
            self.changes.push((i, rom[i]));
        }
    }

    pub fn set(&mut self, address: u16, value: u8) {
        let address = address as usize;

        self.changes.push((address, value));

        if address < 0x8000 {
            // 0000-3FFF   16KB ROM Bank 00
            // 4000-7FFF   16KB ROM Bank 01..NN
            // panic!("Tried to mutate immutable rom data");
        } else if address < 0xA000 {
            // 8000-9FFF   8KB Video RAM (VRAM)
            self.vrams[0][address - 0x8000] = value;
        } else if address < 0xC000 {
            // A000-BFFF   8KB External RAM
            self.external_ram[address - 0xA000] = value;
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
            self.oam[address - 0xFE00] = value;
        } else if address < 0xFF00 {
            // FEA0-FEFF   Not Usable
            // panic!("Tried to write a non usable memory block");
        } else if address < 0xFF80 {
            // FF00-FF7F   I/O Ports
            self.io_ports[address - 0xFF00] = value;
        } else if address < 0xFFFF {
            self.high_ram[address - 0xFF80] = value;
        } else {
            self.ier = value;
        }
    }

    pub fn get(&self, address: u16) -> u8 {
        let address = address as usize;

        if address < 0x4000 {
            // 0000-3FFF   16KB ROM Bank 00
            return self.rom_banks[0][address];
        }
        if address < 0x8000 {
            // 4000-7FFF   16KB ROM Bank 01..NN
            return self.rom_banks[1][address - 0x4000];
        }
        if address < 0xA000 {
            // 8000-9FFF   8KB Video RAM (VRAM)
            return self.vrams[0][address - 0x8000];
        }
        if address < 0xC000 {
            // A000-BFFF   8KB External RAM
            return self.external_ram[address - 0xA000];
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
            return self.oam[address - 0xFE00];
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

    pub fn get_io(&self, address: Io) -> u8 {
        self.get(address as u16)
    }

    pub fn get_u16(&self, address: u16) -> u16 {
        let lsb = self.get(address) as u16; // Get the least significant byte
        let msb = self.get(address + 1) as u16; // Get the most significant byte

        (msb << 8) | lsb
    }

    pub fn set_u16(&mut self, address: u16, value: u16) {

        let lsb = (value & 0xFF) as u8; // Get the least significant byte
        let msb = ((value >> 8) & 0xFF) as u8; // Get the most significant byte

        self.set(address, lsb);
        self.set(address + 1, msb);
    }

    pub fn set_io(&mut self, address: Io, value: u8) {
        self.set(address as u16, value)
    }

}
