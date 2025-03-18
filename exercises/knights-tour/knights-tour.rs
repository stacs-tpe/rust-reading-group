// Knights can move in 8 different directions in the (x,y) plane, at these offsets.
const KNIGHT_MOVES: [(i8, i8); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1)
];
const WIDTH: u8 = 8;
const REQUIRED: usize = (WIDTH * WIDTH) as usize;  // Number of squares required for a full tour

// Main entry point, no arguments
fn main() {
    // Welcome message
    println!("MY_KNIGHT, v3.0");
    println!("A blazingly fast Knight's tour program, written in Rust");
    println!("(Now with a blazingly fast stack array)");
    
    // (visited, path_length) represents the path explored so far. 'visited' has
    // enough space for a full path, but only the range [0..path_length]
    // represents actual data. This is so that 'visited' doesn't have to grow
    // and shrink.
    let mut visited: [(u8, u8); REQUIRED] = [(0, 0); REQUIRED];
    let mut path_length: usize = 1;  // start at top-left square

    // Print the result from the recursive function
    match complete_path(&mut visited, &mut path_length) {
        true => {
            println!("Found path of length {} on {}×{} board", path_length, WIDTH, WIDTH);
            draw_path(&visited);
        },
        false => println!("No path found")
    }
}

// Attempts to complete a partial knight's tour
fn complete_path(visited: &mut [(u8, u8); REQUIRED], path_length: &mut usize) -> bool {
    // Check for completion
    if *path_length >= REQUIRED {
        return true;
    }

    // Get the current location
    let (x, y): &(u8, u8) = &visited[*path_length - 1].clone();

    // Check all 8 directions
    for (dx, dy) in KNIGHT_MOVES {
        // Get the (x, y) coordinate of a move in this direction
        // Skip this one if it's off the edge of the board
        let newx = match x.checked_add_signed(dx) {
            Some(newx) => if newx < WIDTH {newx} else {continue},
            None => continue
        };
        let newy = match y.checked_add_signed(dy) {
            Some(newy) => if newy < WIDTH {newy} else {continue},
            None => continue
        };

        // Skip this one if it's already been visited
        if (visited[0..*path_length]).contains(&(newx, newy)) {
            continue;
        }

        // If we get here it's a legal next move!
        // Add this square to the list
        visited[*path_length] = (newx, newy);
        *path_length += 1;

        // Recursive call using the longer list
        if complete_path(visited, path_length) {
            return true;
        } else {
            *path_length -= 1;  // Failure, so remove that last square
            continue
        }
    }

    // If no solution was found, then no solution exists for this list
    false
}

// Draw the (partial) tour, as a grid with numbered squares
fn draw_path(visited: &[(u8, u8); REQUIRED]) {
    for x in 0..WIDTH {
        for y in 0..WIDTH {
            match visited.iter().position(|a| a == &(x, y)) {
                Some(i) => print!("{:>3}", i),
                None => print!("  .")
            };
        }
        println!();
    }
}
