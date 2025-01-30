use rand::{thread_rng, Rng};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Block {
    SnakeBody,
    SnakeHead,
    Empty,
    Food,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_valid_dir(&self) -> (Direction, Direction, Direction) {
        match self {
            Direction::Up => (Direction::Up, Direction::Left, Direction::Right),
            Direction::Down => (Direction::Down, Direction::Left, Direction::Right),
            Direction::Left => (Direction::Left, Direction::Up, Direction::Down),
            Direction::Right => (Direction::Right, Direction::Up, Direction::Down),
        }
    }

    pub fn get_emoji(&self) -> String {
        match self {
            Direction::Up => "⬆️".to_string(),
            Direction::Down => "⬇️".to_string(),
            Direction::Left => "⬅️".to_string(),
            Direction::Right => "➡️".to_string(),
        }
    }
}

pub struct Game {
    grid: [[Block; 6]; 6],
    // Snake vector is reversed where the head is the last element and the rest of the elements
    // comprise of the body.
    snake: Vec<(usize, usize)>,
    food: (usize, usize),
    score: u8,
    direction: Direction,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: [[Block::Empty; 6]; 6],
            score: 0,
            food: (0, 0),
            snake: vec![],
            direction: Direction::Up,
        }
    }

    pub fn setup(&mut self) {
        let mut rng = thread_rng();
        let mut coord = (0, 0);
        coord.0 = rng.gen_range(0..6);
        coord.1 = rng.gen_range(0..6);

        self.snake = vec![coord];
        self.score = 0;
        self.move_food();
    }

    fn update_grid(&mut self) {
        let mut flag = false;
        self.grid = [[Block::Empty; 6]; 6];

        self.snake.iter().rev().skip(1).for_each(|x| {
            if !flag {
                if self.grid[x.1][x.0] == Block::Empty || self.grid[x.1][x.0] == Block::Food {
                    self.grid[x.1][x.0] = Block::SnakeBody
                } else {
                    flag = true;
                }
            }
        });

        if !flag {
            self.grid[self.food.1][self.food.0] = Block::Food;
            if let Some(head) = self.snake.last() {
                self.grid[head.1][head.0] = Block::SnakeHead;
            } else {
                self.setup();
            }
        } else {
            self.setup();
        }
    }

    fn move_food(&mut self) {
        // Just so that current position of snake is updated onto the grid
        self.update_grid();

        let mut rng = thread_rng();
        self.food.0 = rng.gen_range(0..6);
        self.food.1 = rng.gen_range(0..6);
        while self.snake.contains(&self.food) {
            self.food.0 = rng.gen_range(0..6);
            self.food.1 = rng.gen_range(0..6);
        }

        self.update_grid();
    }

    pub fn move_snake(&mut self, direction: Direction) {
        //if let Some(i) = &self.direction {
            let valid_moves = &self.get_directions();
            let mut coord = *self.snake.last().unwrap();
            let mut flag = false;
            if direction == valid_moves.0
                || direction == valid_moves.1
                || direction == valid_moves.2
                || Some(direction) == valid_moves.3
            {
                // Change in y is reversed since a smaller y value means an array closer to the
                // top. Change in x is the still the same though.
                match direction {
                    Direction::Up => {
                        if coord.1 > 0 {
                            coord.1 -= 1;
                            self.direction = Direction::Up;
                        } else {
                            self.setup();
                            flag = true;
                        }
                    }
                    Direction::Down => {
                        if coord.1 < 4 {
                            coord.1 += 1;
                            self.direction = Direction::Down;
                        } else {
                            self.setup();
                            flag = true;
                        }
                    }
                    Direction::Left => {
                        if coord.0 > 0 {
                            coord.0 -= 1;
                            self.direction = Direction::Left;
                        } else {
                            self.setup();
                            flag = true;
                        }
                    }
                    Direction::Right => {
                        if coord.0 < 4 {
                            coord.0 += 1;
                            self.direction = Direction::Right;
                        } else {
                            self.setup();
                            flag = true;
                        }
                    }
                }
            } else {
                panic!("Direction is not valid.");
            }
            if !flag {
                self.snake.push(coord);

                if coord.1 != self.food.1 || coord.0 != self.food.0 {
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

    fn inc_score(&mut self) {
        self.score += 1;
    }

    pub fn get_directions(&self) -> (Direction, Direction, Direction, Option<Direction>) {
        if self.snake.len() == 1 {
            (
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Some(Direction::Right),
            )
        } else {
            let valid = self.direction.get_valid_dir();
            (valid.0, valid.1, valid.2, None)
        }
    }

    pub fn get_current_direction(&self) -> Direction {
        self.direction
    }
}
