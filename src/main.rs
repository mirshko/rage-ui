use age::{
    armor::{ArmoredReader, ArmoredWriter, Format},
    x25519, Decryptor, Encryptor,
};
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: iced::window::Settings {
            size: (700, 550),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })
}

pub fn keygen() -> String {
    let secret = x25519::Identity::generate();
    let public = secret.to_public();

    String::from(public.to_string())
}

#[derive(Default)]
struct Counter {
    public_key: String,
}

#[derive(Debug, Clone)]
enum Message {
    Keygen,
}

impl Sandbox for Counter {
    type Message = Message;

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn new() -> Self {
        Counter::default()
    }

    fn title(&self) -> String {
        String::from("rage-ui")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Keygen => {
                self.public_key = keygen();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(&self.public_key).size(20),
            button("Generate Age Key").on_press(Message::Keygen),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
