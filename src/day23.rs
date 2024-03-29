use std::collections::{HashMap, HashSet, VecDeque};

const DATA: &str = include_str!("day23.txt");

pub fn part1() -> Option<()> {
    let graph = parse(DATA, false);
    let to_visit = topo_sort(&graph);
    let mut dists = Vec::new();
    for _ in graph.nodes.keys() {
        dists.push(std::isize::MIN);
    }
    dists[graph.start.1] = 0;
    for n in to_visit {
        for e in graph.edges.get(&n).unwrap() {
            let alt = dists[n] + e.weight as isize;
            if dists[e.to] < alt {
                dists[e.to] = alt;
            }
        }
    }
    let result = dists[graph.end.1];
    println!("result = {result}");
    Some(())
}

pub fn part2() -> Option<()> {
    let graph = parse(DATA, true);
    let mut best = 0usize;
    let mut seen = vec![false; graph.nodes.len()];
    seen[graph.start.1] = true;
    find_paths(graph.start.1, 0, &graph, seen.as_mut_slice(), &mut best);
    println!("result = {best}");
    Some(())
}

fn find_paths(node: usize, weight: usize, graph: &Graph, seen: &mut [bool], best: &mut usize) {
    if node == graph.end.1 {
        if *best < weight {
            *best = weight;
        }
        return;
    }
    let edges = graph.edges.get(&node).unwrap();
    for e in edges {
        if seen[e.to] {
            continue;
        }
        seen[e.to] = true;
        find_paths(e.to, weight + e.weight, graph, seen, best);
        seen[e.to] = false;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Up,
    Down,
    Left,
    Right,
}

impl Tile {
    fn is_node(&self) -> bool {
        match self {
            Tile::Path | Tile::Forest => false,
            _ => true,
        }
    }

    fn is_walkable(&self) -> bool {
        match self {
            Tile::Forest => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
struct Edge {
    weight: usize,
    to: usize,
}

type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct Graph {
    start: (Pos, usize),
    end: (Pos, usize),
    pos: HashMap<Pos, usize>,
    nodes: HashMap<usize, (Pos, Tile)>,
    edges: HashMap<usize, Vec<Edge>>,
    grid: Vec<Vec<Tile>>,
}

fn topo_sort(graph: &Graph) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    for n in graph.nodes.keys() {
        process_node(graph, *n, &mut visited, &mut stack);
    }
    stack.reverse();
    stack
}

fn process_node(graph: &Graph, node: usize, visited: &mut HashSet<usize>, stack: &mut Vec<usize>) {
    if visited.contains(&node) {
        return;
    }
    for e in graph.edges.get(&node).unwrap() {
        process_node(graph, e.to, visited, stack);
    }
    visited.insert(node);
    stack.push(node);
}

fn parse(input: &str, part2: bool) -> Graph {
    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();
    let mut pos = HashMap::new();
    let mut start = ((0, 0), 0);
    let mut end = (0, 0);
    let mut grid = Vec::new();
    for (r, row) in input.lines().enumerate() {
        let mut rr = Vec::new();
        for (c, v) in row.chars().enumerate() {
            let t = match v {
                '.' => Tile::Path,
                '#' => Tile::Forest,
                '^' => Tile::Up,
                'v' => Tile::Down,
                '>' => Tile::Right,
                '<' => Tile::Left,
                _ => panic!("bad input: {v}"),
            };
            if r == 0 && t == Tile::Path {
                let idx = nodes.len();
                nodes.insert(idx, ((r, c), Tile::Down));
                edges.insert(idx, Vec::new());
                pos.insert((r, c), idx);
                start = ((r, c), idx);
            } else if t == Tile::Path {
                end = (r, c);
            }
            if t.is_node() {
                let idx = nodes.len();
                nodes.insert(idx, ((r, c), t));
                edges.insert(idx, Vec::new());
                pos.insert((r, c), idx);
            }
            rr.push(t);
        }
        grid.push(rr);
    }

    let idx = nodes.len();
    nodes.insert(idx, (end, Tile::Down));
    edges.insert(idx, Vec::new());
    pos.insert(end, idx);

    let graph = Graph {
        start,
        end: (end, idx),
        pos,
        nodes,
        edges,
        grid,
    };

    explore(graph, part2)
}

fn explore(mut graph: Graph, part2: bool) -> Graph {
    let mut todo = VecDeque::new();
    let mut already = HashSet::new();
    todo.push_back(graph.start);
    already.insert(graph.start);
    while !todo.is_empty() {
        let curr = todo.pop_front().unwrap();
        let (to, weight, is_end) = find_next(&graph.grid, &graph, curr);
        let e = graph.edges.get_mut(&curr.1).unwrap();
        e.push(Edge { weight, to });
        if part2 {
            let q = graph.edges.get_mut(&to).unwrap();
            q.push(Edge { weight, to: curr.1 });
        }
        if !is_end {
            let (uno, ns) = next_nodes(&graph.grid, &graph, to, part2);
            let weight = if part2 { 1 } else { 2 };
            let to_idx = if part2 {
                if graph.pos.contains_key(&uno) {
                    continue;
                }
                let idx = graph.nodes.len();
                graph.nodes.insert(idx, (uno, Tile::Path));
                graph.edges.insert(idx, vec![Edge { weight, to }]);
                graph.pos.insert(uno, idx);
                let to_edges = graph.edges.get_mut(&to).unwrap();
                to_edges.push(Edge { weight, to: idx });
                idx
            } else {
                to
            };
            for (n, should_explore) in ns.into_iter() {
                graph
                    .edges
                    .get_mut(&to_idx)
                    .unwrap()
                    .push(Edge { weight, to: n.1 });
                if part2 {
                    graph
                        .edges
                        .get_mut(&n.1)
                        .unwrap()
                        .push(Edge { weight, to: to_idx });
                }
                if should_explore && already.insert(n) {
                    todo.push_back(n);
                }
            }
        }
    }

    graph
}

fn find_next(grid: &Vec<Vec<Tile>>, graph: &Graph, start: (Pos, usize)) -> (usize, usize, bool) {
    let mut last = start.0;
    let (_, t) = graph.nodes.get(&start.1).unwrap();
    let mut curr = match t {
        Tile::Up => (start.0 .0 - 1, start.0 .1),
        Tile::Down => (start.0 .0 + 1, start.0 .1),
        Tile::Left => (start.0 .0, start.0 .1 - 1),
        Tile::Right => (start.0 .0, start.0 .1 + 1),
        _ => panic!("bad start: {start:?}"),
    };
    let mut weight = 1;
    loop {
        weight += 1;
        let n = get_neighbor(grid, curr, last);
        last = curr;
        curr = n;
        if grid[curr.0][curr.1].is_node() || graph.end.0 == curr {
            let is_end = graph.end.0 == curr;
            return (graph.pos.get(&curr).cloned().unwrap(), weight, is_end);
        }
    }
}

fn get_neighbor(grid: &Vec<Vec<Tile>>, curr: Pos, last: Pos) -> Pos {
    for p in [
        (curr.0 - 1, curr.1),
        (curr.0 + 1, curr.1),
        (curr.0, curr.1 - 1),
        (curr.0, curr.1 + 1),
    ] {
        if p == last {
            continue;
        }
        if grid[p.0][p.1].is_walkable() {
            return p;
        }
    }
    panic!("bad graph: curr={curr:?}, last={last:?}");
}

fn next_nodes(
    grid: &Vec<Vec<Tile>>,
    graph: &Graph,
    from: usize,
    part2: bool,
) -> (Pos, Vec<((Pos, usize), bool)>) {
    let (pos, tile) = graph.nodes.get(&from).unwrap();
    let (uno, holes) = match tile {
        Tile::Up => (
            (pos.0 - 1, pos.1),
            [
                ((pos.0 - 1, pos.1 + 1), Tile::Right),
                ((pos.0 - 1, pos.1 - 1), Tile::Left),
                ((pos.0 - 2, pos.1), Tile::Up),
            ],
        ),
        Tile::Down => (
            (pos.0 + 1, pos.1),
            [
                ((pos.0 + 1, pos.1 + 1), Tile::Right),
                ((pos.0 + 1, pos.1 - 1), Tile::Left),
                ((pos.0 + 2, pos.1), Tile::Down),
            ],
        ),
        Tile::Right => (
            (pos.0, pos.1 + 1),
            [
                ((pos.0 + 1, pos.1 + 1), Tile::Down),
                ((pos.0 - 1, pos.1 + 1), Tile::Up),
                ((pos.0, pos.1 + 2), Tile::Right),
            ],
        ),
        Tile::Left => (
            (pos.0, pos.1 - 1),
            [
                ((pos.0 + 1, pos.1 - 1), Tile::Down),
                ((pos.0 - 1, pos.1 - 1), Tile::Up),
                ((pos.0, pos.1 - 2), Tile::Left),
            ],
        ),
        _ => panic!("bad tile"),
    };
    (
        uno,
        holes
            .into_iter()
            .filter_map(|(p, ok)| {
                let t = grid[p.0][p.1];
                if (part2 && t.is_node()) || t == ok || graph.end.0 == p {
                    let i = graph.pos.get(&p).unwrap();
                    Some(((p, *i), t == ok))
                } else {
                    None
                }
            })
            .collect(),
    )
}
