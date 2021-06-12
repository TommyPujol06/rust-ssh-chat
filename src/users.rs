use colored::Colorize;
use thrussh::ChannelId;

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct User<'a> {
    pub id: usize,
    pub channel: ChannelId,
    pub username: &'a str,
    pub colour: (u8, u8, u8),
}

impl<'a> PartialEq for User<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

impl<'a> User<'a> {
    pub fn new(id: usize, channel: ChannelId, username: &'a str, colour: (u8, u8, u8)) -> Self {
        Self {
            id,
            channel,
            username,
            colour,
        }
    }

    pub fn colourized(&self) -> String {
        let (r, g, b) = self.colour;
        self.username.truecolor(r, g, b).to_string()
    }
}
