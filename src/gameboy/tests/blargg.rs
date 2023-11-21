#[macro_use]
mod common;

create_tests!(
    blargg,
    cpu_instrs,
    instr_timing,
    mem_timing,
    mem_timing_2,
    oam_bug,
    halt_bug,
    interrupt_time,
    dmg_sound
);
