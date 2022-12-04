use rand::{self, Rng};
use std::{thread, time};
const SIZE: usize = 150;
pub mod generations;
fn main() {
    let mut iter = 1;
    let delay = time::Duration::from_millis(50);
    let mut rng = rand::thread_rng();
    let mut parent_array = [[1; SIZE]; SIZE];

    for x in 1..SIZE - 1 as usize {
        for y in 1..SIZE - 1 as usize {
            parent_array[y][x] = rng.gen_range(0..=1);
        }
    }

    println!();
    loop {
        parent_array = generations::generations(parent_array);

        for x in 1..SIZE - 1 as usize {
            for y in 1..SIZE - 1 as usize {
                if parent_array[y][x] == 0 {
                    print!("█")
                } else {
                    print!("░")
                }
            }
            println!();
        }

        println!("iteration {}", iter);
       

        
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        thread::sleep(delay);
        iter += 1;
    }
}
