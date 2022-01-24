use iced::Sandbox;

struct App {}

impl Sandbox for App {
    type Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

fn main() -> iced::Result {}
