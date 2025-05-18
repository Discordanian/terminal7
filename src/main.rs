use std::fmt;

#[derive(Copy, Clone, PartialEq)]
struct Cell {
    value: u8,
    mystery: u8,
    destroy: bool,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.mystery > 0 {
            match self.mystery {
                2 => write!(f, " X "),
                _ => write!(f, " x "),
            }
        } else {
            if self.destroy {
                write!(f, "*{}*", self.value)
            } else {
                write!(f, " {} ", self.value)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum GameState {
    Stable,
    Analyzing,
    Exploding,
    Falling,
    End,
}

impl fmt::Display for GameGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut retval = String::new();
        for row in 0..8 {
            if row == 0 {
                retval.push_str("   ");
            } else {
                retval.push_str(format!("{}  ", 8 - row).as_str());
            }
            for col in 0..7 {
                match self.board[row][col] {
                    Some(c) => retval.push_str(format!("{:?}", c).as_str()),
                    None => retval.push_str("   "),
                }
            }
            retval.push('\n');
        }
        retval.push_str("   [1][2][3][4][5][6][7]\n");
        write!(f, "{}", retval)
    }
}

impl fmt::Debug for GameGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let retval = format!("Game State: {:?}\n{}", self.state, self);
        write!(f, "{}", retval)
    }
}

impl GameGrid {
    pub fn init(&mut self) {
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                self.board[row][col] = None;
            }
        }
        self.board[3][3] = Some(Cell {
            value: 2,
            mystery: 0,
            destroy: false,
        });
        self.board[7][3] = Some(Cell {
            value: 3,
            mystery: 2,
            destroy: false,
        });
        self.board[1][5] = Some(Cell {
            value: 6,
            mystery: 0,
            destroy: false,
        });
        self.state = GameState::Stable;
    }
    pub fn next(&mut self) {
        match self.state {
            GameState::Falling => self.fall(),
            GameState::Analyzing => self.analyze(),
            GameState::Exploding => self.explode(),
            GameState::Stable => self.prompt(),
            GameState::End => {}
        }
    }
    fn fall(&mut self) {
        println!("Current State {:?}", self.state);
        let mut fell = false;
        for r in 0..self.board.len() {
            for c in 0..self.board[r].len() {
                match self.board[r][c] {
                    Some(cell) => {
                        if (r + 1) < self.board.len() && self.board[r + 1][c] == None {
                            // swap current cell with None cell and mark fell true
                            fell = true;
                            self.board[r + 1][c] = Some(cell);
                            self.board[r][c] = None;
                        }
                    }
                    None => {}
                }
            }
        }

        if !fell {
            // Nothing fell so change state
            self.state = GameState::Analyzing;
        }
    }
    fn height(&self, col: usize) -> u8 {
        // Count from current cell from bottom until 'None'.
        let mut retval = 0;
        let last_row = self.board.len() - 1;
        for r in 0..self.board.len() {
            if self.board[last_row - r][col] == None {
                return retval;
            }
            retval += 1;
        }
        retval
    }
    fn analyze(&mut self) {
        println!("Current State {:?}", self.state);
        for r in 0..self.board.len() {
            for c in 0..self.board[r].len() {
                let h = self.height(c);
                //println!("(row,col,height) -> ({},{},{})", r, c, h);
                match self.board[r][c] {
                    Some(cell) => {
                        if cell.value == h {
                            println!("Cell at ({},{}) is set to explode", r, c);
                            // cell.destroy = true;
                            self.board[r][c] = Some(Cell {
                                value: cell.value,
                                mystery: cell.mystery,
                                destroy: true,
                            });
                        }
                    }
                    None => {}
                }
            }
        }
        self.state = GameState::Exploding;
    }
    fn explode(&mut self) {
        println!("Current State {:?}", self.state);
        self.state = GameState::Stable;
    }
    fn prompt(&mut self) {
        println!("Current State {:?}", self.state);
        println!("Prompt");
    }
}

struct GameGrid {
    board: [[Option<Cell>; 7]; 8],
    state: GameState,
}

fn main() {
    let grid: [[Option<Cell>; 7]; 8] = [[None; 7]; 8];
    let mut game = GameGrid {
        board: grid,
        state: GameState::End,
    };
    game.init();
    println!("{:?}", game);
    game.state = GameState::Falling;
    game.next();
    game.next();
    println!("{:?}", game);
    game.next();
    println!("Final Game State\n{:?}", game);
}
