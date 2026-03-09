// use super::layer::Layer;

// #[derive(Clone)]
// pub struct TestLayer {
//     pub name: String, 
//     pub alpha_char: char, 
//     row: usize, 
//     col: usize, 
//     draw: fn(state_Render: State_Render) -> Vec<char>,
//     state_render: State_Render,
// }

// impl TestLayer{
//     pub fn new(name: String, 
//         alpha_char: char,
//         col: usize,
//         row: usize,
//         draw: fn(state_Render: State_Render) -> Vec<char>,
//         state_render: State_Render) -> Self{
            
//         Self { name, alpha_char, row, col, draw, state_render }
//     }
    
//     pub fn update_state(&mut self, state_render: State_Render){
//         self.state_render = state_render;
//     }
// }

// impl TestLayer for Layer{
//     fn draw(&self) -> Vec<char>{
//         (self.draw)(self.state_render)
//     }
// }