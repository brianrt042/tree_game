use crate::systems::layer::Layer;
use crate::systems::util::read_ascii_content;

const POINT_CHAR: char = '∙';
const ALPHA_CHAR: char = ' ';

const MAX_CUR_POS: usize = 3;

const SCALE:(usize, usize) = (20,12);
const POSITION:(usize, usize) = (0,0);

#[derive(Clone)]
pub struct MainMenu{
    pub cursor_pos: usize,
    cached_render: Vec<char>,  // parsed once, reused every frame
}

impl MainMenu{
    pub fn new() -> Self {
        let cached_render = read_ascii_content(include_str!("main_menu.txt"), SCALE);
        Self{
            cursor_pos: 0,
            cached_render,
        }
    }
    
    fn render_cursor(&self, draw_vector: &mut Vec<char>){
        let mut i = 0;
        for c in draw_vector.iter_mut(){
            if *c == POINT_CHAR{
                if i == self.cursor_pos{
                    *c = 'X'
                }
                i += 1;       
            }
        }
    }

    

    pub fn get_max_pos(&self) -> usize{
        MAX_CUR_POS
    }
}

impl Layer for MainMenu{
    fn get_render(&self) -> Vec<char> {    
        let mut draw_vector = self.cached_render.clone();
        self.render_cursor(&mut draw_vector);
        draw_vector
    }

    fn get_scale(&self) -> (usize, usize){
        SCALE
    }

    fn get_position(&self) -> (usize, usize){
        POSITION      
    }
    fn get_alpha_char(&self) -> char{
        ALPHA_CHAR
    }
}