# Punge
Punge is an Application for listening to music. It downloads songs from YouTube and provides a based listening experience. I hate streaming services, let me listen to my stinky unreleased music.

This is a newer version of Punge. As of now, only [old punge](https://github.com/nednoodlehead/old_punge) is working.
Punge uses `rodio` for playing audio and `Iced` for GUI.

New Punge brings on multiple improvements:

1) Improved preformance (significant)
2) A nicer GUI (from Tkinter -> Iced)
3) More sustainable and cleaner codebase
4) Written entirely in rust ! 🦀
5) Much cleaner and nicer interface for audio (we aint using pydub for that no more!)
6) Fully commented codebase
7) Playlist metadata
8) Better data about each individiual song

The main issue is having the rodio::OutputStream in scope when trying to play the music. Clicking play and nothing happens
