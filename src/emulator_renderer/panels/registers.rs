use crate::emulator::Emulator;

use super::Panel;

pub struct RegistersPanel {}

impl RegistersPanel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Panel for RegistersPanel {
    fn update(&mut self, emulator: &Emulator, changes: &[(usize, u8)]) {}

    fn render(&mut self, ui: &imgui::Ui, emulator: &Emulator, width: f32, height: f32) {
        ui.window("Registers")
            .resizable(false)
            .collapsible(false)
            .movable(false)
            .position(
                [width * 2.0 / 3.0, height * 0.5 + 19.0],
                imgui::Condition::Always,
            )
            .size([width / 3.0, height * 0.5], imgui::Condition::Always)
            .build(|| {
                ui.set_window_font_scale(1.2);

                ui.text(format!("pc: {:04x}", emulator.cpu.pc));
                ui.text(format!("sp: {:04x}", emulator.cpu.sp));
                ui.text(format!("af: {:04x}", emulator.cpu.registers.af()));
                ui.text(format!("bc: {:04x}", emulator.cpu.registers.bc()));
                ui.text(format!("de: {:04x}", emulator.cpu.registers.de()));
                ui.text(format!("hl: {:04x}", emulator.cpu.registers.hl()));
            });
    }
}
