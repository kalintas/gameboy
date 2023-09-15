pub mod debugger;
pub mod memory;
pub mod registers;

use imgui::Ui;

use crate::emulator::Emulator;

pub trait Panel {
    fn update(&mut self, emulator: &Emulator, changes: &[(usize, u8)]);

    fn render(&mut self, ui: &Ui, emulator: &mut Emulator, width: f32, height: f32);
}
