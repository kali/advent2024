use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(run(&input, false));
    dbg!(run(&input, true));
    Ok(())
}

fn parse(input: &str) -> (Vec<Vec<u8>>, String, isize, isize) {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = map
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect_vec();
    let moves = moves.lines().join("");
    let (bx, by) = map
        .iter()
        .enumerate()
        .find_map(|(y, l)| {
            l.iter()
                .position(|c| *c == b'@')
                .map(|x| (x as isize, y as isize))
        })
        .unwrap();
    (map, moves, bx, by)
}

fn run(input: &str, p2: bool) -> usize {
    let (mut map, moves, mut bx, mut by) = parse(input);
    if p2 {
        map = map
            .iter()
            .map(|line| {
                line.iter()
                    .flat_map(|c| match c {
                        b'#' => [b'#', b'#'],
                        b'@' => [b'@', b'.'],
                        b'O' => [b'[', b']'],
                        b'.' => [b'.', b'.'],
                        _ => panic!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        bx *= 2;
    }
    macro_rules! at {
        ($x: expr, $y: expr) => {
            map[$y as usize][$x as usize]
        };
    }

    'mov: for m in moves.as_bytes() {
        let (dx, dy) = match m {
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'^' => (0, -1),
            b'v' => (0, 1),
            _ => panic!(),
        };
        fn enque(q: &mut Vec<(isize, isize)>, x: (isize, isize)) {
            if !q.contains(&x) {
                q.push(x);
            }
        }
        let mut moving = vec![(bx, by)];
        let mut checked = 0;
        while checked < moving.len() {
            let (x, y) = moving[checked];
            checked += 1;
            match at!(x + dx, y + dy) {
                b'.' => (),
                b'#' => continue 'mov,
                b'[' if dx == 0 => {
                    enque(&mut moving, (x, y + dy));
                    enque(&mut moving, (x + 1, y + dy));
                }
                b']' if dx == 0 => {
                    enque(&mut moving, (x, y + dy));
                    enque(&mut moving, (x - 1, y + dy));
                }
                _ => {
                    enque(&mut moving, (x + dx, y + dy));
                }
            }
        }
        while let Some((x, y)) = moving.pop() {
            assert!(at!(x + dx, y + dy) == b'.');
            at!(x + dx, y + dy) = at!(x, y);
            at!(x, y) = b'.';
        }
        bx += dx;
        by += dy;
    }

    let mut gps = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if at!(x, y) == b'O' || at!(x, y) == b'[' {
                gps += 100 * y + x;
            }
        }
    }
    gps
}

#[test]
fn t1() {
    assert_eq!(
        run(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
            false
        ),
        2028
    );
}

#[test]
fn t2() {
    assert_eq!(
        run(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
",
            true
        ),
        9021
    );
}
