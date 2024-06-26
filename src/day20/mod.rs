use regex::Regex;

pub fn part01(input: &str) -> Option<i64> {
    let mut broadcaster: Option<Module> = None;

    let regex = Regex::new(r"(?m)^(?<name>[\w+]+) -> (?<ends>.*)").unwrap();
    let modules = regex
        .captures_iter(input)
        .map(|cap| {
            let name = &cap["name"];
            let ends: Vec<String> = cap["ends"]
                .split(",")
                .map(|s| {
                    return s.to_string();
                })
                .collect();
            if name == "broadcaster" {
                broadcaster = Some(Module {
                    module_type: "broadcaster".to_string(),
                    memory: false,
                    ends,
                });
                return None;
            }
            if name.starts_with("%") {
                return Some(Module {
                    module_type: "%".to_string(),
                    memory: false,
                    ends,
                });
            }
            if name.starts_with("&") {
                return Some(Module {
                    module_type: "&".to_string(),
                    memory: false,
                    ends,
                });
            }
            return None::<Module>;
        })
        .filter(|opt| {
            return opt.is_some();
        })
        .map(|opt| {
            return opt.unwrap();
        })
        .collect::<Vec<Module>>();
    return Some(0);
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}

// % If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
// & if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Module {
    module_type: String,
    memory: bool,
    ends: Vec<String>,
}
