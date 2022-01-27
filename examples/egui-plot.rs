use std::env;

use eframe::epi;
use egui::{
    plot::{Line, MarkerShape, Plot, Values},
    Color32, Pos2, Widget,
};
use log::{info, LevelFilter};

#[derive(Default)]
struct App {
    plot_line: PlotLine,
}

impl epi::App for App {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.add(&mut self.plot_line);
            ui.add(&mut self.plot_line);
        });
    }

    fn name(&self) -> &str {
        "Plot"
    }
}

#[derive(Default)]
struct PlotLine {
    cursor_click: Option<Pos2>,
    points: Vec<egui::plot::Value>,
}

impl PlotLine {
    fn makers() {}
}

impl Widget for &mut PlotLine {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let plot = Plot::new("plot line").data_aspect(20.);
        let response = plot
            .show(ui, |plot_ui| {
                if self.cursor_click.is_some() {
                    let pos = self.cursor_click.take();
                    let plot_pos = plot_ui.plot_from_screen(pos.unwrap());
                    info!("renderer pos {:?} ", plot_pos);
                    self.points.push(plot_pos);
                }

                if self.points.len() > 0 {
                    let mut points = egui::plot::Points::new(egui::plot::Values::from_values(
                        self.points.clone(),
                    ))
                    .name("points")
                    .filled(true)
                    .radius(5.)
                    .shape(MarkerShape::Circle)
                    .color(egui::Color32::from_rgb(255, 0, 0));
                    plot_ui.points(points);

                    //line

                    // let line = Line::new(Values::from_explicit_callback(
                    //     move |x| 0.5 * (2.0 * x).sin(),
                    //     ..,
                    //     512,
                    // ))
                    // .color(Color32::from_rgb(200, 100, 100));
                    // plot_ui.line(line);
                }
            })
            .response;
        if response.clicked() {
            // self.cursor_click  =
            let pose = ui.ctx().input().pointer.hover_pos();
            info!("plotline clicked {:?}", pose);
            self.cursor_click = pose;
        }
        response
    }
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), native_options);
}
