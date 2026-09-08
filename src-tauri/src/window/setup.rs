use tauri::WindowBuilder;

pub fn install(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = WindowBuilder::new(app, "main")
        .title("Photon")
        .inner_size(1000.0, 700.0)
        .build()?;
    eprintln!("photon: window created");
    crate::browser::start(&window, app.handle().clone())?;
    Ok(())
}
