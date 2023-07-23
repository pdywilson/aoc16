use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    let file_path = "input.txt";

    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            println!("File content:\n{}", contents);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    }

    let cleaned_contents: String = contents
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter(|c| *c != '\n')
        .collect();

    let directions: Vec<&str> = cleaned_contents
        .split(',')
        .collect();

    println!("{:?}", directions);
    

    let directions_map: Vec<(char, i32)> = directions
        .iter()
        .map(|&direction| {
            let first_char = direction.chars().next().unwrap();
            let rest_string: String = direction.chars().skip(1).collect();
            let rest = match rest_string.parse::<i32>() {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error parsing string: {}", e);
                    // You can set a default value or handle the error in some other way here
                    return (first_char, 0);  
                }
            };
            (first_char, rest)
        })
        .collect();

    println!("{:#?}", directions_map);


    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = 0; // 0 == N, 1 == E, 2 == S, 3 == W
    let mut res: Vec<(i32, i32)> = Vec::new(); 
    let mut done: bool = false;

    for (direction, value) in &directions_map {
        match *direction {
            'R' => dir = (dir + 1) % 4,
            'L' => dir = (dir + 3) % 4,
            _ => println!("Warning: case could not be matched")
        }
        for _ in 1..=*value {
            match dir {
                0 => y = y + 1,
                1 => x = x + 1,
                2 => y = y - 1,
                3 => x = x - 1,
                _ => println!("Warning: case could not be matched")
            }
            if !done && res.contains(&(x, y)) {
                println!("result part 2: {}", x + y);
                done = true;
            }
            res.push((x, y));
        }
    }
    println!("result part 1: {}", x + y);
}
