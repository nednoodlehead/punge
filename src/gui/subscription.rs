use crate::db::metadata::{on_passive_play, on_seek, skipped_song};
use crate::gui::messages::{DatabaseMessages, ProgramCommands, PungeCommand};
use crate::gui::start::App;
use crate::player::interface;
use crate::playliststructs::PungeMusicObject;
use async_std::task::sleep;
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::subscription::{self, Subscription};
use tokio;

impl App {
    // requires a listener. this will be a tokio::UnboundedReceiver<PungeCommand>
    // does not need 2 way communication, as this subscription just listens and inserts into the database
    pub fn database_sub(
        &self,
        mut db_listener: tokio::sync::mpsc::UnboundedReceiver<DatabaseMessages>,
    ) -> Subscription<ProgramCommands> {
        iced::subscription::channel(6, 32, |mut _sender| async move {
            loop {
                match db_listener.try_recv() {
                    Ok(cmd) => match cmd {
                        DatabaseMessages::Played(uniqueid) => {
                            on_passive_play(uniqueid).unwrap();
                        }
                        DatabaseMessages::Skipped(uniqueid) => {
                            skipped_song(uniqueid).unwrap();
                        }
                        DatabaseMessages::Seeked(uniqueid) => {
                            on_seek(uniqueid).unwrap();
                        }
                    },
                    Err(e) => {
                        // ignore!
                    }
                }
                sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }
}
