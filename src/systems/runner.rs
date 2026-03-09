use std::time::{Duration, Instant};
use std::io::stdout;
use std::thread;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    cursor,
    execute,
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode}
};

use crate::systems::layer::Layer;
use super::context_stack::ContextAction;
use super::renderer::Renderer;
use super::context_stack::ContextStack;
use super::context_stack::SceneContext;
use crate::scenes::main_menu::MainMenuScene;
use super::util::TerminalUtil;

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

pub fn runner(mut renderer: Renderer, mut context_stack: ContextStack) {
    let target_fps = 30;
    let frame_duration = Duration::from_millis(1000 / target_fps);

    TerminalUtil::set_env();
    TerminalUtil::clear();
    std::panic::set_hook(TerminalUtil::panic_hook());

    context_stack.push(Box::new(MainMenuScene::new()));

    loop {
        let frame_start = Instant::now();

        let action = match context_stack.get_top_mut() {
            Some(context) => {

                // -- UPDATE --
                context.update();

                // -- INPUT --
                let action = context.handle_input(grab_input());

                // -- RENDER --
                renderer.render(context.get_render());
                
                action
            }
            None => ContextAction::Quit
        };

        // -- SCENE TRANSITION --
        match action {
            ContextAction::Pop => {
                context_stack.pop(); 
            }
            ContextAction::Quit => {
                TerminalUtil::reset_env();
                break;
            }
            ContextAction::Push(new_context) => {
                context_stack.push(new_context);
            }
            ContextAction::None => {}
        }

        // --- SLEEP ---
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}