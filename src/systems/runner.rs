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

fn handle_input(main_menu: &mut MainMenu) -> bool {
    if !event::poll(Duration::from_millis(0)).unwrap() {
        return false;
    }

    match event::read().unwrap() {
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => {
            match code {
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => return true,
                KeyCode::Char('q') => return true,
                KeyCode::Up    => { main_menu.cursor_up(); }
                KeyCode::Down  => { main_menu.cursor_down(); }
                KeyCode::Left  => {  }
                KeyCode::Right => {  }
                _ => {}
            }
        }
        Event::Resize(_, _) => {
            execute!(stdout(), Clear(ClearType::All)).unwrap();
        }
        _ => {}
    }
    false
}

pub fn runner() {
    initial_env();

    // FPS DURATION //
    let target_fps = 30;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    let renderer = renderer::Renderer::new(100, 30);
    

    // UI TEST //
    let mut main_menu = Box::new(MainMenu::new((20,10), (1, 1), '¿'));


    execute!(stdout(), Clear(ClearType::All)).unwrap();
    loop {
        let frame_start = Instant::now();

        // --- UPDATE ---
        if handle_input(&mut main_menu) { // QUIT GAME
            terminal_reset_env();
            break;
        }

        // for layer in layers.iter_mut(){
        //     layer.update_state(state_render);
        // }

        // --- RENDER ---
        let layers: Vec<&dyn Layer> = vec![main_menu.as_ref()];  // borrow, not move
        renderer.render(&layers);

        // --- SLEEP remaining frame time ---
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}