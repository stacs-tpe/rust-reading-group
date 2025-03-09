// Knights can move in 8 different directions in the (x,y) plane, at these offsets.
const KNIGHT_MOVES: [(i8, i8); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1)
];

// Main entry point, no arguments
fn main() {
    // Welcome message
    println!("MY_KNIGHT");
    println!("A blazingly fast Knight's tour program, written in Rust");
    println!("v2.0");
    println!("(Now with blazingly fast Vec references)");
    
    // Set up parameters
    let mut visited: Vec<(u8, u8)> = Vec::new();  // All visited squares so far
    visited.push((0, 0));  // Start at the top-left square
    let width = 8;
    let required = width * width;  // Number of squares required for a full tour

    // Print the result from the recursive function
    match complete_path(&mut visited, width, required) {
        true => {
            println!("Found path of length {} on {}×{} board", visited.len(), width, width);
            draw_path(&visited, width);
        },
        false => println!("No path found")
    }
}

// Attempts to complete a partial knight's tour
// - visited: squares we've already visited
// - width: dimention of the board (8)
// - required: number of squares we need to visit for completion (64)
// Returns optional list of squares (immutable if present)
fn complete_path(visited: &mut Vec<(u8, u8)>, width: u8, required: u8) -> bool {

    // Check for completion
    if visited.len() >= required.into() {
        return true;
    }

    // Get the current location
    let (x, y): &(u8, u8) = &visited.last()
        .expect("visited should never be empty")
        .clone();  // We have to copy this because it's a reference to 'visited'!

    // Check all 8 directions
    for (dx, dy) in KNIGHT_MOVES {
        // Get the (x, y) coordinate of a move in this direction
        // Skip this one if it's off the edge of the board
        let newx = match x.checked_add_signed(dx) {
            Some(newx) => if newx < width {newx} else {continue},
            None => continue
        };
        let newy = match y.checked_add_signed(dy) {
            Some(newy) => if newy < width {newy} else {continue},
            None => continue
        };

        // Skip this one if it's already been visited
        if visited.contains(&(newx, newy)) {
            continue;
        }

        // If we get here it's a legal next move!
        // Add this square to the list
        visited.push((newx, newy));

        // Recursive call using the longer list
        if complete_path(visited, width, required) {
            return true;
        } else {
            visited.pop();  // Failure, so remove that last square
            continue
        }
    }

    // If no solution was found, then no solution exists for this list
    false
}

// Draw the (partial) tour, as a grid with numbered squares
fn draw_path(visited: &Vec<(u8, u8)>, width: u8) {
    for x in 0..width {
        for y in 0..width {
            match visited.iter().position(|a| a == &(x, y)) {
                Some(i) => print!("{:>3}", i),
                None => print!("  .")
            };
        }
        println!();
    }
}
