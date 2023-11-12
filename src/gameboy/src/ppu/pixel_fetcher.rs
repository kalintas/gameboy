
use super::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PixelFetcherMode {
    GetTile,              // 2 dot
    GetTileLow(u8),       // 2 dot
    GetTileHigh(u16, u8), // 2 dot
    Sleep((u8, u8)),      // 2 dot
    Push((u8, u8)),       // 1 dot until success
}

#[derive(Clone)]
pub struct PixelFetcher {
    pub pos_x: u16,

    pub fetching_object: Option<Object>,
    pub hit_object: bool, // This is must be set to true in case of an sprite hit.

    pub is_window: bool,

    pub mode: PixelFetcherMode,
}

impl PixelFetcher {
    pub fn new() -> Self {
        Self {
            pos_x: 0,

            fetching_object: None,
            hit_object: false,
            is_window: false,

            mode: PixelFetcherMode::GetTile,
        }
    }

    pub fn cycle(
        &mut self,
        memory_map: &MemoryMap,
        fifo: &mut PixelFifo,
        oam_fifo: &mut PixelFifo,
        window_internal_line_counter: u16
    ) -> u32 {
        let lcdc = memory_map.get_io(Io::LCDC);
        let ly = memory_map.get_io(Io::LY) as u16;

        let bg_tile_map_start = if lcdc & 0x8 == 0 { 0x9800 } else { 0x9C00 };
        let w_tile_map_start = if lcdc & 0x40 == 0 { 0x9800 } else { 0x9C00 };

        let bg_w_tile_data_start = if lcdc & 0x10 == 0 {
            0x9000 // Although it says 8800, if this is the case, patterns have signed numbers from -128 to 127.
        } else {
            0x8000
        };

        let scy = memory_map.get_io(Io::SCY) as u16;

        // Bit 2 - OBJ (Sprite) Size (0=8x8, 1=8x16)
        let object_height = if lcdc & 0x4 == 0 { 8 } else { 16 };

        self.mode = match self.mode {
            PixelFetcherMode::GetTile => {
                let tile_map_value = if let Some(object) = self.fetching_object {
                    object.tile_index
                } else {
                    
                    if self.is_window {
                        memory_map.ppu_get_vram(
                            w_tile_map_start
                                + ((self.pos_x / 8) & 0x1F)
                                + (window_internal_line_counter / 8) * 32,
                        )
                    } else {
                        let scx = memory_map.get_io(Io::SCX) as u16;

                        // Get background tile from memory map.
                        memory_map.ppu_get_vram(
                            bg_tile_map_start
                                + ((scx / 8 + self.pos_x / 8) & 0x1F)
                                + (((ly + scy) % 256) / 8) * 32,
                        )
                    }
                };

                PixelFetcherMode::GetTileLow(tile_map_value)
            }
            PixelFetcherMode::GetTileLow(tile_map_value) => {
                // Check tile_data_start
                // Vertical Flip
                // Pass tile_low to next mode

                let mut tile_map_value = tile_map_value as i32;
                let mut tile_y; // Y location of the current tile.
                let tile_data_start;

                if let Some(object) = self.fetching_object {
                    tile_y = ly as i32 - (object.pos_y as i32 - 16);

                    if object_height == 16 {
                        tile_map_value &= 0xFE;
                    }

                    if object.attributes & 0x40 != 0 {
                        // Vertical flip
                        tile_y = object_height as i32 - tile_y - 1;
                    }

                    tile_data_start = 0x8000;
                } else {
                    if lcdc & 0x10 == 0 {
                        // Convert tile_map_value to signed value.
                        tile_map_value = ((tile_map_value as u8) as i8) as i32;
                    }

                    let pos_y = if self.is_window {
                        ly
                    } else {
                        ly + scy  
                    };

                    tile_y = ((pos_y % 256) % 8) as i32;
                    tile_data_start = bg_w_tile_data_start;
                }

                let tile_index = (tile_data_start + tile_map_value * 16 + tile_y * 2) as u16;
                let tile_low = memory_map.ppu_get_vram(tile_index);

                PixelFetcherMode::GetTileHigh(tile_index, tile_low)
            }
            PixelFetcherMode::GetTileHigh(tile_index, tile_low) => {
                let tile_high = memory_map.ppu_get_vram(tile_index + 1);

                PixelFetcherMode::Sleep((tile_low, tile_high))
            }
            PixelFetcherMode::Sleep(tile) => PixelFetcherMode::Push(tile),
            PixelFetcherMode::Push((tile_low, tile_high)) => {
                // Push 8 pixels to FIFO
                // Only pushed if fifo is half empty.
                // If horizontal flip is set just push LSB instead of MSB.

                // In case of an sprite hit PPU must wait until pixel fetcher reaches PixelFetcherMode::Push.
                // But any tiles fetched in this process will go away and must be computed again.
                if self.hit_object {
                    self.hit_object = false;
                    self.mode = PixelFetcherMode::GetTile;
                    return 1;
                }

                // Wait until fifo is empty if currently fetching background or window tile.
                if self.fetching_object.is_none() && fifo.pixel_count > 8 {
                    return 1;
                }

                let mut tile = 0;

                for i in (0..8).rev() {
                    // Pixel value in range [0, 3]
                    let pixel = (((tile_high >> i) & 0x1) << 1) | ((tile_low >> i) & 0x1);

                    let horizontal_flip = if let Some(object) = self.fetching_object {
                        (object.attributes & 0x20) != 0
                    } else {
                        false
                    };

                    if horizontal_flip {
                        tile |= (pixel as u16) << (i * 2); // Horizontal flip
                    } else {
                        tile |= (pixel as u16) << ((7 - i) * 2);
                    }
                }

                if let Some(object) = self.fetching_object {
                    let source = if object.attributes & 0x10 == 0 {
                        PixelFifo::OBJECT0_PIXEL
                    } else {
                        PixelFifo::OBJECT1_PIXEL
                    };

                    // Push transparent pixels with lowest priority.
                    oam_fifo.push(tile, source, (object.attributes >> 7) as u16);
                } else {
                    let source = if self.is_window {
                        PixelFifo::WINDOW_PIXEL
                    } else {
                        PixelFifo::BACKGROUND_PIXEL
                    };

                    fifo.push(tile, source, 0);
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