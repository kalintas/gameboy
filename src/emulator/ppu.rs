
use arrayvec::ArrayVec;

use super::memory_map::{Io, MemoryMap};

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub const PPU_CLOCK_RATE: u32 = 4_194_304;
pub const PPU_ONE_FRAME: u32 = 70_224;
pub const PPU_ONE_LINE: u32 = 456;

const COLOR_SHADES: [u32; 4] = [0xff0fbc9b, 0xff0fac8b, 0xff306230, 0xff0f380f];

// Object AKA Sprite
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
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
        top_y <= ly && ly < bottom_y
    }
}

/*
    There are two pixel FIFOs. 
    One for background pixels and one for object (sprite) pixels. 
    These two FIFOs are not shared.
*/
#[derive(Clone, Copy)]
struct PixelFifo {

    color_values: u32,
    pixel_sources: u32,
    background_priority: u16,
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
            background_priority: 0,
            pixel_count: 0,
        }
    }

    fn pop(&mut self, memory_map: &MemoryMap) -> u32 {

        assert!(self.pixel_count >= 0);

        let color = self.color_values & 0x3;
        let source = self.pixel_sources & 0x3;

        self.color_values >>= 2;
        self.pixel_sources >>= 2;
        self.background_priority >>= 1;

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

    fn push(&mut self, tile: u16, source: u32) {

        self.color_values |= (tile as u32) << (self.pixel_count * 2);
        self.pixel_sources |= source << (self.pixel_count * 2);
        self.pixel_count += 8;
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PixelFetcherMode {
    GetTile,              // 2 dot
    GetTileLow(u8),       // 2 dot
    GetTileHigh(u16, u8), // 2 dot
    Sleep((u8, u8)),      // 2 dot
    Push((u8, u8)),       // 1 dot until success
}

struct PixelFetcher {
    pos_x: u16,

    fetching_object: Option<Object>,

    mode: PixelFetcherMode
}

/*
    TODOS:
    1. Dry background render run takes 176 dots instead of 176 dots.
*/

impl PixelFetcher {

    fn new() -> Self {
        Self {
            pos_x: 0,

            fetching_object: None,

            mode: PixelFetcherMode::GetTile
        }
    }

    fn cycle(&mut self, memory_map: &MemoryMap, fifo: &mut PixelFifo, oam_fifo: &mut PixelFifo) -> u32 {

        let lcdc = memory_map.get_io(Io::LCDC);
        let ly = memory_map.get_io(Io::LY) as u16;

        let bg_tile_map_start = if lcdc & 0x8 == 0 { 0x9800 } else { 0x9C00 };
        let bg_tile_data_start = if lcdc & 0x10 == 0 {
            0x9000 // Although it says 8800, if this is the case, patterns have signed numbers from -128 to 127.
        } else {
            0x8000
        };

        self.mode = match self.mode {
            PixelFetcherMode::GetTile => {

                let tile_map_value = if let Some(object) = self.fetching_object {
                    object.tile_index
                } else {
                    // Get background tile from memory map.
                    memory_map.ppu_get_vram(bg_tile_map_start + self.pos_x / 8 + (ly / 8) * 32)
                };

                PixelFetcherMode::GetTileLow(tile_map_value)
            },
            PixelFetcherMode::GetTileLow(tile_map_value) => {
                
                // Check tile_data_start
                // Vertical Flip
                // Pass tile_low to next mode
                
                let mut tile_map_value = tile_map_value as i32;
                let tile_y; // Y location of the current tile.
                let tile_data_start;

                if let Some(object) = self.fetching_object {
                    
                    tile_y = ly as i32 - (object.pos_y as i32 - 16);
                    tile_data_start = 0x8000;
                } else {
                    if lcdc & 0x10 == 0 {
                        // Convert tile_map_value to signed value.
                        tile_map_value = (((tile_map_value as u8) as i8)) as i32;
                    }

                    tile_y = (ly % 8) as i32;
                    tile_data_start = bg_tile_data_start;
                }

                let tile_index = (tile_data_start + tile_map_value * 16 + tile_y * 2) as u16;
                let tile_low = memory_map.ppu_get_vram(tile_index);

                PixelFetcherMode::GetTileHigh(tile_index, tile_low)
            },
            PixelFetcherMode::GetTileHigh(tile_index, tile_low) => {

                let tile_high = memory_map.ppu_get_vram(tile_index + 1);
                
                PixelFetcherMode::Sleep((tile_low, tile_high)) 
            },
            PixelFetcherMode::Sleep(tile) => {
                PixelFetcherMode::Push(tile)
            },
            PixelFetcherMode::Push((tile_low, tile_high)) => {

                // Push 8 pixels to FIFO
                // Only pushed if fifo is half empty.
                // If horizontal flip is set just push LSB instead of MSB.
                
                // Wait until fifo is empty if currently fetching background or window tile.
                if !self.fetching_object.is_some() && fifo.pixel_sources > 8 {
                    return 1;
                }

                let mut tile = 0;

                for i in (0..8).rev() {
                    // Pixel value in range [0, 3]
                    let pixel = (((tile_high >> i) & 0x1) << 1) | ((tile_low >> i) & 0x1);
                    
                    // TODO:
                    // tile |= (pixel as u16) << (i * 2); // Horizontal flip
                    tile |= (pixel as u16) << ((7 - i) * 2);
                }

                if let Some(object) = self.fetching_object {
                    
                    let source = if object.attributes & 0x10 == 0 {
                        PixelFifo::OBJECT0_PIXEL
                    } else {
                        PixelFifo::OBJECT1_PIXEL
                    };

                    // Push transparent pixels with lowest priority.
                    oam_fifo.push(tile, source);
                } else {
                    fifo.push(tile, PixelFifo::BACKGROUND_PIXEL);
                    self.pos_x += 8;
                }

                self.fetching_object = None;

                PixelFetcherMode::GetTile
            }
        };

        // If the mode is GetTile end of the above statement 
        // this means it was the Push mode so it has 1 dot length. 
        if self.mode == PixelFetcherMode::GetTile {
            1
        } else {
            2
        }
    }
}

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

    pos_x: usize,
    fifo: PixelFifo,
    oam_fifo: PixelFifo,

    pixel_fetcher: PixelFetcher,

    found_objects: ArrayVec<Object, 10>,

    pub screen_buffer: Box<[u32; SCREEN_WIDTH * SCREEN_HEIGHT]>,
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            clock_cycles: 0,

            mode: Mode::OamSearch,
            
            pos_x: 0,
            fifo: PixelFifo::new(),
            oam_fifo: PixelFifo::new(),
            pixel_fetcher: PixelFetcher::new(),
            
            found_objects: ArrayVec::new(),

            screen_buffer: Box::new([0u32; SCREEN_WIDTH * SCREEN_HEIGHT]),
        }
    }

    fn oam_search(&mut self, memory_map: &mut MemoryMap, dots: u32) {
        
        // Proccess of searching object is has to be done once per OAM search.
        // Because that this process does not affect CPU it can be done in the beginning of OAM search mode.
        if self.clock_cycles % PPU_ONE_LINE == 0 {
            // Sprite attributes reside in the Sprite Attribute Table (OAM - Object Attribute Memory) at $FE00-FE9F. 
            // Each of the 40 entries consists of four bytes with the following meanings:
            // Byte0 - Y Position
            // Byte1 - X Position
            // Byte2 - Tile/Pattern Number
            // Byte3 - Attributes/Flags
            self.found_objects.clear();

            for index in 0..40 {

                let address = 0xFE00 + index * 4;

                let object = Object {
                    pos_y: memory_map.ppu_get_oam(address),
                    pos_x: memory_map.ppu_get_oam(address + 1),
                    tile_index: memory_map.ppu_get_oam(address + 2),
                    attributes: memory_map.ppu_get_oam(address + 3)
                };

                if object.is_visible(memory_map.get_io(Io::LY)) {

                    self.found_objects.push(object);

                    if self.found_objects.is_full() {
                        break;
                    }
                }            
            }

            // TODO:
            self.found_objects.sort_by(|lhs, rhs| rhs.pos_x.cmp(&lhs.pos_x));
        }

        self.clock_cycles += dots;
        self.update_mode(memory_map);
    }

    fn pixel_transfer(&mut self, memory_map: &mut MemoryMap, dots: u32) {
        
        /*
            Unlike most game consoles, the Game Boy does not always output pixels steadily: 
            some features cause the rendering process to stall for a couple dots
            Three things can cause Mode 3 “penalties”:
            Background scrolling: At the very beginning of Mode 3, rendering is paused for SCX % 8 dots 
                while the same number of pixels are discarded from the leftmost tile.
            Window: After the last non-window pixel is emitted, 
                a 6-dot penalty is incurred while the BG fetcher is being set up for the window.
            Objects: Each object drawn during the scanline (even partially) 
                incurs a 6- to 11-dot penalty.
        */
        
        let ly = memory_map.get_io(Io::LY);

        let mut fetcher_cycles = 0;

        for _ in 0..dots {

            if self.pos_x >= SCREEN_WIDTH {
                break;
            }

            // Pixel fetcher cycle takes 2 dots most of the time 
            // but there is a condition that it can take 1 dot. 
            if fetcher_cycles < dots {
                fetcher_cycles += self.pixel_fetcher.cycle(memory_map, &mut self.fifo, &mut self.oam_fifo);
            }

            // TODO: is there a way that this approach makes an infinite loop of pixel_transfer mode?
            // Check the last element since the found_objects is sorted in decreasing order.
            if let Some(object) = self.found_objects.last() {

                // +8 is added because object positions are biased.
                if self.pos_x + 8 == object.pos_x as usize {

                    // Wait until the pixel fetcher finishes tile fetching.
                    if self.pixel_fetcher.mode != PixelFetcherMode::GetTile {
                        continue;
                    }

                    // Remove the object from the self.found_objects so it doesnt get rendered again.
                    self.pixel_fetcher.fetching_object = self.found_objects.pop();
                }
            }

            // Fifo popping is paused when an object is fetched.
            // This kind of actions makes the pixel transfer mode longer.
            if self.pixel_fetcher.fetching_object.is_some() {
                continue;
            }

            // Popping the pixel fifo is always 1 dot.
            if self.fifo.pixel_count > 8 {

                let mut color = self.fifo.pop(memory_map);

                if self.oam_fifo.pixel_count > 0 {
                    color = self.oam_fifo.pop(memory_map);
                }

                self.screen_buffer[self.pos_x + ly as usize * SCREEN_WIDTH] = color;
                self.pos_x += 1;
            }
        }

        self.clock_cycles += dots;
        self.update_mode(memory_map);
    }
    
    fn hblank(&mut self, memory_map: &mut MemoryMap, dots: u32) {
        
        self.clock_cycles += dots;
        self.update_mode(memory_map);
    }

    fn vblank(&mut self, memory_map: &mut MemoryMap, dots: u32) {

        self.clock_cycles += dots;
        self.update_mode(memory_map);
    }

    fn update_mode(&mut self, memory_map: &mut MemoryMap) {
        
        let line_remainder = self.clock_cycles % PPU_ONE_LINE;
        let line = ((self.clock_cycles % PPU_ONE_FRAME) / PPU_ONE_LINE) as u8;

        // Update LY and LYC.
        if line_remainder == 0 {
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

        let mode = match self.mode {
            Mode::OamSearch => {
                if line_remainder >= 80 {

                    self.pos_x = 0;
                    self.fifo = PixelFifo::new();
                    self.oam_fifo = PixelFifo::new();
                    self.pixel_fetcher = PixelFetcher::new();

                    Mode::PixelTransfer
                } else {
                    return;
                }
            },
            Mode::PixelTransfer => {
                // TODO:
                if self.pos_x >= SCREEN_WIDTH {
                    Mode::HBlank
                } else {
                    return;
                }
            },
            Mode::HBlank => {
                if line_remainder == 0 {
                    if line == 144 {
                        Mode::VBlank
                    } else {
                        Mode::OamSearch
                    }
                } else {
                    return;
                }
            },
            Mode::VBlank => {
                if self.clock_cycles % PPU_ONE_FRAME == 0 {
                    Mode::OamSearch
                } else {
                    return;
                }
            }
        };

        // if mode == Mode::HBlank { println!("{}", (self.clock_cycles % 456) - 80); }

        // Update flags and io registers.
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

        self.mode = mode;
    }

    pub fn cycle(&mut self, memory_map: &mut MemoryMap, dots: u32) {
        assert_eq!(dots, 4);
        match self.mode {
            Mode::OamSearch => self.oam_search(memory_map, dots),
            Mode::PixelTransfer => self.pixel_transfer(memory_map, dots),
            Mode::HBlank => self.hblank(memory_map, dots),
            Mode::VBlank => self.vblank(memory_map, dots)
        };
    }
}