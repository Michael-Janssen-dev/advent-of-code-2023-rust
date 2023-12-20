use std::collections::{HashMap, HashSet, VecDeque};

use aoc_macros::aoc;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
struct ReceivedPulse {
    sender: String,
    pulse: Pulse,
}

impl ReceivedPulse {
    fn new(sender: String, pulse: Pulse) -> Self {
        Self { sender, pulse }
    }
}

trait Gate {
    fn apply(&mut self, pulse: ReceivedPulse) -> Option<Pulse>;
    fn label(&self) -> &str;
    fn after_setup(&mut self, ins: Vec<String>);
}

struct Conjuction {
    label: String,
    ins: Vec<String>,
    mem: HashMap<String, Pulse>,
}

impl Gate for Conjuction {
    fn apply(&mut self, pulse: ReceivedPulse) -> Option<Pulse> {
        *self.mem.get_mut(&pulse.sender).unwrap() = pulse.pulse;
        if self.mem.iter().all(|(_k, v)| *v == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn after_setup(&mut self, ins: Vec<String>) {
        for i in &ins {
            self.mem.insert(i.to_string(), Pulse::Low);
        }
        self.ins = ins;
    }
}

impl Conjuction {
    fn new(label: String) -> Self {
        Self {
            label,
            ins: vec![],
            mem: HashMap::new(),
        }
    }
}

struct Broadcast {
    label: String,
}

impl Gate for Broadcast {
    fn apply(&mut self, pulse: ReceivedPulse) -> Option<Pulse> {
        Some(pulse.pulse)
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn after_setup(&mut self, _ins: Vec<String>) {}
}

impl Broadcast {
    fn new(label: String) -> Self {
        Self { label }
    }
}

struct FlipFlop {
    label: String,
    mem: bool,
}

impl Gate for FlipFlop {
    fn apply(&mut self, pulse: ReceivedPulse) -> Option<Pulse> {
        if pulse.pulse == Pulse::High {
            return None;
        }
        self.mem = !self.mem;
        match self.mem {
            true => Some(Pulse::High),
            false => Some(Pulse::Low),
        }
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn after_setup(&mut self, _ins: Vec<String>) {}
}

impl FlipFlop {
    fn new(label: String) -> Self {
        Self { label, mem: false }
    }
}

fn parse_gate(label: &str) -> Box<dyn Gate> {
    match label.chars().next().unwrap() {
        '&' => Box::new(Conjuction::new(label.chars().skip(1).collect())),
        '%' => Box::new(FlipFlop::new(label.chars().skip(1).collect())),
        _ => Box::new(Broadcast::new(label.to_string())),
    }
}

#[aoc(test = "11687500")]
fn part_1(inp: &str) -> u64 {
    let mut gates: HashMap<String, Box<dyn Gate>> = HashMap::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut conjunctions = HashSet::new();
    for line in inp.lines() {
        let (label, out) = line.split_once(" -> ").unwrap();
        let gate = parse_gate(label);
        if label.starts_with('&') {
            conjunctions.insert(gate.label().to_string());
        }
        edges.insert(
            gate.label().to_string(),
            out.split(", ").map(|s| s.to_string()).collect(),
        );
        gates.insert(gate.label().to_string(), gate);
    }
    for c in &conjunctions {
        gates.get_mut(c).unwrap().after_setup(
            edges
                .iter()
                .filter_map(|(i, out)| {
                    if out.contains(c) {
                        Some(i.to_string())
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
    let mut queue = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        queue.push_back((
            "broadcaster".to_string(),
            ReceivedPulse::new("button".to_string(), Pulse::Low),
        ));
        while let Some((gate, pulse)) = queue.pop_front() {
            match pulse.pulse {
                Pulse::Low => {
                    low_pulses += 1;
                }
                Pulse::High => {
                    high_pulses += 1;
                }
            };
            if let Some(gate) = gates.get_mut(&gate) {
                if let Some(pulse) = gate.apply(pulse) {
                    for g in edges.get(gate.label()).unwrap() {
                        queue.push_back((
                            g.to_string(),
                            ReceivedPulse::new(gate.label().to_string(), pulse.clone()),
                        ));
                    }
                }
            }
        }
    }
    low_pulses * high_pulses
}

fn part_2(inp: &str) -> u64 {
    let mut gates: HashMap<String, Box<dyn Gate>> = HashMap::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut conjunctions = HashSet::new();
    for line in inp.lines() {
        let (label, out) = line.split_once(" -> ").unwrap();
        let gate = parse_gate(label);
        if label.starts_with('&') {
            conjunctions.insert(gate.label().to_string());
        }
        edges.insert(
            gate.label().to_string(),
            out.split(", ").map(|s| s.to_string()).collect(),
        );
        gates.insert(gate.label().to_string(), gate);
    }
    for c in &conjunctions {
        gates.get_mut(c).unwrap().after_setup(
            edges
                .iter()
                .filter_map(|(i, out)| {
                    if out.contains(c) {
                        Some(i.to_string())
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
    let rx = "rx".to_string();
    let before_rx: String = edges
        .iter()
        .find_map(|(i, out)| {
            if out.contains(&rx) {
                Some(i.to_string())
            } else {
                None
            }
        })
        .unwrap();
    let before_rx: Vec<_> = edges
        .iter()
        .filter_map(|(i, out)| {
            if out.contains(&before_rx) {
                Some(i.to_string())
            } else {
                None
            }
        })
        .collect();

    let mut queue = VecDeque::new();
    let mut presses = 0;
    let mut cycles: HashMap<String, u64> = HashMap::new();
    'outer: loop {
        presses += 1;
        queue.push_back((
            "broadcaster".to_string(),
            ReceivedPulse::new("button".to_string(), Pulse::Low),
        ));
        while let Some((gate, pulse)) = queue.pop_front() {
            if before_rx.contains(&gate) && pulse.pulse == Pulse::Low && !cycles.contains_key(&gate)
            {
                cycles.insert(gate.clone(), presses);
                if cycles.len() == before_rx.len() {
                    break 'outer;
                }
            }
            if let Some(gate) = gates.get_mut(&gate) {
                if let Some(pulse) = gate.apply(pulse) {
                    for g in edges.get(gate.label()).unwrap() {
                        queue.push_back((
                            g.to_string(),
                            ReceivedPulse::new(gate.label().to_string(), pulse.clone()),
                        ));
                    }
                }
            }
        }
    }
    cycles.values().product()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
