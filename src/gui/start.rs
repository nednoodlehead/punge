// can we rename this to lib.rs at some point maybe??
use iced::widget::{button, text, column, row, container};
use iced::{Alignment, Length, Color, Settings, Sandbox, Element, Error, Theme, executor, Application};
use crate::gui::messages::PungeCommand;
use crate::player::interface;
use crate::{gui, playliststructs};
use std::thread;
use iced_native::Command;
use crate::gui::subscription as sub;

use iced_native::subscription::{self, Subscription};
use iced_native::futures::channel::mpsc;
use iced_native::futures::sink::SinkExt;
use crate::gui::subscription::Event;


pub fn begin() -> iced::Result {
    App::run(Settings::default())
}



struct App {
    theme: Theme,
    is_paused: bool,
    current_song: (String, String, String),
    sender: Option<mpsc::Sender<PungeCommand>>, // was not an option before !
}

#[derive(Debug, Clone)]
enum ProgramCommands {
    Test,
    Alt(sub::Event),
    Send(PungeCommand)
}


impl Application for App {
    type Executor = executor::Default;
    type Message = ProgramCommands;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
        App {
            theme: Default::default(),
            is_paused: false,
            current_song: ("".to_string(), "".to_string(), "".to_string()),
            sender: None
        },
        Command::none())
    }

    fn title(&self) -> String {
        String::from("Punge!!")
    }

    fn update(&mut self, msg: Self::Message) -> iced::Command<ProgramCommands> {
        match msg {
            Self::Message::Test => {
                println!("doing play, here?");
                    //self.sender.as_mut().unwrap().send(PungeCommand::Play);  // does it work?
                // self.sender.send(Command::Play).unwrap();  // i dont think this unwrap() can fail ..
            }
            Self::Message::Alt(sender) => {
                match sender {
                    Event::Ready(sender) => {
                        println!("yup, sender: {:?}", &sender);
                        self.sender = Some(sender);
                    }
                    Event::AppClosed => {
                        println!("when does this happen?")
                    }
                }
            }
            Self::Message::Send(cmd) => {
                self.sender.as_mut().unwrap().try_send(cmd).expect("failure sending msg");
            }
            _ => println!("inumplmented")
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(row![
                button(text("Go back")),
                button(text("Pause / Play")).on_press(ProgramCommands::Send(PungeCommand::Play)),
                button(text("Go forwards")),
                button(text("Shuffle"))
            ].spacing(50)
            .padding(iced::Padding::new(10 as f32)))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        sub::punge_listening_thread().map(Self::Message::Alt)
    }

}
