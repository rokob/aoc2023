use std::collections::{HashMap, HashSet};

use ndarray::{Array2, Axis};
use ndarray_linalg::*;

const DATA: &str = include_str!("day25.txt");

pub fn part1() -> Option<()> {
    let mut intern = HashMap::new();
    let graph = parse(DATA, &mut intern);

    let node_count = graph.links.keys().len();
    let mut lap: Array2<f64> = Array2::zeros((node_count, node_count));
    for (n, es) in graph.links.iter() {
        lap[[*n, *n]] = es.len() as f64;
        for e in es.iter() {
            lap[[*n, *e]] = -1.0;
        }
    }

    let (eigs, vecs) = lap.eig().unwrap();

    let mut i0_val = std::f64::MAX;
    let mut i1 = 0;
    let mut i1_val = std::f64::MAX;
    for (i, e) in eigs.iter().enumerate() {
        if e.re < i0_val {
            i0_val = e.re;
        } else if e.re < i1_val {
            i1 = i;
            i1_val = e.re;
        }
    }
    let v1 = vecs.index_axis(Axis(1), i1);

    let mut a = 0;
    let mut b = 0;
    for v in v1.iter() {
        if v.re < 0.0 {
            a += 1;
        } else {
            b += 1;
        }
    }

    println!("result = {}", a * b);

    Some(())
}

struct Graph {
    links: HashMap<usize, HashSet<usize>>,
    edges: Vec<Edge>,
}

impl Graph {
    fn edge_is(&self, idx: usize, a: usize, b: usize) -> bool {
        let e = &self.edges[idx];
        (e.from == a && e.to == b) || (e.from == b && e.to == a)
    }
}

struct Edge {
    from: usize,
    to: usize,
}

fn parse<'a>(input: &'a str, intern: &mut HashMap<&'a str, usize>) -> Graph {
    let mut edges = Vec::new();
    let mut graph = HashMap::new();

    for line in input.lines() {
        let (name, links) = line.split_once(": ").unwrap();

        let this_idx = get_idx(name, intern);
        let others = links
            .split_whitespace()
            .map(|s| get_idx(s, intern))
            .collect::<Vec<_>>();
        let n = graph.entry(this_idx).or_insert_with(|| HashSet::new());
        for o in others.iter() {
            n.insert(*o);
            edges.push(Edge {
                from: this_idx,
                to: *o,
            });
        }
        for o in others.into_iter() {
            let e = graph.entry(o).or_insert_with(|| HashSet::new());
            e.insert(this_idx);
        }
    }

    Graph {
        links: graph,
        edges,
    }
}

fn get_idx<'a>(s: &'a str, intern: &mut HashMap<&'a str, usize>) -> usize {
    let idx = intern.len();
    *intern.entry(s).or_insert(idx)
}
