pub trait Layer{
    fn get_render(&self) -> Vec<char>;
    fn get_position(&self) -> (usize, usize);
    fn get_scale(&self) -> (usize, usize);
    fn get_alpha_char(&self) -> char;
}