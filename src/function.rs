use rand::*;
use minifb::*;
use grid::*;

pub fn CreateWorld(Size: usize) -> (Grid <u8>, Grid <u8>){
    let world1 = Grid::new(Size, Size);
    let world2 = Grid::new(Size, Size);
    return (world1, world2);
}

pub fn SteelPixel(row: usize, col: usize, mut world:(Grid<u8>, Grid<u8>)) -> (Grid<u8>, Grid<u8>){
    *world.1.get_mut(row,col).unwrap() = 1;
    return world;
}
pub fn SandPixel(row:usize, col: usize, mut world:(Grid<u8>, Grid<u8>)) -> (Grid<u8>, Grid<u8>){
    if world.0.get(row + 1,col).unwrap() == &0{
        *world.1.get_mut(row,col).unwrap() = 0;
        *world.1.get_mut(row + 1,col).unwrap() = 2
    }
    return world;
}