#![cfg_attr(not(test), no_std)]

use core::panic;

use bare_metal_modulo::{MNum, ModNumC, ModNumIterator};
use num::traits::SaturatingAdd;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{
    is_drawable, plot, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH,
};

/* GAME IDEA: SNAKE GAME
   -- Draw a Snake
   -- Draw food
   -- if Snake collides with food, it  grows (increase the snake's length)
   -- Increase the score from 0 (increment it by 1) and the score is displayed on the screen
   -- If the Snake hits the wall, the game ends and the user fails.
   __ If the Snake hits itself, the game ends as well.
   -- I need to know how to get the snake  and food on the qemu, move the snake, and put food on a different location.
   -- Press R to restart when the game ends,
   -- Use the UP, DOWN, LEFT AND RIGHT keys to move the snake.
   --
   __ That's the idea of the game - the normal snake game we know
*/

// Constants
pub const ROWS: usize = 25;
pub const COLS: usize = 80;
pub const BLOCK_SIZE: usize = 2;
pub const MAX_SNAKE_LENGTH: usize = ROWS * COLS / (BLOCK_SIZE * BLOCK_SIZE);

// Colors
const COLOR_BACKGROUND: Color = Color::Black;
const COLOR_SNAKE: Color = Color::LightGreen;
const COLOR_FOOD: Color = Color::Red;

#[derive(Copy, Debug, Clone, Eq, PartialEq)]
// Directions
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// Game state
pub struct GameState {
    pub snake: [(u16, u16); MAX_SNAKE_LENGTH],
    pub length: usize,
    pub direction: Direction,
    pub food: (u16, u16),
    pub score: usize,
    // col: ModNumC<usize, BUFFER_WIDTH>,
    // row: ModNumC<usize, BUFFER_HEIGHT>,
    // dx: ModNumC<usize, BUFFER_WIDTH>,
    // dy: ModNumC<usize, BUFFER_HEIGHT>
}

impl GameState {
    pub fn new() -> GameState {
        let mut snake = [(0, 0); MAX_SNAKE_LENGTH];
        snake[0] = (ROWS as u16 / 2, COLS as u16 / 2);

        GameState {
            snake,
            length: 10,
            direction: Direction::Right,
            food: (0, 0),
            score: 0,
        }
    }

    pub fn update_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn update(&mut self) {
        // Move snake
        let head = self.snake[0];
        let mut new_head = head;
        match self.direction {
            Direction::Up => new_head.0 = new_head.0.wrapping_sub(BLOCK_SIZE as u16),
            Direction::Down => new_head.0 = new_head.0.wrapping_add(BLOCK_SIZE as u16),
            Direction::Left => new_head.1 = new_head.1.wrapping_sub(BLOCK_SIZE as u16),
            Direction::Right => new_head.1 = new_head.1.wrapping_add(BLOCK_SIZE as u16),
        }
        self.snake.rotate_right(1);
        self.snake[0] = new_head;

        // Check for collision with wall border
        if new_head.0 >= ROWS as u16 || new_head.1 >= COLS as u16 {
            panic!("Game Over!");
        }

        // Check for collision with food, if the snake's head collides with food, the snake's lengh increases and the score increases.
        if new_head == self.food {
            self.score += 1;
            self.length += 1;
            self.spawn_food();
        }

        // Check for collision with self, if the snake collide with itself, the game ends
        for i in 1..self.length {
            if new_head == self.snake[i] {
                panic!("Game Over!");
            }
        }
    }

    // Return true if snake is occupying a given position, else return false.
    fn is_snake_on_position(&self, position: (u16, u16)) -> bool {
        for &(row, col) in &self.snake {
            if row == position.0 && col == position.1 {
                return true;
            }
        }
        false
    }

    /* In the spawn_food function we use the current food position as a starting point for generating the new food position.
    We add 1 to both the row and column of the current position (with wrapping) to get the initial new position.
    We then enter a loop and keep incrementing the row and column of the new position (again with wrapping)
    until we find a position that's not occupied by the snake.
    Once we've found a valid position, we update the food field in the game state and return the new food position.*/

    pub fn spawn_food(&mut self) -> (u16, u16) {
        let mut new_food = (
            (self.food.0 + 1) % ROWS as u16,
            (self.food.1 + 1) % COLS as u16,
        );
        while self.is_snake_on_position(new_food) {
            new_food = (
                (new_food.0 + 1) % ROWS as u16,
                (new_food.1 + 1) % COLS as u16,
            );
        }
        self.food = new_food;
        new_food
    }

    pub fn draw_snake(&self, buffer: &mut [u8]) {
        for &(row, col) in self.snake.iter().take(self.length) {
            plot(
                '*',
                col as usize,
                row as usize,
                ColorCode::new(COLOR_SNAKE, COLOR_BACKGROUND),
            );
            /*
            let start_idx = ((row as usize) * COLS + (col as usize)) * BLOCK_SIZE;
            for i in 0..BLOCK_SIZE {
                for j in 0..BLOCK_SIZE {
                    let idx = start_idx + (i * COLS + j) * 4;
                    buffer[idx..idx+4].copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
                }
            }
            */
        }
    }

    // In this function, I want to let the user type 'R' to restart the game when the game is over.
    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(KeyCode::R) | DecodedKey::Unicode('r') => {}
            _ => {}
        }
    }
}
