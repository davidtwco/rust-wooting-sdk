use std::thread::sleep;
use std::time::Duration;

use wooting_sdk::{rgb, Key};

fn main() {
    println!(
        "Keyboard connected? {}",
        rgb::is_wooting_keyboard_connected()
    );
    let mut keyboard = rgb::RgbKeyboard::default();

    keyboard.direct_set_key(Key::Q, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    keyboard.direct_reset_key(Key::Q);
    sleep(Duration::from_millis(1000));

    keyboard.direct_set_key(Key::W, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    keyboard.direct_reset_key(Key::W);
    sleep(Duration::from_millis(1000));

    keyboard.direct_set_key(Key::E, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    keyboard.direct_reset_key(Key::E);
    sleep(Duration::from_millis(1000));

    keyboard.direct_set_key(Key::R, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    keyboard.direct_reset_key(Key::R);
    sleep(Duration::from_millis(1000));

    keyboard.direct_set_key(Key::T, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    keyboard.direct_reset_key(Key::T);
    sleep(Duration::from_millis(1000));

    keyboard.direct_set_key(Key::Y, 255, 255, 255);
    sleep(Duration::from_millis(1000));
    keyboard.direct_reset_key(Key::Y);
    sleep(Duration::from_millis(1000));

    println!("Finished!");
}
