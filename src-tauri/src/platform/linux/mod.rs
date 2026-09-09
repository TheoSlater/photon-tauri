mod compatibility;

pub use compatibility::apply_pre_init_compatibility;

use gtk::prelude::*;
use std::{cell::Cell, rc::Rc};
use wry::{
    dpi::LogicalPosition, dpi::LogicalSize, Rect, WebViewBuilder, WebViewBuilderExtUnix,
    WebViewExtUnix,
};

use super::{BrowserHost, PlatformPage};

pub fn new_host(window: &tauri::WebviewWindow) -> Result<BrowserHost, wry::Error> {
    eprintln!("photon: backend = webkitgtk, render_mode = composited");
    // Keep native page input inside the React-provided viewport.
    let root = gtk::Fixed::new();
    let page_origin = Rc::new(Cell::new((0.0, 0.0)));
    let input_viewport = Rc::new(Cell::new(Rect::default()));
    let vbox = window
        .default_vbox()
        .map_err(|error| wry::Error::Io(std::io::Error::other(error.to_string())))?;
    let frontend = vbox
        .children()
        .into_iter()
        .next()
        .ok_or_else(|| wry::Error::Io(std::io::Error::other("missing frontend webview")))?;
    vbox.remove(&frontend);

    let frontend: gtk::Widget = frontend.upcast();
    let content = gtk::Fixed::new();
    root.set_halign(gtk::Align::Start);
    root.set_valign(gtk::Align::Start);
    content.show();
    root.show();
    frontend.show();
    vbox.pack_start(&content, true, true, 0);
    vbox.pack_start(&root, true, true, 0);
    vbox.pack_start(&frontend, true, true, 0);
    Ok(BrowserHost {
        root,
        page_origin,
        input_viewport,
    })
}

pub fn build_page<'a>(
    _window: &tauri::WebviewWindow,
    host: &BrowserHost,
    builder: WebViewBuilder<'a>,
    bounds: Rect,
) -> Result<PlatformPage, wry::Error> {
    let webview = builder
        .with_bounds(local_bounds(host, bounds))
        .build_gtk(&host.root)?;
    Ok(PlatformPage {
        webview,
        page_origin: host.page_origin.clone(),
    })
}

pub fn set_viewport(host: &BrowserHost, bounds: Rect) {
    host.input_viewport.set(bounds);
    let scale_factor = host.root.scale_factor() as f64;
    let position = bounds.position.to_logical::<i32>(scale_factor);
    let size = bounds.size.to_logical::<i32>(scale_factor);
    host.page_origin.set((position.x as f64, position.y as f64));
    host.root.set_margin_start(position.x);
    host.root.set_margin_top(position.y);
    host.root.set_size_request(size.width, size.height);
}

pub fn set_bounds(page: &PlatformPage, bounds: Rect) -> Result<(), wry::Error> {
    let webview = page.webview.webview();
    page.webview.set_bounds(local_bounds_from_origin(
        &page.page_origin,
        &webview,
        bounds,
    ))
}

fn local_bounds(host: &BrowserHost, bounds: Rect) -> Rect {
    local_bounds_from_origin(&host.page_origin, &host.root, bounds)
}

fn local_bounds_from_origin<W: IsA<gtk::Widget>>(
    origin: &Rc<Cell<(f64, f64)>>,
    widget: &W,
    bounds: Rect,
) -> Rect {
    let scale_factor = widget.scale_factor() as f64;
    let position = bounds.position.to_logical::<f64>(scale_factor);
    let size = bounds.size.to_logical::<f64>(scale_factor);
    let (origin_x, origin_y) = origin.get();
    Rect {
        position: LogicalPosition::new(position.x - origin_x, position.y - origin_y).into(),
        size: LogicalSize::new(size.width, size.height).into(),
    }
}
