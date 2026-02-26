#[derive(Debug, Clone)]
pub enum Space{
    Entrance, 
    SapRefinery,
    ShroomMill,
    HoneyComb,
    GrassPit,
    RockFroge,
    StoragePit,
    Empty,
}

#[derive(Debug, Clone)]
pub struct TreeGrid{
    width: usize,
    height: usize,
    tiles: Vec<Space>,
}


impl TreeGrid {
    // Create a new map filled with Empty tiles
    fn total_spaces(&self) -> usize{
        self.height * self.width
    }

    fn clear_console() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles: Vec<Space> = Vec::new();
        tiles.push(Space::Entrance);

        for _n in 1..(width*height){
            tiles.push(Space::Empty)
        }

        return TreeGrid { width, height, tiles }
    }


    pub fn pretty_print(&self) {
        TreeGrid::clear_console();

        for n in (0..(self.width * self.height)).rev(){
            print!("{:?} ", self.tiles[n]);
            if (self.total_spaces() - n) % 3 == 0{
                print!("\n");
            }
        }
    }

    // // Convert (x, y) to a flat index
    // fn index(&self, x: usize, y: usize) -> usize {
    //     y * self.width + x
    // }

    // pub fn get(&self, x: usize, y: usize) -> &Tile {
    //     &self.tiles[self.index(x, y)]
    // }

    // pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
    //     let i = self.index(x, y);
    //     self.tiles[i] = tile;
    // }
}