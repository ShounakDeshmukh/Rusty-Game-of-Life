use super::SIZE;

pub fn generations(parent_array: [[u8; SIZE]; SIZE]) -> [[u8; SIZE]; SIZE] {
    let mut child_array = [[0u8; SIZE]; SIZE];
    for y in 1..SIZE - 1 as usize {
        for x in 1..SIZE - 1 as usize {
            let sum = parent_array[x - 1][y - 1]
                + parent_array[x][y - 1]
                + parent_array[x + 1][y - 1]
                + parent_array[x - 1][y]
                + parent_array[x][y]
                + parent_array[x + 1][y]
                + parent_array[x - 1][y + 1]
                + parent_array[x][y + 1]
                + parent_array[x + 1][y + 1]
                - parent_array[x][y];

            if parent_array[x][y] == 0 {
                if sum == 3 {
                    child_array[x][y] = 1;
                } else {
                    continue;
                }
            } else {
                match sum {
                    0 | 1 => child_array[x][y] = 0,
                    2 | 3 => child_array[x][y] = 1,
                    4..=8 => child_array[x][y] = 0,
                    _ => print!("ERROR"),
                }
            }
        }
    }
    child_array
}
