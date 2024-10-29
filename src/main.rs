use std::{io};
use std::io::Write;
use base64::{encode, decode};
use clipboard::{ClipboardContext, ClipboardProvider};
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{SetWindowPos, SWP_NOMOVE};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{SWP_SHOWWINDOW, SW_SHOWNORMAL, ShowWindow};

fn set_window_size(width: i32, height: i32) {
    unsafe {
        let hwnd: HWND = GetConsoleWindow();
        if hwnd.is_null() {
            return;
        }
        ShowWindow(hwnd, SW_SHOWNORMAL);
        SetWindowPos(hwnd, 0 as HWND, 0, 0, width, height, SWP_NOMOVE | SWP_SHOWWINDOW);
    }
}

fn title() {
    print!(
        r#"
███████ ██ ███    ███ ██████  ██      ███████     ██████   █████  ███████ ███████  ██████  ██   ██     ██████  ███████  ██████  ██████  ██████  ███████ ██████  
██      ██ ████  ████ ██   ██ ██      ██          ██   ██ ██   ██ ██      ██      ██       ██   ██     ██   ██ ██      ██      ██    ██ ██   ██ ██      ██   ██ 
███████ ██ ██ ████ ██ ██████  ██      █████       ██████  ███████ ███████ █████   ███████  ███████     ██   ██ █████   ██      ██    ██ ██   ██ █████   ██████  
     ██ ██ ██  ██  ██ ██      ██      ██          ██   ██ ██   ██      ██ ██      ██    ██      ██     ██   ██ ██      ██      ██    ██ ██   ██ ██      ██   ██ 
███████ ██ ██      ██ ██      ███████ ███████     ██████  ██   ██ ███████ ███████  ██████       ██     ██████  ███████  ██████  ██████  ██████  ███████ ██   ██                                                                                                                                                                                                                                                                                                                           
"#
    );
}

fn menu() {

    println!("1. Encode");
    println!("2. Decode");
    println!("3. Quit");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input: String  = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "1" {
        encoded();
    } else if input.trim() == "2" {
        decoded();
    } else if input.trim() == "3"{
        quit();
    } else {
        println!("Invalid choice");
        
    }
}

fn quit() {
    std::process::exit(0);
}

fn encoded() {

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    println!();
    println!("===================================");
    println!("Encoding Selected"); 

    print!("Enter your text to encode: ");
    io::stdout().flush().unwrap();

    let mut base64encode: String = String::new();
    io::stdin().read_line(&mut base64encode)
        .expect("Failed to read line");

    let encoded = encode(base64encode);
    clipboard.set_contents(encoded.clone()).unwrap();
    println!("Encoded text: {}", encoded);
    println!();
    println!("(Automatically copied to clipboard)");
}

fn decoded() {

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    println!();
    println!("===================================");
    println!("Decoding Selected"); 

    print!("Enter your text to decode: ");
    io::stdout().flush().unwrap();  

    let mut base64decode: String = String::new();
    io::stdin().read_line(&mut base64decode)
        .expect("Failed to read line");


    match decode(base64decode.trim()) {
        Ok(decoded_bytes) => {
            match String::from_utf8(decoded_bytes) {
                Ok(decoded_string) => {
                    clipboard.set_contents(decoded_string.clone()).unwrap();
                    println!("Decoded: {}\n(Automatically copied to clipboard)", decoded_string);
                }
                Err(e) => println!("Error decoding to string: {}", e),
            }
        }
        Err(e) => println!("Error decoding Base64: {}", e),
    }
}

fn main() {
    set_window_size(1500, 600);

    title();

    loop {
        menu();
    }
}
