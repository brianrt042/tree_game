mod util;
use util::{read_ascii_content};

pub struct Background { ch: char }

impl Layer for Background {
    fn draw(&self, buf: &mut FrameBuffer) {
        let background = include_str!("background.txt");
        read_ascii_content();
    }
}