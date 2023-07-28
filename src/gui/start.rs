// can we rename this to lib.rs at some point maybe??
use iced::widget::{button, text, column, row, container};
use iced::{Alignment, Length, Color, Settings, Sandbox, Element, Error, Theme, executor, Application};
use crate::gui::messages::PungeCommand;
use crate::player::interface;
use crate::{gui, playliststructs};
use iced_native::Command;

use iced_native::subscription::{self, Subscription};
use std::sync::mpsc;
use iced_native::futures::sink::SinkExt;
use rodio::Sink;
use crate::player::interface::{ read_file_from_beginning};  // used too import AUdio_PLAYER

use std::sync::atomic::{Ordering, AtomicBool, AtomicUsize};
use crate::playliststructs::PungeMusicObject;


pub fn begin() -> iced::Result {
    App::run(Settings::default())
}



struct App {
    theme: Theme,
    // player: playliststructs::AudioPlayer this is held inside a static in interface.rs
    sender: mpsc::Sender<PungeCommand>
}

#[derive(Debug, Clone)]
enum ProgramCommands {
    Test,
    PungeSend(PungeCommand)
}


impl Application for App {
    type Executor = executor::Default;
    type Message = ProgramCommands;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (stream, streamhandle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&streamhandle).unwrap();
        let (sender, receiver) = mpsc::channel();
        
        let temp_item = PungeMusicObject {
            title: "da title".to_string(),
            author: "da authoir".to_string(),
            album: "".to_string(),
            features: "".to_string(),
            length: "".to_string(),
            savelocationmp3: r#"F:\spingus.mp3"#.to_string(),
            savelocationjpg: "".to_string(),
            datedownloaded: Default::default(),
            lastlistenedto: Default::default(),
            ischild: false,
            uniqueid: "bruh".to_string(),
            plays: 0,
            weight: 0
        };
        
        let temp_2 = PungeMusicObject {
            title: "".to_string(),
            author: "".to_string(),
            album: "".to_string(),
            features: "".to_string(),
            length: "".to_string(),
            savelocationmp3: r#"F:\Punge Downloads\Downloads\McKinley Dixon - Live From The Kitchen Table feat Ghais GuevaraZ9VvZpeemO04.mp3"#.to_string(),
            savelocationjpg: "".to_string(),
            datedownloaded: Default::default(),
            lastlistenedto: Default::default(),
            ischild: false,
            uniqueid: "".to_string(),
            plays: 0,
            weight: 0
        };
        let temp_3 = PungeMusicObject {
            title: "".to_string(),
            author: "".to_string(),
            album: "".to_string(),
            features: "".to_string(),
            length: "".to_string(),
            savelocationmp3: r#"F:\testing.mp3"#.to_string(),
            savelocationjpg: "".to_string(),
            datedownloaded: Default::default(),
            lastlistenedto: Default::default(),
            ischild: false,
            uniqueid: "".to_string(),
            plays: 0,
            weight: 0
        };
        let new_player = interface::MusicPlayer {  // this is a 100% requirement for this to be constructed like this and not using ::new()
            // ::new() will not play the music. something something where the OutputStream is, and it getting dropped somewhere
            list: vec![temp_item, temp_2, temp_3],
            sink,
            count: 1,
            to_play: false,
            shuffle: false, // should be read from cache eventually
            stream,
            listener: receiver
        };
        let mut audio_player = new_player;  // update the static to be an audio player
        let thread_handle =  std::thread::spawn(move || {
            audio_player.play_loop()
        });
        (
        App {
            theme: Default::default(),
            sender,
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
            Self::Message::PungeSend(cmd) => {
                println!("sent cmd: {:?}", &cmd);
                self.sender.send(cmd).unwrap();
            }


            _ => println!("inumplmented")
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(row![
                button(text("Go back")),
                button(text("Pause / Play")).on_press(ProgramCommands::PungeSend(PungeCommand::Play)),
                button(text("pause")).on_press(ProgramCommands::PungeSend(PungeCommand::Stop)),
                button(text("Shuffle")),
                button(text("is paused::")),
                button(text("Hard resume"))
            ].spacing(50)
            .padding(iced::Padding::new(10 as f32)))
            .into()
    }

    // fn subscription(&self) -> Subscription<Self::Message> {
    //     sub::punge_listening_thread().map(Self::Message::Alt)
    // }

}
