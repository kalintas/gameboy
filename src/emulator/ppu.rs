
use arrayvec::ArrayVec;

use super::memory_map::{Io, MemoryMap};

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

const COLOR_SHADES: [u32; 4] = [0xff0fbc9b, 0xff0fac8b, 0xff306230, 0xff0f380f];

// Object AKA Sprite
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Object {
    pos_y: u8,
    pos_x: u8,
    tile_index: u8,
    attributes: u8,
}

impl Object {

    fn is_visible(&self, ly: u8) -> bool {

        // TODO: Check 8x16
        let top_y = self.pos_y as i32 - 16;
        let bottom_y = top_y + 8;
        let ly = ly as i32;

        // Check if this sprite has to be drawn in this line.
        top_y <= ly && ly <= bottom_y
    }
}

/*
struct PixelFifo {

    color_values: u32,
    pixel_sources: u32,
    pixel_count: i32,
}

impl PixelFifo {

    const BACKGROUND_PIXEL: u32 = 0;
    const OBJECT0_PIXEL: u32 = 1;
    const OBJECT1_PIXEL: u32 = 2;
    const WINDOW_PIXEL: u32 = 3;

    fn new() -> Self {
        Self {
            color_values: 0,
            pixel_sources: 0,
            pixel_count: 0,
        }
    }

    fn clear(&mut self) {
        self.color_values = 0;
        self.pixel_sources = 0;
        self.pixel_count = 0;
    }

    fn pop(&mut self, memory_map: &MemoryMap) -> u32 {

        let color = self.color_values & 0x3;
        let source = self.pixel_sources & 0x3;

        self.color_values <<= 2;
        self.pixel_sources <<= 2;

        self.pixel_count -= 1;

        let pallete_index = memory_map.get(match source {
            Self::BACKGROUND_PIXEL | Self::WINDOW_PIXEL => 0xFF47,
            Self::OBJECT0_PIXEL => 0xFF48,
            Self::OBJECT1_PIXEL => 0xFF49,
            _ => unreachable!()
        });

        let pallete = [
            COLOR_SHADES[(pallete_index & 0x3) as usize],        // White
            COLOR_SHADES[((pallete_index >> 2) & 0x3) as usize], // Light gray
            COLOR_SHADES[((pallete_index >> 4) & 0x3) as usize], // Dark gray
            COLOR_SHADES[((pallete_index >> 6) & 0x3) as usize], // Black
        ];

        pallete[color as usize]
    }
}

struct PixelFetcher {
    pos_x: u32,
    pos_y: u32,
}

impl PixelFetcher {

    fn new() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0
        }
    }

    fn push_fifo(&mut self, fifo: &mut PixelFifo) {


    }
}*/ 

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    OamSearch = 2,
    PixelTransfer = 3,
    HBlank = 0, // Horizontal blank period
    VBlank = 1, // Vertical blank period
}

pub struct Ppu {
    pub clock_cycles: u32,

    mode: Mode,

    found_objects: ArrayVec<Object, 10>,

    pub screen_buffer: Box<[u32; SCREEN_WIDTH * SCREEN_HEIGHT]>,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            clock_cycles: 0,

            mode: Mode::OamSearch,
            found_objects: ArrayVec::new(),

            screen_buffer: Box::new([0u32; SCREEN_WIDTH * SCREEN_HEIGHT]),
        }
    }

    fn oam_search(&mut self, memory_map: &mut MemoryMap) {

        // TODO: fill self.found_objects
            
        // Sprite attributes reside in the Sprite Attribute Table (OAM - Object Attribute Memory) at $FE00-FE9F. 
        // Each of the 40 entries consists of four bytes with the following meanings:
        // Byte0 - Y Position
        // Byte1 - X Position
        // Byte2 - Tile/Pattern Number
        // Byte3 - Attributes/Flags
        for index in 0..40 {

            let address = 0xFE00 + index * 4;

            let object = Object {
                pos_y: memory_map.get(address),
                pos_x: memory_map.get(address + 1),
                tile_index: memory_map.get(address + 2),
                attributes: memory_map.get(address + 3)
            };

            if object.is_visible(memory_map.get_io(Io::LY)) {

                self.found_objects.push(object);

                if self.found_objects.is_full() {
                    break;
                }
            }            
        }

        self.found_objects.sort_by(|lhs, rhs| rhs.pos_x.cmp(&lhs.pos_x));

        self.change_mode(memory_map, Mode::OamSearch);
    }

    // Draws a line of the screen.
    fn pixel_transfer(&mut self, memory_map: &mut MemoryMap) {
        let lcdc = memory_map.get_io(Io::LCDC);
        let ly = memory_map.get_io(Io::LY) as u16;

        // Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)
        let bg_tile_map_start = if lcdc & 0x8 == 0 { 0x9800 } else { 0x9C00 };

        // Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)
        let tile_data_start = if lcdc & 0x10 == 0 {
            0x9000 // Although it says 8800, if this is the case, patterns have signed numbers from -128 to 127.
        } else {
            0x8000
        };

        let get_pallete_data = |index: u8| {
            [
                COLOR_SHADES[(index & 0x3) as usize],        // White
                COLOR_SHADES[((index >> 2) & 0x3) as usize], // Light gray
                COLOR_SHADES[((index >> 4) & 0x3) as usize], // Dark gray
                COLOR_SHADES[((index >> 6) & 0x3) as usize], // Black
            ]
        };

        // TODO: The palettes are defined through registers FF47-FF49 (Non CGB Mode).
        let bg_palette_data = get_pallete_data(memory_map.get(0xFF47));
        let obp0_pallete_data = get_pallete_data(memory_map.get(0xFF48));
        let obp1_pallete_data = get_pallete_data(memory_map.get(0xFF49));

        let obp_palletes = [obp0_pallete_data, obp1_pallete_data];

        // Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)
        let obj_height = if lcdc & 0x4 == 0 {
            8
        } else {
            16
        };

        assert_eq!(obj_height, 8, "unimplemented");
        
        // Render background.
        // Screen is in 20x18 tiles resolution.
        for tile_col in 0..20 {
            let tile_row = ly / 8; // Current tile in y dimension.
            let tile_y = ly % 8; // Y location of the current tile.

            // First get the value from the background tile map.
            let mut tile_map_value =
                memory_map.get(bg_tile_map_start + tile_col + tile_row * 32) as i32;

            if lcdc & 0x10 == 0 {
                // Convert tile_map_value to signed value.
                tile_map_value = (((tile_map_value as u8) as i8)) as i32;
            }

            // Find the current tiles index.
            // Every tile is 16 bytes long so tile_map_value is multiplied by 16.
            // Every tiles width is 2 bytes long so tile_y is multiplied by 2.
            let tile_index = (tile_data_start + tile_map_value * 16 + tile_y as i32 * 2) as u16;

            let tile = memory_map.get_u16(tile_index);

            for j in (0..8).rev() {
                let upper = (tile >> (j + 8)) & 0x1;
                let lower = (tile >> j) & 0x1;

                let color = bg_palette_data[((upper << 1) | lower) as usize];

                let x = tile_col * 8 + 7 - j;

                self.screen_buffer[x as usize + ly as usize * SCREEN_WIDTH] = color;
            }
        }

        // Render objects.
        for object in self.found_objects.iter() {

            // Object is off-screen
            if object.pos_x == 0 || object.pos_x >= 168 {
                continue;
            }
            
            // TODO:
            let tile_y = ly as i32 - (object.pos_y as i32 - 16); // Y location of the current tile.
            let tile = memory_map.get_u16(0x8000 + object.tile_index as u16 * 16 + tile_y as u16 * 2);

            for j in (0..8).rev() {
                let upper = (tile >> (j + 8)) & 0x1;
                let lower = (tile >> j) & 0x1;

                let color_index = ((upper << 1) | lower) as usize;

                // Transparent pixel.
                if color_index == 0 {
                    continue;
                }

                let x = (object.pos_x as i32 - 8) + (7 - j as i32);

                if x < 0 || x > SCREEN_WIDTH as i32 {
                    continue;
                }

                let screen_index = x as usize + ly as usize * SCREEN_WIDTH;

                if object.attributes & 0x80 != 0 {
                    // OBJ is behind background.
                    if self.screen_buffer[screen_index] != bg_palette_data[0] {
                        continue;
                    }
                }

                let color = obp_palletes[((object.attributes >> 4) & 0x1) as usize][color_index];
                self.screen_buffer[screen_index] = color;
            }
        }

        self.found_objects.clear();

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
        if (mode == Mode::OamSearch && stat & 0x20 != 0)
            || (mode == Mode::VBlank && stat & 0x10 != 0)
            || (mode == Mode::HBlank && stat & 0x08 != 0)
        {
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
            memory_map.set_io(
                Io::STAT,
                (memory_map.get_io(Io::STAT) & 0xFB) | ((lyc == line) as u8) << 2,
            );

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
                _ => {}
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

// FIFO: 4MHz 
// Fetch 2MHz
