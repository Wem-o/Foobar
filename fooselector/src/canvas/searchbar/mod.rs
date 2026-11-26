//! This module implements a searchbar using canvas. 

use std::cell::{Cell, RefCell};

use iced::{
    Border, Color, Element, Font, Length, Padding, Pixels, Point, Rectangle, Renderer, Shadow, Size, Theme, advanced::{
        Layout, Text, layout::Node, renderer::Quad, text::{
            self, Paragraph as _,
            Shaping, Wrapping
        }, widget::{Operation, Tree, tree::{self, Tag}}
    }, widget::{
        canvas::{self, Stroke}, 
        text::LineHeight
    }
};
use iced::advanced::graphics::text::Paragraph;
// Trait magic
use iced::advanced::{Renderer as _, text::Renderer as _};

#[derive(Debug, Clone)]
pub struct Searchbar {
    pub content: String,    
    pub placeholder: String,

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

    
    // Internal stuff
    pub text_height: Cell<Option<f32>>,
    pub paragraph: RefCell<Option<Paragraph>>,
    pub should_redraw: Cell<bool>,
}

impl<Message> iced::advanced::Widget<Message, Theme, Renderer> for Searchbar 
{
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
        // A shortcut to the persistent state
        let state: &Self = tree.state.downcast_ref();
        
        let width = limits.max().width;
        let mut height = state.text_height.get().unwrap_or(0_f32) 
            + self.padding.top + self.padding.bottom 
            + self.inner_padding.top + self.inner_padding.bottom;

        // Ensure the height never becomes 0
        // (if height is 0, draw will not be called, and we cannot 
        // determine the height of the text, so it'll never render)
        if height == 0.0 {height = 1.0;}

        println!("Layout is called! The width is {width} and the height is {height}");
        println!("Layout is called! The text_height is {:?}", state.text_height.get());
        
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
        let state: &Self = tree.state.downcast_ref();
        
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
        
        // A reference to either the placeholder or the content to render
        let text: &str = if self.content.is_empty() {&self.placeholder} else {&self.content};

        // The bounding box for the text
        let text_size = size - self.inner_padding.into();
        let text_origin: Point = Point::new(
            boundary.x + self.inner_padding.left,
            boundary.y + self.inner_padding.top
        );
        let text_bounds = Rectangle::new(text_origin, text_size);

        if state.paragraph.borrow().is_none() {
            *state.paragraph.borrow_mut() = Some(
                Paragraph::with_text(Text {
                    content: text.into(),
                    bounds: text_size,
                    size: self.font_size,
                    line_height: self.line_height,
                    font: self.font.unwrap_or(Font::DEFAULT),
                    wrapping: iced::widget::text::Wrapping::None,
                    align_x: iced::widget::text::Alignment::Left,
                    align_y: iced::alignment::Vertical::Top,
                    hint_factor: None,
                    shaping: iced::widget::text::Shaping::Auto
                })
            );
        }
        // Safe to unwrap: because of the last if it'll alway have a value
        state.text_height.set(Some(
            state.paragraph.borrow().as_ref().unwrap().min_height()
        ));

        renderer.fill_paragraph(
            state.paragraph.borrow().as_ref().unwrap(),
            text_origin,
            self.text_color.unwrap_or(theme.palette().text), 
            text_bounds
        );
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        Tag::of::<Self>()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        tree::State::new(self.clone())
    }

    // fn operate(
    //     &mut self,
    //     _tree: &mut Tree,
    //     layout: Layout<'_>,
    //     _renderer: &Renderer,
    //     operation: &mut dyn Operation,
    // ) {
    //     operation.text(None, layout.bounds(), &self.content);
    // }
}

impl<Message> From<Searchbar> for Element<'_, Message> {
    fn from(value: Searchbar) -> Self {
        Self::new(value)
    }
}
    
//     fn draw(
//         &self,
//         state: &Self::State,
//         renderer: &iced::Renderer,
//         theme: &Theme,
//         bounds: Rectangle,
//         _cursor: Cursor,
//     ) -> Vec<iced::widget::canvas::Geometry<iced::Renderer>>
//     {
//         let (cache) = state;
// 
//         if self.should_redraw.get() {
//             cache.clear();
//             self.should_redraw.set(false);
//         }
// 
//         let geo = cache.draw_with_bounds(renderer, bounds, |frame| {
//             // The area after the padding happened
//             let size = Size::new(
//                 frame.width() - self.padding.left - self.padding.right, 
//                 frame.height() - self.padding.top - self.padding.bottom
//             );
//             let boundary = Rectangle::new(
//                 Point::new(self.padding.left, self.padding.top),
//                 size
//             );
// 
//             // == Draw the box bounding the text (this will be the bar) ==
//             let border_size = Size::new(self.border.width, self.border.width);
//             let bar = canvas::Path::rounded_rectangle(
//                 boundary.position() + iced::Vector::from(border_size / 2.0), 
//                 size - border_size, 
//                 self.border.radius
//             );
//             // Draw background
//             frame.fill(
//                 &bar, 
//                 self.bar_color.unwrap_or(theme.palette().background)
//             );
// 
//             // Draw border
//             if self.border.width > 0.0 {
//                 let bar = canvas::Path::rounded_rectangle(
//                     boundary.position() + iced::Vector::from(border_size / 2.0_f32), 
//                     // Subtract the size of the border from the space it'll take
//                     // up from the rectangle. 
//                     size - border_size,
//                     self.border.radius
//                 );
//                 frame.stroke(&bar, Stroke {
//                     width: self.border.width,
//                     style: canvas::Style::Solid(self.border.color),
//                     line_join: canvas::LineJoin::Round,
//                     ..Default::default()
//                 });
//             }
// 
//             // == Draw the text inside the bar == 
//             
//             // A reference to either the placeholder or the content to render
//             let text: &str = if self.content.is_empty() {&self.placeholder} else {&self.content};
// 
//             let text_size = size - self.inner_padding.into();
//             let text_origin: Point = Point::new(
//                 boundary.x + self.inner_padding.left,
//                 boundary.y + self.inner_padding.top
//             );
//             let text_bounds = Rectangle::new(text_origin, text_size);
// 
//             // Welcome to hell
//             let text_path = Paragraph::with_text(Text {
//                 content: text.into(),
//                 bounds: text_size,
//                 size: self.font_size,
//                 line_height: self.line_height,
//                 font: self.font.unwrap_or(Font::DEFAULT),
//                 wrapping: iced::widget::text::Wrapping::None,
//                 align_x: iced::widget::text::Alignment::Left,
//                 align_y: iced::alignment::Vertical::Center,
//                 hint_factor: None,
//                 shaping: iced::widget::text::Shaping::Advanced
//             }); 
//             self.text_height = Some(text_path.min_height());
//             
//             renderer.fill_paragraph(
//                 &text_path, 
//                 text_origin, 
//                 self.text_color.unwrap_or(Color::BLACK), 
//                 text_bounds
//             );
//             
//             // {
//             //     content: text.into(), 
//             //     color: self.text_color.unwrap_or(theme.palette().text),
//             //     font: self.font.unwrap_or(Font::DEFAULT),
//             //     max_width: text_size.width,
//             //     line_height: self.line_height,
//             //     size: self.font_size,
//             //     position: text_origin,
//             //     ..Default::default()
//             // };
//             // frame.fill_text(text_path);
//         });
//         
//         vec![geo]
//     }