use aoc_macros::aoc;

fn hash(inp: &str) -> u32 {
    inp.chars()
        .fold(0_u32, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

#[aoc(test = "1320")]
fn part_1(inp: &str) -> u32 {
    let commands = inp.split(',');
    commands.map(hash).sum()
}

enum CommandType {
    Set(u32),
    Remove,
}

impl From<&str> for CommandType {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            '=' => CommandType::Set(value.chars().skip(1).collect::<String>().parse().unwrap()),
            '-' => CommandType::Remove,
            _ => unimplemented!("Unknown command type"),
        }
    }
}

struct Command {
    label: String,
    ty: CommandType,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let label: String = value.chars().take_while(|x| x.is_alphabetic()).collect();
        let ty: String = value.chars().skip_while(|x| x.is_alphabetic()).collect();
        Self {
            label,
            ty: CommandType::from(ty.as_ref()),
        }
    }
}

#[aoc(test = "145")]
fn part_2(inp: &str) -> u32 {
    let commands: Vec<Command> = inp.split(',').map(Command::from).collect();
    let mut map: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    for command in &commands {
        let box_id = hash(&command.label);
        match command.ty {
            CommandType::Set(number) => {
                let bx = &mut map[box_id as usize];
                let mut found = false;
                for b in bx.iter_mut() {
                    if b.0 == command.label {
                        b.1 = number;
                        found = true;
                    }
                }
                if !found {
                    bx.push((command.label.clone(), number))
                }
            }
            CommandType::Remove => {
                let index = map[box_id as usize]
                    .iter()
                    .position(|x| x.0 == command.label);
                if let Some(x) = index {
                    map[box_id as usize].remove(x);
                }
            }
        }
    }
    map.iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(|(j, y)| y.1 * (i + 1) as u32 * (j + 1) as u32)
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}
