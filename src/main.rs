mod world;
mod layers;
mod systems;
mod scenes;

// use world::tree_grid;

// use std::vec;

// use crossterm::{execute, terminal::SetSize};
// use std::io::stdout;

// use std::fs;

use systems::renderer::Renderer;
use systems::context_stack::ContextStack;
use systems::runner::runner;


fn main() {
    let mut renderer = Renderer::new(100, 30);
    let mut context_stack = ContextStack::new(vec![]);
    runner(renderer, context_stack);
}