// Knights can move in 8 different directions in the (x,y) plane, at these offsets.
const KNIGHT_MOVES: [(i8, i8); 8] = [
    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
    (1, -2), (1, 2), (2, -1), (2, 1)
];

// Main entry point, no arguments
fn main() {
    // Set up parameters
    let mut visited: Vec<(u8, u8)> = Vec::new();  // All visited squares so far
    visited.push((0, 0));  // Start at the top-left square
    let width = 8;
    let required = width * width;  // Number of squares required for a full tour

    // Print the result from the recursive function
    match find_path(visited, width, required) {
        Some(path) => {
            println!("Found path of length {} on {}×{} board", path.len(), width, width);
            draw_path(path, width);
        },
        None => println!("No path found")
    }
}

// Attempts to complete a partial knight's tour
// - visited: squares we've already visited
// - width: dimention of the board (8)
// - required: number of squares we need to visit for completion (64)
fn find_path(visited: Vec<(u8, u8)>, width: u8, required: u8) -> Option<Vec<(u8, u8)>> {

    // Check for completion
    if visited.len() >= required.into() {
        return Some(visited);
    }

    // Get the current location
    let (x, y): &(u8, u8) = visited.last()
        .expect("visited should never be empty");

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
        // Copy the visited list and add the new square to it
        // TODO: Modify the list instead of copying
        let mut new_visited = visited.to_vec();
        new_visited.push((newx, newy));

        // Recursive call using the new longer list
        return match find_path(new_visited, width, required) {
            Some(path) => Some(path),
            None => continue
        };
    }

    // If no solution was found, then none exists for this
    return None;
}

// Draw the (partial) tour, as a grid with numbered squares
fn draw_path(visited: Vec<(u8, u8)>, width: u8) {
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
