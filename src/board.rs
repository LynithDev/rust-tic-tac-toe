use crate::{player::Player, utils::read_input};

pub struct Board {
    size: u8,
    in_a_row: u8,
    current_player: Player,
    items: Vec<(usize, usize, Player)>,
    game_over: bool,
}

impl Board {
    pub fn create(size: u8, in_a_row: u8, starting_player: Player) -> Board {
        Board {
            size,
            in_a_row,
            current_player: starting_player,
            items: Vec::new(),
            game_over: false
        }
    }

    pub fn start(&mut self) {
        self.draw();
        while !self.game_over {
            self.do_move();
            self.draw();
        }
        println!("Player {} has won the game!", self.current_player.to_string());
    }

    fn game_over(&mut self, player: Player) {
        self.game_over = true;
    }

    fn do_move(&mut self) {
        loop {
            let input;
            println!("Player {}'s turn", self.current_player.to_string());
            input = read_input("Place at (Format: 'x y'):  ");
            let split = input.split_once(" ");

            if split.is_none() { 
                println!("Invalid format. Format is 'x y' where x and y are numbers.");
                continue;
            }

            let a = if let Ok(i) = str::parse::<usize>(split.unwrap().0) {
                i
            } else {
                println!("Invalid format. Format is 'x y' where x and y are numbers.");
                continue;
            };

            let b = if let Ok(i) = str::parse::<usize>(split.unwrap().1) {
                i
            } else {
                println!("Invalid format. Format is 'x y' where x and y are numbers.");
                continue;
            };

            if a > self.size as usize || b > self.size as usize {
                println!("Out of bounds! Please choose a place on the board");
                continue;
            }

            if !self.place_at(a as u8, b as u8, self.current_player) {
                println!("Tile is taken! Please try a different tile player {}.", self.current_player.to_string());
                continue;
            };

            if let Some(player) = self.check_condition(a, b) {
                self.game_over(player);
                break;
            };
            self.next_player();
            break;
        };
    }

    fn next_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            _ => Player::X
        }
    }

    fn get_item(&self, row: u8, column: u8) -> Player {
        for (x, y, player) in self.items.to_owned() {
            if x == row.into() && y == column.into() {
                return player.to_owned();
            }
        };
        return Player::None;
    }

    fn draw(&self) {
        print!("\x1b[2J");
        print!("\n  {}  ", " ".repeat(1));
        for i in 0..self.size {
            print!("  {} ", i);
        }
        println!("");

        for row in 0..self.size {
            let mut r = String::new();
            r += format!("  {}  ", " ".repeat(row.to_string().len())).as_str();
            r += "----".repeat(self.size.into()).as_str();
            r += "-\n";
            r += format!("  {row}  ").as_str();
            for colum in 0..self.size {
                r += format!("| {} ", self.get_item(colum, row).to_string()).as_str()
            }
            println!("{}|", r);
        }
        println!("  {}  {}-", " ".repeat(1), "----".repeat(self.size.into()).as_str());
        println!("");
    }

    fn place_at(&mut self, row: u8, column: u8, player: Player) -> bool {
        if self.get_item(row, column) == Player::None {
            self.items.push((row.into(), column.into(), player));
            true
        } else {
            false
        }
    }

    fn check_condition(&self, x: usize, y: usize) -> Option<Player> {
        // WARNING: Messy code up ahead!

        // Check Row
        let mut row = 0;
        for i in 0..self.size {
            if self.get_item(i, y as u8) == self.current_player {
                row += 1;
            } else {
                row = 0;
            }
            if row == self.in_a_row {
                return Some(self.current_player);
            }
        }

        // Check Column
        let mut column = 0;
        for i in 0..self.size {
            if self.get_item(x as u8, i) == self.current_player {
                column += 1;
            } else {
                column = 0;
            }
            if column == self.in_a_row {
                return Some(self.current_player);
            }
        }

        // Check Diagonal
        let mut diagonal = 0;
        for i in 0..self.size {
            if self.get_item(i, i) == self.current_player {
                diagonal += 1;
            } else {
                diagonal = 0;
            }
            if diagonal == self.in_a_row {
                return Some(self.current_player);
            }
        }

        // Check Anti-Diagonal
        let mut anti_diagonal = 0;
        for i in 0..self.size {
            if self.get_item(i, self.size - i - 1) == self.current_player {
                anti_diagonal += 1;
            } else {
                anti_diagonal = 0;
            }
            if anti_diagonal == self.in_a_row {
                return Some(self.current_player);
            }
        }

        return None;
    }
}