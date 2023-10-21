#![feature(const_mut_refs)]

mod emulator;
mod emulator_renderer;
mod renderer;

fn main() {
    // let mut emulator = emulator::Emulator::new("./roms/boot/dmg_boot.gb");
    let mut emulator = emulator::Emulator::after_boot();

    emulator.load_cartidge("./roms/zelda.gb");

    emulator_renderer::EmulatorRenderer::new().render(&mut emulator);
}
