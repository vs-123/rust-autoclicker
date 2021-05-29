use enigo::{Enigo, MouseButton, MouseControllable};
use inputbot::{KeybdKey::*, *};
use std::{sync::atomic::{AtomicBool, Ordering}, thread, time, sync::Arc};

fn main() {
    println!("Auto Clicker");
    let clicker_is_active = Arc::new(AtomicBool::new(false));
    // Provides shared ownership of a value of AtomicBool, a boolean type which can be shared safely between threads.
    let clicker_is_active_clone = Arc::clone(&clicker_is_active);
    // Cloning clicker_is_active for the thread which will click on the screen
    thread::spawn(move || { // Spawning a thread that will check if clicker_is_active is true, if so then click on the screen
        let mut enigo = Enigo::new();
        loop {
            if clicker_is_active_clone.load(Ordering::Relaxed) {
                enigo.mouse_click(MouseButton::Left);
                thread::sleep(time::Duration::from_millis(1));
            }
        }
    });

    F6Key.bind(move || {
        // Inverting the clicker_is_active for each press of the F6 key (F6 is our hotkey)
        clicker_is_active.store(!clicker_is_active.load(Ordering::Relaxed), Ordering::Relaxed); 
    });

    handle_input_events(); // Start listening for keyboard events
}
