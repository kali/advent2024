use itertools::Itertools;

// P2 MAY NOT WORK ON ARBITRARY INPUT
// observed program:
// * output a value computed from just A (I suspect this is the varying part)
// * A >>= 3
// * repeat until A == 0

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    println!("p1= {}", run(&input, None).iter().join(","));
    let prog = input
        .lines()
        .nth(4)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let p2 = find_seq(&input, &prog, 0, 0).unwrap();
    dbg!(p2);
    Ok(())
}

fn run(input: &str, override_a: Option<usize>) -> Vec<usize> {
    let mut regs = input
        .lines()
        .take(3)
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .collect_vec();
    let prog = input
        .lines()
        .nth(4)
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    macro_rules! combo {
        ($x: expr) => {
            if $x < 4 {
                $x
            } else {
                regs[$x - 4]
            }
        };
    }
    if let Some(override_a) = override_a {
        regs[0] = override_a
    };
    let mut pc = 0;
    let mut out = vec![];
    while pc < prog.len() {
        let (o, v) = (prog[pc], prog[pc + 1]);
        /*
        let mnem = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"][o];
        println!(
            "{pc:2} | {mnem} {:o} |  {:>10o} {:>10o} {:>10o}  |  {:?}",
            v, regs[0], regs[1], regs[2], out
        );
        */
        pc += 2;
        match o {
            0 => {
                regs[0] = regs[0] >> combo!(v);
            }
            1 => {
                regs[1] = regs[1] ^ v;
            }
            2 => {
                regs[1] = combo!(v) % 8;
            }
            3 if regs[0] == 0 => (),
            3 => {
                pc = v;
            }
            4 => regs[1] ^= regs[2],
            5 => {
                out.push(combo!(v) % 8);
            }
            6 => {
                regs[1] = regs[0] >> combo!(v);
            }
            7 => {
                regs[2] = regs[0] >> combo!(v);
            }
            _ => panic!(),
        }
    }
    out
}

fn find_seq(input: &str, seq: &[usize], suffix: usize, reg: usize) -> Option<usize> {
    if suffix == seq.len() {
        Some(reg)
    } else {
        for i in 0..8 {
            let attempt = (reg << 3) | i;
            if attempt == 0 {
                continue;
            }
            let result = run(input, Some(attempt));
            if seq.ends_with(&result) {
                if let Some(x) = find_seq(input, seq, suffix + 1, attempt) {
                    return Some(x);
                }
            }
        }
        None
    }
}


#[test]
fn t() {
    assert_eq!(
        &run(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
            None
        ),
        &[4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
    );
}
