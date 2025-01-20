use core::panic;

use rand::{Rng, thread_rng};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Block {
    SnakeBody,
    SnakeHead,
    Empty,
    Food,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_valid_dir(&self) -> (Direction, Direction, Direction) {
        match self {
            Direction::Up => (Direction::Up, Direction::Left, Direction::Right),
            Direction::Down => (Direction::Down, Direction::Left, Direction::Right),
            Direction::Left => (Direction::Left, Direction::Up, Direction::Down),
            Direction::Right => (Direction::Right, Direction::Up, Direction::Down),
        }
    }
}

pub struct Game {
    grid: [[Block; 5]; 5],
    // Snake vector is reversed where the head is the last element and the rest of the elements
    // comprise of the body.
    snake: Vec<(usize, usize)>,
    food: (usize, usize),
    score: u8,
    direction: Option<Direction>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: [[Block::Empty; 5]; 5],
            score: 0,
            food: (0, 0),
            snake: vec![],
            direction: None,
        }
    }

    pub fn setup(&mut self) {
        let mut rng = thread_rng();
        let mut coord = (0, 0);
        coord.0 = rng.gen_range(0..5);
        coord.1 = rng.gen_range(0..5);

        self.snake.push(coord);

        let mut coord = (0, 0);

        coord.0 = rng.gen_range(0..5);
        coord.1 = rng.gen_range(0..5);
        while coord == self.snake[0] {
            coord.0 = rng.gen_range(0..5);
            coord.1 = rng.gen_range(0..5);
        }

        self.update_grid();
    }

    fn update_grid(&mut self) {
        self.grid = [[Block::Empty; 5]; 5];
        self.grid[self.snake[self.snake.len() - 1].1][self.snake[self.snake.len() - 1].0] =
            Block::SnakeHead;
        self.snake
            .iter()
            .rev()
            .skip(1)
            .for_each(|x| self.grid[x.1][x.0] = Block::SnakeBody);
        self.grid[self.food.1][self.food.0] = Block::Food;
    }

    fn move_food(&mut self) {
        let mut rng = thread_rng();
        self.food.0 = rng.gen_range(0..5);
        self.food.1 = rng.gen_range(0..5);
        while self.grid[self.food.1][self.food.0] == Block::SnakeBody
            || self.grid[self.food.1][self.food.0] == Block::SnakeHead
        {
            self.food.0 = rng.gen_range(0..5);
            self.food.1 = rng.gen_range(0..5);
        }

        self.update_grid();
    }

    pub fn move_snake(&mut self, direction: Direction) {
        if let Some(i) = &self.direction {
            let valid_moves = i.get_valid_dir();
            let mut change_x = 0;
            let mut change_y = 0;
            if direction == valid_moves.0
                || direction == valid_moves.1
                || direction == valid_moves.2
            {
                // Change in y is reversed since a smaller y value means an array closer to the
                // top. Change in x is the still the same though.
                match direction {
                    Direction::Up => change_y -= 1,
                    Direction::Down => change_y += 1,
                    Direction::Left => change_x -= 1,
                    Direction::Right => change_x += 1,
                }
            } else {
                panic!("Direction is not valid.");
            }
            let mut coord = self.snake[self.snake.len() - 1];
            coord.1 += change_y;
            coord.0 += change_x;
            self.snake.push(coord);

            if coord.0 != self.food.0 || coord.1 != self.food.1 {
                self.snake.remove(0);
                self.update_grid();
            } else {
                self.inc_score();
                self.move_food();
            }
        } else {
            let mut change_x = 0;
            let mut change_y = 0;

            match direction {
                Direction::Up => change_y -= 1,
                Direction::Down => change_y += 1,
                Direction::Left => change_x -= 1,
                Direction::Right => change_x += 1,
            }

            let mut coord = self.snake[self.snake.len() - 1];
            coord.1 += change_y;
            coord.0 += change_x;
            self.snake.push(coord);

            if coord.0 != self.food.0 || coord.1 != self.food.1 {
                self.snake.remove(0);
                self.update_grid();
            } else {
                self.inc_score();
                self.move_food();
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::from("");
        for row in &self.grid {
            for block in row {
                match block {
                    Block::SnakeHead => string += "🟢",
                    Block::SnakeBody => string += "🟩",
                    Block::Empty => string += "⬛",
                    Block::Food => string += "🍎",
                }
            }
            string += "\n";
        }
        string
    }

    pub fn get_score(&self) -> u8 {
        return self.score;
    }

    pub fn inc_score(&mut self) {
        self.score += 1;
    }
}
