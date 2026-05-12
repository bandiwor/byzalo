fn generate_magic_square(k: u32) -> Vec<Vec<u32>> {
    let side_length = 1_usize << k;
    let total_cells = side_length * side_length;

    let mut grid = vec![vec![0; side_length]; side_length];

    for current_value in 1..=total_cells {
        let mut layer_index = current_value - 1;
        let mut row = 0;
        let mut col = 0;

        let mut stack_size = total_cells;
        let mut current_height = 1;
        let mut current_width = 1;

        for step in 1..=(2 * k) {
            stack_size /= 2;

            if step % 2 != 0 {
                if layer_index >= stack_size {
                    row = 2 * current_height - 1 - row;
                    layer_index = 2 * stack_size - 1 - layer_index;
                }
                current_height *= 2;
            } else {
                if layer_index >= stack_size {
                    col = 2 * current_width - 1 - col;
                    layer_index = 2 * stack_size - 1 - layer_index;
                }
                current_width *= 2;
            }
        }

        grid[row][col] = current_value as u32;
    }

    grid
}

fn main() {
    println!("Морозов К.О. 090301-ПОВа-о25");
    let k = 2;
    let matrix = generate_magic_square(k);

    println!("Матрица для k = {}:\n", k);
    for row in matrix {
        for val in row {
            print!("{:4} ", val);
        }
        println!();
    }
}
