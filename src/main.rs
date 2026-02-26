mod world;
mod systems;

use world::tree_grid;
use systems::runner::runner;
use std::vec;

use crossterm::{execute, terminal::SetSize};
use std::io::stdout;

use std::fs;

fn main() {
    let mut grid = tree_grid::TreeGrid::new(3, 3); // IGNORE, NOT NEEDED //
    runner(&grid, tree_grid::TreeGrid::pretty_print);

}