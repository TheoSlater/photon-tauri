mod compatibility;

pub use compatibility::apply_pre_init_compatibility;

use gtk::prelude::*;
use std::{cell::Cell, rc::Rc};
use wry::{
    dpi::LogicalPosition, dpi::LogicalSize, Rect, WebViewBuilder, WebViewBuilderExtUnix,
    WebViewExtUnix,
};

use super::{BrowserHost, PlatformPage};
use crate::browser::overlays::{OverlayInteractionMode, OverlayRegion};

pub fn new_host(window: &tauri::WebviewWindow) -> Result<BrowserHost, wry::Error> {
    eprintln!("photon: backend = webkitgtk, render_mode = composited");
    // Keep native page input inside the React-provided viewport.
    let root = gtk::Fixed::new();
    let page_origin = Rc::new(Cell::new((0.0, 0.0)));
    let input_viewport = Rc::new(Cell::new(Rect::default()));
    let overlay_regions = Rc::new(std::cell::RefCell::new(Vec::new()));
    let pointer_position = Rc::new(Cell::new(None));
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
    let overlay = gtk::Overlay::new();
    overlay.add(&content);
    overlay.add_overlay(&root);
    overlay.add_overlay(&frontend);
    overlay.set_overlay_pass_through(&frontend, false);
    overlay.add_events(
        gtk::gdk::EventMask::POINTER_MOTION_MASK | gtk::gdk::EventMask::LEAVE_NOTIFY_MASK,
    );
    frontend.add_events(gtk::gdk::EventMask::POINTER_MOTION_MASK);
    {
        let frontend = frontend.clone();
        let input_viewport = input_viewport.clone();
        let overlay_regions = overlay_regions.clone();
        let pointer_position = pointer_position.clone();
        overlay.connect_motion_notify_event(move |overlay, event| {
            let (x, y) = event.position();
            pointer_position.set(Some((x, y)));
            route_pointer(
                overlay,
                &frontend,
                input_viewport.get(),
                &overlay_regions.borrow(),
                x,
                y,
            );
            gtk::glib::Propagation::Proceed
        });
    }
    {
        let frontend_for_handler = frontend.clone();
        let overlay = overlay.clone();
        let input_viewport = input_viewport.clone();
        let overlay_regions = overlay_regions.clone();
        let pointer_position = pointer_position.clone();
        frontend.connect_motion_notify_event(move |_, event| {
            let (x, y) = event.position();
            pointer_position.set(Some((x, y)));
            route_pointer(
                &overlay,
                &frontend_for_handler,
                input_viewport.get(),
                &overlay_regions.borrow(),
                x,
                y,
            );
            gtk::glib::Propagation::Proceed
        });
    }
    {
        let frontend = frontend.clone();
        let pointer_position = pointer_position.clone();
        overlay.connect_leave_notify_event(move |overlay, _| {
            pointer_position.set(None);
            overlay.set_overlay_pass_through(&frontend, false);
            gtk::glib::Propagation::Proceed
        });
    }
    root.set_halign(gtk::Align::Start);
    root.set_valign(gtk::Align::Start);
    root.show();
    overlay.show_all();
    vbox.pack_start(&overlay, true, true, 0);
    Ok(BrowserHost {
        root,
        page_origin,
        frontend,
        overlay,
        input_viewport,
        overlay_regions,
        pointer_position,
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
    let page = webview.webview();
    page.add_events(gtk::gdk::EventMask::POINTER_MOTION_MASK);
    {
        let overlay = host.overlay.clone();
        let frontend = host.frontend.clone();
        let input_viewport = host.input_viewport.clone();
        let overlay_regions = host.overlay_regions.clone();
        let pointer_position = host.pointer_position.clone();
        let page_origin = host.page_origin.clone();
        page.connect_motion_notify_event(move |_, event| {
            let (x, y) = event.position();
            let (origin_x, origin_y) = page_origin.get();
            let x = x + origin_x;
            let y = y + origin_y;
            pointer_position.set(Some((x, y)));
            route_pointer(
                &overlay,
                &frontend,
                input_viewport.get(),
                &overlay_regions.borrow(),
                x,
                y,
            );
            gtk::glib::Propagation::Proceed
        });
    }
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

pub fn set_overlay_regions(host: &BrowserHost, viewport: Rect, regions: &[OverlayRegion]) {
    host.input_viewport.set(viewport);
    *host.overlay_regions.borrow_mut() = regions.to_vec();
    if regions
        .iter()
        .any(|region| region.interaction == OverlayInteractionMode::Modal)
    {
        host.frontend.grab_focus();
    }
    if let Some((x, y)) = host.pointer_position.get() {
        route_pointer(
            &host.overlay,
            &host.frontend,
            viewport,
            &host.overlay_regions.borrow(),
            x,
            y,
        );
    } else {
        host.overlay.set_overlay_pass_through(&host.frontend, false);
    }
}

fn route_pointer(
    overlay: &gtk::Overlay,
    frontend: &gtk::Widget,
    viewport: Rect,
    regions: &[OverlayRegion],
    x: f64,
    y: f64,
) {
    let overlay_hit = regions
        .iter()
        .filter(|region| region.bounds.contains(x, y))
        .max_by_key(|region| region.order)
        .is_some();
    let outside_policy = if regions
        .iter()
        .filter(|region| region.bounds.has_area())
        .any(|region| region.interaction == OverlayInteractionMode::Modal)
    {
        Some(OverlayInteractionMode::Modal)
    } else if regions
        .iter()
        .filter(|region| region.bounds.has_area())
        .any(|region| region.interaction == OverlayInteractionMode::Dismiss)
    {
        Some(OverlayInteractionMode::Dismiss)
    } else {
        None
    };
    let frontend_receives = !viewport_contains(viewport, x, y)
        || overlay_hit
        || matches!(
            outside_policy,
            Some(OverlayInteractionMode::Dismiss | OverlayInteractionMode::Modal)
        );
    eprintln!(
        "photon: route pointer ({x:.0},{y:.0}) overlay_hit={overlay_hit} policy={outside_policy:?} frontend_receives={frontend_receives}"
    );
    overlay.set_overlay_pass_through(frontend, !frontend_receives);
}

fn viewport_contains(bounds: Rect, x: f64, y: f64) -> bool {
    let position = bounds.position.to_logical::<f64>(1.0);
    let size = bounds.size.to_logical::<f64>(1.0);
    size.width > 0.0
        && size.height > 0.0
        && x >= position.x
        && y >= position.y
        && x < position.x + size.width
        && y < position.y + size.height
}
