pub mod bg_map;
pub mod debugger;
pub mod io_map;
pub mod keyboard_map;
pub mod memory;
pub mod registers;

use std::collections::HashMap;

use imgui::{internal::DataTypeKind, Ui};

use crate::emulator::Emulator;

use self::{
    bg_map::BgMapPanel, debugger::DebuggerPanel, io_map::IoMapPanel,
    keyboard_map::KeyboardMapPanel, memory::MemoryPanel, registers::RegistersPanel,
};

pub trait Panel {
    fn update(&mut self, emulator: &Emulator, changes: &HashMap<u16, u8>);

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

pub struct Panels {
    pub debugger: DebuggerPanel,
    pub memory: MemoryPanel,
    pub registers: RegistersPanel,

    pub keyboard_map: KeyboardMapPanel,
    pub io_map: IoMapPanel,
    pub bg_map: BgMapPanel,
}

impl Panels {
    pub fn new() -> Self {
        Self {
            debugger: DebuggerPanel::new(),
            memory: MemoryPanel::new(),
            registers: RegistersPanel::new(),

            keyboard_map: KeyboardMapPanel::new(),
            io_map: IoMapPanel::new(),
            bg_map: BgMapPanel::new(),
        }
    }
}

macro_rules! call_small_panels {
    ($panels:expr, $function:ident, $($arguments:expr),*) => {
        $panels.keyboard_map.$function($($arguments,)*);
        $panels.io_map.$function($($arguments,)*);
        $panels.bg_map.$function($($arguments,)*);
    };
}

macro_rules! call_all_panels {
    ($panels:expr, $function:ident, $($arguments:expr),*) => {
        $panels.debugger.$function($($arguments,)*);
        $panels.memory.$function($($arguments,)*);
        $panels.registers.$function($($arguments,)*);

        panels::call_small_panels!($panels, $function, $($arguments), +);
    };
}

pub(crate) use call_all_panels;
pub(crate) use call_small_panels;

// Utility
pub fn hex_input_u8<T: DataTypeKind>(ui: &imgui::Ui, name: &str, value: &mut T) -> bool {
    ui.input_scalar(name, value)
        .chars_hexadecimal(true)
        .chars_uppercase(true)
        .display_format("%02x")
        .build()
}

pub fn hex_input_u16<T: DataTypeKind>(ui: &imgui::Ui, name: &str, value: &mut T) -> bool {
    ui.input_scalar(name, value)
        .chars_hexadecimal(true)
        .chars_uppercase(true)
        .display_format("%04x")
        .build()
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

                hex_input_u8(ui, "Line", &mut self.line);

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
