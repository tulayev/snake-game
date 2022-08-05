use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;


#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Board {
    width: usize,
    snake: Snake
}

struct Snake {
    body: Vec<SnakeCell>
}

struct SnakeCell(usize);

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        Board { 
            width: 16, 
            snake: Snake::new(18)
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake { 
            body: vec!(SnakeCell(spawn_index))
        }
    }
}