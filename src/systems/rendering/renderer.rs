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

use crate::systems::rendering::layer::Layer;


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

    pub fn apply_layer(&mut self, layer: &Layer){
        let layer_vec = layer.draw();
        assert!(layer_vec.len() <= self.data.len()); // TODO FIX THIS?
        for i in 0..layer_vec.len(){
            if layer_vec[i] == layer.alpha_char{
                continue;
            }
            self.data[i] = layer_vec[i];
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
        if width < 80 || height < 24 {
            return true;
        }
        false
    }

    pub fn render(&self, layers:&[Layer]){
        let mut out = stdout();
        // execute!(stdout(), Clear(ClearType::All)).unwrap(); Causes flickering, instead write over
        queue!(out, cursor::MoveTo(0, 0)).unwrap(); // move to top instead of clear
    
        // Enforce Screen Size
        if self.screen_too_small() { 
            print!("Please resize your terminal to at least 80x24\r\n");
            out.flush().unwrap();
            return;
        }

        // Update Buffer
        let mut buffer = FrameBuffer::new(self.width, self.height);
        for layer in layers{
            buffer.apply_layer(layer);
        }

        buffer.print(&mut out);
        out.flush().unwrap();
    }
}