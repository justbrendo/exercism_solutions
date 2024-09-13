pub fn increase_bomb(matrix: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    if let Some(row) = matrix.get_mut(i) {
        if let Some(value) = row.get_mut(j) {
            if *value != -1 {
                *value += 1;
            }
        }
    }
}

pub fn bomb(matrix: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    matrix[i][j] = -1;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), /*(0, 0),*/ (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for &(di, dj) in &directions {
        // Use checked arithmetic to avoid overflow and handle edge cases
        if let Some(new_i) = i.checked_add_signed(di) {
            if let Some(new_j) = j.checked_add_signed(dj) {
                increase_bomb(matrix, new_i, new_j);
            }
        }
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = if let Some(first_row) = minefield.first() {
        first_row.len()
    } else {
        0
    };

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

    for (i, line) in minefield.iter().enumerate() {
        for (j, byte) in line.as_bytes().iter().enumerate() {
            let char_value = *byte as char;
            match char_value {
                '*' => bomb(&mut matrix, i, j),
                _ => (),
            }
        }
    }

    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&num| match num {
                    0 => ' '.to_string(),
                    -1 => '*'.to_string(),
                    _ => num.to_string()
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}
