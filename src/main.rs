extern crate device_query;

use device_query::{DeviceQuery, DeviceState, MouseState};
use std::fs::File;
use std::io::Write;

fn loop_for_device_state(path : &str) {
    // Create a file to write to
    let mut output = File::create(path).expect("Failed to create file");

    // Initialize device state 
    let device_state = DeviceState::new();

    // Initialize mouse and key states
    let mut prev_mouse = MouseState::default();
    let mut prev_keys = vec![];

    // Loop until sigint
    loop {
        // Get the current state of the keys
        let keys = device_state.get_keys();
        if keys != prev_keys {
            writeln!(output, "{:?}", keys).unwrap()
            
        } 
        // Update the previous key state
        prev_keys = keys;

        // Get the current state of the mouse
        let mouse = device_state.get_mouse();
        if mouse != prev_mouse {
            writeln!(output, "{:?}", mouse).unwrap()
        }
        // Update the previous mouse state
        prev_mouse = mouse;
        
    }
}


fn set_sigint_handler() {
    // Register a signal handler for keyboard interrupts
    ctrlc::set_handler(move || {
        println!("Keyboard interrupt detected, exiting gracefully...");
        std::process::exit(0);
    }).expect("Error setting signal handler");
}

fn main() {

    // Register a signal handler for keyboard interrupts
    set_sigint_handler();

    // Path to log file
    let path = "output.log";

    // Start the loop
    loop_for_device_state(path);

}
