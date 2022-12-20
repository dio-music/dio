#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        // TODO: Set real values for minimum and initial window size
        initial_window_size: Some(egui::vec2(1400., 1000.)),
        min_window_size: Some(egui::vec2(750., 250.)),
        ..Default::default()
    };
    eframe::run_native(
        "Extended Spotify Wrapped",
        native_options,
        Box::new(|cc| Box::new(extended_spotify_wrapped::ESWApp::new(cc))),
    );
}

// TODO: Implement WASM version
// // when compiling to web using trunk.
// #[cfg(target_arch = "wasm32")]
// fn main() {
//     // Make sure panics are logged using `console.error`.
//     console_error_panic_hook::set_once();

//     // Redirect tracing to console.log and friends:
//     tracing_wasm::set_as_global_default();

//     let web_options = eframe::WebOptions::default();
//     eframe::start_web(
//         "the_canvas_id", // hardcode it
//         web_options,
//         Box::new(|cc| Box::new(extended_spotify_wrapped::ESWApp::new(cc))),
//     )
//     .expect("failed to start eframe");
// }
