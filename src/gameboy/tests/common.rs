
use std::{path::PathBuf, time::Duration};
use gameboy::Gameboy;
use image::EncodableLayout;

fn run_test_rom(mut path: PathBuf) {

    let mut emulator = Gameboy::after_boot(); 

    emulator.load_cartidge(&path);

    emulator.ppu.color_shades = [
        0xFFFFFFFF,
        0xFFAAAAAA,
        0xFF555555,
        0xFF000000,
    ];

    let mut finished = false;
    let mut old_pc = 0;

    emulator.debug_cycle(Duration::from_secs(30), |emulator| {

        if old_pc == emulator.cpu.pc {
            finished = true;
        } else {
            old_pc = emulator.cpu.pc;
        }

        finished
    });

    emulator.cycle(Duration::from_secs(1));

    assert!(finished, "Could not finished the rom in 30 seconds.");

    // Compare results.
    path.set_extension("png"); // Turn it to image path.
    let success_image = image::open(&path).unwrap().into_rgba8();
    
    let screen_buffer = unsafe {
        
        let length = emulator.ppu.screen_buffer.len() * std::mem::size_of::<u32>();
        
        std::slice::from_raw_parts(Box::into_raw(emulator.ppu.screen_buffer) as *const u8, length)
    };
    
    assert!(success_image.as_bytes() == screen_buffer, "Test case {} failed.", &path.file_stem().unwrap().to_str().unwrap());
}

pub fn execute_tests(test_dir_path: &'static str) {

    let mut path = std::path::PathBuf::from(test_dir_path);

    if path.exists() {
        // Folder
        let directory = std::fs::read_dir(path).unwrap();
        
        for dir_entry in directory {

            let path = dir_entry.unwrap().path();

            if let Some(extension) = path.extension() {
                if extension.to_str() == Some("gb") {
                    run_test_rom(path);
                }
            }
        }
    } else {
        // File
        path.set_extension("gb");
        run_test_rom(path);
    }

}

#[macro_export]
macro_rules! create_tests {
    ($parent: ident, $($name: ident),+) => {
        #[cfg(test)]
        mod $parent {
            $(
                #[test]
                fn $name() {
                    crate::common::execute_tests(concat!("../../roms/test/", stringify!($parent), "/", stringify!($name)));
                }
            )+
        }
    };
}
