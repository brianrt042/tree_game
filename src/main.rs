mod world;
mod systems;

use world::tree_grid;
use systems::runner::runner;
use std::vec;

use crossterm::{execute, terminal::SetSize};
use std::io::stdout;

use std::fs;

fn main() {
    runner();
}