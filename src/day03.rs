use std;

fn num_ports(n: u64) -> u64 {
    let sum = (n * (n + 1)) / 2;
    return 2 + sum * 8;
}

fn ports(n: u64) -> Vec<u64> {
    let count = n * 8;
    let start = num_ports(n - 1);
    (0..count).map(|i| start + i).collect()
}

fn port_location(p: u64) -> u64 {
    let t = ((p - 2) / 8) * 2;
    let d = 1 + 4 * t;
    let s = ((d as f64).sqrt() as u64 - 1) / 2;
    s
}

fn port_offsets(n: u64) -> Vec<u64> {
    (0..4).map(|i| (n - 1) * 8 + 1 + i * 2).collect()
}

fn adj_port(port: u64) -> u64 {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    let l = port_location(port) + 1;
    let perimeter = ports(l);
    let corners: HashSet<u64> = HashSet::from_iter(
        (1..5).map(|i| l * i * 2 - 1).map(|i| perimeter[i as usize])
    );
    if corners.contains(&port) {
        port - 1
    } else {
        let quadrant = (port - perimeter[0]) as usize / (perimeter.len() / 4);
        port - port_offsets(l)[quadrant]
    }
}

pub fn manhattan_distance(port: u64) -> u64 {
    let mut current = port;
    let mut steps = 0;
    while current != 1 {
        steps += 1;
        current = adj_port(current);
    }
    return steps;
}

fn neighbors(i: usize, j: usize, dim: usize) -> Vec<(usize, usize)> {
    let si = i as i64;
    let sj = j as i64;
    let n = vec![
        (si-1, sj-1), (si, sj-1), (si+1, sj-1),
        (si-1, sj), (si+1, sj),
        (si-1, sj+1), (si, sj+1), (si+1, sj+1),
    ];
    n.into_iter().
        filter(|&(x, y)| !(x < 0 || y < 0 || x as usize == dim || y as usize == dim)).
        map(|(x, y)| (x as usize, y as usize)).collect()
}

fn sum_adj_ports(m: &Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    let n = neighbors(i, j, m.len());
    n.iter().fold(0, |sum, &(x, y)| sum + std::cmp::max(0, m[x][y]))
}

pub fn next_in_sum_spiral(n: i64) -> i64 {
    enum Direction {
        Right,
        Up,
        Left,
        Down,
    }

    let dim = 16 as usize;
    let mut m = vec![vec![-1 as i64; dim]; dim];
    let mut dir = Direction::Down;
    let mut i = dim / 2;
    let mut j = dim / 2;

    loop {
        let sum = std::cmp::max(1, sum_adj_ports(&m, i, j));
        if sum > n {
            return sum;
        }
        m[i][j] = sum;

        match dir {
            Direction::Right => if m[i-1][j] == -1 { dir = Direction::Up; },
            Direction::Up => if m[i][j-1] == -1 { dir = Direction::Left; },
            Direction::Left => if m[i+1][j] == -1 { dir = Direction::Down; },
            Direction::Down => if m[i][j+1] == -1 { dir = Direction::Right; },
        }

        match dir {
            Direction::Right => j += 1,
            Direction::Up => i -= 1,
            Direction::Left => j -= 1,
            Direction::Down => i += 1,
        }

        assert!(j < dim, "Matrix overflowed");
    }
}
