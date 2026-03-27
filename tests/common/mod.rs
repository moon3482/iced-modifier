use iced::Element;

pub type E<'a> = Element<'a, Msg>;

#[derive(Debug, Clone)]
pub enum Msg {
    A,
    B,
    #[allow(dead_code)]
    C,
    Hover(#[allow(dead_code)] bool),
}
