mod function;

use std::time::Duration;
use rand::*;
use minifb::*;
use grid::*;
use minifb::Key::P;
use minifb::MouseMode::Clamp;
use crate::function::{CreateWorld, SandPixel, SteelPixel};

const WorldSize: usize = 100;
const ScalingFactor: usize = 8;

fn main() {
    //Create Window
    let mut window2D = Window::new(
        "Pixel-World",
        WorldSize * ScalingFactor,
        WorldSize * ScalingFactor,
        WindowOptions::default(),
    )
        .expect("Unable to create window");
    window2D.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    let mut world = CreateWorld(WorldSize);
    let mut buffer: Vec<u32> = Vec::with_capacity(WorldSize*WorldSize);
    while window2D.is_open() && !window2D.is_key_down(Key::Escape) {
        buffer.clear();
        //Get mouse position
        let FloatPos = window2D.get_mouse_pos(Clamp).unwrap();
        let MousePos =((FloatPos.0 as u32) / ScalingFactor as u32, (FloatPos.1 as u32) / ScalingFactor as u32);
        let mut BoolKey= window2D.is_key_down(Key::F);
        println!("{:?}", MousePos);
        //Choose material
        let mut choice = 2;
        if BoolKey == true{
            *world.0.get_mut(MousePos.1 as usize, MousePos.0 as usize).unwrap() = choice;
        }
        //Per frame loop
        world.1 = Grid::new(WorldSize,WorldSize);
        for row in 0..WorldSize{
            for col in 0..WorldSize{
                match world.0.get(row,col){
                    Some(0) => {
                        buffer.push(0);
                    }
                    Some(1) => {
                        world = SteelPixel(row,col,world);
                        buffer.push(100000);
                    }
                    Some(2) => {
                        world = SandPixel(row,col,world);
                        buffer.push(637377);
                    }
                    Some(value) => {}
                    None => {}
                };

            }
        }
        world.0 = world.1;
        window2D.update_with_buffer(&buffer, WorldSize, WorldSize)
            .unwrap();
    };
}


/*
To-Do
Create classes for each pixel type

 */