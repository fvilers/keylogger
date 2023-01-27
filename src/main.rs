use std::{thread, time};

#[cfg(windows)]
extern crate winapi;

fn is_pressed(virtual_key: &i32) -> bool {
    use winapi::um::winuser::GetAsyncKeyState;

    let key_state = unsafe { GetAsyncKeyState(*virtual_key) };
    let key_state = key_state as u32 >> 31;

    key_state == 1
}

#[cfg(windows)]
fn listen_for_keys(excluded_keys: &[i32], on_key_pressed: fn(i32)) {
    loop {
        (0..255)
            .into_iter()
            .filter(|key| !excluded_keys.contains(key))
            .filter(is_pressed)
            .for_each(on_key_pressed);

        thread::sleep(time::Duration::from_millis(100));
    }
}

#[cfg(not(windows))]
fn listen_for_keys() {
    panic!("OS not supported");
}

fn main() {
    // Exclude mouse buttons
    // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    let excluded_keys = [0x01, 0x02, 0x04, 0x05, 0x06];

    listen_for_keys(&excluded_keys, |key| println!("{key} pressed"));
}
