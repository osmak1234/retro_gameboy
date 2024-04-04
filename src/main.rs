use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub mod main_menu;

fn main() {
    let shared_state = Arc::new(Mutex::new(String::new()));

    let receive_shared_state = Arc::clone(&shared_state);
    let send_shared_state = Arc::clone(&shared_state);

    let receive_handle = thread::spawn(move || loop {
        //TODO:
        ////////////////////
        // Display render //
        ////////////////////
        let mut shared_state = receive_shared_state.lock().unwrap();

        if !shared_state.is_empty() {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .unwrap();
            writeln!(file, "{}", shared_state).unwrap();
            shared_state.clear();
        }
        drop(shared_state);
    });

    let send_handle = thread::spawn(move || loop {
        //TODO:
        ///////////////
        // Main loop //
        ///////////////

        thread::sleep(Duration::from_secs(1));

        let mut shared_state = send_shared_state.lock().unwrap();

        *shared_state = String::from("Data to display will be here");
        drop(shared_state);
    });

    // Wait for both threads to finish
    receive_handle.join().unwrap();
    send_handle.join().unwrap();
}

