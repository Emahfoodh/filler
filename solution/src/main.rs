use std::fs::OpenOptions;
use std::io::{self, Write};


#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    score: i32,
}

fn main() {
    // for debugging
    let log_file_path = "debug.log";
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_file_path)
        .expect("Failed to open log file");

    writeln!(log_file, "Starting the game").expect("Failed to write to log file");

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut piece: Vec<Vec<char>> = Vec::new();
    let mut input = String::new();
    let mut x: usize;
    let mut y: usize;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    let player_char = match player_number {
        '1' => vec!['@', 'a'],
        '2' => vec!['$', 's'],
        _ => {
            writeln!(log_file, "Invalid player number: {}", player_number)
                .expect("Failed to write to log file");
            return;
        }
    };
    // writeln!(log_file, "Player number: {}", player_number).expect("Failed to write to log file");
    // writeln!(log_file, "Initial input: {}", input.trim()).expect("Failed to write to log file");
    loop {
        writeln!(log_file, "New round").expect("Failed to write to log file");
        grid.clear();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        // writeln!(log_file, "Received input: {}", input.trim())
        //     .expect("Failed to write to log file");
        if !input.starts_with("Anfield") {
            writeln!(
                log_file,
                "Input does not start with 'Anfield', exiting loop"
            )
            .expect("Failed to write to log file");
            return;
        }

        match parse_size_input(&input) {
            Ok((x_val, y_val)) => {
                x = x_val;
                y = y_val;
                writeln!(log_file, "Anfield: x = {}, y = {}", x, y)
                    .expect("Failed to write to log file");
            }
            Err(e) => {
                writeln!(log_file, "Error parsing size input: {}", e)
                    .expect("Failed to write to log file");
                return;
            }
        }

        // skip this input
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // fill the grid
        writeln!(log_file, "Grid size: y = {}", y).expect("Failed to write to log file");
        for _ in 0..y {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            grid.push(row);
        }

        // to debug
        writeln!(log_file, "Grid:").expect("Failed to write to log file");
        for row in &grid {
            let row_str: String = row.iter().collect();
            writeln!(log_file, "{}", row_str).expect("Failed to write to log file");
        }

        // the piece
        piece.clear();
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if !input.starts_with("Piece") {
            writeln!(log_file, "Input does not start with 'Piece', exiting loop")
                .expect("Failed to write to log file");
            return;
        }
        match parse_size_input(&input) {
            Ok((x_val, y_val)) => {
                x = x_val;
                y = y_val;
                writeln!(log_file, "Piece: x = {}, y = {}", x, y)
                    .expect("Failed to write to log file");
            }
            Err(e) => {
                writeln!(log_file, "Error parsing size input: {}", e)
                    .expect("Failed to write to log file");
                return;
            }
        }
        // fill the piece
        writeln!(log_file, "Piece size: y = {}", y).expect("Failed to write to log file");
        for _ in 0..y {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            let row: Vec<char> = input[..input.len() - 1].chars().collect();
            piece.push(row);
        }
        // write the piece to the file
        writeln!(log_file, "Piece:").expect("Failed to write to log file");
        for row in &piece {
            let row_str: String = row.iter().collect();
            writeln!(log_file, "{}", row_str).expect("Failed to write to log file");
        }

        let (piece_x, piece_y) = choose_coordinates(&grid, &piece, &player_char);
        writeln!(
            log_file,
            "Chosen coordinates: x = {}, y = {}",
            piece_x, piece_y
        )
        .expect("Failed to write to log file");

        // Place the piece on the grid
        for py in 0..piece.len() {
            for px in 0..piece[py].len() {
                if piece[py][px] == 'O' {
                    grid[piece_y + py][piece_x + px] = player_char[0];
                }
            }
        }

        // Write the updated grid to the log file
        writeln!(log_file, "Updated Grid:").expect("Failed to write to log file");
        for row in &grid {
            let row_str: String = row.iter().collect();
            writeln!(log_file, "{}", row_str).expect("Failed to write to log file");
        }
        println!("{} {}", piece_x, piece_y);
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

// New function to find opponent's territory center
fn find_opponent_center(grid: &Vec<Vec<char>>, player_char: &Vec<char>) -> Position {
    let mut sum_x = 0;
    let mut sum_y = 0;
    let mut count = 0;
    let opponent_chars = if player_char[0] == '@' {
        vec!['$', 's']
    } else {
        vec!['@', 'a']
    };

    for (y, row) in grid.iter().enumerate() {
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
            x: grid[0].len() / 2,
            y: grid.len() / 2,
            score: 0,
        };
    }

    Position {
        x: sum_x / count,
        y: sum_y / count,
        score: 0,
    }
}

// Modified choose_coordinates function with strategic placement
fn choose_coordinates(
    grid: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    player_char: &Vec<char>,
) -> (usize, usize) {
    let opponent_center = find_opponent_center(grid, player_char);
    let mut valid_positions: Vec<Position> = Vec::new();

    // Find all valid positions
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if is_valid_placement(grid, piece, (x, y), player_char) {
                // Calculate score based on distance to opponent's center
                let distance = calculate_distance(x, y, opponent_center.x, opponent_center.y);
                let surrounding_score = calculate_surrounding_score(grid, x, y, player_char);
                let score = surrounding_score - (distance as i32);

                valid_positions.push(Position { x, y, score });
            }
        }
    }

    // If no valid positions found, return (0, 0)
    if valid_positions.is_empty() {
        return (0, 0);
    }

    // Sort positions by score (highest first)
    valid_positions.sort_by(|a, b| b.score.cmp(&a.score));
    
    (valid_positions[0].x, valid_positions[0].y)
}

// Calculate Manhattan distance between two points
fn calculate_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    dx + dy
}

// Calculate score based on surrounding opponent pieces
fn calculate_surrounding_score(grid: &Vec<Vec<char>>, x: usize, y: usize, player_char: &Vec<char>) -> i32 {
    let mut score = 0;
    let opponent_chars = if player_char[0] == '@' {
        vec!['$', 's']
    } else {
        vec!['@', 'a']
    };

    // Check surrounding cells (including diagonals)
    for dy in -1..=1 {
        for dx in -1..=1 {
            let new_y = y as i32 + dy;
            let new_x = x as i32 + dx;

            if new_y >= 0 && new_y < grid.len() as i32 && 
               new_x >= 0 && new_x < grid[0].len() as i32 {
                let cell = grid[new_y as usize][new_x as usize];
                if opponent_chars.contains(&cell) {
                    score += 10; // Increase score for each nearby opponent piece
                }
            }
        }
    }

    score
}

fn is_valid_placement(
    grid: &Vec<Vec<char>>,
    piece: &Vec<Vec<char>>,
    pos: (usize, usize),
    player_char: &Vec<char>,
) -> bool {
    let mut overlap_count = 0;
    for py in 0..piece.len() {
        for px in 0..piece[py].len() {
            if piece[py][px] == 'O' {
                let gx = pos.0 + px;
                let gy = pos.1 + py;

                if gy >= grid.len() || gx >= grid[gy].len() {
                    return false; // Out of bounds
                }

                match grid[gy][gx] {
                    '.' => continue,                                     // Empty cell, valid
                    c if player_char.contains(&c) => overlap_count += 1, // Overlapping own territory
                    _ => return false,                                   // Invalid overlap
                }
            }
        }
    }
    overlap_count == 1
}
