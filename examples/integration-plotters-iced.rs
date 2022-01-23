use iced::{
    executor, image, Application, Clipboard, Command, Container, Image, Length, Row, Sandbox,
    Settings, Subscription,
};
use iced_native::{window, Event};
use log::{debug, error};
use nalgebra::{Point2, Vector4};
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, LabelAreaPosition},
    style::{RGBAColor, RGBColor},
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
#[derive(Debug, Default)]
struct App {
    points: Vec<Point2<f32>>,
    image_buffer: Vec<u8>,
    window_width: u32,
    window_height: u32,
    image_viewer: image::viewer::State,
}

#[derive(Debug)]
enum Message {
    EventOccurred(iced_native::Event),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        // todo!()
        debug!("message : {:?}", message);
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        debug!("view");
        let buffer = {
            if self.image_buffer.len() == 0 {
                debug!("len = 0");
                let buffer_len = (HEIGHT * WIDTH * 3) as usize;
                let mut buffer = Vec::with_capacity(buffer_len);
                for i in 0..buffer_len {
                    buffer.push(255);
                }
                {
                    let area = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT))
                        .into_drawing_area();
                    area.fill(&plotters::prelude::WHITE).unwrap();

                    let mut ctx = ChartBuilder::on(&area)
                        .caption("Figure Sample", ("Arial", 30))
                        .build_cartesian_2d(0..100, 0..100)
                        .unwrap();

                    ctx.configure_mesh().draw().unwrap();
                }
                let buffer = buffer
                    .iter()
                    .enumerate()
                    .fold(vec![], |mut acc, (index, value)| {
                        if (index + 1) % 3 != 1 {
                            return acc;
                        }
                        let r = *value;
                        let g = *&buffer[index + 1];
                        let b = *&buffer[index + 2];
                        let a = 255 as u8;
                        acc.push(Vector4::new(r, g, b, a));
                        acc
                    })
                    .iter()
                    .fold(vec![], |mut acc, rgba| {
                        let b = rgba[2];
                        let g = rgba[1];
                        let r = rgba[0];
                        let a = rgba[3];
                        acc.push(b);
                        acc.push(g);
                        acc.push(r);
                        acc.push(a);
                        acc
                    });
                self.image_buffer = buffer;

                &self.image_buffer
            } else {
                debug!("len > 0");
                &self.image_buffer
            }
        };
        debug!("buffer {:?}", buffer.len());
        // let i = include_bytes!("./../images/ferris.png").clone().to_vec();
        // error!("i {:?}", i.len());
        let handle = iced::image::Handle::from_pixels(WIDTH, HEIGHT, self.image_buffer.clone());
        // handle.save("./../images/test.png");
        Container::new(
            image::Viewer::new(&mut self.image_viewer, handle)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .center_x()
        .into()
    }

    fn title(&self) -> String {
        "Hello World".into()
    }
}

fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.size = (WIDTH, HEIGHT);
    App::run(settings)
}
