fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    dbg!(p1(&mut build(&input)));
    dbg!(p2(&mut build(&input)));
    Ok(())
}

fn build(input: &str) -> Vec<Option<usize>> {
    let mut chars = input.bytes();
    let mut current = 0;
    let mut map:Vec<Option<usize>> = vec!();
    loop {
        for _ in 0..(chars.next().unwrap() - b'0') {
            map.push(Some(current));
        }
        current += 1;
        if let Some(c) = chars.next() {
            for _ in 0..(c - b'0') {
                map.push(None);
            }
        } else {
            break;
        }
    }
    map
}

fn p1(map: &mut Vec<Option<usize>>) -> usize {
    let mut pos = 0;
    loop {
        while map[map.len() -1 ].is_none() {
            map.truncate(map.len() - 1);
        }
        if pos >= map.len() {
            break;
        }
        if map[pos].is_none() {
            map[pos] = map[map.len() - 1];
            map.truncate(map.len() - 1);
        } else {
            pos += 1;
        }
    }
    map.iter().enumerate().map(|(a,b)| a * b.unwrap_or(0)).sum()
}

fn p2(map: &mut Vec<Option<usize>>) -> usize {
    let mut pos = map.len() - 1;
    loop {
        while map[pos].is_none() {
            pos -= 1;
        }
        let mut start = pos;
        while map[start] == map[pos] && start > 0 {
            start -= 1;
        }
        if start == 0 {
            break;
        }
        start += 1;
        let len = pos - start + 1;
        for i in 0..start {
            if map[i..][..len].iter().all(|x| x.is_none()) {
                for j in 0..len {
                    map[i+j] = map[start + j];
                    map[start+j] = None;
                }
                break;
            }
        }
        pos = start - 1;
    }
    map.iter().enumerate().map(|(a,b)| a * b.unwrap_or(0)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t() {
        assert_eq!(p1(&mut build("2333133121414131402")), 1928);
        assert_eq!(p2(&mut build("2333133121414131402")), 2858);
    }
}
