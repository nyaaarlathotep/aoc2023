use regex::Regex;

// Pick's Theorem is essentially a formula for determining the area of points within a polygon, encompassing the boundary. The formula is:

//  Area = Internal + (Boundary / 2) - 1
// Since we can find the area of polygon using the shoelace formula, and we don't have the number of "Internal" points, we can rearrange the above formula as:

// Internal = Area - (boundary/ 2) + 1
// As we are interested in both internal and boundary points, the formula becomes:

// Internal + boundary = Area + (boundary/ 2) + 1 
// Now replace the area with the output of the shoelace formula and we get:

// Internal + boundary = ((Shoelace + boundary)/ 2) + 1 
// In my code, I incorporated the boundary value into the area within the loop instead of adding them separately.



// Thanks, reddit.

pub fn part01(input: &str) -> Option<i64> {
    return Some(compute(input));
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}
fn read_directions(text: &str) -> Vec<(char, i64)> {
    let regex = Regex::new(r"(?m)^([RLDU]) ([[:digit:]]+)").unwrap();
    regex
        .captures_iter(text)
        .map(|cap| {
            let (_, [digit, number]) = cap.extract();
            (digit.chars().next().unwrap(), number.parse().unwrap())
        })
        .collect()
}

fn read_directions_2(text: &str) -> Vec<(char, i64)> {
    let regex = Regex::new(r"(?m)\(\#([[:xdigit:]]{5})([0-3])\)$").unwrap();
    regex
        .captures_iter(text)
        .map(|cap| {
            let (_, [hexstr, d]) = cap.extract();
            let d_int = usize::from_str_radix(d, 16).unwrap();
            let dir = ['R', 'D', 'L', 'U'][d_int];
            let hex = i64::from_str_radix(hexstr, 16).unwrap();
            (dir, hex)
        })
        .collect()
}

fn get_area(dirs: &[(char, i64)]) -> i64 {
    let (perimeter, area, _) = dirs
        .iter()
        .fold((0, 0, (0, 0)), |(p, a, (y, x)), (d, l)| match d {
            'R' => (p + l, a, (y, x + l)),
            'L' => (p + l, a, (y, x - l)),
            'D' => (p + l, a + x * l, (y + l, x)),
            'U' => (p + l, a - x * l, (y - l, x)),
            _ => panic!("Got {d}, expected R, L, D, or U"),
        });
    area + perimeter / 2 + 1
}

fn compute(text: &str) -> i64 {
    let dirs = read_directions(text);
    get_area(&dirs)
}

fn compute_2(text: &str) -> i64 {
    let dirs = read_directions_2(text);
    get_area(&dirs)
}

fn main() {
    let text = std::fs::read_to_string("input/18.txt").unwrap();
    let result = compute(&text);
    println!("First = {result}");

    let result = compute_2(&text);
    println!("Second = {result}");
}
