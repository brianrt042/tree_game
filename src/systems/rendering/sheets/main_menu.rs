use super::util::read_ascii_content;
use super::layer::Layer;

const POINT_CHAR: char = '∙';

#[derive(Clone)]
pub struct MainMenu{
    cursor_pos: usize,
    scale: (usize, usize),
    position: (usize, usize),
    alpha_char: char, 
}

impl MainMenu{
    pub fn new(scale: (usize, usize), position: (usize, usize), alpha_char: char) -> Self {
        Self{
            cursor_pos: 0,
            scale,
            position,
            alpha_char,
        }
    }
    pub fn cursor_up(&mut self){
        if self.cursor_pos == 0 {
            return;
        }
        self.cursor_pos -= 1;
    }
    pub fn cursor_down(&mut self){
        if self.cursor_pos == 2 {
            return;
        }
        self.cursor_pos += 1;
    }
    fn render_cursor(&self, draw_vector: &mut Vec<char>){
        let mut i = 0;
        for c in draw_vector.iter_mut(){
            if *c == POINT_CHAR{
                if(i == self.cursor_pos){
                    *c = 'X'
                }
                i += 1;       
            }
        }
    }
}

impl Layer for MainMenu{
    fn get_render(&self) -> Vec<char> {    
        let mut draw_vector = read_ascii_content(include_str!("main_menu.txt"), self.scale);
        self.render_cursor(&mut draw_vector);
        draw_vector
    }

    fn get_scale(&self) -> (usize, usize){
        self.scale
    }

    fn get_position(&self) -> (usize, usize){
        self.position        
    }
    fn get_alpha_char(&self) -> char{
        self.alpha_char
    }
}