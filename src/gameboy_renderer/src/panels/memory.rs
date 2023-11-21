use std::{
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use gameboy::Gameboy;

use super::{GoToLinePopup, Panel};

pub struct MemoryPanel {
    go_to_line_popup: GoToLinePopup,
}

impl MemoryPanel {
    pub fn new() -> Self {
        Self {
            go_to_line_popup: GoToLinePopup::new("Go To Line###memory"),
        }
    }

    pub fn dump_to_file(
        &self,
        path: impl AsRef<Path>,
        emulator: &Gameboy,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut file = BufWriter::new(file);

        for i in 0..0x10000usize {
            let mut line = [' ' as u8; 58];
            Self::format_line(emulator, &mut line, i as _);
            line[line.len() - 1] = '\n' as u8;
            file.write(&line)?;
        }

        Ok(())
    }

    fn format_line<'a>(emulator: &Gameboy, line: &'a mut [u8], current_row: u16) -> &'a str {
        let index = current_row * 0x10;

        super::format_in_place(line, index, 0);

        line[4] = ':' as u8;
        for i in 0..16 {
            let memory_value = emulator.memory_map.get(index + i as u16);
            super::format_in_place(line, memory_value, i * 2 + 6);

            let mut character = memory_value as char;

            if !character.is_ascii_graphic() {
                character = '.';
            }

            line[39 + i] = character as u8;
        }

        unsafe { std::str::from_utf8_unchecked(line) }
    }

    // A round function for hexadecimal integer rounding.
    // 0x105 -> 0x100, 0x108 -> 0x110
    fn round(val: u32) -> u32 {
        let floored = val & 0xFFF0;

        if val % 0x10 < 0x8 {
            floored
        } else {
            (floored) + 0x10
        }
    }
}

impl Panel for MemoryPanel {
    fn update(&mut self, _: &Gameboy) {}

    fn render(&mut self, ui: &imgui::Ui, emulator: &mut Gameboy, width: f32, height: f32) {
        ui.window("Memory")
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .bring_to_front_on_focus(false)
            .position([0.0, height * 0.5 + 19.0], imgui::Condition::Always)
            .size([width * 2.0 / 3.0, height * 0.5], imgui::Condition::Always)
            .build(|| {
                ui.set_window_font_scale(1.2);

                self.go_to_line_popup.render(
                    ui,
                    |scroll| Self::round((scroll * 0xFFF0 as f32) as u32),
                    |line| Self::round(line) as f32 / 0xFFF0 as f32,
                );

                // Use a list clipper for efficient rendering.
                // This will only render visible lines of the text.
                let clipper = imgui::ListClipper::new(0x1000)
                    .items_height(ui.current_font_size())
                    .begin(ui);

                clipper.iter().for_each(|current_row| {
                    let mut line = [' ' as u8; 57];
                    ui.text(Self::format_line(emulator, &mut line, current_row as _));
                });
            });
    }
}
