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

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("rage-ui")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
