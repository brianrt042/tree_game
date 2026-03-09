use std::fs;
use std::io::{stdout, {self, Write}};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{Event, KeyCode, read},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode},
    cursor, 
    queue,
};

pub struct TerminalUtil;

impl TerminalUtil{
    pub fn clear(){
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn set_env(){
        execute!(stdout(), cursor::Hide).unwrap();
        terminal::enable_raw_mode().unwrap();
    }
    
    pub fn reset_env(){
        execute!(stdout(), cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    pub fn panic_hook() -> Box<dyn Fn(&std::panic::PanicInfo) + Send + Sync + 'static> {
        Box::new(|info| {
            TerminalUtil::reset_env();
            eprintln!("{}", info);
        })
    }
}

pub fn read_ascii_content(string: &str, scale: (usize, usize)) -> Vec<char> {
    let (width, height) = scale;

    let mut lines: Vec<Vec<char>> = string
        .lines()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            // Pad or truncate to exact width
            chars.resize(width, ' ');
            chars
        })
        .collect();

    // Pad or truncate to exact height
    lines.resize_with(height, || vec![' '; width]);

    lines.into_iter().flatten().collect()
}