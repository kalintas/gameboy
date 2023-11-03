use imgui::TextureId;

use crate::{
    emulator::{
        memory_map::Io,
        ppu::{self},
    },
    renderer::framebuffer::Framebuffer,
};

use super::Panel;

const PANEL_WIDTH: usize = 32 * 8;
const PANEL_HEIGHT: usize = 32 * 8;

const SCROLL_LINE_COLOR: u32 = 0xFF0000FF;

pub struct BgMapPanel {
    opened: bool,
    first_run: bool,

    old_scx: u8,
    old_scy: u8,

    panel_buffer: Box<[u32; PANEL_WIDTH * PANEL_HEIGHT]>,
    framebuffer: Framebuffer,
}

impl BgMapPanel {
    pub fn new() -> Self {
        Self {
            opened: false,
            first_run: false,

            old_scx: 0,
            old_scy: 0,

            panel_buffer: Box::new([0; PANEL_WIDTH * PANEL_HEIGHT]),
            framebuffer: Framebuffer::new(),
        }
    }
}

impl Panel for BgMapPanel {
    fn update(
        &mut self,
        emulator: &crate::emulator::Emulator,
        changes: &std::collections::HashMap<u16, u8>,
    ) {
        if !self.opened {
            self.first_run = true;
            return;
        }

        let lcdc = emulator.memory_map.get_io(Io::LCDC);
        let bg_tile_map_start = if lcdc & 0x8 == 0 { 0x9800 } else { 0x9C00 };

        let bg_tile_data_start = if lcdc & 0x10 == 0 {
            0x9000 // Although it says 8800, if this is the case, patterns have signed numbers from -128 to 127.
        } else {
            0x8000
        };

        let pallete = ppu::Ppu::get_color_pallete(emulator.memory_map.get_io(Io::BGP));

        let scx = emulator.memory_map.get_io(Io::SCX);
        let scy = emulator.memory_map.get_io(Io::SCY);

        if !self.first_run
            && (self.old_scx == scx && self.old_scy == scy)
            && !changes.iter().any(|(&address, _)| {
                (bg_tile_map_start <= address && address < bg_tile_map_start + 0x400)
                    || (bg_tile_data_start <= address && address < bg_tile_map_start + 0x1000)
            })
        {
            return;
        }
        self.first_run = false;

        for y in 0..PANEL_HEIGHT {
            for x in 0..32 {
                let mut tile_map_value = unsafe {
                    emulator
                        .memory_map
                        .get_vram(bg_tile_map_start + x + (y as u16 / 8) * 32)
                        as i32
                };

                if lcdc & 0x10 == 0 {
                    // Convert tile_map_value to signed value.
                    tile_map_value = ((tile_map_value as u8) as i8) as i32;
                }

                let tile_index = (bg_tile_data_start as i32
                    + tile_map_value * 16
                    + (y as u16 % 8) as i32 * 2) as u16;

                let lower = unsafe { emulator.memory_map.get_vram(tile_index) };
                let upper = unsafe { emulator.memory_map.get_vram(tile_index + 1) };

                for j in (0..8).rev() {
                    let upper = (upper >> j) & 0x1;
                    let lower = (lower >> j) & 0x1;

                    let color = pallete[((upper << 1) | lower) as usize];

                    self.panel_buffer[(x * 8 + 7 - j) as usize + y as usize * PANEL_WIDTH] = color;
                }
            }
        }

        self.old_scx = scx;
        self.old_scy = scy;

        let scx = scx as usize;
        let scy = scy as usize;

        let mut draw_line = |start| {
            let width = scx + ppu::SCREEN_WIDTH;

            self.panel_buffer[start + scx..start + PANEL_WIDTH.min(width)].fill(SCROLL_LINE_COLOR);
            if width > PANEL_WIDTH {
                self.panel_buffer[start..start + width % PANEL_WIDTH].fill(SCROLL_LINE_COLOR);
            }
        };

        // Draw horizontal lines.
        draw_line(scy * PANEL_WIDTH);
        draw_line(((scy + ppu::SCREEN_HEIGHT) % PANEL_HEIGHT) * PANEL_WIDTH);

        // Draw vertical lines.
        for i in 0..ppu::SCREEN_HEIGHT {
            let left = ((scy as usize + i) % PANEL_HEIGHT) * PANEL_WIDTH;

            self.panel_buffer[left + scx] = SCROLL_LINE_COLOR;
            self.panel_buffer[left + (scx + ppu::SCREEN_WIDTH) % PANEL_WIDTH] = SCROLL_LINE_COLOR;
        }

        self.framebuffer.update_buffer(
            PANEL_WIDTH as _,
            PANEL_HEIGHT as _,
            self.panel_buffer.as_ptr() as _,
            gl::RGBA8,
            gl::RGBA,
        );
    }

    fn render(&mut self, ui: &imgui::Ui, _: &mut crate::emulator::Emulator, _: f32, _: f32) {
        if !self.opened {
            return;
        }

        self.opened &= ui
            .window(self.get_name())
            .opened(&mut self.opened)
            .resizable(true)
            .size([400.0, 420.0], imgui::Condition::FirstUseEver)
            .collapsible(true)
            .movable(true)
            .build(|| {
                ui.set_window_font_scale(1.2);

                if ui.is_window_focused() && ui.is_key_down(imgui::Key::Escape) {
                    return false;
                }

                self.framebuffer.bind_buffer();

                let cursor_pos = ui.cursor_screen_pos();
                let window_size = ui.window_size();

                // TODO: colors are a little off in imgui rendering (like a grayed out version)
                ui.get_window_draw_list()
                    .add_image(
                        TextureId::new(self.framebuffer.get_texture_id() as _),
                        [cursor_pos[0], cursor_pos[1]],
                        [
                            cursor_pos[0] + window_size[0] - 15.0,
                            cursor_pos[1] + window_size[1] - 40.0,
                        ],
                    )
                    .build();

                true
            })
            .unwrap_or(true);
    }

    fn is_opened(&self) -> bool {
        self.opened
    }

    fn set_opened(&mut self, opened: bool) {
        self.opened = opened;
    }

    fn get_name(&self) -> &'static str {
        "BG Map"
    }
}
