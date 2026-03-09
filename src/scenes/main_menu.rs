use crossterm::{ event::{KeyCode} };

use crate::systems::context_stack::SceneContext;
use crate::systems::context_stack::ContextAction;
use crate::systems::layer::Layer;
use crate::layers::main_menu::MainMenu;

const TEST: usize = 0;
const PLAY: usize = 1;
const SETTINGS: usize = 2;
const QUIT: usize = 3;

pub struct MainMenuScene{
    main_menu: MainMenu,
}

impl MainMenuScene{
    pub fn new() -> Self{
        MainMenuScene { main_menu: MainMenu::new() }
    }

    fn up(&mut self){
        if self.main_menu.cursor_pos == 0{
            return;
        }
        self.main_menu.cursor_pos -= 1;
    }

    fn down(&mut self){
        if self.main_menu.cursor_pos == self.main_menu.get_max_pos(){
            return;
        }
        self.main_menu.cursor_pos += 1;
    }

    fn enter(&mut self) -> ContextAction{
        match self.main_menu.cursor_pos {
            TEST => ContextAction::Push(Box::new(MainMenuScene::new())),
            PLAY => ContextAction::None,
            SETTINGS => ContextAction::None,
            QUIT => ContextAction::Pop, // quit
            _ => ContextAction::None,
        }
    }
}

impl SceneContext for MainMenuScene{
    fn handle_input(&mut self, key: Option<crossterm::event::KeyCode>) -> ContextAction {
        if let Some(key_val) = key {
            match key_val {
                KeyCode::Char('q') => {
                    return ContextAction::Pop
                }
                KeyCode::Up => {self.up();}
                KeyCode::Down => {self.down();}
                KeyCode::Enter => {return self.enter()}
                _ => {}
            }
        }
        return ContextAction::None;
    }

    fn get_render(&self) -> Vec<&dyn Layer> {
        vec![&self.main_menu]
    }

    fn update(&mut self) {
        // Do nothing
    }

    fn onPop(&mut self) {
        // Also do nothing, empty stack should quit game
    }

    fn onPush(&mut self) {
        // Nope
    }
}