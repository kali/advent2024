use std::collections::HashSet;

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(p1(&input, 101, 103, 100));
    p2(&input, 101, 103);
    Ok(())
}

fn p1(input: &str, w: isize, h: isize, t: isize) -> usize {
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for bot in input.lines() {
        let (px, py, vx, vy) =
            scan_fmt::scan_fmt!(bot, " p={d},{d} v={d},{d}", isize, isize, isize, isize).unwrap();
        let fx = (px + vx * t).rem_euclid(w);
        let fy = (py + vy * t).rem_euclid(h);
        let left = fx < w / 2;
        let right = fx > w / 2;
        let top = fy < h / 2;
        let bottom = fy > h / 2;
        q1 += (left && top) as usize;
        q2 += (right && top) as usize;
        q3 += (left && bottom) as usize;
        q4 += (right && bottom) as usize;
    }
    q1 * q2 * q3 * q4
}

fn p2(input: &str, w: isize, h: isize) {
    let mut seen = HashSet::new();
    for t in 0.. {
        let bots: HashSet<_> = input
            .lines()
            .map(|line| {
                let (px, py, vx, vy) =
                    scan_fmt::scan_fmt!(line, " p={d},{d} v={d},{d}", isize, isize, isize, isize)
                        .unwrap();
                let fx = (px + vx * t).rem_euclid(w);
                let fy = (py + vy * t).rem_euclid(h);
                (fx, fy)
            })
            .collect();
        if !seen.insert(bots.iter().copied().sorted().collect_vec()) {
            return;
        }
        // adhoc, depends on my case, but looking at the output shown a concentration of bots on
        // the mid column of the picture for these steps
        if t % 101 != 82 {
            continue;
        }
        println!("{t}");
        for y in 0..h {
            for x in 0..w {
                print!("{}", if bots.contains(&(x, y)) { "#" } else { " " });
            }
            println!("");
        }
        println!("");
        println!("");
    }
}

#[test]
fn t() {
    assert_eq!(
        p1(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            11,
            7,
            100
        ),
        12
    );
}
