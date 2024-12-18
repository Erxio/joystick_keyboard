use tauri::{AppHandle, Emitter};
use std::{thread, time::Duration};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn hover_left(app: AppHandle, index: u16) {
    println!("{}", index);
    app.emit_to("sub", "custom-event", index).unwrap();

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            
            let handler_clone_left = handle.clone();

            let _left_pointer = thread::spawn(move || {
                let mut x: f64 = 0.0;
                let mut y: f64 = 0.0;

                loop {
                    thread::sleep(Duration::from_millis(1000/60));
                    y = f64::sin(x);
                    x = (x + 0.01) % 1.0;
                    println!("[Left Pointer] x: {} y: {}", x*500.0, y*500.0);
                    handler_clone_left.emit("left_pointer_x",x).unwrap();
                    handler_clone_left.emit("left_pointer_y",y).unwrap();
                }

            });
            let handler_clone_right = handle.clone();

            let _right_pointer = thread::spawn(move ||  {
                let mut x: f64 = 0.0;
                let mut y: f64 = 0.0;

                loop {
                    thread::sleep(Duration::from_millis(1000/60));
                    y = f64::cos(x);
                    x = (x + 0.01) % 1.0;
                    println!("[Right Pointer] x: {} y: {}", x*500.0, y*500.0);
                    handler_clone_right.emit("right_pointer_x",x).unwrap();
                    handler_clone_right.emit("right_pointer_y",y).unwrap();
                }

            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![hover_left])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


    
}
