use age::{
    armor::{ArmoredWriter, Format},
    x25519, Encryptor, Recipient,
};
use iced::widget::{button, column, row, text, text_input};
use iced::{Element, Sandbox, Settings};
use std::{io::Write, vec};

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: iced::window::Settings {
            platform_specific: iced::window::PlatformSpecific {
                title_hidden: true,
                titlebar_transparent: true,
                fullsize_content_view: true,
            },
            ..Default::default()
        },
        ..Default::default()
    })
}

fn encrypt_error<T>(_: T) -> String {
    String::from("")
}

pub fn encrypt_with_x25519(public_key: &str, data: &[u8]) -> Result<Box<[u8]>, String> {
    let key: x25519::Recipient = public_key.parse().map_err(encrypt_error)?;

    let recipients = vec![Box::new(key) as Box<dyn Recipient + Send>];

    let encryptor = Encryptor::with_recipients(recipients).ok_or_else(|| encrypt_error(""))?;

    let mut output = vec![];

    let armor =
        ArmoredWriter::wrap_output(&mut output, Format::AsciiArmor).map_err(encrypt_error)?;

    let mut writer = encryptor.wrap_output(armor).map_err(encrypt_error)?;

    writer.write_all(data).map_err(encrypt_error)?;

    writer
        .finish()
        .and_then(|armor| armor.finish())
        .map_err(encrypt_error)?;

    Ok(output.into_boxed_slice())
}

#[derive(Default)]
struct Counter {
    public_key: String,
    message: String,
    encrypted_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    Encrypt,
    PublicKeyChanged(String),
    MessageChanged(String),
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
        String::from("rage UI")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Encrypt => {
                let encrypted = encrypt_with_x25519(&self.public_key, self.message.as_bytes());
                match encrypted {
                    Ok(data) => {
                        self.encrypted_message = String::from_utf8(data.to_vec()).unwrap();
                    }
                    Err(_) => {
                        self.encrypted_message = String::from("Error encrypting message");
                    }
                }
            }
            Message::PublicKeyChanged(data) => {
                self.public_key = data;
            }
            Message::MessageChanged(data) => {
                self.message = data;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input = text_input("Paste your age Public Key", &self.public_key)
            .on_input(Message::PublicKeyChanged);

        let message_input =
            text_input("Message to encrypt", &self.message).on_input(Message::MessageChanged);

        column![
            column![text("Public Key"), input].spacing(10),
            column![
                text("Message"),
                row![message_input, button("Encrypt").on_press(Message::Encrypt)].spacing(10)
            ]
            .spacing(10),
            column![text("Encrypted Message"), text(&self.encrypted_message),].spacing(10)
        ]
        .spacing(20)
        .padding([40, 20])
        .into()
    }
}
