// can we rename this to lib.rs at some point maybe??
use iced::widget::{button, text, column, row, container};
use iced::{Alignment, Length, Color, Settings, Sandbox, Element, Error, Theme, executor, Application};
use crate::gui::messages::PungeCommand;
use crate::player::interface;
use crate::{gui, playliststructs};
use std::thread;
use iced_native::Command;

use iced_native::subscription::{self, Subscription};
use iced_native::futures::channel::mpsc;
use iced_native::futures::sink::SinkExt;
use rodio::Sink;
use crate::player::interface::{AUDIO_PLAYER, read_file_from_beginning};

use std::sync::atomic::{Ordering, AtomicBool, AtomicUsize};


pub fn begin() -> iced::Result {
    App::run(Settings::default())
}



struct App {
    theme: Theme,
    // player: playliststructs::AudioPlayer this is held inside a static in interface.rs
}

#[derive(Debug, Clone)]
enum ProgramCommands {
    Test,
    Send(PungeCommand)
}


impl Application for App {
    type Executor = executor::Default;
    type Message = ProgramCommands;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (stream, streamhandle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&streamhandle).unwrap();
        let new_player = interface::MusicPlayer {  // this is a 100% requirement for this to be constructed like this and not using ::new()
            // ::new() will not play the music. something something where the OutputStream is, and it getting dropped somewhere
            list: vec![],
            sink,
            count: AtomicUsize::new(1),
            shuffle: AtomicBool::new(false), // should be read from cache eventually
            stream,
        };
        *interface::AUDIO_PLAYER.lock().unwrap() = Some(new_player);  // update the static to be an audio player
        (
        App {
            theme: Default::default(),
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
                println!("yeah, command: {:?}", cmd);
                match cmd {
                    PungeCommand::Play => {
                if AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.is_paused() {
                    println!("ok just continuing, since it is true?");
                    AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.play()
                }
                else {
                let val = read_file_from_beginning(String::from(r#"F:\spingus.mp3"#));
                interface::AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.append(val);
                interface::AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.play();
                        }
                    }
                    PungeCommand::Stop => {
                        interface::AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.pause();
                    }
                    PungeCommand::GoToAlbum => {
                        println!("status of is_paused: {}", AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.is_paused());
                    }
                    PungeCommand::StaticVolumeUp => {
                        // testing play with no specific appendation
                        AUDIO_PLAYER.lock().unwrap().as_ref().unwrap().sink.play();
                    }
                    PungeCommand::ToggleShuffle => {
                        println!("yeah, toggle shguffle!!")
                    }
                    _ => {
                        println!("all others!!");
                    }
                }
            }


            _ => println!("inumplmented")
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        container(row![
                button(text("Go back")),
                button(text("Pause / Play")).on_press(ProgramCommands::Send(PungeCommand::Play)),
                button(text("pause")).on_press(ProgramCommands::Send(PungeCommand::Stop)),
                button(text("Shuffle")),
                button(text("is paused::")).on_press(ProgramCommands::Send(PungeCommand::GoToAlbum)),
                button(text("Hard resume")).on_press(ProgramCommands::Send(PungeCommand::StaticVolumeUp))
            ].spacing(50)
            .padding(iced::Padding::new(10 as f32)))
            .into()
    }

    // fn subscription(&self) -> Subscription<Self::Message> {
    //     sub::punge_listening_thread().map(Self::Message::Alt)
    // }

}
