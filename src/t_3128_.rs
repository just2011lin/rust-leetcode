// 3128.直角三角形

pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let mut result: i64 = 0;
    let max_x = grid[0].len();
    let max_y = grid.len();
    let mut cols: Vec<i64> = vec![0; max_x];
    for x in 0..max_x {
        for y in 0..max_y {
            cols[x] += grid[y][x] as i64;
        }
    }
    for y in 0..max_y {
        let row: i32 = grid[y].iter().sum();
        for x in 0..max_x {
            if grid[y][x] == 1 {
                result += (cols[x] - 1) * (row as i64 - 1);
            }
        }
    }
    result
}
