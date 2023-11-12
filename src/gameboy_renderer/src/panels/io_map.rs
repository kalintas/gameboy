use std::collections::HashMap;

use strum::IntoEnumIterator;

use gameboy::memory_map::Io;

use super::Panel;

pub struct IoMapPanel {
    opened: bool,
}

impl IoMapPanel {
    pub fn new() -> Self {
        Self { opened: false }
    }
}

impl Panel for IoMapPanel {
    fn update(&mut self, _: &gameboy::Gameboy, _: &HashMap<u16, u8>) {}

    fn render(&mut self, ui: &imgui::Ui, emulator: &mut gameboy::Gameboy, _: f32, _: f32) {
        if !self.opened {
            return;
        }

        let mut register_slider = |io| {
            let mut register = emulator.memory_map.get_io(io);

            ui.set_next_item_width(50.0);

            let mut empty = [' ' as u8; 5];
            // Add padding spaces after the string.
            let name = unsafe {
                // SAFETY:
                // Provided io.as_ref() string is bound to be ascii string.
                // "io.as_ref().len() <= 5" is always true.

                std::ptr::copy_nonoverlapping(
                    io.as_ref().as_bytes().as_ptr(),
                    empty.as_mut_ptr(),
                    io.as_ref().len(),
                );

                std::str::from_utf8_unchecked(&empty)
            };

            if super::hex_input_u8(ui, name, &mut register) {
                emulator.memory_map.set_io(io, register);
            }
        };

        self.opened &= ui
            .window(self.get_name())
            .opened(&mut self.opened)
            .resizable(false)
            .collapsible(true)
            .movable(true)
            .build(|| {
                ui.set_window_font_scale(1.2);

                if ui.is_window_focused() && ui.is_key_down(imgui::Key::Escape) {
                    return false;
                }

                let mut i = 0;

                Io::iter().for_each(|io| {
                    if i % 5 != 0 {
                        ui.same_line();
                    }
                    register_slider(io);

                    i += 1;
                });

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
        "Io Map"
    }
}
