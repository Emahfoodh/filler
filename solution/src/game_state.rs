use crate::{logger::Logger, player::Player};
use std::io;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    score: i32,
}
pub struct GameState {
    grid: Vec<Vec<char>>,
    piece: Vec<Vec<char>>,
    player: Player,
}

impl GameState {
    pub fn new(player: Player) -> Self {
        GameState {
            grid: Vec::new(),
            piece: Vec::new(),
            player,
        }
    }
    pub fn read_grid(&mut self, logger: &mut Logger) -> Result<(), String> {
        self.grid.clear();
        let mut input = String::new();

        // Read grid dimensions
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        if !input.starts_with("Anfield") {
            return Err("Invalid grid input format".to_string());
        }

        let (width, height) = parse_size_input(&input)?;
        logger
            .log(&format!("Anfield: {}x{}", width, height))
            .unwrap();

        // Skip player line
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        // Read grid
        self.grid.clear();
        for _ in 0..height {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            self.grid.push(row);
        }

        // Log grid
        logger.log("Grid:").unwrap();
        for row in &self.grid {
            logger.log(&row.iter().collect::<String>()).unwrap();
        }

        Ok(())
    }
    pub fn read_piece(&mut self, logger: &mut Logger) -> Result<(), String> {
        self.piece.clear();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        if !input.starts_with("Piece") {
            return Err("Invalid piece input format".to_string());
        }

        let (width, height) = parse_size_input(&input)?;
        logger.log(&format!("Piece: {}x{}", width, height)).unwrap();

        self.piece.clear();
        for _ in 0..height {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let row: Vec<char> = input[..input.len() - 1].chars().collect();
            self.piece.push(row);
        }

        logger.log("Piece:").unwrap();
        for row in &self.piece {
            logger.log(&row.iter().collect::<String>()).unwrap();
        }

        Ok(())
    }
    // Modified choose_coordinates function with strategic placement
    pub fn choose_coordinates(&self) -> Position {
        let opponent_center = self.find_opponent_center();
        let mut valid_positions: Vec<Position> = Vec::new();

        // Find all valid positions
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                if self.is_valid_placement((x, y)) {
                    // Calculate score based on distance to opponent's center
                    let distance = Self::calculate_distance(x, y, opponent_center.x, opponent_center.y);
                    let surrounding_score =
                        self.calculate_surrounding_score(x, y);
                    let score = surrounding_score - (distance as i32);

                    valid_positions.push(Position { x, y, score });
                }
            }
        }

        // If no valid positions found, return (0, 0)
        if valid_positions.is_empty() {
            return Position {
                x: 0,
                y: 0,
                score: 0,
            };
        }

        // Sort positions by score (highest first)
        valid_positions.sort_by(|a, b| b.score.cmp(&a.score));

        valid_positions[0]
    }

    // New function to find opponent's territory center
    fn find_opponent_center(&self) -> Position {
        let mut sum_x = 0;
        let mut sum_y = 0;
        let mut count = 0;
        let opponent_chars = self.player.opponent_chars();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if opponent_chars.contains(&cell) {
                    sum_x += x;
                    sum_y += y;
                    count += 1;
                }
            }
        }

        if count == 0 {
            // If no opponent pieces found, return center of grid
            return Position {
                x: self.grid[0].len() / 2,
                y: self.grid.len() / 2,
                score: 0,
            };
        }

        Position {
            x: sum_x / count,
            y: sum_y / count,
            score: 0,
        }
    }

    fn is_valid_placement(&self, pos: (usize, usize)) -> bool {
        let mut overlap_count = 0;
        for py in 0..self.piece.len() {
            for px in 0..self.piece[py].len() {
                if self.piece[py][px] == 'O' {
                    let gx = pos.0 + px;
                    let gy = pos.1 + py;

                    if gy >= self.grid.len() || gx >= self.grid[gy].len() {
                        return false; // Out of bounds
                    }

                    match self.grid[gy][gx] {
                        '.' => continue,                                             // Empty cell, valid
                        c if self.player.chars().contains(&c) => overlap_count += 1, // Overlapping own territory
                        _ => return false, // Invalid overlap
                    }
                }
            }
        }
        overlap_count == 1
    }
    // Calculate Manhattan distance between two points
    fn calculate_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
        let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
        let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
        dx + dy
    }

    // Calculate score based on surrounding opponent pieces
fn calculate_surrounding_score(&self, x: usize, y: usize) -> i32 {
    let mut score = 0;
    let opponent_chars = self.player.opponent_chars();

    // Check surrounding cells (including diagonals)
    for dy in -1..=1 {
        for dx in -1..=1 {
            let new_y = y as i32 + dy;
            let new_x = x as i32 + dx;

            if new_y >= 0 && new_y < self.grid.len() as i32 && new_x >= 0 && new_x < self.grid[0].len() as i32
            {
                let cell = self.grid[new_y as usize][new_x as usize];
                if opponent_chars.contains(&cell) {
                    score += 10; // Increase score for each nearby opponent piece
                }
            }
        }
    }

    score
}

}

fn parse_size_input(input: &str) -> Result<(usize, usize), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() < 3 {
        return Err("Invalid input format, expected at least 3 parts".to_string());
    }
    let x: usize = parts[1]
        .parse()
        .map_err(|_| "Failed to parse width".to_string())?;
    let y: usize = parts[2]
        .trim_end_matches(':')
        .parse()
        .map_err(|_| "Failed to parse height".to_string())?;
    Ok((x, y))
}
