use iced::{Element, Task};
use iced_layershell::to_layer_message;

use crate::Mode;

#[to_layer_message]
#[derive(Debug)]
pub enum Msg {
    
}

pub struct State {
    mode: Mode
}

impl State {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode
        }
    }

    pub fn view(&self) -> Element<Msg> {
        return iced::widget::text!("Hello world!").into();
    }

    pub fn update(&mut self, message: Msg) -> Task<Msg> {
        // match message {}
        
        Task::none()
    }
}