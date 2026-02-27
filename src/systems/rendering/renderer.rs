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

use super::sheets::layer::Layer;
use super::sheets::main_menu::MainMenu;


// --- FRAME BUFFER ---//

struct FrameBuffer {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl FrameBuffer {
    pub fn new(cols: usize, rows: usize) -> Self{
        let area_size = rows * cols;
        FrameBuffer { 
            rows: rows, 
            cols: cols, 
            data: vec![' '; area_size]}
    }

    pub fn apply_layer(&mut self, layer: &dyn Layer) {
        let layer_vec = layer.get_render();
        let (pos_x, pos_y) = layer.get_position();
        let (width, height) = layer.get_scale();

        assert!(pos_x + width <= self.cols);
        assert!(pos_y + height <= self.rows);

        for r in 0..height {
            for c in 0..width {
                let layer_idx = r * width + c;
                let canvas_idx = (pos_y + r) * self.cols + (pos_x + c);

                if layer_vec[layer_idx] == layer.get_alpha_char() {
                    continue;
                }

                self.data[canvas_idx] = layer_vec[layer_idx];
            }
        }
    }

    pub fn print(&self, out: &mut impl Write){
        let mut iter = 0;
        for _c in 0..self.rows{
            for _r in 0..self.cols{
                write!(out, "{}", self.data[iter]).unwrap();
                iter += 1;
            }
            write!(out, "\r\n").unwrap();
        }
    }
}

// --- RENDERER --- //

pub struct Renderer {
    pub width: usize,
    pub height: usize,
}

impl Renderer {
    pub fn new(width: usize, height:usize) -> Self{
        return Self{
            width, 
            height,
        };
    }

    /// Returns true, if screen too small - TODO
    fn screen_too_small(&self) -> bool{
        let (width, height) = terminal::size().unwrap();
        if width < self.width as u16 || height < self.height as u16 {
            return true;
        }
        false
    }

    pub fn render(&self, layers: &[&dyn Layer]){
        let mut out = stdout();
        // execute!(stdout(), Clear(ClearType::All)).unwrap(); Causes flickering, instead write over
        queue!(out, cursor::MoveTo(0, 0)).unwrap(); // move to top instead of clear
    
        // Enforce Screen Size
        if self.screen_too_small() { 
            print!("Please resize your terminal to at least {}x{}\r\n", self.width, self.height);
            out.flush().unwrap();
            return;
        }

        // Update Buffer
        let mut buffer = FrameBuffer::new(self.width, self.height);
        for layer in layers {
            buffer.apply_layer(*layer);
        }

        buffer.print(&mut out);
        out.flush().unwrap();
    }
}