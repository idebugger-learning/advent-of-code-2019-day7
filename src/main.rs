use crate::cpu::CPU;
use itertools::Itertools;

mod cpu;

fn main() {
    let input = include_str!("../data/input.txt");
    let program = input
        .split(',')
        .map(|s| s.parse::<isize>().expect("Failed to parse isize"))
        .collect();

    let phases: Vec<isize> = vec![5, 6, 7, 8, 9];
    let max_thrust = phases
        .iter()
        .permutations(phases.len())
        .map(|phases| {
            let mut cpus = phases
                .iter()
                .map(|&&phase| {
                    let mut cpu = CPU::new(&program);
                    cpu.push_stdin(phase);
                    cpu
                })
                .collect::<Vec<_>>();

            let mut last_value = 0;
            while !cpus.last().unwrap().is_halted() {
                for cpu in &mut cpus {
                    cpu.push_stdin(last_value);
                    cpu.run();
                    last_value = *cpu.get_stdout().last().unwrap();
                }
            }

            (phases, last_value)
        })
        .max_by_key(|(_, thrust)| *thrust);

    println!("Max thrust: {:?}", max_thrust);
}
