// can we rename this to lib.rs at some point maybe??
use iced::widget::{button, text, column, row, container};
use iced::{Alignment, Length, Color, Settings, Sandbox, Element, Error, Theme, executor, Application};
use crate::gui::messages::PungeCommand;
use crate::player::interface;
use crate::{gui, playliststructs};
use std::thread;
use iced::Command;
use crate::gui::subscription as sub;

use iced::subscription::{self, Subscription};
use std::sync::mpsc;
use iced::futures::sink::SinkExt;
use tokio::sync::mpsc as async_sender;
use crate::db::fetch;
use crate::playliststructs::PungeMusicObject;

pub fn begin() -> iced::Result {
    App::run(Settings::default())
}



struct App {
    theme: Theme,
    is_paused: bool,
    current_song: (String, String, String),
    sender: Option<async_sender::UnboundedSender<PungeCommand>>, // was not an option before !
}

#[derive(Debug, Clone)]
enum ProgramCommands {
    Test,
    Send(PungeCommand),
    UpdateSender(Option<async_sender::UnboundedSender<PungeCommand>>),
    NewData(String, String, String) // for sending back title, artist and album to GUI
}


impl Application for App {
    type Executor = executor::Default;
    type Message = ProgramCommands;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
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
            Self::Message::Send(cmd) => {
                self.sender.as_mut().unwrap().send(cmd).expect("failure sending msg");
            }
            Self::Message::UpdateSender(sender) => {
                println!("updated sender!");
                self.sender = sender;
            }
            Self::Message::NewData(art, title, alb) => {
                println!("The new information given to update: {art} {title} {alb}")
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
        use iced::futures::SinkExt;
        iced::subscription::channel(0, 32, |mut sender| async move {
        let (gui_send, mut gui_rec) = tokio::sync::mpsc::unbounded_channel();
        sender.send(Self::Message::UpdateSender(Some(gui_send))).await.unwrap(); // send the sender to the gui !!

        let items: Vec<PungeMusicObject> = fetch::get_all_main().unwrap();

        let mut music_obj = interface::MusicPlayer::new(items);


        // main music loop!
            println!("starting main loop");
        loop {
            match gui_rec.try_recv() {
                Ok(cmd) => {
                    match cmd {
                        PungeCommand::Play => {
                            println!("obj list: {:?}", music_obj.list);
                            let song = interface::read_file_from_beginning(music_obj.list[0].savelocationmp3.clone());
                            music_obj.sink.append(song);
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            println!("playing here... {}", music_obj.count);
                            sender.send(ProgramCommands::NewData("one".to_string(), "two".to_string(), "three".to_string())).await.unwrap();
                        }
                        _ => {
                            println!("yeah, other stuff... {:?}", cmd)
                        }
                    }
                }
                _ => {
                    // what gets hit when nothing happens
                }
            }
            async_std::task::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
    }

}
