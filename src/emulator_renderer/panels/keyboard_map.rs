use crate::emulator::JoypadKeys;
use imgui::Key;
use sdl2::keyboard::Scancode;

use super::Panel;

const DEFAULT_MAP: [(JoypadKeys, Scancode); 8] = [
    (JoypadKeys::BUTTONA, Scancode::Z),
    (JoypadKeys::BUTTONB, Scancode::X),
    (JoypadKeys::START, Scancode::Return),
    (JoypadKeys::SELECT, Scancode::Backspace),
    (JoypadKeys::LEFT, Scancode::A),
    (JoypadKeys::RIGHT, Scancode::D),
    (JoypadKeys::DOWN, Scancode::S),
    (JoypadKeys::UP, Scancode::W),
];

pub struct KeyboardMap {
    keyboard_map: [(JoypadKeys, Scancode); 8],
    opened: bool,
    pressed_keys: Vec<Scancode>,
    changing_key_index: Option<usize>,
}

impl KeyboardMap {

    pub fn new() -> Self {
        Self {
            keyboard_map: DEFAULT_MAP,
            opened: false,
            pressed_keys: Vec::new(),
            changing_key_index: None,
        }
    }

    pub fn update_keys(&mut self, pressed_keys: Vec<Scancode>) -> JoypadKeys {
        self.pressed_keys = pressed_keys;

        if let Some(index) = self.changing_key_index {
            if let Some(scancode) = self.pressed_keys.first() {
                
                if *scancode != Scancode::Escape {
                    self.keyboard_map[index].1 = *scancode;
                }
                self.changing_key_index = None;
            }
        }

        let mut joypad_keys: JoypadKeys = JoypadKeys::NONE;

        for (joypad_key, keyboard_key) in self.keyboard_map.iter() {

            if self.pressed_keys.contains(keyboard_key) {
                joypad_keys = *joypad_key | joypad_keys;
            }
        }

        joypad_keys
    }
}

impl Panel for KeyboardMap {

    fn update(&mut self, _: &crate::emulator::Emulator, _: &[(usize, u8)]) {}

    fn render(&mut self, ui: &imgui::Ui, _: &mut crate::emulator::Emulator, _: f32, _: f32) {

        if !self.opened {
            return;
        }

        self.opened = ui.window(self.get_name())
        .opened(&mut self.opened)
        .resizable(true)
        .collapsible(true)
        .movable(true)
        .size([300.0, 260.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.set_window_font_scale(1.2);

            if ui.is_window_focused() && ui.is_key_down(Key::Escape) {
                return false;
            }

            if ui.button("Reset layout") {
                self.keyboard_map = DEFAULT_MAP;
            }
            
            for (index, (joypad_key, keyboard_key)) in self.keyboard_map.iter().enumerate() {

                let color = if self.pressed_keys.contains(keyboard_key) {
                    Some(ui.push_style_color(imgui::StyleColor::Text, [0.0, 1.0, 0.0, 1.0]))
                } else {
                    None
                };

                ui.text(format!("{:<8} =>", joypad_key.as_ref()));
                
                if let Some(color) = color {
                    color.pop();
                }
                
                ui.same_line(); 
                if ui.button(format!("{:?}###{}", keyboard_key, index)) {
                    self.changing_key_index = Some(index);
                }
                if let Some(changing_index) = self.changing_key_index {
                    if changing_index == index {
                        ui.text("Waiting key, press ESC to cancel");
                    }
                }
            }
            true
        }).unwrap_or(true); 
    }

    fn is_opened(&self) -> bool {
        self.opened
    }

    fn set_opened(&mut self, opened: bool) {
        self.opened = opened;
    }

    fn get_name(&self) -> &'static str {
        "Keyboard Map"
    }
}


