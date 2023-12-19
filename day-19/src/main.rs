use std::{collections::HashMap, mem::replace};

use aoc_macros::aoc;
use lazy_static::lazy_static;
use regex::Regex;

const REGEX_PATTERN: &str =
    r#"(?P<label>\w+)\{(?P<workflow_lines>(\w+(<|>)\d+:\w+,)+)(?P<no_match>\w+)\}"#;

const PART_PATTERN: &str = r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}";

lazy_static! {
    static ref RE: Regex = Regex::new(REGEX_PATTERN).unwrap();
    static ref PART_RE: Regex = Regex::new(PART_PATTERN).unwrap();
}

#[derive(Clone)]
enum WorkflowNextType {
    Accept,
    Reject,
    Next(String),
}

impl From<&str> for WorkflowNextType {
    fn from(value: &str) -> Self {
        match value {
            "A" => WorkflowNextType::Accept,
            "R" => WorkflowNextType::Reject,
            _ => WorkflowNextType::Next(value.to_string()),
        }
    }
}

enum WorkflowLineType {
    Gt,
    Lt,
}

enum PartType {
    X,
    M,
    A,
    S,
}

impl From<&str> for PartType {
    fn from(value: &str) -> Self {
        match value {
            "x" => PartType::X,
            "m" => PartType::M,
            "a" => PartType::A,
            "s" => PartType::S,
            _ => unimplemented!("Unknown part type"),
        }
    }
}

struct WorkflowLine {
    ty: WorkflowLineType,
    next: WorkflowNextType,
    part: PartType,
    condition: u64,
}

impl From<&str> for WorkflowLine {
    fn from(value: &str) -> Self {
        let (cond, next) = value.split_once(':').unwrap();
        if let Some((part, num)) = cond.split_once('<') {
            Self {
                ty: WorkflowLineType::Lt,
                next: WorkflowNextType::from(next),
                part: PartType::from(part),
                condition: num.parse().unwrap(),
            }
        } else {
            let (part, num) = cond.split_once('>').unwrap();
            Self {
                ty: WorkflowLineType::Gt,
                next: WorkflowNextType::from(next),
                part: PartType::from(part),
                condition: num.parse().unwrap(),
            }
        }
    }
}

impl WorkflowLine {
    fn apply(&self, part: &Part) -> Option<&WorkflowNextType> {
        let value = match self.part {
            PartType::X => part.x,
            PartType::M => part.m,
            PartType::A => part.a,
            PartType::S => part.s,
        };
        let applies = match self.ty {
            WorkflowLineType::Gt => value > self.condition,
            WorkflowLineType::Lt => value < self.condition,
        };
        if applies {
            Some(&self.next)
        } else {
            None
        }
    }
}

struct Workflow {
    label: String,
    lines: Vec<WorkflowLine>,
    no_match: WorkflowNextType,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let mat = RE.captures(value).unwrap();
        let lines: Vec<&str> = mat
            .name("workflow_lines")
            .unwrap()
            .as_str()
            .split(',')
            .collect();
        Self {
            label: mat.name("label").unwrap().as_str().to_string(),
            lines: lines[..lines.len() - 1]
                .iter()
                .map(|l| WorkflowLine::from(*l))
                .collect(),
            no_match: WorkflowNextType::from(mat.name("no_match").unwrap().as_str()),
        }
    }
}

impl Workflow {
    fn apply(&self, part: &Part) -> &WorkflowNextType {
        self.lines
            .iter()
            .find_map(|l| l.apply(part))
            .unwrap_or(&self.no_match)
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mat = PART_RE.captures(value).unwrap();
        Self {
            x: mat.name("x").unwrap().as_str().parse().unwrap(),
            m: mat.name("m").unwrap().as_str().parse().unwrap(),
            a: mat.name("a").unwrap().as_str().parse().unwrap(),
            s: mat.name("s").unwrap().as_str().parse().unwrap(),
        }
    }
}

#[aoc(test = "19114")]
fn part_1(inp: &str) -> u64 {
    let (workflows, parts) = inp.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(Workflow::from)
        .map(|w| (w.label.clone(), w))
        .collect();
    let parts: Vec<_> = parts.lines().map(Part::from).collect();
    parts
        .iter()
        .filter(|p| {
            let mut cur = "in";
            loop {
                let wf = workflows.get(cur).unwrap();
                match wf.apply(p) {
                    WorkflowNextType::Accept => return true,
                    WorkflowNextType::Reject => return false,
                    WorkflowNextType::Next(label) => {
                        cur = label;
                    }
                }
            }
        })
        .map(|p| p.x + p.m + p.a + p.s)
        .sum()
}
type Range = [u64; 2];
type Ranges = [Range; 4];

fn search(workflows: &HashMap<String, Workflow>, cur: &str, ranges: &mut Ranges) -> u64 {
    if ranges.iter().any(|r| r[0] > r[1]) {
        return 0;
    }
    let copy = *ranges;
    let wf = workflows.get(cur).unwrap();
    let mut sum: u64 = 0;
    for line in &wf.lines {
        let part = match line.part {
            PartType::X => 0,
            PartType::M => 1,
            PartType::A => 2,
            PartType::S => 3,
        };
        let bound = match line.ty {
            WorkflowLineType::Gt => 0,
            WorkflowLineType::Lt => 1,
        };
        let copy = ranges[part][bound];
        ranges[part][bound] = line.condition;
        if bound == 0 {
            ranges[part][bound] += 1;
        } else {
            ranges[part][bound] -= 1;
        }

        match &line.next {
            WorkflowNextType::Accept => {
                sum += ranges
                    .iter()
                    .map(|r| r[1].saturating_sub(r[0] - 1))
                    .product::<u64>();
            }
            WorkflowNextType::Reject => {}
            WorkflowNextType::Next(lab) => {
                sum += search(workflows, lab, ranges);
            }
        }
        ranges[part][bound] = copy;
        ranges[part][1 - bound] = line.condition;
    }
    match &wf.no_match {
        WorkflowNextType::Accept => {
            sum += ranges
                .iter()
                .map(|r| r[1].saturating_sub(r[0] - 1))
                .product::<u64>();
        }
        WorkflowNextType::Reject => {}
        WorkflowNextType::Next(lab) => {
            sum += search(workflows, lab, ranges);
        }
    }
    let _ = replace(ranges, copy);
    sum
}

#[aoc(test = "167409079868000")]
fn part_2(inp: &str) -> u64 {
    let (workflows, _) = inp.split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows
        .lines()
        .map(Workflow::from)
        .map(|w| (w.label.clone(), w))
        .collect();
    let mut ranges = [[1_u64, 4000_u64]; 4];
    search(&workflows, "in", &mut ranges)
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
