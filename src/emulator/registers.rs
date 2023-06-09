use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Registers {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8, // flag register
    pub a: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn after_boot() -> Self {
        let mut reg = Self::new();

        reg.set_af(0x01B0);
        reg.set_bc(0x0013);
        reg.set_de(0x00D8);
        reg.set_hl(0x014D);

        reg
    }

    // NOTE: endiannes?
    pub fn get_u16_at(&self, index: usize) -> u16 {
        match index {
            0x0 => ((self.b as u16) << 8) | self.c as u16, // BC register
            0x1 => ((self.d as u16) << 8) | self.e as u16, // DE register
            0x2 => ((self.h as u16) << 8) | self.l as u16, // HL register
            0x3 => ((self.a as u16) << 8) | self.f as u16, // AF register
            _ => panic!("wrong index parameter!"),
        }
    }

    pub fn set_u16_at(&mut self, index: usize, val: u16) {
        let upper = (val >> 8) as u8;
        let lower = (val & 0xFF) as u8;

        match index {
            0x0 => {
                self.b = upper;
                self.c = lower;
            } // BC register
            0x1 => {
                self.d = upper;
                self.e = lower;
            } // DE register
            0x2 => {
                self.h = upper;
                self.l = lower;
            } // HL register
            0x3 => {
                self.a = upper;
                self.f = lower;
            } // AF register
            _ => panic!("wrong index parameter!"),
        }
    }

    pub fn bc(&self) -> u16 {
        self.get_u16_at(0x0)
    }
    pub fn de(&self) -> u16 {
        self.get_u16_at(0x1)
    }
    pub fn hl(&self) -> u16 {
        self.get_u16_at(0x2)
    }
    pub fn af(&self) -> u16 {
        self.get_u16_at(0x3)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.set_u16_at(0x0, val)
    }
    pub fn set_de(&mut self, val: u16) {
        self.set_u16_at(0x1, val)
    }
    pub fn set_hl(&mut self, val: u16) {
        self.set_u16_at(0x2, val)
    }
    pub fn set_af(&mut self, val: u16) {
        self.set_u16_at(0x3, val)
    }

    // CY flag -> "set to 1 when an operation results in carrying from or borrowing to bit 7"
    // H flag  -> "set to 1 when an operation results in carrying from or borrowing to bit 3"
    // N flag  -> "set to 1 following execution of the substruction instruction"
    // Z flag  -> "set to 1 when the result of an operation is 0 otherwise reset"

    pub fn get_cy(&self) -> u8 {
        (self.f >> 4) & 0x1
    }
    pub fn get_h(&self) -> u8 {
        (self.f >> 5) & 0x1
    }
    pub fn get_n(&self) -> u8 {
        (self.f >> 6) & 0x1
    }
    pub fn get_z(&self) -> u8 {
        (self.f >> 7) & 0x1
    }
}

impl Index<usize> for Registers {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 8, "wrong index parameter!");
        unsafe { &*((self as *const Self) as *const u8).add(index) }
    }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 8, "wrong index parameter!");
        unsafe { &mut *((self as *mut Self) as *mut u8).add(index) }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "registers = ( AF: {:#06x} BC: {:#06x} DE: {:#06x} HL: {:#06x} )",
            self.af(),
            self.bc(),
            self.de(),
            self.hl()
        )
    }
}
