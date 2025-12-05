use std::fs;

fn count_neighbors(grid: &[Vec<char>], row: isize, col: isize) -> u32 {
    let mut count = 0;

    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (direction_row, direction_col) in directions {
        let new_row = row + direction_row;
        let new_col = col + direction_col;

        if new_row >= 0
            && (new_row as usize) < grid.len()
            && new_col >= 0
            && (new_col as usize) < grid[0].len()
            && grid[new_row as usize][new_col as usize] == '@'
        {
            count += 1;
        }
    }

    count
}

fn solve_part_two(grid: &mut [Vec<char>]) {
    let mut total_count = 0;

    loop {
        let mut count = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '@' && count_neighbors(grid, row as isize, col as isize) < 4 {
                    grid[row][col] = 'X';
                    count += 1;
                }
            }
        }

        total_count += count;

        if count == 0 {
            break;
        }
    }

    println!("day_four [2] => {}", total_count);
}

fn solve_part_one(grid: &[Vec<char>]) {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '@' && count_neighbors(grid, row as isize, col as isize) < 4 {
                count += 1;
            }
        }
    }

    println!("day_four [1] => {}", count);
}

pub fn solve() {
    let input = fs::read_to_string("days/4.txt").unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    solve_part_one(&grid);
    solve_part_two(&mut grid);
}
