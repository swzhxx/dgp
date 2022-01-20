use iced::{Sandbox, Settings};
use log::{info, LevelFilter};

#[derive(Default, Debug)]
struct App {
    chart: chart::State,
}

#[derive(Debug)]
enum Message {}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Hello Integration Plotters Iced".into()
    }

    fn update(&mut self, message: Self::Message) {
        // todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Text::new("Hello Rust Gui").into()
    }
}

mod chart {
    use iced::canvas::event::Event;
    use iced::canvas::{self, Canvas, Cursor, Frame};
    use nalgebra::Point2;

    #[derive(Default)]
    pub struct State {
        cache: canvas::Cache,
    }

    pub struct Point<'a> {
        state: &'a mut State,
        points: &'a Vec<Point2<f32>>,
    }

    impl<'a> canvas::Program<Point2<f32>> for Point<'a> {
        fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor) {
            todo!()
        }
        fn draw(&self, bounds: iced::Rectangle, cursor: Cursor) -> Vec<canvas::Geometry> {
            todo!()
        }
    }

    impl State {
        pub fn view<'a>() {}
    }

    enum Message {}
}

fn main() {
    env_logger::builder().filter_module("integration_plotters_iced", LevelFilter::Trace);
    info!("App start");
    App::run(Settings::default());
}
