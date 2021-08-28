#[tauri::command]
pub async fn register_ui(window: tauri::Window)
{
    let size = match window.inner_size() {
        Ok(window) => window,
        Err(err) => {
            eprintln!("Failed to obtain the current Tauri window's size: {}", err);
            return;
        },
    };

    println!("The current window is of size: {}x{}", size.width, size.height);

    // TODO: Register Neovim UI here
    // let mut nvim = Neovim::new();
}
