mod pixel_fetcher;
mod pixel_fifo;

use arrayvec::ArrayVec;
use strum_macros::AsRefStr;

use self::{
    pixel_fetcher::{PixelFetcher, PixelFetcherMode},
    pixel_fifo::PixelFifo,
};

use super::memory_map::{Io, MemoryMap};

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub const PPU_CLOCK_RATE: u32 = 4_194_304;
pub const PPU_ONE_FRAME: u32 = 70_224;
pub const PPU_ONE_LINE: u32 = 456;

// Object AKA Sprite
#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Object {
    pos_y: u8,
    pos_x: u8,
    tile_index: u8,
    attributes: u8,
}

impl Object {
    fn is_visible(&self, ly: u8, lcdc: u8) -> bool {
        let object_height = if lcdc & 0x4 == 0 { 8 } else { 16 };

        let top_y = self.pos_y as i32 - 16;
        let bottom_y = top_y + object_height as i32;
        let ly = ly as i32;

        // Check if this sprite has to be drawn in this line.
        top_y <= ly && ly < bottom_y
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, AsRefStr)]
pub enum Mode {
    OamSearch = 2,
    PixelTransfer = 3,
    HBlank = 0, // Horizontal blank period
    VBlank = 1, // Vertical blank period,
}

#[derive(Clone)]
pub struct Ppu {
    pub clock_cycles: u32,

    mode: Mode,

    enabled: bool,
    is_first_frame: bool,

    pos_x: usize,
    scroll_x: u8,

    // The window keeps an internal line counter that’s functionally similar to LY, and increments alongside it.
    window_line_counter: u16,

    fifo: PixelFifo,
    oam_fifo: PixelFifo,

    pixel_fetcher: PixelFetcher,

    found_objects: ArrayVec<Object, 10>,

    pub screen_buffer: Box<[u32; SCREEN_WIDTH * SCREEN_HEIGHT]>,
    pub color_shades: [u32; 4],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            clock_cycles: 0,

            mode: Mode::OamSearch,

            enabled: false,
            is_first_frame: true,

            pos_x: 0,
            scroll_x: 0,
            window_line_counter: 0,

            fifo: PixelFifo::new(),
            oam_fifo: PixelFifo::new(),
            pixel_fetcher: PixelFetcher::new(),

            found_objects: ArrayVec::new(),

            screen_buffer: Box::new([0u32; SCREEN_WIDTH * SCREEN_HEIGHT]),
            color_shades: [0xff0fbc9b, 0xff0fac8b, 0xff306230, 0xff0f380f],
        }
    }

    pub fn after_boot() -> Self {
        todo!()
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
                    attributes: memory_map.ppu_get_oam(address + 3),
                };

                if object.is_visible(memory_map.get_io(Io::LY), memory_map.get_io(Io::LCDC)) {
                    self.found_objects.push(object);

                    if self.found_objects.is_full() {
                        break;
                    }
                }
            }

            // TODO:
            self.found_objects
                .sort_by(|lhs, rhs| rhs.pos_x.cmp(&lhs.pos_x));
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

        let lcdc = memory_map.get_io(Io::LCDC);

        let bg_w_enable = lcdc & 0x1 != 0; // LCDC.0 — BG and Window enable/priority
        let obj_enable = lcdc & 0x2 != 0; // LCDC.1 — OBJ enable
        let window_enable = bg_w_enable && lcdc & 0x20 != 0; // LCDC.5 — Window enable

        let wx = memory_map.get_io(Io::WX);
        let wy = memory_map.get_io(Io::WY);

        let ly = memory_map.get_io(Io::LY);
        let scx = memory_map.get_io(Io::SCX);

        let mut fetcher_cycles = 0;

        for _ in 0..dots {
            if self.pos_x >= SCREEN_WIDTH + 8 {
                break;
            }

            if window_enable && ly >= wy && self.pos_x == wx as usize + 1 {
                // Start of the window.
                if !self.pixel_fetcher.is_window {
                    self.fifo = PixelFifo::new(); // Flush the fifo.
                    self.pixel_fetcher.mode = PixelFetcherMode::GetTile;
                    self.pixel_fetcher.pos_x = 0;
                }
                self.pixel_fetcher.is_window = true;
            }

            // Pixel fetcher cycle takes 2 dots most of the time
            // but there is a condition that it can take 1 dot.
            if fetcher_cycles < dots {
                fetcher_cycles += self.pixel_fetcher.cycle(
                    memory_map,
                    &mut self.fifo,
                    &mut self.oam_fifo,
                    self.window_line_counter,
                );
            }

            // Check the last element since the found_objects is sorted in decreasing order.
            if let Some(object) = self.found_objects.last() {
                if self.pos_x == object.pos_x as usize {
                    // Wait until the pixel fetcher finishes tile fetching.
                    if self.pixel_fetcher.mode != PixelFetcherMode::GetTile {
                        self.pixel_fetcher.hit_object = true;
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
                let (mut color, color_index, _) = self.fifo.pop(memory_map, &self.color_shades);

                // Discard the first scrolled pixels that are smaller than a tile.
                // This creates a smooth scrolling effect.
                if self.scroll_x < scx % 8 {
                    self.scroll_x += 1;
                    continue;
                }

                if self.oam_fifo.pixel_count > 0 {
                    let (object_color, object_color_index, priority) =
                        self.oam_fifo.pop(memory_map, &self.color_shades);

                    if obj_enable {
                        if !bg_w_enable
                            || (object_color_index != 0 && (priority == 0 || color_index == 0))
                        {
                            color = object_color;
                        }
                    }
                } else if !bg_w_enable {
                    color = self.color_shades[0]; // Background and window disabled. Render a white color.
                }

                /*
                    From pandocs:
                    When re-enabling the LCD, the PPU will immediately start drawing again,
                        but the screen will stay blank during the first frame.
                */
                if !self.is_first_frame && self.pos_x >= 8 {
                    self.screen_buffer[(self.pos_x - 8) + ly as usize * SCREEN_WIDTH] = color;
                }

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

        memory_map.current_oam_row = if line_remainder == 0 { Some(1) } else { None };

        let mode = match self.mode {
            Mode::OamSearch => {
                memory_map.current_oam_row = Some(line_remainder as u16 / 4 + 1);

                if line_remainder >= 80 {
                    self.pos_x = 0;
                    self.scroll_x = 0;
                    self.fifo = PixelFifo::new();
                    self.oam_fifo = PixelFifo::new();
                    self.pixel_fetcher = PixelFetcher::new();

                    // Push a background tile that will never be on screen.
                    // This is done for rendering objects with x < 8.
                    self.fifo.push(0, PixelFifo::BACKGROUND_PIXEL, 0);

                    Mode::PixelTransfer
                } else {
                    return;
                }
            }
            Mode::PixelTransfer => {
                // TODO:
                if self.pos_x >= SCREEN_WIDTH + 8 {
                    if self.pixel_fetcher.is_window {
                        // Increment the window internal line counter.
                        self.window_line_counter += 1;
                    }

                    Mode::HBlank
                } else {
                    return;
                }
            }
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
            }
            Mode::VBlank => {
                if self.clock_cycles % PPU_ONE_FRAME == 0 {
                    // Finished a frame.
                    self.is_first_frame = false;
                    self.window_line_counter = 0; // Reset the window internal line counter.
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
        let lcdc = memory_map.get_io(Io::LCDC);

        if lcdc & 0x80 == 0 {
            // LCDC is disabled.

            if self.enabled {
                // Ppu renders an empty screen when LCD is turned off.
                // Clear the screen.
                self.screen_buffer.fill(self.color_shades[0]);

                // Reset ppu flags.
                memory_map.set_io(Io::LY, 0);
                memory_map.set_io(Io::STAT, 0);
                self.clock_cycles = dots;
                self.mode = Mode::OamSearch;

                self.enabled = false;
                self.is_first_frame = true;
            }
            return;
        }

        self.enabled = true;

        assert_eq!(dots, 4);
        match self.mode {
            Mode::OamSearch => self.oam_search(memory_map, dots),
            Mode::PixelTransfer => self.pixel_transfer(memory_map, dots),
            Mode::HBlank => self.hblank(memory_map, dots),
            Mode::VBlank => self.vblank(memory_map, dots),
        };
    }

    #[allow(dead_code)]
    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    #[allow(dead_code)]
    pub fn is_first_frame(&self) -> bool {
        self.is_first_frame
    }

    pub fn get_color_pallete(color_shades: &[u32; 4], pallete_index: u8) -> [u32; 4] {
        [
            color_shades[(pallete_index & 0x3) as usize], // White
            color_shades[((pallete_index >> 2) & 0x3) as usize], // Light gray
            color_shades[((pallete_index >> 4) & 0x3) as usize], // Dark gray
            color_shades[((pallete_index >> 6) & 0x3) as usize], // Black
        ]
    }
}
