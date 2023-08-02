// use iced::subscription::{self, Subscription};
// use iced::futures::channel::mpsc;
// use iced::futures::sink::SinkExt;
// use crate::gui::messages::PungeCommand;
// use crate::player::interface;
// use crate::playliststructs::PungeMusicObject;
// use tokio;


// pub fn punge_listening_thread() -> Subscription<Self::Message> {
//         use iced::futures::SinkExt;
//     iced::subscription::channel(0, 32, |mut sender| async move {
//         let (gui_send, mut gui_rec) = tokio::sync::mpsc::unbounded_channel();
//         sender.send(Self::M)
//     })
//
// }
