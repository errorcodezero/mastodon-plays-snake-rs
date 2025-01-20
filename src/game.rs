use rand::{Rng, thread_rng};

#[derive(Copy, Clone)]
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

        self.food = coord;

        self.update_grid();
    }

    fn update_grid(&mut self) {
        self.grid = [[Block::Empty; 5]; 5];
        self.grid[self.snake[0].1][self.snake[0].0] = Block::SnakeHead;
        self.snake
            .iter()
            .skip(1)
            .for_each(|x| self.grid[x.1][x.0] = Block::SnakeBody);
        self.grid[self.food.1][self.food.0] = Block::Food;
    }

    pub fn move_snake(&mut self, direction: Direction) {
        if let Some(i) = &self.direction {
            let valid_moves = i.get_valid_dir();
            if direction == valid_moves.0 || direction == valid_moves.1 || direction == valid_moves.2 {
                match direction {
                    Direction::Up => {},
                    Direction::Down => {}
                    Direction::Left => {}
                    Direction::Right => {}
                }
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
