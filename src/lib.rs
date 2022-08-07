use wee_alloc::WeeAlloc;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


pub struct SnakePart(usize);

struct Snake {
    body: Vec<SnakePart>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakePart(spawn_index - i));
        }

        Snake { 
            body,
            direction: Direction::Right
        }
    }
}

#[wasm_bindgen]
pub struct Board {
    width: usize,
    size: usize,
    snake: Snake
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down, 
    Right, 
    Left
}
    
#[wasm_bindgen]
impl Board {
    pub fn new(width: usize, spawn_index: usize) -> Board {
        Board { 
            width,
            size: width * width, 
            snake: Snake::new(spawn_index, 3)
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, dir: Direction) {
        self.snake.direction = dir
    }

    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_parts(&self) -> *const SnakePart {
        self.snake.body.as_ptr()
    }

    pub fn update(&mut self) {
        let snake_index = self.snake_head_index();
        
        let (row, col) = (snake_index / self.width, snake_index % self.width);

        let (row, col) = match self.snake.direction {
            Direction::Right => {
                (row, (col + 1) % self.width)
            },
            Direction::Left => {
                (row, (col - 1) % self.width)
            },
            Direction::Up => {
                ((row - 1) % self.width, col)
            },
            Direction::Down => {
                ((row + 1) % self.width, col)
            }
        };

        self.snake.body[0].0 = row * self.width + col;
    }
}