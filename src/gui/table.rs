// shoutout to github.com/tarkah for this banger !!!
// and we will sit here praying until https://github.com/iced-rs/iced/issues/160 comes out!
use crate::gui::messages::ProgramCommands;
use iced::widget::{button, checkbox, container, horizontal_space, row, text};
use iced::Element;
use iced::{Length, Renderer, Theme};
use iced_table::table;
// also we are removing the resizing functionality on ppurpose. i dont think that would be useful for punge..
// we are pretty much replicating the example while keeping much of the boilerplate over here

pub struct Column {
    kind: ColumnKind,
    pub width: f32,
    resize_offset: Option<f32>,
}

impl Column {
    pub fn new(kind: ColumnKind) -> Self {
        let width = match kind {
            ColumnKind::PlayButton => 35.0,
            ColumnKind::Author => 325.0,
            ColumnKind::Title => 375.0,
            ColumnKind::Album => 275.0,
            ColumnKind::Edit => 35.0,
        };
        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

impl<'a> table::Column<'a, ProgramCommands, Theme, Renderer> for Column {
    type Row = Row;

    fn header(&'a self, _col_index: usize) -> Element<'a, ProgramCommands> {
        let content: Element<'a, ProgramCommands> = match self.kind {
            ColumnKind::PlayButton => text("").into(),
            ColumnKind::Author => text("Author").into(),
            ColumnKind::Title => text("Title").into(),
            ColumnKind::Album => text("Album").into(),
            ColumnKind::Edit => row![
                button(text("\u{F4CA}").size(10).font(iced_aw::BOOTSTRAP_FONT))
                    .width(40)
                    .height(40)
                    .on_press(ProgramCommands::ToggleEditMode),
                button(text("  ").size(10))
                    .on_press(ProgramCommands::ToggleList)
                    .width(40)
                    .height(40)
            ]
            .into(),
        };

        container(content).height(24).center_y().into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        row_index: usize,
        row: &'a Self::Row,
    ) -> Element<'a, ProgramCommands> {
        let content: Element<_> = match self.kind {
            ColumnKind::PlayButton => button(text("\u{F4F4}").font(iced_aw::BOOTSTRAP_FONT))
                .on_press(ProgramCommands::PlaySong(row.uniqueid.clone()))
                .into(),
            ColumnKind::Author => text(row.author.clone()).into(),
            ColumnKind::Title => text(row.title.clone()).into(),
            ColumnKind::Album => text(row.album.clone()).into(),
            ColumnKind::Edit => row![
                checkbox("", row.ischecked).on_toggle(move |bol| {
                    ProgramCommands::SelectSong(row.uniqueid.clone(), bol, row_index)
                }),
                button(text("\u{F148}").font(iced_aw::BOOTSTRAP_FONT))
                    .on_press(ProgramCommands::MoveSongUp(row.uniqueid.clone(), row_index)),
                button(text("\u{F128}").font(iced_aw::BOOTSTRAP_FONT)).on_press(
                    ProgramCommands::MoveSongDown(row.uniqueid.clone(), row_index)
                )
            ]
            .into(),
        };

        container(content)
            .width(Length::Fill)
            .height(32)
            .center_y()
            .into()
    }

    fn footer(
        &'a self,
        _col_index: usize,
        rows: &'a [Self::Row],
    ) -> Option<Element<'a, ProgramCommands>> {
        let content = if matches!(self.kind, ColumnKind::Title) {
            let total_enabled = rows.len();

            Element::from(text(format!("Total: {total_enabled}")))
        } else {
            horizontal_space().into()
        };

        Some(container(content).height(24).center_y().into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}

pub enum ColumnKind {
    PlayButton,
    Author,
    Title,
    Album,
    Edit,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub title: String,
    pub author: String,
    pub album: String,
    pub uniqueid: String,
    pub ischecked: bool,
}
