use winit::platform::android::activity::AndroidApp;

#[no_mangle]
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
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.spacing.interact_size = egui::vec2(48.0, 48.0);
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
            cc.egui_ctx.set_style(style);

            Ok(Box::new(filmcraft_ui_egui::FilmcraftApp::new(cc)))
        }),
    );
}
