use winit::platform::android::activity::AndroidApp;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let mut native_options = eframe::NativeOptions::default();
    native_options.android_app = Some(app);
    native_options.renderer = eframe::Renderer::Wgpu;

    let _ = eframe::run_native(
        "FilmCraft",
        native_options,
        Box::new(|_cc| {
            let session = filmcraft_engine::Session::default();
            Ok(Box::new(filmcraft_ui_egui::FilmcraftApp::new(session)))
        }),
    );
}
