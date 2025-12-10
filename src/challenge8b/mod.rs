use linked_hash_map::LinkedHashMap;
use std::collections::HashSet;

pub fn challenge8b(input: String) -> i64 {
    let points = input
        .lines()
        .map(|ln| {
            let mut ln = ln.split(",").map(|n| n.parse::<i64>().unwrap());
            (ln.next().unwrap(), ln.next().unwrap(), ln.next().unwrap())
        })
        .collect::<Vec<_>>();
    let mut distances = Vec::new();
    for (i, &p1) in points.iter().enumerate() {
        for (j, &p2) in points.iter().skip(i + 1).enumerate() {
            distances.push((i, j + i + 1, distance(p1, p2)));
        }
    }
    distances.sort_by_key(|(_, _, d)| *d);

    let mut edges: LinkedHashMap<usize, HashSet<usize>> =
        (0..points.len()).fold(LinkedHashMap::new(), |mut acc, p| {
            acc.insert(p, HashSet::new());
            acc
        });
    for (p1, p2, _) in distances {
        edges.entry(p1).or_default().insert(p2);
        edges.entry(p2).or_default().insert(p1);

        if find_circuits(&edges, points.len()).len() == 1 {
            return points[p1].0 * points[p2].0;
        }
    }

    0
}

#[allow(unused)]
fn format_point(points: &[Point], idx: usize) -> String {
    let (x, y, z) = points[idx];
    format!("{x},{y},{z}")
}

fn find_circuits(edges: &LinkedHashMap<usize, HashSet<usize>>, size: usize) -> Vec<Vec<usize>> {
    let mut visited = vec![false; size];

    let mut circuits = Vec::new();
    for &k in edges.keys() {
        if !visited[k] {
            let mut circuit = Vec::new();
            find_circuit(edges, k, &mut visited, &mut circuit);
            circuits.push(circuit);
        }
    }
    circuits
}

fn find_circuit(
    edges: &LinkedHashMap<usize, HashSet<usize>>,
    k: usize,
    visited: &mut Vec<bool>,
    res: &mut Vec<usize>,
) {
    visited[k] = true;
    res.push(k);
    for &next in edges.get(&k).unwrap() {
        if !visited[next] {
            find_circuit(edges, next, visited, res);
        }
    }
}

fn distance((x1, y1, z1): Point, (x2, y2, z2): Point) -> i64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let dz = z1 - z2;
    dx * dx + dy * dy + dz * dz
}

type Point = (i64, i64, i64);

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge8b::challenge8b;

    #[test]
    fn test() {
        let input = indoc! {"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "}
        .to_string();
        assert_eq!(challenge8b(input), 25272);
    }
}
