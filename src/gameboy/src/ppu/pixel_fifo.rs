use crate::{
    memory_map::{Io, MemoryMap},
    ppu::Ppu,
};

/*
    There are two pixel FIFOs.
    One for background pixels and one for object (sprite) pixels.
    These two FIFOs are not shared.
*/
#[derive(Clone, Copy)]
pub struct PixelFifo {
    color_values: u32,
    pixel_sources: u32,
    background_priority: u16,
    pub pixel_count: i32,
}

impl PixelFifo {
    pub const BACKGROUND_PIXEL: u32 = 0;
    pub const OBJECT0_PIXEL: u32 = 1;
    pub const OBJECT1_PIXEL: u32 = 2;
    pub const WINDOW_PIXEL: u32 = 3;

    pub fn new() -> Self {
        Self {
            color_values: 0,
            pixel_sources: 0,
            background_priority: 0,
            pixel_count: 0,
        }
    }

    pub fn pop(&mut self, memory_map: &MemoryMap, color_palette: &[u32; 4]) -> (u32, u16, u16) {
        // TODO:
        assert!(self.pixel_count >= 0);

        let color = self.color_values & 0x3;
        let source = self.pixel_sources & 0x3;
        let priority = self.background_priority & 0x1;

        self.color_values >>= 2;
        self.pixel_sources >>= 2;
        self.background_priority >>= 1;

        self.pixel_count -= 1;

        let pallete_index = memory_map.get_io(match source {
            Self::BACKGROUND_PIXEL | Self::WINDOW_PIXEL => Io::BGP,
            Self::OBJECT0_PIXEL => Io::OBP0,
            Self::OBJECT1_PIXEL => Io::OBP1,
            _ => unreachable!(),
        });

        let pallete = Ppu::get_color_pallete(color_palette, pallete_index);

        (pallete[color as usize], color as u16, priority)
    }

    pub fn push(&mut self, tile: u16, source: u32, priority: u16) {
        match source {
            Self::OBJECT0_PIXEL | Self::OBJECT1_PIXEL => {
                let mut color_values = 0;
                let mut pixel_sources = 0;
                let mut background_priority = 0;

                for i in 0..8 {
                    let t = i * 2;

                    let current_source = (self.pixel_sources >> t) & 0x3;
                    let current_color = (self.color_values >> t) & 0x3;

                    // TODO: source < current_source is not correct should be source > current_source
                    // Currently doesnt work properly.
                    if i >= self.pixel_count || current_color == 0 || source < current_source {
                        color_values |= (tile as u32) & (0x3 << t);
                        pixel_sources |= source << t;
                        background_priority |= priority << i;
                    } else {
                        color_values |= current_color << t;
                        pixel_sources |= current_source << t;
                        background_priority |= self.background_priority & (0x1 << i);
                    }
                }

                self.pixel_count = 8;
                self.color_values = color_values;
                self.pixel_sources = pixel_sources;
                self.background_priority = background_priority;
            }
            _ => {
                self.color_values |= (tile as u32) << (self.pixel_count * 2);
                self.pixel_sources |= source << (self.pixel_count * 2);
                self.pixel_count += 8;
            }
        }
    }
}
