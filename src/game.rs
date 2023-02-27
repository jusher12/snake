use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const GAME_OVER: Color = [0.90, 0.00, 0.00, 0.50];

const MOVING_PERIOD: f64 = 0.1; //Game run speed
const RESTART_TIME: f64 = 1.0;  //Pause time after fail

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game{
    // New instance of a game
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None
        };
        // If key press is opposite direction of snake's curr direction
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return; // No action
        }
        self.update_snake(dir);
    }

    pub fn draw_board(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g); // Draw snake
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);                  // Top
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);    // Bottom
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);                 // Left
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);    // Right

        if self.game_over {
            draw_rectangle(GAME_OVER, 0, 0, self.width, self.height, con, g);
        }

    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {  // Restart if wait time is longer than restart time
                self.restart();
            }
            return;
        }

        if !self.food_exists {  // Add food if there is none
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD { // Update snake's position
            self.update_snake(None);
        }
    }

    pub fn check_eating(&mut self) { //Checks if snakes head overlaps food
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists == false;
            self.snake.restore_tail();
        }
    }

    pub fn check_if_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) { // Check if snake hits itself next move
            return false;
        }
        // Check if snake is in boundaries next move
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();
        
        let mut new_x = rng.gen_range(1..self.width); // New x coord
        let mut new_y = rng.gen_range(1..self.height); // New y coord
        while self.snake.overlap_tail(new_x, new_y) { // Update coords if they overlap snake
            let mut new_x = rng.gen_range(1..self.width);
            let mut new_y = rng.gen_range(1..self.height);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    pub fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        }
        else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2,2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}

