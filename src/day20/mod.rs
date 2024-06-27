use core::panic;
use std::{collections::HashMap, hash::Hash};

use regex::Regex;

pub fn part01(input: &str) -> Option<i64> {
    let mut broadcaster: Option<Module> = None;

    let regex = Regex::new(r"(?m)^(?<name>.*) -> (?<ends>.*)").unwrap();
    let mut modules = regex
        .captures_iter(input)
        .map(|cap| {
            let name = &cap["name"];
            let ends: Vec<String> = cap["ends"]
                .split(", ")
                .map(|s| {
                    return s.to_string();
                })
                .collect();
            if name == "broadcaster" {
                broadcaster = Some(Module {
                    name: "broadcaster".to_string(),
                    module_type: "broadcaster".to_string(),
                    memory: false,
                    ends,
                });
                return None;
            }
            if name.starts_with("%") {
                return Some(Module {
                    name: name[1..].to_string(),
                    module_type: "%".to_string(),
                    memory: false,
                    ends,
                });
            }
            if name.starts_with("&") {
                return Some(Module {
                    name: name[1..].to_string(),
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
            let module = opt.unwrap();
            return (module.name.clone(), module);
        })
        .collect::<HashMap<String, Module>>();
    let broad = broadcaster.expect("where");
    let mut pulses: HashMap<String, Vec<bool>> = HashMap::new();
    let mut high_count = 0;
    let mut low_count = 0;
    let mut push_count = 0;
    loop {
        push_count += 1;
        let mut init_pulse = Vec::new();
        init_pulse.push(false);
        pulses.insert("broadcaster".to_string(), init_pulse);
        loop {
            if pulses.len() == 0 {
                break;
            }
            let (new_pulses, new_low, new_high) = next_pulse(&mut pulses, &broad, &mut modules);
            pulses = new_pulses;
            high_count += new_high;
            low_count += new_low;
        }
        if push_count == 1 {
            break;
        }
    }
    return Some((high_count * low_count) as i64);
}

fn next_pulse(
    pulses: &mut HashMap<String, Vec<bool>>,
    broad: &Module,
    modules: &mut HashMap<String, Module>,
) -> (HashMap<String, Vec<bool>>, usize, usize) {
    let mut new_pulse: HashMap<String, Vec<bool>> = HashMap::new();
    let mut high_count = 0;
    let mut low_count = 0;
    println!("============");
    for pulse in &*pulses {
        for sig in pulse.1 {
            println!(" -{}-> {}", if *sig { "high" } else { "low" }, pulse.0);
        }
        for p in pulse.1 {
            if *p {
                high_count += 1;
            } else {
                low_count += 1;
            }
        }
        if pulse.0 == "broadcaster" {
            for end in &broad.ends {
                new_pulse
                    .entry(end.clone())
                    .and_modify(|e| {
                        e.append(&mut pulse.1.clone());
                    })
                    .or_insert_with(|| pulse.1.clone());
            }
            continue;
        }
        let module_opt = modules.get_mut(pulse.0);
        if module_opt.is_none() {
            // println!("missed {:?}", pulse);
            continue;
        }
        let module = module_opt.unwrap();
        if module.module_type == "%" {
            for p_signal in pulse.1 {
                if *p_signal {
                    continue;
                }
                let to_send_pulse = if module.memory {
                    module.memory = false;
                    false
                } else {
                    module.memory = true;
                    true
                };
                for end in &module.ends {
                    new_pulse
                        .entry(end.clone())
                        .and_modify(|e| {
                            e.push(to_send_pulse);
                        })
                        .or_insert_with(|| {
                            let mut send_pulse = Vec::new();
                            send_pulse.push(to_send_pulse);
                            send_pulse
                        });
                }
            }

            continue;
        }

        if module.module_type == "&" {
            let mut to_send_pulse = !module.memory;
            for p in pulse.1 {
                module.memory = *p;
                if !*p {
                    to_send_pulse = true;
                }
            }

            for end in &module.ends {
                new_pulse
                    .entry(end.clone())
                    .and_modify(|e| {
                        e.push(to_send_pulse);
                    })
                    .or_insert_with(|| {
                        let mut send_pulse = Vec::new();
                        send_pulse.push(to_send_pulse);
                        send_pulse
                    });
            }
            continue;
        }
        *pulses = new_pulse;
        panic!("?what type: {}", module.module_type);
    }
    (new_pulse, low_count, high_count)
}

pub fn part02(input: &str) -> Option<i64> {
    return Some(0);
}

// % If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
// & if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Module {
    name: String,
    module_type: String,
    memory: bool,
    ends: Vec<String>,
}
