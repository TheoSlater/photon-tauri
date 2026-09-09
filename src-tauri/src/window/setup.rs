use tauri::{WebviewUrl, WebviewWindowBuilder};

pub fn install(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .title("Photon")
        .decorations(false)
        .transparent(true)
        .inner_size(1000.0, 700.0)
        .theme(None)
        .build()?;
    eprintln!("photon: window created");
    crate::browser::start(&window, app.handle().clone())?;
    Ok(())
}
