use super::{instructions::{INSTRUCTIONS, self}, memory_map::MemoryMap};


pub struct MemoryString {
    string: Box<[u8; 0x1000 * 57]>,
}

impl MemoryString {
    pub fn new() -> Self {
        let mut string = Box::new([' ' as u8; 0x1000 * 57]);

        for i in 0u32..0x1000u32 {
            let t = i as usize * 57;

            string[t] = char::from_digit(((i * 0x10) & 0xF000) >> 12, 16).unwrap() as u8;
            string[t + 1] = char::from_digit(((i * 0x10) & 0x0F00) >> 8, 16).unwrap() as u8;
            string[t + 2] = char::from_digit(((i * 0x10) & 0x00F0) >> 4, 16).unwrap() as u8;
            string[t + 3] = char::from_digit((i * 0x10) & 0x000F, 16).unwrap() as u8;
            string[t + 4] = ':' as u8;
            
            for j in 0..32 {
                string[t + j + 6] = '0' as u8;
            }

            for j in 0..16 {
                string[t + j + 39] = '.' as u8;
            }

            string[t + 56] = '\n' as u8;
        }

        Self { string }
    }

    pub fn update(&mut self, address: usize, value: u8) {
        let index = (address / 0x10) * 57 + (address % 0x10) * 2 + 6;

        self.string[index] = char::from_digit(((value & 0xF0) >> 4) as _, 16).unwrap() as u8;
        self.string[index + 1] = char::from_digit((value & 0x0F) as _, 16).unwrap() as u8;

        let text_index = (address / 0x10) * 57 + (address % 0x10) + 39;

        let mut character = value as char;

        if !character.is_ascii_graphic() {
            character = '.';
        }

        self.string[text_index] = character as u8;
    }
}

impl AsRef<str> for MemoryString {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&*self.string).unwrap()
    }
}

pub struct DisassemblyString {
    
    pub strings: Vec<String>
}

impl DisassemblyString {

    pub fn new() -> Self {
        let mut strings = Vec::with_capacity(0x10000);

        for i in 0..0x10000 {

            strings.push(format!("{:04x} 00       NOP",  i));
        }

        Self {
            strings
        }
    }

    pub fn update(&mut self, memory_map: &MemoryMap) {

        // let instruction = INSTRUCTIONS[value as usize];
        


    }
}


