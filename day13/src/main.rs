fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input")?.trim().to_string();
    for part in 0..2 {
        let mut tokens = 0;
        for machine in input.split("\n\n") {
            let (ax, ay, bx, by, mut x, mut y) = scan_fmt::scan_fmt!(
                machine,
                "Button A: X+{d}, Y+{d}
        Button B: X+{d}, Y+{d}
        Prize: X={d}, Y={d}",
                i64,
                i64,
                i64,
                i64,
                i64,
                i64
            )
            .unwrap();
            x += 10000000000000 * part;
            y += 10000000000000 * part;
            // a * ax + b * bx = x     )   *ay
            // a * ay + b * by = y     ) - *ax
            // b * (bx * ay - ax * by) = x * ay - y * ax
            assert!(bx * ay != ax * by);
            // b = (x * ay - y * ax) / (bx ay - ax by)
            let b = (x * ay - y * ax) / (bx * ay - ax * by);
            let a = (x - b * bx) / ax;
            if a * ax + b * bx == x
                && a * ay + b * by == y
                && a >= 0
                && b >= 0
            {
                tokens += 3 * a + b;
            }
        }
        dbg!(tokens);
    }
    Ok(())
}
