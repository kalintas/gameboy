pub mod debugger;
pub mod io_map;
pub mod keyboard_map;
pub mod memory;
pub mod registers;

use imgui::Ui;

use crate::emulator::Emulator;

pub trait Panel {
    fn update(&mut self, emulator: &Emulator, changes: &[(usize, u8)]);

    fn render(&mut self, ui: &Ui, emulator: &mut Emulator, width: f32, height: f32);

    fn is_opened(&self) -> bool {
        unimplemented!();
    }
    fn set_opened(&mut self, _: bool) {
        unimplemented!();
    }
    fn get_name(&self) -> &'static str {
        unimplemented!();
    }
}

pub struct GoToLinePopup {
    name: &'static str,
    is_open: bool,
    is_initial_run: bool,
    line: u32,
}

impl GoToLinePopup {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            is_open: false,
            is_initial_run: true,
            line: 0,
        }
    }

    pub fn render(
        &mut self,
        ui: &imgui::Ui,
        scroll_to_line: impl FnOnce(f32) -> u32,
        line_to_scroll: impl FnOnce(u32) -> f32,
    ) {
        if ui.is_window_focused()
            && ui.is_key_down(imgui::Key::LeftCtrl)
            && ui.is_key_down(imgui::Key::G)
        {
            if self.is_open {
                return;
            }

            ui.open_popup(self.name);
            self.is_open = true;
            self.is_initial_run = true;
            self.line = scroll_to_line(ui.scroll_y() / ui.scroll_max_y());
        }

        if let Some(line) = ui
            .modal_popup_config(self.name)
            .movable(false)
            .resizable(false)
            .build(|| {
                if self.is_initial_run {
                    ui.set_keyboard_focus_here();
                    self.is_initial_run = false;
                }

                ui.set_next_item_width(100.0);
                ui.input_scalar("Line", &mut self.line)
                    .chars_hexadecimal(true)
                    .chars_uppercase(true)
                    .display_format("%02x")
                    .build();

                if ui.button("Go") || ui.is_key_down(imgui::Key::Enter) {
                    ui.close_current_popup();
                    self.is_open = false;

                    return Some(self.line);
                }
                ui.same_line();
                if ui.button("Close") {
                    ui.close_current_popup();
                    self.is_open = false;
                }

                None
            })
        {
            if let Some(line) = line {
                let scroll_y = line_to_scroll(line) * ui.scroll_max_y();
                ui.set_scroll_y(scroll_y);
            }
        }
    }
}
