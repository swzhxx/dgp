use eframe::epi;

#[derive(Default)]
struct App {}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let label = egui::Label::new("Hello World");
            ui.add(label);
            let label = egui::Label::new("Let's Start DGP");
            ui.add(label);
        });
    }

    fn name(&self) -> &str {
        "Hello World"
    }
}
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}
