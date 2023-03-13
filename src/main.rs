#![no_std]
#![no_main]

use crossbeam::atomic::AtomicCell;
use pc_keyboard::DecodedKey;
use pluggable_interrupt_os::vga_buffer::clear_screen;
use pluggable_interrupt_os::HandlerTable;
use snake_game_rust::{GameState, BLOCK_SIZE, COLS, ROWS};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn _start() -> ! {
    static LAST_KEY: AtomicCell<Option<DecodedKey>> = AtomicCell::new(None);
    const SCREEN_SIZE: usize = ROWS * COLS * BLOCK_SIZE * BLOCK_SIZE * 4;

    let mut kernel = GameState::new();

    // Spawn the initial food
    kernel.spawn_food();

    // Draw game screen
    let buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, SCREEN_SIZE) };
    for byte in buffer.iter_mut() {
        *byte = 0x00;
    }
    loop {
        if let Some(key) = LAST_KEY.load() {
            LAST_KEY.store(None);
            kernel.key(key);
        }

        // Clear the screen buffer
        // buffer.iter_mut().for_each(|b| *b = 0);

        // Draw the food
        let (food_row, food_col) = kernel.food;
        let food_start_idx =
            ((food_row as usize) * COLS + (food_col as usize)) * BLOCK_SIZE * BLOCK_SIZE * 4;
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                let idx = food_start_idx + (i * COLS + j) * 4;
                buffer[idx..idx + 4].copy_from_slice(&[0xff, 0x00, 0x00, 0xff]);
            }
        }

        kernel.draw_snake(buffer);

        // ... keyboard input handling ...
    }
}

fn startup() {
    clear_screen();
}

/*
fn render_buffer(buffer: &[u8]) {
    let width = 80;
    let height = 25;

    let vga_buffer = 0xb8000 as *mut u8;

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let char_code = buffer[index];
            let color = buffer[index + 1];

            // We need to convert the color from our 256-color palette to a VGA color, which uses 4 bits for the background color and 4 bits for the foreground color
            let vga_color = ((color >> 4) << 4) | (color & 0x0f);

            // Write the character and color to the VGA buffer
            unsafe {
                *vga_buffer.offset(index as isize * 2) = char_code;
                *vga_buffer.offset(index as isize * 2 + 1) = vga_color;
            }
        }
    }
}
*/
