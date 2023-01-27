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
fn listen_for_keys(on_key_pressed: fn(i32)) {
    loop {
        // TODO: exclude mouse buttons
        // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
        (0..255)
            .into_iter()
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
    listen_for_keys(|key| println!("{key} pressed"));
}
