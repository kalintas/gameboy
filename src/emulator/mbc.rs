use dyn_clone::DynClone;

pub trait Mbc: DynClone {

    fn new(rom_bank_count: usize, ram_bank_count: usize) -> Self where Self: Sized;

    fn set(&mut self, address: u16, value: u8);

    fn get_rom_bank(&self) -> usize;
    fn get_ram_bank(&self) -> Option<usize>;
}

dyn_clone::clone_trait_object!(Mbc);

#[derive(Clone)]
pub struct NoMbc;

impl Mbc for NoMbc {

    fn new(_: usize, _: usize) -> Self where Self: Sized {
        Self {}
    }

    fn set(&mut self, _: u16, _: u8) {}
    fn get_rom_bank(&self) -> usize {
        1
    }
    fn get_ram_bank(&self) -> Option<usize> {
        Some(0)
    }
}

#[derive(Clone)]
pub struct Mbc1 {
    rom_bank_count: usize,
    ram_bank_count: usize,
    
    rom_bank: u8,
    secondary_bank: u8,
    
    banking_mode: u8,

    ram_enabled: bool,
}

impl Mbc for Mbc1 {

    fn new(rom_bank_count: usize, ram_bank_count: usize) -> Self where Self: Sized {
        Self {
            rom_bank_count,
            ram_bank_count,
            rom_bank: 1,
            secondary_bank: 0,
            banking_mode: 0,
            ram_enabled: false,
        }
    }

    fn set(&mut self, address: u16, value: u8) {

        if address < 0x2000 {
            if value & 0xF == 0xA {
                // Enable ram.
                self.ram_enabled = true;
            } else {
                // Disable ram.
                self.ram_enabled = false;
            }
        } else if address < 0x4000 {

            self.rom_bank = value & 0x1F;
            
            // 00->01 translation
            if self.rom_bank == 0 {
                self.rom_bank = 1;
            }

            self.rom_bank = self.rom_bank & (self.rom_bank_count as u8 - 1);
        } else if address < 0x6000 {

            // TODO: check ram size also
            self.secondary_bank = value & 0x3;

        } else if address < 0x8000 {
            self.banking_mode = value & 0x1;
        }
    }
    fn get_rom_bank(&self) -> usize {
        
        (if self.banking_mode == 0 && self.rom_bank_count > 0x20 {
            (self.secondary_bank << 5) + self.rom_bank
        } else {
            self.rom_bank
        }) as usize
    }
    fn get_ram_bank(&self) -> Option<usize> {

        if self.ram_enabled {
            
            if self.banking_mode == 1 && self.ram_bank_count <= 4 {
                Some(self.secondary_bank as usize)
            } else {
                Some(0)
            }
        } else {
            None
        }
    }
}

