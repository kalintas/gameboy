#![feature(const_mut_refs)]

mod emulator;
mod renderer;

fn main() {
    emulator::Emulator::new().run();
}
