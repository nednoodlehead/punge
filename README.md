# Punge
Punge is an Application for listening to music. It downloads songs from YouTube and provides a based listening experience. I hate streaming services, let me listen to my stinky unreleased music.

This is a newer version of Punge. As of now, it is missing some important features (visually seeing your music, lol), but works fine for playing and downloading music
Punge uses `rodio` for playing audio and `Iced` for GUI.

New Punge brings on multiple improvements:

1) Improved preformance (significant, like up to 100x faster in certain scenarios)
2) A nicer GUI (from Tkinter -> Iced)
3) More sustainable and cleaner codebase
4) Written entirely in rust ! 🦀
5) Much cleaner and nicer interface for audio (we aint using pydub for that no more!)
6) Fully commented codebase
7) Playlist metadata
8) Better data about each individiual song

# Requirements:
1. Cargo on path

2. ffmpeg on path

3. git on path (only technically needed to follow the instructions, you can still download .zip from github)

# Build Instructions
1. Clone the repo `git clone https://github.com/nednoodlehead/punge`
2. `cd punge`
3. `cargo build --release`
4. Once it is built, copy `punge.exe` from `./target/` into the project root
5. Launch `punge.exe`
