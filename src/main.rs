use std::fs;
use std::time::Instant;

use crate::day01::part01;

pub mod day01;

fn main() {
    let res = read_file_string("./src/day01/input");
    let start = Instant::now();
    match res {
        Ok(v) => {
            println!("part 1 res: {:?}", part01(v.as_str()));
        }
        Err(e) => {
            println!("err:{e:?}")
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed :{:?}", duration)
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
