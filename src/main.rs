use std::fs;
use std::time::Instant;

pub mod day22;
use crate::day22::part01;
use crate::day22::part02;

const DAY: &str = "22";
fn main() {
    let input = format!("./src/day{}/input", DAY);
    let res = read_file_string(input.as_str());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test = format!("./src/day{}/test", DAY);
        let res = read_file_string(test.as_str());
        match res {
            Ok(v) => {
                println!("part 1 res: {:?}", part01(v.as_str()));
                println!("part 2 res: {:?}", part02(v.as_str()));
            }
            Err(e) => {
                println!("err:{e:?}")
            }
        }
    }

    #[test]
    fn temp_test() {
    }
}
