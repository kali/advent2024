use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect_vec();
    let at = |x: isize, y: isize| {
        if x < 0 || y < 0 {
            b'.'
        } else {
            input
                .get(y as usize)
                .and_then(|line| line.get(x as usize))
                .copied()
                .unwrap_or(b'.')
        }
    };
    let mut p1 = 0;
    for (dx, dy) in [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ] {
        for x in 0..input[0].len() as isize {
            for y in 0..input.len() as isize {
                if at(x, y) == b'X'
                    && at(x + dx, y + dy) == b'M'
                    && at(x + 2 * dx, y + 2 * dy) == b'A'
                    && at(x + 3 * dx, y + 3 * dy) == b'S'
                {
                    p1 += 1;
                }
            }
        }
    }
    dbg!(p1);
    let mut p2 = 0;
    for x in 0..input[0].len() as isize {
        for y in 0..input.len() as isize {
            if at(x, y) == b'A' {
                let ne = at(x - 1, y - 1);
                let nw = at(x - 1, y + 1);
                let sw = at(x + 1, y + 1);
                let se = at(x + 1, y - 1);
                if ne.min(sw) == b'M'
                    && ne.max(sw) == b'S'
                    && nw.min(se) == b'M'
                    && nw.max(se) == b'S'
                {
                    p2 += 1;
                }
            }
        }
    }
    dbg!(p2);
    Ok(())
}
