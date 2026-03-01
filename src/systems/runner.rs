use std::time::{Duration, Instant};
use std::io::stdout;
use std::thread;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    cursor,
    execute,
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode}
};

use crate::world::tree_grid::TreeGrid;
use crate::systems::rendering::renderer;
use crate::systems::rendering::sheets::layer::Layer;
use crate::systems::rendering::sheets::main_menu::MainMenu;
use crate::systems::rendering::state_render::State_Render;
use super::rendering::context_manager::SceneContext;
use super::rendering::context_manager::SceneContextMngr;

// SETTING TERMINAL SHIT //

fn terminal_set_env(){
    execute!(stdout(), cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();
}
fn terminal_reset_env(){
    execute!(stdout(), cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}

// RUNNER //

fn initial_env(){
    std::panic::set_hook(Box::new(|info| {
        terminal_reset_env();
        eprintln!("{}", info);
    }));
    terminal_set_env()
}

fn grab_input() -> Option<KeyCode> {
    if !event::poll(Duration::from_millis(0)).unwrap() {
        return None;
    }

    match event::read().unwrap() {
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) => Some(code),
        Event::Resize(_, _) => {
            execute!(stdout(), Clear(ClearType::All)).unwrap();
            None
        }
        _ => None,
    }
}

pub fn runner() {
    initial_env();

    // FPS DURATION //
    let target_fps = 30;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    let renderer = renderer::Renderer::new(100, 30);
    let scene_context_mngr = SceneContextMngr::new();


    execute!(stdout(), Clear(ClearType::All)).unwrap();
    loop {
        let frame_start = Instant::now();

        // --- UPDATE --- 
        let key_input = grab_input();
        if let Some(key) = key_input {
            match key {
                KeyCode::Char('c') | KeyCode::Char('q') => {
                    // QUIT
                    terminal_reset_env();
                    break;
                }
                _ => {}
            }
        }


        // SCENE_MANAGER.update_on_tick
            // vvv HIDDEN IN IMPL vvv
            // 
        // get_input
            // if quit... quit? (for now at least)
            
        // SCENE_MANAGER.current_scene.handle_input

        // --- RENDER ---
        
        // LAYERS are constant?
        // SCENE_MANAGER dynamically adds references to a layer, and returns
            // Get scene layers
        
        let layers_to_render = scene_context_mngr.get_scene_layers();
        renderer.render(layers_to_render);

        // --- SLEEP remaining frame time ---
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}