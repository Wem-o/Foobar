use iced::{
    Alignment, Color, Element, Length, Task, Theme, 
    border, widget::{container, row}
};
use iced_layershell::to_layer_message;

use crate::Mode;
use crate::canvas::Searchbar;

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Msg {
    Redraw
}

pub struct State {
    pub theme: iced::Theme,
    _mode: Mode,

    _input_buffer: String,

}

impl State {
    pub fn new(mode: Mode) -> Self {
        Self {
            theme: iced::Theme::Dracula,
            _mode: mode,
            _input_buffer: String::new(),
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, Msg> {
        let searchbar = Searchbar {
            placeholder: "The quick brown fox jumps over the lazy dog üóúűáé".into(),

            text_color: Some(Color::BLACK),
            font: None,
            font_size: 30.into(),
            line_height: 1.35.into(),

            border: iced::Border { color: Color::from_rgb8(136, 57, 239), width: 1.0, radius: 10.0.into() },
            bar_color: Some(Color::WHITE),
            box_shadow: None,

            padding: 0.0.into(),
            inner_padding: 0.0.into(),
        };
        
        return container(row![
            searchbar,
        ])
        .style(|theme: &Theme| {
            container::Style::default()
                .background(theme.palette().background)
                .border(border::rounded(10))
        })
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
    }

    pub fn update(&mut self, message: Msg) -> Task<Msg> {
        match message {
            

            _ => {}
        }
        
        Task::none()
    }

    pub fn style(_: &State, theme: &Theme) -> iced::theme::Style {
        iced::theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}