use eframe::egui;

#[derive(Default)]
pub struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.label("hello world");
                ctx.fonts(|fonts| {
                    let fonts = fonts.lock();
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("hello world");
        });
    }
}
