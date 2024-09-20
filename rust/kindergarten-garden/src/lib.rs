fn get_plant_name(c: char) -> &'static str {
    match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => unreachable!(),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let names = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];
    let mut result: Vec<&'static str> = Vec::new();

    let student_offset = names.iter().position(|&name| name == student).unwrap() * 2;
    let sill_offset = diagram.len() / 2 + 1;

    let indexes = vec![
        student_offset,
        student_offset + 1,
        student_offset + sill_offset,
        student_offset + sill_offset + 1,
    ];

    for i in indexes {
        result.push(get_plant_name(diagram.chars().nth(i).unwrap()));
    }

    result
}
