#![feature(const_mut_refs)]

mod emulator;
mod emulator_renderer;
mod renderer;

fn main() {
    let mut emulator = emulator::Emulator::after_boot();

    emulator.load_cartidge("./roms/tetris.gb");

    emulator_renderer::EmulatorRenderer::new().render(&mut emulator);
}
