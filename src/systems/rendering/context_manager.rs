use std::{collections::HashMap, hash::Hash};
use crate::systems::rendering::sheets::layer;
use std::rc::Rc;
use super::sheets::main_menu::MainMenu;
use super::sheets::layer::Layer;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    cursor,
    execute,
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode}
};

pub struct SceneContext{
    
}

impl SceneContext{
    
}

//

pub struct SceneContextMngr{
    scenes_store: HashMap<String, SceneContext>,
    layers_store: HashMap<String, Box<dyn Layer>>,
    curr_scene: String,
    // context_layers: Vec<layers>
}

impl SceneContextMngr{
    pub fn new() -> Self{

        let mut main_menu = MainMenu::new((20,10), (1, 1), '¿');

        let mut layers: HashMap<String, Box<dyn Layer>> = HashMap::new();
        let mut scenes: HashMap<String, SceneContext> = HashMap::new();
        
        layers.insert("main_menu".to_string(), Box::new(main_menu));

        Self{
            scenes_store: scenes,
            layers_store: layers,
            curr_scene: "main_menu".to_string(),
        }
    }

    pub fn set_context(&mut self, scene_name: &str){
        if let Some(scene) = self.scenes_store.get(scene_name){
            self.curr_scene = scene_name.to_string();
        }
    }



    pub fn handle_input(){

    }

    pub fn get_curr_scene_name(&self) -> String{
        return self.curr_scene.clone();
    }

    pub fn get_scene_layers(&self) -> Vec<&dyn Layer> {
        let mut layers = Vec::new();
        if let Some(scene) = self.layers_store.get("main_menu") {
            layers.push(scene.as_ref());
        }
        layers
    }
}