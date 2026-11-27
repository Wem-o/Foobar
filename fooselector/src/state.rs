use iced::{
    Alignment, Color, Element, Length, Task, Theme, border, 
    widget::{container, row}
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
        // let searchbar = container(
        //      text_input("Start typing....", &self.input_buffer)
        //         .on_input(Msg::Typed)
        //         .width(Length::Fill)
        // )
        //     .width(Length::Fill)
        //     .height(Length::Fill);

        // let searchbar = iced::widget::canvas(&self.searchbar)
        //     .width(Length::Fill)
        //     .height(
        //         self.searchbar.line_height.to_absolute(self.searchbar.font_size)
        //         + self.searchbar.padding.top + self.searchbar.padding.bottom 
        //         + self.searchbar.inner_padding.top + self.searchbar.inner_padding.bottom
        //     );

        let searchbar = Searchbar {
            content: "".into(),
            placeholder: "Start typing".into(),

            text_color: Some(Color::BLACK),
            font: None,
            font_size: 30.into(),
            line_height: 1.0.into(),

            border: iced::Border { color: Color::from_rgb8(136, 57, 239), width: 0.0, radius: 00.0.into() },
            bar_color: Some(Color::WHITE),
            box_shadow: None,

            padding: 0.0.into(),
            inner_padding: 0.0.into(),
        };
        
        return container(row![
            searchbar,
            iced::widget::button("Redraw!").on_press(Msg::Redraw)
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