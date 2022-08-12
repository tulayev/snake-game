use wee_alloc::WeeAlloc;
use wasm_bindgen::prelude::*;


#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


#[wasm_bindgen(module = "/static/utils/random.js")]
extern {
    fn random(size: usize) -> usize;
}

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

#[derive(PartialEq, Clone, Copy)]
pub struct SnakePart(usize);

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Win, 
    Loose,
    Playing
}

#[wasm_bindgen]
pub struct Board {
    width: usize,
    size: usize,
    snake: Snake,
    next_part: Option<SnakePart>,
    rat: Option<usize>,
    status: Option<GameStatus>,
    points: usize
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
        let snake = Snake::new(spawn_index, 3);
        let size = width * width;
        
        Board { 
            width,
            size, 
            rat: Board::generate_rat(size, &snake.body),
            snake,
            next_part: Option::None,
            status: Option::None,
            points: 0
        }
    }

    fn generate_rat(size: usize, snake_body: &Vec<SnakePart>) -> Option<usize> {
        let mut rat;

        loop {
            rat = random(size);

            if !snake_body.contains(&SnakePart(rat)) {
                break;
            }
        }

        Option::Some(rat)
    }

    pub fn start_game(&mut self) {
        self.status = Option::Some(GameStatus::Playing)
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Option::Some(GameStatus::Win) => String::from("Siz yutdingiz!"),
            Option::Some(GameStatus::Loose) => String::from("Siz yutqazdingiz!"),
            Option::Some(GameStatus::Playing) => String::from("O'ynalmoqda"),
            Option::None => String::from("")
        }
    }
    
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn get_rat(&self) -> Option<usize> {
        self.rat
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, dir: Direction) {
        let next_part = self.get_next_part(&dir);

        if self.snake.body[1].0 == next_part.0 {
            return;
        }

        self.next_part = Option::Some(next_part);
        self.snake.direction = dir;
    }

    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_parts(&self) -> *const SnakePart {
        self.snake.body.as_ptr()
    }

    pub fn update(&mut self) {
        match self.status {
            Some(GameStatus::Playing) => {
                let temp = self.snake.body.clone();

                match self.next_part {
                    Option::Some(part) => {
                        self.snake.body[0] = part;
                        self.next_part = Option::None;
                    },
                    Option::None => {
                        self.snake.body[0] = self.get_next_part(&self.snake.direction);
                    }
                }

                for i in 1..self.snake.body.len() {
                    self.snake.body[i] = SnakePart(temp[i - 1].0);
                }

                if self.snake.body[1..self.snake_len()].contains(&self.snake.body[0]) {
                    self.status = Option::Some(GameStatus::Loose);
                }

                if self.rat == Option::Some(self.snake_head_index()) {
                    if self.snake_len() < self.size {
                        self.points += 1;
                        self.rat = Board::generate_rat(self.size, &self.snake.body);
                    } else {
                        self.rat = Option::None;
                        self.status = Option::Some(GameStatus::Win);
                    }

                    self.snake.body.push(SnakePart(self.snake.body[1].0));
                }
            },
            _ => {}
        }
    }

    fn get_next_part(&self, dir: &Direction) -> SnakePart {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width; 

        return match dir {
            Direction::Right => {
                SnakePart(row * self.width + (snake_index + 1) % self.width)
            },
            Direction::Left => {
                SnakePart(row * self.width + (snake_index - 1) % self.width)
            },
            Direction::Up => {
                SnakePart((snake_index - self.width) % self.size)
            },
            Direction::Down => {
                SnakePart((snake_index + self.width) % self.size)
            }
        };
    }
}