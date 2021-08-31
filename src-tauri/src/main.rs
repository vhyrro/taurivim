#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod commands;

// use mlua::prelude::*;

/*
* TODO: Get logging library
* Load init.lua manually whilst overriding vim.gui
* Find way to do two-way API calls rather than just TypeScript => rust
* Register a Neovim UI
*/

fn main() {
    use neovim_lib::Session;

    // TODO: Fix this ending the process prematurely
    // let _ = Command::new("nvim").arg("--headless").arg("--listen").arg("127.0.0.1:5555").spawn().expect("Unable to spawn Neovim process");

    let mut session = Session::new_tcp("127.0.0.1:5555").expect("Unable to connect to TCP port 5555 :(");

    session.start_event_loop();

	tauri::Builder::default()
	    .invoke_handler(tauri::generate_handler![commands::register_ui])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
