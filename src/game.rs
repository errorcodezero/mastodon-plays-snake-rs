#[derive(Copy, Clone)]
pub enum Block {
    SnakeBody,
    SnakeHead,
    Empty,
    Food
}

pub struct Game {
    grid: [[Block; 5]; 5]
}

impl Game {
    pub fn new() -> Self {
        Self { grid: [[Block::Empty; 5]; 5]}
    }

    pub fn to_string(&self) -> String {
        let mut string = String::from("");
        for row in &self.grid {
            for block in row {
                match block {
                    Block::SnakeHead => string += "🟢",
                    Block::SnakeBody => string += "🟩",
                    Block::Empty => string += "⬛",
                    Block::Food => string += "🍎"
                }
            }
            string += "\n";
        }
        string
    }

    pub fn put(&mut self, column: usize, row: usize, block: Block) {
        self.grid[column][row] = block; 
    }
}
