//! This module implements a searchbar widget. 

use std::range::RangeInclusive;

use iced::{
    Border, Color, Element, Font, Length, Padding, Pixels, Point, Rectangle, Renderer, Shadow, Size, Theme, advanced::{
        Text, layout::Node, renderer::Quad, text::{Paragraph as _, paragraph::Plain}, widget::tree::{self, Tag}
    }, keyboard::{Modifiers, key::Named}, widget::text::LineHeight,
};
use iced::advanced::graphics::text::Paragraph;
// Trait magic
use iced::advanced::{Renderer as _, text::Renderer as _};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Copy, Clone)]
enum CursorState {
    /// The cursor is selecting [`.0`; `.1`] area. If .2 is true that means 
    /// that if the selection ends by pressing esacpe the cursor should be 
    /// sent to the begining (the selection was started from the back).
    Selecting(usize, usize, bool),
    /// The cursor is before the `.0`th byte in the content
    AtCharacter(usize)
}
impl Default for CursorState {
    fn default() -> Self {
        Self::AtCharacter(0)
    }
}
impl CursorState {
    pub fn character_entered(&self, length: usize) -> Self {
        match self {
            Self::AtCharacter(position) => Self::AtCharacter(position + length),
            Self::Selecting(start, _, _) => Self::AtCharacter(start + length)
        }
    }

    pub fn deleting(&self, is_backspace: bool, length: usize) -> Self {
        match (self, is_backspace) {
            (CursorState::AtCharacter(pos), true) => Self::AtCharacter(pos - length),
            (CursorState::AtCharacter(_), false) => *self,
            
            (CursorState::Selecting(_, end, _), _) => 
                Self::AtCharacter(*end - length)
        }
    }

    /// Replicates the action when escape is pressed. This cancels any selection.
    /// 
    /// Note: due to this function getting no information about the 
    /// string, it's the caller's responisbility to verify whether 
    /// the set position is a valid utf8 position or grapheme.
    pub fn escape_pressed(&self) -> Self {
        match *self {
            CursorState::AtCharacter(_) => *self,
            CursorState::Selecting(start, end, selection_started_from_back) => {
                if selection_started_from_back {Self::AtCharacter(start)}
                else {Self::AtCharacter(end)}
            }
        }
    }
}

#[derive(Debug, Default)]
struct SearchbarState {
    pub paragraph: Plain<Paragraph>,
    content: String,
    
    pub cursor_state: CursorState,

    text_max_width: f32
}
impl SearchbarState {
    fn update_paragraph(&mut self, params: &Searchbar) -> bool {
        self.paragraph.update( Text {
            content: &self.content,
            bounds: Size::new(self.text_max_width, f32::MAX),
            size: params.font_size,
            line_height: params.line_height,
            font: params.font.unwrap_or(Font::DEFAULT),
            wrapping: iced::widget::text::Wrapping::None,
            align_x: iced::widget::text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            hint_factor: None,
            shaping: iced::widget::text::Shaping::Auto
        })
    }
    
    fn keypress(&mut self, params: &Searchbar, str: &str) {
        println!("key pressed: {str}");
        
        match self.cursor_state {
            CursorState::AtCharacter(pos) => {
                self.content.insert_str(pos, str);
                self.cursor_state = self.cursor_state.character_entered(str.len());
            }
            CursorState::Selecting(start, end, _) => {
                self.content.replace_range(start..end, str);
                self.cursor_state = self.cursor_state.character_entered(str.len());
            }
        }

        self.update_paragraph(params);
    }

    fn backspace(&mut self, params: &Searchbar) {
        if self.content.is_empty() {return;}
        
        match self.cursor_state {
            CursorState::AtCharacter(pos) => {
                if pos == 0 {return;}
                
                let mut grapheme_bounds: Option<RangeInclusive<usize>> = None;
                let mut grapheme_byte_position = 0_usize;
                
                for len in self.content
                    .graphemes(true)
                    .map(|grapheme| grapheme.len()) 
                {
                    if grapheme_byte_position == pos {break;}

                    grapheme_bounds = Some(RangeInclusive 
                        { start: grapheme_byte_position, last: grapheme_byte_position + len - 1 }
                    );

                    grapheme_byte_position += len;
                }

                let grapheme_bounds = grapheme_bounds.expect("Backspacing went wrong");
                self.content.replace_range(
                    grapheme_bounds,
                    ""
                );

                self.cursor_state = self.cursor_state.deleting(
                    true, 
                    grapheme_bounds.last - grapheme_bounds.start + 1
                )
            }
            CursorState::Selecting(_start, _end, _) => {
                unimplemented!("Backspace while selecting");
            }
        }

        self.update_paragraph(params);
    }

    fn escape_pressed(&mut self) {
        self.cursor_state.escape_pressed();
    }
}

#[derive(Debug, Clone)]
pub struct Searchbar {
    pub placeholder: &'static str,

    /// The color of the text. If none is given the theme's default
    /// is used
    pub text_color: Option<Color>,
    pub font: Option<Font>,
    pub font_size: Pixels,
    pub line_height: LineHeight,
    

    /// The border of the bar
    pub border: Border,
    /// The background color of the bar (if none the theme's background is used)
    pub bar_color: Option<Color>,
    pub box_shadow: Option<Shadow>,
    
    /// The padding between the canva's bounds and the bar
    pub padding: Padding,
    /// The padding between the the text and the bar
    pub inner_padding: Padding,
}

impl<Message> iced::advanced::Widget<Message, Theme, Renderer> for Searchbar 
{
    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        Tag::of::<SearchbarState>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        tree::State::new(SearchbarState::default())
    }
    
    fn size(&self) -> Size<iced::Length> {
        Size { height: Length::Shrink, width: Length::Fill }
    }

    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node
    {
        let bounds = limits.max();
        
        // A shortcut to the persistent state
        let state: &mut SearchbarState = tree.state.downcast_mut();

        // A reference to either the placeholder or the content to render
        let text: &str = if state.content.is_empty() 
            {&self.placeholder} else {&state.content};
        

        let width = bounds.width;
        state.text_max_width = width;

        let _ = state.paragraph.update( Text {
            content: text.into(),
            bounds: Size::new(width, f32::MAX),
            size: self.font_size,
            line_height: self.line_height,
            font: self.font.unwrap_or(Font::DEFAULT),
            wrapping: iced::widget::text::Wrapping::None,
            align_x: iced::widget::text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            hint_factor: None,
            shaping: iced::widget::text::Shaping::Auto
        });

        let glyph_height: f32 = state
            .paragraph
            .raw()
            .line_height()
            .to_absolute(self.font_size)
            .into()
        ;
        
        let height = glyph_height
            + self.padding.top + self.padding.bottom 
            + self.inner_padding.top + self.inner_padding.bottom;

        // println!("Layout is called! The width is {width} and the height is {height}");
        // println!("Layout is called! The text_height is {:?}", state.text_height.get());
        
        Node::new(Size::new(width, height))
    }
    
    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &Rectangle,
    )
    {
        // A shortcut to the persistent state
        let state: &SearchbarState = tree.state.downcast_ref();
        
        // The area after the padding is applied
        let size = Size::new(
            layout.bounds().width - self.padding.left - self.padding.right, 
            layout.bounds().height - self.padding.top - self.padding.bottom
        );
        let boundary = Rectangle::new(
            Point::new(
                layout.position().x + self.padding.left, 
                layout.position().y + self.padding.top
            ),
            size
        );

        // == Draw the bar (/background box) == 
        let bar = Quad {
            border: self.border,
            bounds: boundary,
            shadow: self.box_shadow.unwrap_or(Shadow::default()),
            snap: false
        };
        renderer.fill_quad(
            bar, 
            self.bar_color.unwrap_or(theme.palette().background)
        );

        // == Draw the text inside the bar == 
        let text_size = size - self.inner_padding.into();
        let text_origin: Point = Point::new(
            boundary.x + self.inner_padding.left,
            boundary.y + self.inner_padding.top
        );
        let text_bounds = Rectangle::new(text_origin, text_size);

        let mut color = self.text_color.unwrap_or(theme.palette().text);
        if state.content.is_empty() {color.a = 0.65;}

        renderer.fill_paragraph(
            state.paragraph.raw(),
            text_origin,
            color, 
            text_bounds
        );
    }

    fn update(
        &mut self,
        tree: &mut tree::Tree,
        event: &iced::Event,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &Rectangle,
    )
    {
        let state: &mut SearchbarState = tree.state.downcast_mut();
        
        if let iced::Event::Keyboard(kyb) = event 
            && let iced::keyboard::Event::KeyPressed { 
                modified_key: key, modifiers: mods,
                key: _,  physical_key: _, location: _, 
                text: _, repeat: _ } = kyb
        {            
            // Imported here to avoid confusion (iced likes to call different 
            // struct the same thing)
            use iced::keyboard::key::Key;
            
            match (key, *mods) {
                // TODO! Close the whole app if esc is spammed
                (Key::Named(Named::Escape), _) => state.escape_pressed(),
                (Key::Named(Named::Space), _) => state.keypress(self, " "),
                (Key::Named(Named::Backspace), _) => state.backspace(self),

                //  Handle shortcuts with ctrl
                (Key::Character(key), Modifiers::CTRL) => {
                    match key.as_str() {
                        // Common keycombination to jump to the start of the line
                        "a" => state.cursor_state = CursorState::AtCharacter(0),
                        _ => {}
                    }
                }
                
                (Key::Character(smolstr), _) => state.keypress(self, smolstr),
                _ => {}
            }

            println!("The cursor is: {:?} editor buffer is: {}", state.cursor_state, state.content);
            shell.request_redraw();
        }
    }
}

impl<Message> From<Searchbar> for Element<'_, Message> {
    fn from(value: Searchbar) -> Self {
        Self::new(value)
    }
}