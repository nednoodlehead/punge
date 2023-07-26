// use iced_native::subscription::{self, Subscription};
// use iced_native::futures::channel::mpsc;
// use iced_native::futures::sink::SinkExt;
// use crate::gui::messages::PungeCommand;
// use crate::player::interface;
// use crate::playliststructs::PungeMusicObject;
//
// #[derive(Debug, Clone)]
// pub enum Event { // events that the worker will produce
//     AppClosed  // the work is only finished when app closes i guess
// }
//
// pub enum State {  // what the state of the subscriber is (im pretty sure)
//     Starting,
//     Ready(mpsc::Receiver<PungeCommand>, interface::MusicPlayer)
// }
//
// pub fn punge_listening_thread() -> Subscription<Event> {
//     pub struct Worker;
//     subscription::channel(std::any::TypeId::of::<Worker>(), 100, |mut output| async move {
//         let mut state = State::Starting;
//
//         loop {
//             println!("loopin lol");
//             match &mut state {
//                 State::Starting => {
//                     let (sender, receiver) = mpsc::channel(100);
//
//                                             let item_1 = PungeMusicObject {
//                         title: "guess what bitch".to_string(),
//                         author: "danny and jpeg".to_string(),
//                         album: "expanded".to_string(),
//                         features: "asd".to_string(),
//                         length: "sadds".to_string(),
//                         savelocationmp3: r#"F:\Punge Downloads\Downloads\JPEGMAFIA & Danny Brown - Guess What Bitch, We Back Hoe!cIXklqch9zc.mp3"#.to_string(),
//                         savelocationjpg: "bruh".to_string(),
//                         datedownloaded: Default::default(),
//                         lastlistenedto: Default::default(),
//                         ischild: false,
//                         uniqueid: "penbis".to_string(),
//                         plays: 0,
//                         weight: 0
//                     };
//                     let items = vec![item_1];
//                     println!("CREATED MUSIC PLAYER !!");
//                     let music_part = interface::MusicPlayer::new(streamhandle, items);
//
//
//                     output.send(Event::Ready(sender)).await.expect("failure sending data");
//                     state = State::Ready(receiver, music_part);
//
//                 }
//                 State::Ready(receiver, music_obj) => {
//                     use iced_native::futures::StreamExt;
//
//                     println!("here? {:?}", music_obj.count);
//                     let input = receiver.select_next_some().await;
//                     println!("yeah we got input: {:?}", &input);
//
//                     match input {
//                         PungeCommand::Play => {
//                             println!("doing play! {}", music_obj.count);
//                             let file: String = String::from(r#"F:\Punge Downloads\Downloads\Baby Keem & Kendrick Lamar - family tiesv6HBZC9pZHQ.mp3"#);
//                             let real_file = interface::read_file_from_beginning(file);
//
//                             //music_obj.sink.append(real_file);
//                             //music_obj.sink.play()
//
//                         }
//                         _ => {
//                             println!("anything else!!")
//                         }
//                     }
//
//
//                 }
//             }
//
//         }
//
//     })
// }
//
//
//
