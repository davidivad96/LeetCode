use std::{collections::VecDeque, usize, vec};

struct Cell {
    x: usize,
    y: usize,
}

fn init_visited_matrix(n: usize, m: usize) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    for _i in 0..n {
        result.push(vec![false; m]);
    }
    result
}

fn is_save_cell(i: usize, n: usize, j: usize, m: usize) -> bool {
    i < n && j < m
}

fn mark_island_as_visited(
    grid: &Vec<Vec<char>>,
    visited_vector: &Vec<Vec<bool>>,
    i: usize,
    n: usize,
    j: usize,
    m: usize,
) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = visited_vector.to_vec();
    let mut children: VecDeque<Cell> = VecDeque::new();
    children.push_back(Cell { x: i, y: j });
    while !children.is_empty() {
        let current_cell: Option<Cell> = children.pop_front();
        match current_cell {
            Some(cell) => {
                if is_save_cell(cell.x, n, cell.y, m)
                    && grid[cell.x][cell.y] == '1'
                    && !result[cell.x][cell.y]
                {
                    result[cell.x][cell.y] = true;
                    children.push_back(Cell { x: cell.x + 1, y: cell.y });
                    children.push_back(Cell { x: cell.x, y: cell.y + 1 });
                    if cell.x > 0 { children.push_back(Cell { x: cell.x - 1, y: cell.y + 1, }); }
                    if cell.y > 0 { children.push_back(Cell { x: cell.x, y: cell.y - 1, }); }
                }
            }
            None => println!("An error occurred"),
        }
    }
    result
}

fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let mut visited: Vec<Vec<bool>> = init_visited_matrix(n, m);
    let mut result: i32 = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '1' && !visited[i][j] {
                visited = mark_island_as_visited(&grid, &visited, i, n, j, m);
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let grid: Vec<Vec<char>> = vec![
        vec!['1', '1', '1'],
        vec!['0', '1', '0'],
        vec!['1', '1', '1'],
    ];
    println!("Number of islands: {}", num_islands(grid))
}
