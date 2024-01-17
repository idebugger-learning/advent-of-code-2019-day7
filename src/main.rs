use crate::cpu::CPU;
use itertools::Itertools;

mod cpu;

fn main() {
    let input = include_str!("../data/input.txt");
    let program = input
        .split(',')
        .map(|s| s.parse::<isize>().expect("Failed to parse isize"))
        .collect();

    let phases: Vec<isize> = vec![0, 1, 2, 3, 4];
    let max_thrust = phases
        .iter()
        .permutations(phases.len())
        .map(|phases| {
            let thrust = phases.iter().fold(0, |acc, &&phase| {
                let mut cpu = CPU::new(&program);
                cpu.run(Some(vec![phase, acc]));
                let output = cpu.get_stdout();
                output[0]
            });
            (phases, thrust)
        })
        .max_by_key(|(_, thrust)| *thrust);

    println!("Max thrust: {:?}", max_thrust);
}
