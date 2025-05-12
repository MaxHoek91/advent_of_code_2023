use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::util::lcm;

pub fn determine_pulse_product(file_path: &str) -> (usize, usize) {
    let data: String = fs::read_to_string(file_path).unwrap();

    let mut network = create_network(&data);

    // Part One
    let mut low_pulse: usize = 0;
    let mut high_pulse: usize = 0;

    for _ in 0..1000 {
        let (low, high, _) = network.push_button("");
        low_pulse += low;
        high_pulse += high;
    }

    // Part Two
    // If all the traced values are "High" we send a signal to "rx"
    // Find when each of these becomes "High" then lcm to find "rx"
    let mut tracers: Vec<&str> = Vec::new();
    for (name, module) in network.modules.iter() {
        if module.operation == Operation::Conjunction
            && network.receivers.get(name).is_some_and(| r | r.contains(&"rx")) {
            tracers.extend(module.conj_states.keys());
        }
    }

    let mut button_pushes: Vec<usize> = Vec::new();

    for trace in tracers.iter() {
        network.reset();
        let mut n_pushes: usize = 0;

        let mut high_pulse_on_trace = false;
        while !high_pulse_on_trace {
            let (_, _, pulse) = network.push_button(trace);
            high_pulse_on_trace = pulse;
            n_pushes += 1;
        }

        button_pushes.push(n_pushes);
    }

    (low_pulse * high_pulse, lcm(&button_pushes))
}


fn create_network(data: &str) -> Network {
    let mut network: Network = Network::new();

    // Parse the lines to create the initial network
    for line in data.lines() {
        let (sender, receiver) = line.split_once(" -> ").unwrap();
        let receiver: Vec<&str> = receiver.split(',').map(| rec | rec.trim()).collect();

        match sender.as_bytes()[0] {
            b'%' => {
                let sender = sender.get(1..).unwrap();
                network.modules.insert(sender, Module::new(Operation::FlipFlop));
                network.receivers.insert(sender, receiver);
            },
            b'&' => {
                let sender = sender.get(1..).unwrap();
                network.modules.insert(sender, Module::new(Operation::Conjunction));
                network.receivers.insert(sender, receiver);
            },
            _ => {
                network.modules.insert(sender, Module::new(Operation::None));
                network.receivers.insert(sender, receiver);
            },
        };
    }

    // Parse the receivers to add the conjunction state logic.
    for (name, receivers) in network.receivers.iter() {
        for rec in receivers {
            if let Some(m) = network.modules.get_mut(rec) {
                m.conj_states.insert(name, false);
            }
        }
    }

    network
}


#[derive(Debug)]
struct Network<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    receivers: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Network<'a> {
    fn new() -> Self {
        Self { modules: HashMap::new(), receivers: HashMap::new() }
    }

    fn reset(&mut self) {
        // Reset everything to the Low pulse state
        for module in self.modules.values_mut() {
            module.flip_state = false;
            for state in module.conj_states.values_mut() {
                *state = false;
            }
        }
    }

    fn push_button(&mut self, trace_input: &str) -> (usize, usize, bool) {
        // Propagate a button press.

        let mut low_pulses: usize = 1;  // Button push is the first low signal.
        let mut high_pulses: usize = 0;
        let mut high_pulse_on_trace: bool = false;

        let mut signals: VecDeque<(&str, &str, bool)> = VecDeque::new();
        signals.push_back(("button", "broadcaster", false));

        while let Some((sender, receiver, sig)) = signals.pop_front() {
            // println!("{sender} -{sig} -> {receiver}");  // Debug print.

            let module = match self.modules.get_mut(receiver) {
                Some(m) => m,
                None => continue  // reached an output module that has no receivers.
            };

            let output = match module.propagate(sender, sig) {
                Some(signal) => signal,
                None => continue  // Signal terminates here (can happen on FlipFlop)
            };

            let next_receivers = self.receivers.get(receiver).unwrap();

            match output {  // Add the pulses to the counters.
                true => high_pulses += next_receivers.len(),
                false => low_pulses += next_receivers.len()
            }

            for rec in next_receivers.iter() {
                signals.push_back((receiver, rec, output));

                if rec == &trace_input && !output {
                    high_pulse_on_trace = true;
                }
            }
        }
        (low_pulses, high_pulses, high_pulse_on_trace)
    }
}

#[derive(Debug)]
struct Module<'a> {
    operation: Operation,
    flip_state: bool,  // false = Low Signal, true = High Signal
    conj_states: HashMap<&'a str, bool>
}

impl<'a> Module<'a> {
    // TODO make cleaner by using an interface
    fn new(operation: Operation) -> Self {
        Self { operation, flip_state: false, conj_states: HashMap::new() }
    }

    fn propagate(&mut self, sender: &'a str, signal: bool) -> Option<bool> {
        // TODO separate into 3 distinct structs
        match self.operation {
            Operation::FlipFlop => self.propagate_flipflop(signal),
            Operation::Conjunction => self.propagate_conjunction(sender, signal),
            Operation::None => Some(signal)
        }
    }

    fn propagate_flipflop(&mut self, signal: bool) -> Option<bool> {
        match signal {
            true => None,
            false if !self.flip_state => { self.flip_state = true; Some(true) },
            false if self.flip_state => { self.flip_state = false; Some(false) },
            _ => unreachable!()
        }
    }

    fn propagate_conjunction(&mut self, sender: &'a str, signal: bool) -> Option<bool> {
        self.conj_states.entry(sender).and_modify(| val| *val = signal);
        match self.conj_states.iter().all(| (_, val) | *val) {
            true => Some(false),
            false => Some(true)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    FlipFlop,
    Conjunction,
    None
}


// type Pulse = bool;
// const LOW: Pulse = false;
// const HIGH: Pulse = true;
//
//
// trait Propagate<'a> {
//     fn propagate(&mut self, sender: &'a str, signal: Pulse) -> Option<Pulse>;
// }
//
//
// struct FlipFlop {
//     state: Pulse
// }
//
// impl FlipFlop {
//     fn new() -> Self { Self { state: LOW } }
// }
//
// impl<'a> Propagate<'a> for FlipFlop {
//     fn propagate(&mut self, sender: &str, signal: Pulse) -> Option<Pulse> {
//         match signal {
//             HIGH => None,
//             LOW if self.state == LOW => { self.state = HIGH; Some(HIGH) },
//             LOW  => { self.state = LOW; Some(LOW) },  // if self.state == HIGH
//         }
//     }
// }
//
//
// struct Conjuction<'a> {
//     states: HashMap<&'a str, Pulse>
// }
//
// impl<'a> Conjuction<'a> {
//     fn new() -> Self { Self { states: HashMap::new() } }
// }
//
// impl<'a> Propagate<'a> for Conjuction<'a> {
//     fn propagate(&mut self, sender: &'a str, signal: Pulse) -> Option<Pulse> {
//         self.states.entry(sender).and_modify(| val| *val = signal);
//         match self.states.iter().all(| (_, val) | *val == HIGH) {
//             HIGH => Some(LOW),
//             LOW => Some(HIGH)
//         }
//     }
// }
//
// struct Broadcast {}
//
// impl Broadcast {
//     fn new() -> Self { Self {} }
// }
//
// impl<'a> Propagate<'a> for Broadcast {
//     fn propagate(&mut self, sender: &str, signal: Pulse) -> Option<Pulse> {
//         Some(signal)
//     }
// }