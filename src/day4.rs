pub fn find_matches_amount(pattern: &str, input: &Vec<Vec<char>>) -> i64 {
    let mut amount = 0;
   for i in 0..input.len(){
        for j in 0..input[0].len(){
            if pattern_matches_in_horizontal_onwards(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_horizontal_backwards(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_vertical_onwards(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_vertical_backwards(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_diagonal_onwards(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_diagonal_backwards(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_diagonal_down_left(pattern, input, (i,j)) {amount +=1;};
            if pattern_matches_in_diagonal_up_right(pattern, input, (i,j)) {amount +=1;};
        }
    }
    amount
}


pub fn find_matches_amount_on_x(input: &Vec<Vec<char>>) -> i64{
    let mut amount = 0;
    for i in 0..input.len(){
        for j in 0..input[0].len(){
            if pattern_is_in_x(input, (i,j)){amount +=1;}
        }
    }
    amount
}

fn pattern_is_in_x(input: &Vec<Vec<char>>, (i0, j0): (usize, usize)) -> bool {
    if input[i0][j0] != 'A' {
        return false;
    }

    let rows = input.len();
    let cols = input[0].len();

    // Helper function to safely access elements
    let get = |row: isize, col: isize| -> Option<char> {
        if row >= 0 && col >= 0 && (row as usize) < rows && (col as usize) < cols {
            Some(input[row as usize][col as usize])
        } else {
            None
        }
    };

    let down_left = get(i0 as isize - 1, j0 as isize - 1);
    let down_right = get(i0 as isize - 1, j0 as isize + 1);
    let up_left = get(i0 as isize + 1, j0 as isize - 1);
    let up_right = get(i0 as isize + 1, j0 as isize + 1);

    // Check for missing neighbors
    if down_left.is_none() || down_right.is_none() || up_left.is_none() || up_right.is_none() {
        return false;
    }

    let down_left = down_left.unwrap();
    let down_right = down_right.unwrap();
    let up_left = up_left.unwrap();
    let up_right = up_right.unwrap();

    let first_cross = (down_left == 'M' && up_right == 'S') || (down_left == 'S' && up_right == 'M');
    let second_cross = (down_right == 'M' && up_left == 'S') || (down_right == 'S' && up_left == 'M');

    first_cross && second_cross
}




fn pattern_matches_in_diagonal_onwards(pattern: &str, input: &Vec<Vec<char>>, current_character_position: (usize, usize)) -> bool {
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        if i0 + i >= input.len() || j0 + i >= input[0].len() || pattern_chars[i] != input[i0 + i][j0 + i] {
            return false;
        }
    }
    true
}


fn pattern_matches_in_diagonal_backwards(pattern: &str, input: &Vec<Vec<char>>, current_character_position: (usize, usize)) -> bool{
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        // Ensure no underflow for both i0 and j0, and check match
        if i0 < i || j0 < i || pattern_chars[i] != input[i0 - i][j0 - i] {
            return false;
        }
    }
    true
}

fn pattern_matches_in_diagonal_down_left(
    pattern: &str,
    input: &Vec<Vec<char>>,
    current_character_position: (usize, usize),
) -> bool {
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        if i0 + i >= input.len() || j0 < i || pattern_chars[i] != input[i0 + i][j0 - i] {
            return false;
        }
    }
    true
}

fn pattern_matches_in_diagonal_up_right(
    pattern: &str,
    input: &Vec<Vec<char>>,
    current_character_position: (usize, usize),
) -> bool {
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        if i0 < i || j0 + i >= input[0].len() || pattern_chars[i] != input[i0 - i][j0 + i] {
            return false;
        }
    }
    true
}



fn pattern_matches_in_vertical_onwards(pattern: &str, input: &Vec<Vec<char>>, current_character_position: (usize, usize)) -> bool {
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        if i0 + i >= input.len() || pattern_chars[i] != input[i0 + i][j0] {
            return false;
        }
    }
    true
}

fn pattern_matches_in_vertical_backwards(
    pattern: &str,
    input: &Vec<Vec<char>>,
    current_character_position: (usize, usize),
) -> bool {
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        if i0 < i || pattern_chars[i] != input[i0 - i][j0] {
            return false;
        }
    }
    true
}


fn pattern_matches_in_horizontal_onwards(
    pattern: &str,
    input: &Vec<Vec<char>>,
    current_character_position: (usize, usize)
) -> bool{
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        if j0 + i >= input[0].len() || pattern_chars[i] != input[i0][j0 + i] {
            return false;
        }
    }
    true
}

fn pattern_matches_in_horizontal_backwards(
    pattern: &str,
    input: &Vec<Vec<char>>,
    current_character_position: (usize, usize)
) -> bool {
    let (i0, j0) = current_character_position;
    let pattern_chars: Vec<char> = pattern.chars().collect();
    for i in 0..pattern_chars.len() {
        // Ensure j0 >= i to prevent underflow and check match
        if j0 < i || pattern_chars[i] != input[i0][j0 - i] {
            return false;
        }
    }
    true
}

