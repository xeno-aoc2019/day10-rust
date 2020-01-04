use std::path::Path;
use std::fs::File;
use std::io::{Lines, Result, BufRead, BufReader};


fn to_coords(lines: Vec<String>) -> Vec<(i32, i32)> {
    let mut v = vec!();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                v.push((x as i32, y as i32));
                // println!("({},{})", x, y);
            }
        }
    }
    v
}

pub fn read_points<P>(file_name: P) -> Vec<(i32, i32)> where P: AsRef<Path> {
    let lines = read_lines(file_name);
    to_coords(lines)
}

fn read_lines<P>(file_name: P) -> Vec<String> where P: AsRef<Path>, {
    let mut line_vector = vec!();
    let lines_result = lines(file_name);
    if let Ok(lines) = lines_result {
        for line_result in lines {
            if let Ok(line) = line_result {
                println!("{}", &line);
                line_vector.push(line);
            }
        }
    }
    line_vector
}

fn lines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}