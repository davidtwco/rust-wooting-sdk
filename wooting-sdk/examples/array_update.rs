use std::thread::sleep;
use std::time::Duration;

use wooting_sdk::{rgb, Key};

fn main() {
    println!(
        "Keyboard connected? {}",
        rgb::is_wooting_keyboard_connected()
    );
    let mut keyboard = rgb::RgbKeyboard::default();
    keyboard.array_set_single(Key::Q, 255, 255, 255);
    keyboard.array_set_single(Key::W, 255, 255, 255);
    keyboard.array_set_single(Key::E, 255, 255, 255);
    keyboard.array_set_single(Key::R, 255, 255, 255);
    keyboard.array_set_single(Key::T, 255, 255, 255);
    keyboard.array_set_single(Key::Y, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    println!("Updating... {}", keyboard.array_update());
    sleep(Duration::from_millis(1000));
    println!("Finished!");
}
