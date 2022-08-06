use wee_alloc::WeeAlloc;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


#[derive(PartialEq)]
enum Direction {
    Up,
    Down, 
    Right, 
    Left
}

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

struct SnakeCell(usize);

#[wasm_bindgen]
pub struct Board {
    width: usize,
    size: usize,
    snake: Snake
}
    
#[wasm_bindgen]
impl Board {
    pub fn new(width: usize, spawn_index: usize) -> Board {
        Board { 
            width,
            size: width * width, 
            snake: Snake::new(spawn_index)
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
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

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake { 
            body: vec!(SnakeCell(spawn_index)),
            direction: Direction::Up
        }
    }
}