use std::fs::File;
use std::io::{BufRead, BufReader};


fn read_grid(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        let row: Vec<i32> = line.chars().map(|c| if c == '@' { 10 } else { 0 } as i32).collect();
        grid.push(row);
    }
    grid
}

fn process_neighbors(grid: &mut Vec<Vec<i32>>) {    
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                continue;
            }

            if i != 0 {
                grid[i][j] += grid[i - 1][j] / 10;
            }
            if j != 0 {
                grid[i][j] += grid[i][j-1] / 10;
            }
            if i != 0 && j != 0 {
                grid[i][j] += grid[i - 1][j - 1] / 10;
            }
            if i != 0 && j + 1 < grid[i].len() {
                grid[i][j] += grid[i - 1][j + 1] / 10;
            }
            if j != 0 && i + 1 < grid.len() {
                grid[i][j] += grid[i + 1][j - 1] / 10;
            }
            if j + 1 < grid[i].len() {
                grid[i][j] += grid[i][j + 1] / 10;
            }
            if i + 1 < grid.len() {
                grid[i][j] += grid[i + 1][j] / 10;  
            }
            if i + 1 < grid.len() && j + 1 < grid[i].len() {
                grid[i][j] += grid[i + 1][j + 1] / 10; 
            }
        }
    }
}

fn count_moveable(grid: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] >= 10 && grid[i][j] < 14 {
                count += 1;
            }
        }
    }
    count
}

fn remove_rolls(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut removed = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] >= 10 && grid[i][j] < 14 {
                grid[i][j] = 0;
                removed += 1;
            } else if grid[i][j] >= 10 {
                grid[i][j] = 10;
            }
        }
    }
    process_neighbors(grid);
    removed
}

fn remove_all(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut total_removed = 0;
    loop {
        let removed = remove_rolls(grid);
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }
    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut grid = read_grid("inputs/example.txt");
        process_neighbors(&mut grid);
        assert_eq!(count_moveable(&grid), 13);
    }

    #[test]
    fn example_part2() {
        let mut grid = read_grid("inputs/example.txt");
        process_neighbors(&mut grid);
        let total_removed = remove_all(&mut grid);
        assert_eq!(total_removed, 43);
    }
}

fn main() {
    let mut grid = read_grid("inputs/input.txt");
    process_neighbors(&mut grid);
    let moveable_papers = count_moveable(&grid);
    println!("Moveable papers, Part 1: {}", moveable_papers);
    let total_removed = remove_all(&mut grid);
    println!("Total removed papers, Part 2: {}", total_removed);
}
