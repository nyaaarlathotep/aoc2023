use std::fs;
use std::time::Instant;

pub mod day02;
use crate::day02::part01;
use crate::day02::part02;

fn main() {
    let _test = "./src/day02/test";
    let _input = "./src/day02/input";
    let res = read_file_string(_input);
    // let res = read_file_string(_test);
    let start = Instant::now();
    match res {
        Ok(v) => {
            println!("part 1 res: {:?}", part01(v.as_str()));
            println!("part 2 res: {:?}", part02(v.as_str()));
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