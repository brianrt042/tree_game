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
use crate::systems::rendering::layer::Layer;
use crate::systems::rendering::state_render::State_Render;

// UI LAYER
fn my_draw(state_render: State_Render) -> Vec<char> {
    let loc = state_render.y_loc * 3 + state_render.x_loc;
    let mut vector = vec!['-'; 9];
    vector[loc] = 'X';

    return vector;
}

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

fn handle_input(state_render: &mut State_Render) -> bool {
    if !event::poll(Duration::from_millis(0)).unwrap() {
        return false;
    }

    match event::read().unwrap() {
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => {
            match code {
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => return true,
                KeyCode::Char('q') => return true,
                KeyCode::Up    => { if state_render.y_loc > 0 { state_render.y_loc -= 1; } }
                KeyCode::Down  => { if state_render.y_loc < 2 { state_render.y_loc += 1; } }
                KeyCode::Left  => { if state_render.x_loc > 0 { state_render.x_loc -= 1; } }
                KeyCode::Right => { if state_render.x_loc < 2 { state_render.x_loc += 1; } }
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

pub fn runner(grid: &TreeGrid, render: impl Fn(&TreeGrid)) {
    initial_env();

    // FPS DURATION //
    let target_fps = 30;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    let renderer = renderer::Renderer::new(3, 3);
    let mut layers: Vec<Layer> = Vec::new();

    // UI TEST //
    let mut state_render: State_Render = State_Render{x_loc: 0, y_loc: 0};
    layers.push(Layer::new("TEST".to_string(), '-', 3, 3, my_draw, state_render));

    execute!(stdout(), Clear(ClearType::All)).unwrap();
    loop {
        let frame_start = Instant::now();

        // --- UPDATE ---
        if handle_input(&mut state_render) { // QUIT GAME
            terminal_reset_env();
            break;
        }

        for layer in layers.iter_mut(){
            layer.update_state(state_render);
        }

        // --- RENDER ---
        renderer.render(&layers);

        // --- SLEEP remaining frame time ---
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}