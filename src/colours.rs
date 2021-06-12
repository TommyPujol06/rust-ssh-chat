use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub enum Colours {
    Red,
    Green,
    Cyan,
    Magenta,
    Yellow,
    Pink,
}

const COLOURS_LEN: usize = 6;

impl Colours {
    pub fn value(&self) -> (u8, u8, u8) {
        match self {
            Colours::Red => return (198, 071, 086),
            Colours::Green => return (150, 197, 124),
            Colours::Cyan => return (000, 255, 255),
            Colours::Magenta => return (147, 050, 158),
            Colours::Yellow => return (255, 226, 104),
            Colours::Pink => return (255, 105, 180),
        }
    }
}

impl Distribution<Colours> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colours {
        match rng.gen_range(0..COLOURS_LEN) {
            0 => Colours::Red,
            1 => Colours::Green,
            2 => Colours::Cyan,
            3 => Colours::Magenta,
            4 => Colours::Yellow,
            _ => Colours::Pink,
        }
    }
}
