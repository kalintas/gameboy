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
            panic!("Tried to mutate immutable rom data");
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
            panic!("Tried to write a non usable memory block");
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

    pub fn get_u16(&self, address: u16) -> u16 {
        let lsb = self.get(address) as u16; // Get the least significant byte
        let msb = self.get(address + 1) as u16; // Get the most significant byte

        (msb << 8) | lsb
    }
}
