use iced::{
    executor,
    image::{self, Handle},
    Application, Clipboard, Command, Container, Image, Length, Point, Row, Sandbox, Settings,
    Subscription,
};
use iced_native::{window, Event};
use log::{debug, error, info, trace, LevelFilter};
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
    image_handle: Option<Handle>,
    window_width: u32,
    window_height: u32,
    image_viewer: image::viewer::State,
    cursor_position: Point,
}

#[derive(Debug)]
enum Message {
    EventOccurred(iced_native::Event),
}

impl App {
    fn draw_plotters(&mut self) {
        let len = (WIDTH * HEIGHT * 3) as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(255);
        }
        {
            let area = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT)).into_drawing_area();
            area.fill(&plotters::prelude::WHITE);
            let mut ctx = ChartBuilder::on(&area)
                .build_cartesian_2d(0..WIDTH as i32, 0..HEIGHT as i32)
                .unwrap();
            ctx.configure_mesh().draw().unwrap();
            if self.points.len() != 0 {
                ctx.draw_series(self.points.iter().map(|p| {
                    plotters::prelude::Circle::new(
                        (p.x as i32, p.y as i32),
                        3,
                        &plotters::prelude::RED,
                    )
                }));
            }
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
        // self.image_buffer = buffer;
        self.image_handle = Some(iced::image::Handle::from_pixels(WIDTH, HEIGHT, buffer));
    }
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
        info!("message : {:?}", message);
        match message {
            Message::EventOccurred(message) => match message {
                Event::Keyboard(_) => {}
                Event::Mouse(event) => match event {
                    iced::mouse::Event::CursorEntered => {}
                    iced::mouse::Event::CursorLeft => {}
                    iced::mouse::Event::CursorMoved { position } => self.cursor_position = position,
                    iced::mouse::Event::ButtonPressed(button) => match button {
                        // iced::mouse::Button::Left => todo!(),
                        iced::mouse::Button::Right => {
                            // let position;
                            self.points.push(Point2::new(
                                self.cursor_position.x,
                                (HEIGHT as f32) - self.cursor_position.y,
                            ));
                            self.draw_plotters()
                        }
                        // iced::mouse::Button::Middle => todo!(),
                        _ => {} // iced::mouse::Button::Other(_) => todo!(),
                    },
                    iced::mouse::Event::ButtonReleased(_) => {}
                    iced::mouse::Event::WheelScrolled { delta } => {}
                },
                Event::Window(_) => {}
                Event::Touch(_) => {}
            },
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        info!("view");
        // let buffer = {
        //     if self.image_buffer.len() == 0 {
        //         let buffer_len = (HEIGHT * WIDTH * 3) as usize;
        //         let mut buffer = Vec::with_capacity(buffer_len);
        //         for i in 0..buffer_len {
        //             buffer.push(255);
        //         }
        //         {
        //             let area = BitMapBackend::with_buffer(&mut buffer, (WIDTH, HEIGHT))
        //                 .into_drawing_area();
        //             area.fill(&plotters::prelude::WHITE).unwrap();

        //             let mut ctx = ChartBuilder::on(&area)
        //                 .caption("Figure Sample", ("Arial", 30))
        //                 .build_cartesian_2d(0..100, 0..100)
        //                 .unwrap();

        //             ctx.configure_mesh().draw().unwrap();
        //         }
        //         let buffer = buffer
        //             .iter()
        //             .enumerate()
        //             .fold(vec![], |mut acc, (index, value)| {
        //                 if (index + 1) % 3 != 1 {
        //                     return acc;
        //                 }
        //                 let r = *value;
        //                 let g = *&buffer[index + 1];
        //                 let b = *&buffer[index + 2];
        //                 let a = 255 as u8;
        //                 acc.push(Vector4::new(r, g, b, a));
        //                 acc
        //             })
        //             .iter()
        //             .fold(vec![], |mut acc, rgba| {
        //                 let b = rgba[2];
        //                 let g = rgba[1];
        //                 let r = rgba[0];
        //                 let a = rgba[3];
        //                 acc.push(b);
        //                 acc.push(g);
        //                 acc.push(r);
        //                 acc.push(a);
        //                 acc
        //             });
        //         self.image_buffer = buffer;

        //         &self.image_buffer
        //     } else {
        //         trace!("len > 0");
        //         &self.image_buffer
        //     }
        // };
        // trace!("buffer {:?}", buffer.len());

        // let handle = iced::image::Handle::from_pixels(WIDTH, HEIGHT, self.image_buffer.clone());

        match self.image_handle {
            None => self.draw_plotters(),
            _ => {}
        };
        Container::new(
            image::Viewer::new(
                &mut self.image_viewer,
                self.image_handle.as_ref().unwrap().clone(),
            )
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
    env_logger::builder()
        .filter_module("integration_plotters_iced", LevelFilter::Debug)
        .init();
    let mut settings = Settings::default();
    settings.window.size = (WIDTH, HEIGHT);
    App::run(settings)
}
