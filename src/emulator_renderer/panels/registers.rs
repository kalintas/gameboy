use crate::emulator::{Emulator, memory_map};

use super::{Panel, memory};

pub struct RegistersPanel {}

impl RegistersPanel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Panel for RegistersPanel {
    fn update(&mut self, emulator: &Emulator, changes: &[(usize, u8)]) {}

    fn render(&mut self, ui: &imgui::Ui, emulator: &mut Emulator, width: f32, height: f32) {
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
                
                // Cpu registers
                let mut z = emulator.cpu.registers.get_z() == 1;
                let mut h = emulator.cpu.registers.get_h() == 1;
                let mut n = emulator.cpu.registers.get_n() == 1;
                let mut cy = emulator.cpu.registers.get_cy() == 1;

                ui.text(format!("af: {:04x}", emulator.cpu.registers.af()));
                ui.same_line(); ui.checkbox("Z", &mut z);
                ui.text(format!("bc: {:04x}", emulator.cpu.registers.bc()));
                ui.same_line(); ui.checkbox("H", &mut h);
                ui.text(format!("de: {:04x}", emulator.cpu.registers.de()));
                ui.same_line(); ui.checkbox("N", &mut n);
                ui.text(format!("hl: {:04x}", emulator.cpu.registers.hl()));
                ui.same_line(); ui.checkbox("CY", &mut cy);
                ui.text(format!("sp: {:04x}", emulator.cpu.sp));
                ui.text(format!("pc: {:04x}", emulator.cpu.pc));

                emulator.cpu.registers.set_flags(z as u8, n as u8, h as u8, cy as u8);
                
                ui.separator();
                
                // Interrupt registers

                let old_ie = emulator.memory_map.get_io(memory_map::Io::IE);
                let old_if = emulator.memory_map.get_io(memory_map::Io::IF);

                let mut ie_values = [(old_ie & 0x1) == 1, ((old_ie >> 1) & 0x1) == 1, ((old_ie >> 2) & 0x1) == 1, ((old_ie >> 3) & 0x1) == 1, ((old_ie >> 4) & 0x1) == 1];
                let mut if_values = [(old_if & 0x1) == 1, ((old_if >> 1) & 0x1) == 1, ((old_if >> 2) & 0x1) == 1, ((old_if >> 3) & 0x1) == 1, ((old_if >> 4) & 0x1) == 1];


                ui.text("        IE  IF");
                ui.text("V-Blank");
                ui.same_line(); ui.checkbox("###0", &mut ie_values[0]);
                ui.same_line(); ui.checkbox("###1", &mut if_values[0]);

                ui.same_line();
                ui.text("IME");
                ui.same_line();
                ui.checkbox("###IME", &mut emulator.cpu.ime);

                ui.text("LCD    ");
                ui.same_line(); ui.checkbox("###2", &mut ie_values[1]);
                ui.same_line(); ui.checkbox("###3", &mut if_values[1]);
                ui.text("Timer  "); 
                ui.same_line(); ui.checkbox("###4", &mut ie_values[2]);
                ui.same_line(); ui.checkbox("###5", &mut if_values[2]);
                ui.text("Serial ");
                ui.same_line(); ui.checkbox("###6", &mut ie_values[3]);
                ui.same_line(); ui.checkbox("###7", &mut if_values[3]);
                ui.text("Joypad ");
                ui.same_line(); ui.checkbox("###8", &mut ie_values[4]);
                ui.same_line(); ui.checkbox("###9", &mut if_values[4]);
                
                let new_ie = (ie_values[0] as u8) | ((ie_values[1] as u8) << 1) | ((ie_values[2] as u8) << 2) | ((ie_values[3] as u8) << 3) | ((ie_values[4] as u8) << 4);
                let new_if = (if_values[0] as u8) | ((if_values[1] as u8) << 1) | ((if_values[2] as u8) << 2) | ((if_values[3] as u8) << 3) | ((if_values[4] as u8) << 4);
                
                if old_ie != new_ie { emulator.memory_map.set_io(memory_map::Io::IE, new_ie); }
                if old_if != new_if { emulator.memory_map.set_io(memory_map::Io::IF, new_if); }

                ui.separator();
                // LCD Control Registers

                // let mut lcdc = emulator.memory_map.get(0xFF40) as i32;

                // ui.text("LCDC");
                // ui.input_int("###11", &mut lcdc).step(0).chars_hexadecimal(true).no_horizontal_scroll(true).build();

            });
    }
}
