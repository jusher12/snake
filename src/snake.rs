use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use draw::draw_block;
// Snake color
const SNAKE_COLOR: Color = [0.00, 1.00, 0.00, 1.00];
// Directions of snake / Keyboard input.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Snake cannot go the opposite direction it
    // is currently going.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Struct for block type.
struct Block {
    x: i32,
    y: i32,
}
// Public struct for snake.
pub struct Snake {
   direction: Direction,
   body: LinkedList<Blcok>,
   tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y});
        body.push_back(Block { x: x + 1, y});
        body.push_back(Block {x, y});

        Snake { direction: Direction::Right, body, tail: None, }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {    
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }
}