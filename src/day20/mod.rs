use std::collections::{HashMap, VecDeque};

pub fn part01(input: &str) -> Option<i64> {
    return Some(part_one(input).unwrap() as i64);
}

// fn next_pulse(
// pulses: &mut LinkedHashMap<String, Vec<bool>>,
// broad: &Module,
// modules: &mut HashMap<String, Module>,
// ) -> (LinkedHashMap<String, Vec<bool>>, usize, usize) {
//     let mut high_count = 0;
//     let mut low_count = 0;
//     loop {
//         let pulse_opt = pulses.pop_front();
//         if pulse_opt.is_none() {
//             // if new_pulse.len() != 0 {
//             //     let res = next_pulse(&mut new_pulse, broad, modules);
//             //     return (res.0, res.1 + low_count, res.2 + high_count);
//             // }
//             return (pulses.clone(), low_count, high_count);
//         }
//         let pulse = pulse_opt.unwrap();
//         for sig in &pulse.1 {
//             println!(" -{}-> {}", if *sig { "high" } else { "low" }, pulse.0);
//         }
//         for p in &pulse.1 {
//             if *p {
//                 high_count += 1;
//             } else {
//                 low_count += 1;
//             }
//         }
//         if pulse.0 == "broadcaster" {
//             for end in &broad.ends {
//                 pulses
//                     .entry(end.clone())
//                     .and_modify(|e| {
//                         e.append(&mut pulse.1.clone());
//                     })
//                     .or_insert_with(|| pulse.1.clone());
//             }
//             continue;
//         }
//         let module_opt = modules.get_mut(&pulse.0);
//         if module_opt.is_none() {
//             // println!("missed {:?}", pulse);
//             continue;
//         }
//         let module = module_opt.unwrap();
//         if module.module_type == "%" {
//             for p_signal in pulse.1 {
//                 if p_signal {
//                     continue;
//                 }
//                 let to_send_pulse = if module.memory {
//                     module.memory = false;
//                     false
//                 } else {
//                     module.memory = true;
//                     true
//                 };
//                 for end in &module.ends {
//                     pulses
//                         .entry(end.clone())
//                         .and_modify(|e| {
//                             e.push(to_send_pulse);
//                         })
//                         .or_insert_with(|| {
//                             let mut send_pulse = Vec::new();
//                             send_pulse.push(to_send_pulse);
//                             send_pulse
//                         });
//                     // new_pulse
//                     //     .entry(end.clone())
//                     //     .and_modify(|e| {
//                     //         e.push(to_send_pulse);
//                     //     })
//                     //     .or_insert_with(|| {
//                     //         let mut send_pulse = Vec::new();
//                     //         send_pulse.push(to_send_pulse);
//                     //         send_pulse
//                     //     });
//                 }
//             }

//             continue;
//         }

//         if module.module_type == "&" {
//             let mut to_send_pulse = !module.memory;
//             for p in pulse.1 {
//                 module.memory = p;
//                 if !p {
//                     to_send_pulse = true;
//                     break;
//                 }
//             }

//             for end in &module.ends {
//                 pulses
//                     .entry(end.clone())
//                     .and_modify(|e| {
//                         e.push(to_send_pulse);
//                     })
//                     .or_insert_with(|| {
//                         let mut send_pulse = Vec::new();
//                         send_pulse.push(to_send_pulse);
//                         send_pulse
//                     });
//             }
//             continue;
//         }
//         panic!("?what type: {}", module.module_type);
//     }
// }

pub fn part02(input: &str) -> Option<i64> {
    return Some(part_two(input).unwrap() as i64);
}

// % If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
// & if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct Module {
//     name: String,
//     module_type: String,
//     memory: bool,
//     ends: Vec<String>,
// }

pub fn part_one(input: &str) -> Option<u64> {
    let mut network = parse(input);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut queue = VecDeque::with_capacity(64);
    for _ in 0..1000 {
        queue.push_back((network.broadcaster_id, false, network.broadcaster_id));
        while let Some((id, pulse, from)) = queue.pop_front() {
            if pulse {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            if let Some(outgoing_pulse) = network.machines[id].receive_pulse(from, pulse) {
                for &outgoing_id in &network.cables[id] {
                    queue.push_back((outgoing_id, outgoing_pulse, id));
                }
            }
        }
    }
    Some(low_pulses * high_pulses)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut network = parse(input);
    let mut button_presses = 0;
    let mut cycles = Vec::with_capacity(network.leads_to_rx.len());
    let mut found_cycle = [false; 64];
    let mut queue = VecDeque::with_capacity(64);
    while cycles.len() < network.leads_to_rx.len() {
        button_presses += 1;
        queue.push_back((network.broadcaster_id, false, network.broadcaster_id));
        let mut pulsed = [false; 64];
        while let Some((id, pulse, from)) = queue.pop_front() {
            if !pulse {
                pulsed[id] = true;
            }
            if let Some(outgoing_pulse) = network.machines[id].receive_pulse(from, pulse) {
                for &outgoing_id in &network.cables[id] {
                    queue.push_back((outgoing_id, outgoing_pulse, id));
                }
            }
        }
        for &id in &network.leads_to_rx {
            if pulsed[id] && !found_cycle[id] {
                cycles.push(button_presses);
                found_cycle[id] = true;
            }
        }
    }
    cycles.into_iter().reduce(num_integer::lcm)
}

fn parse<'a>(input: &'a str) -> Network {
    let mut network = Network::default();
    network.machines.resize_with(64, Module::default);
    network.cables.resize_with(64, || Vec::with_capacity(64));
    let mut next_id = 0usize;
    let mut label_to_id = HashMap::new();
    let mut get_id = |label: &'a str| {
        let id = *label_to_id.entry(label).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        id
    };
    let mut before_rx_id = 0;
    for line in input.lines() {
        let (module, cables) = line.split_once(" -> ").unwrap();
        let (label, module) = match module.chars().next().unwrap() {
            '%' => (&module[1..], Module::FlipFlop { on: false }),
            '&' => (
                &module[1..],
                Module::Conjunction {
                    last_pulses: 0,
                    pulse_mask: u64::MAX,
                },
            ),
            'b' => (module, Module::Broadcaster),
            _ => (module, Module::Output),
        };
        let id = get_id(label);
        if matches!(module, Module::Broadcaster) {
            network.broadcaster_id = id;
        }
        network.machines[id] = module;
        for next_label in cables.split(", ") {
            let next_id = get_id(next_label);
            network.cables[id].push(next_id);
            if next_label == "rx" {
                before_rx_id = id;
            }
        }
    }
    for id in 0..next_id {
        let num_cables = network.cables[id].len();
        for cable_index in 0..num_cables {
            let next_id = network.cables[id][cable_index];
            if let Module::Conjunction {
                last_pulses: _,
                pulse_mask,
            } = &mut network.machines[next_id]
            {
                *pulse_mask &= !(1 << id);
            }
            if next_id == before_rx_id {
                network.leads_to_rx.push(id);
            }
        }
    }
    network
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
enum Module {
    FlipFlop {
        on: bool,
    },
    Conjunction {
        last_pulses: u64,
        pulse_mask: u64,
    },
    Broadcaster,
    #[default]
    Output,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Network {
    broadcaster_id: usize,
    machines: Vec<Module>,
    cables: Vec<Vec<usize>>,
    leads_to_rx: Vec<usize>,
}

impl Module {
    pub fn receive_pulse(&mut self, from: usize, pulse: bool) -> Option<bool> {
        match self {
            Module::FlipFlop { on: _ } if pulse => None,
            Module::FlipFlop { on } => {
                let new_on = !*on;
                *on = new_on;
                Some(new_on)
            }
            Module::Conjunction {
                last_pulses,
                pulse_mask,
            } => {
                if pulse {
                    *last_pulses |= 1 << from;
                } else {
                    *last_pulses &= !(1 << from);
                }
                Some((*last_pulses) | (*pulse_mask) != u64::MAX)
            }
            Module::Broadcaster => Some(pulse),
            Module::Output => None,
        }
    }
}
