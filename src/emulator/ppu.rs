use super::memory_map::{MemoryMap, Io};

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const COLOR_SHADES: [u32; 4] = [
    0xff0fbc9b,
    0xff0fac8b,
    0xff306230,
    0xff0f380f
];

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    OamSearch = 2,
    PixelTransfer = 3,
    HBlank = 0, // Horizontal blank period
    VBlank = 1  // Vertical blank period
}

pub struct Ppu {

    pub clock_cycles: u32,

    mode: Mode,

    pub screen_buffer: Box<[u32; SCREEN_WIDTH * SCREEN_HEIGHT]>
}

impl Ppu {

    pub fn new() -> Self {

        Self {

            clock_cycles: 0,

            mode: Mode::OamSearch,

            screen_buffer: Box::new([0u32; SCREEN_WIDTH * SCREEN_HEIGHT])
        }
    }

    fn oam_search(&mut self, memory_map: &mut MemoryMap) {
        self.change_mode(memory_map, Mode::OamSearch);

    }

    fn pixel_transfer(&mut self, memory_map: &mut MemoryMap) {

        // Draw a line of the screen.
        let lcdc = memory_map.get_io(Io::LCDC);
        let ly = memory_map.get_io(Io::LY) as u16;

        // Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
        let bg_tile_map_start = if lcdc & 0x8 == 0 {
            0x9800
        } else {
            0x9C00
        };

        // // Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
        let tile_data_start = if lcdc & 0x10 != 0 {
            0x9000 // Although it says 8800, if this is the case, patterns have signed numbers from -128 to 127.
        } else {
            0x8000
        };

        // // TODO: The palettes are defined through registers FF47-FF49 (Non CGB Mode).
        // let color_shades: [u32; 4] = [
        //     memory_map.get(0xFF47),
        //     memory_map.get(0xFF48),
        //     memory_map.get(0xFF49),
        //     memory_map.get(0xFF4A),
        // ];

        // Screen is in 20x18 tiles resolution.
        for tile_col in 0..20 {
            
            let tile_row = ly / 8; // Current tile in y dimension.
            let tile_y = ly % 8; // Current tiles y location.

            // First get the value from the background tile map.
            let tile_map_value = memory_map.get(bg_tile_map_start + tile_col + tile_row * 32) as i32;

            // TODO: currently doesnt work as supposed to. 
            // if lcdc & 0x10 == 0 {
            //     // Convert tile_map_value to signed value.
            //     tile_map_value = (((tile_map_value as u8) as i8)) as i32;
            // }

            // Find the current tiles index.
            // Every tile is 16 bytes long so tile_map_value is multiplied by 16.
            // Every tiles width is 2 bytes long so tile_y is multiplied by 2.
            let tile_index = (tile_data_start + tile_map_value * 16 + tile_y as i32 * 2) as u16;

            let first = memory_map.get(tile_index);
            let second = memory_map.get(tile_index + 1);

            for j in (0..8).rev() {

                let upper = (second >> j) & 0x1;
                let lower = (first >> j) & 0x1;

                let color =  COLOR_SHADES[((upper << 1) | lower) as usize];

                let x = tile_col * 8 + 7 - j;

                self.screen_buffer[x as usize + ly as usize * SCREEN_WIDTH] = color;
            }
        }
        
        // Change the mode after the proccess. Because memory_map wont allow access in Pixel Transfer mode.
        self.change_mode(memory_map, Mode::PixelTransfer);
    }

    fn hblank(&mut self, memory_map: &mut MemoryMap) {
        self.change_mode(memory_map, Mode::HBlank);
    }

    fn vblank(&mut self, memory_map: &mut MemoryMap) {
        self.change_mode(memory_map, Mode::VBlank);
    }

    fn change_mode(&mut self, memory_map: &mut MemoryMap, mode: Mode) {

        self.mode = mode;

        let stat = memory_map.get_io(Io::STAT);

        // "The two lower STAT bits show the current status of the LCD controller."
        // Change the mode in the STAT register.
        memory_map.set_io(Io::STAT, (stat & 0xFC) | (mode as u8));

        // Request a LDC STAT interrupt in case of interrupt bits set.
        if (mode == Mode::OamSearch && stat & 0x20 != 0) || 
            (mode == Mode::VBlank && stat & 0x10 != 0) || 
            (mode == Mode::HBlank && stat & 0x08 != 0) {

            memory_map.set_io(Io::IF, memory_map.get_io(Io::IF) | 0x2);
        }

        // Request VBlank interrupt when mode changes to VBlank
        if mode == Mode::VBlank {
            memory_map.set_io(Io::IF, memory_map.get_io(Io::IF) | 0x1);
        }

        // println!("{} {:?}", self.clock_cycles % (114 * 154), mode);
    }   

    pub fn cycle(&mut self, memory_map: &mut MemoryMap) {

        let cycle = self.clock_cycles % (114 * 154);

        let line = (cycle / 114) as u8;

        if cycle % 114 == 0 {
            memory_map.set_io(Io::LY, line);
            
            let lyc = memory_map.get_io(Io::LYC);

            // Set the coincident flag.
            memory_map.set_io(Io::STAT, (memory_map.get_io(Io::STAT) & 0xFB) | ((lyc == line) as u8) << 2);

            if lyc == line {
                // Cause interrupt in case of Coincidence interrupt is set.
                if memory_map.get_io(Io::STAT) & 0x40 != 0 {
                    // Request a LCD STAT interrupt.
                    memory_map.set_io(Io::IF, memory_map.get_io(Io::IF) | 0x2);
                }
            }
        }

        if line < 144 {

            match cycle % 114 {
                0 => self.oam_search(memory_map),
                20 => self.pixel_transfer(memory_map),
                63 => self.hblank(memory_map),
                _ => {},
            }
        } else if line == 144 && cycle % 114 == 0 {
            self.vblank(memory_map);

        } else if line >= 154 {
            // self.clock_cycles = 0;
            return;
        }

        self.clock_cycles += 1;
    }
    
    

}