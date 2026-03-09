use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    cursor,
    execute,
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode}
};
use crate::systems::layer::Layer;

pub enum ContextAction {
    None,
    Push(Box<dyn SceneContext>),
    Pop,
    Quit,
}

pub trait SceneContext{
    fn handle_input(&mut self, key: Option<KeyCode>) -> ContextAction;
    fn update(&mut self);
    fn get_render(&self) -> Vec<&dyn Layer>;
    fn onPush(&mut self);
    fn onPop(&mut self);
} 
    
pub struct ContextStack{
    stack: Vec<Box<dyn SceneContext>>,
}

impl ContextStack{
    pub fn new(stack: Vec<Box<dyn SceneContext>>) -> Self{
        ContextStack { stack }
    }

    pub fn push(&mut self, mut scene_context: Box<dyn SceneContext>){
        scene_context.onPush();
        self.stack.push(scene_context);
    }
    pub fn pop(&mut self){
        if let Some(val) = self.stack.last_mut() {
            val.onPop();
        }
        self.stack.pop();
    }
    pub fn get_top(&mut self) -> Option<&Box<dyn SceneContext>> {
        self.stack.last()
    }
    pub fn get_top_mut(&mut self) -> Option<&mut Box<dyn SceneContext>> {
        self.stack.last_mut()
    }
    pub fn clear(&mut self){
        self.stack.clear();
    }
}