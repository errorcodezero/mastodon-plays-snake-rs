use rand::{Rng, rng};

const SIZE: usize = 6;

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
    fn as_str(&self) -> &str {
        match self {
            Direction::Up => "u",
            Direction::Down => "d",
            Direction::Left => "l",
            Direction::Right => "r",
        }
    }

    fn import_str(&mut self, string: &str) {
        let dir = match string {
            "u" => Some(Direction::Up),
            "d" => Some(Direction::Down),
            "l" => Some(Direction::Left),
            "r" => Some(Direction::Right),
            _ => None,
        };

        if let Some(i) = dir {
            *self = i;
        }
    }

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
    grid: [[Block; SIZE]; SIZE],
    // Snake vector is reversed where the head is the last element and the rest of the elements
    // comprise of the body.
    snake: Vec<(usize, usize)>,
    food: (usize, usize),
    score: u8,
    high_score: u8,
    direction: Direction,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: [[Block::Empty; SIZE]; SIZE],
            score: 0,
            high_score: 0,
            food: (0, 0),
            snake: vec![],
            direction: Direction::Up,
        }
    }

    pub fn setup(&mut self) {
        let mut rng = rng();
        let mut coord = (0, 0);
        coord.0 = rng.random_range(0..SIZE);
        coord.1 = rng.random_range(0..SIZE);

        self.snake = vec![coord];
        self.score = 0;
        self.move_food();
    }

    fn update_grid(&mut self) {
        let mut flag = false;
        self.grid = [[Block::Empty; SIZE]; SIZE];

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

        let mut rng = rng();
        self.food.0 = rng.random_range(0..SIZE);
        self.food.1 = rng.random_range(0..SIZE);
        while self.snake.contains(&self.food) {
            self.food.0 = rng.random_range(0..SIZE);
            self.food.1 = rng.random_range(0..SIZE);
        }

        self.update_grid();
    }

    pub fn move_snake(&mut self, direction: Direction) {
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
                    if coord.1 < SIZE - 1 {
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
                    if coord.0 < SIZE - 1 {
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
                self.score += 1;
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                self.move_food();
            }
        }
    }

    #[allow(clippy::style)]
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
        string += "\n";
        string += "Score: ";
        string += &self.score.to_string();

        string
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

    pub fn create_backup(&self) -> String {
        let mut string = "".to_string();
        string += self.direction.as_str();
        string += ",";
        string += format!("{},{}", self.food.0, self.food.1).as_str();
        self.snake
            .iter()
            .for_each(|x| string += format!(",{},{}", x.0, x.1).as_str());

        string
    }

    pub fn import(&mut self, backup: String) {
        self.setup();
        self.snake = vec![];

        let mut ptr = &mut self.food.0;

        let iter = backup.split(",");

        for (i, x) in iter.enumerate() {
            if i == 0 {
                self.direction.import_str(x);
            } else if i < 3 {
                *ptr = x.parse().unwrap();
                ptr = &mut self.food.1;
            } else if i % 2 == 1 {
                self.snake.push((0, 0));
                self.snake.last_mut().unwrap().0 = x.parse().unwrap();
            } else {
                self.snake.last_mut().unwrap().1 = x.parse().unwrap();
            }
        }

        self.score = self.snake.len() as u8 - 1;
        self.update_grid();
    }

    pub fn get_highscore(&self) -> u8 {
        self.high_score
    }

    pub fn import_highscore(&mut self, score: u8) {
        self.high_score = score;
    }
}
