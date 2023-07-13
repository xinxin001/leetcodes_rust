fn main() {}
use std::collections::{HashMap, HashSet};

fn dfs(
    node: &str,
    target: &str,
    nodes: &HashMap<String, HashSet<String>>,
    edges: &HashMap<(String, String), f64>,
    visited: &mut HashSet<String>,
    output: f64,
) -> f64 {
    if node == target {
        return output;
    }
    nodes[node].iter().fold(-1.0, |valid_calc, neighbor| {
        if visited.contains(neighbor) {
            return valid_calc;
        }
        visited.insert(neighbor.clone());
        let transform = edges[&(node.to_string(), neighbor.clone())];
        let calc = output * transform;
        f64::max(
            valid_calc,
            dfs(neighbor, target, nodes, edges, visited, calc),
        )
    })
}

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let (mut nodes, mut edges) = (HashMap::new(), HashMap::new());

    for (i, eq) in equations.iter().enumerate() {
        let (el1, el2) = (eq[0].clone(), eq[1].clone());
        nodes
            .entry(el1.clone())
            .or_insert_with(HashSet::new)
            .insert(el2.clone());
        nodes
            .entry(el2.clone())
            .or_insert_with(HashSet::new)
            .insert(el1.clone());
        edges.insert((el1.clone(), el2.clone()), values[i]);
        edges.insert((el2, el1), 1.0 / values[i]);
    }

    queries
        .into_iter()
        .map(|query| {
            let (el1, el2) = (query[0].as_str(), query[1].as_str());
            if nodes.contains_key(el1) && nodes.contains_key(el2) {
                let mut visited = HashSet::new();
                visited.insert(el1.to_string());
                dfs(el1, el2, &nodes, &edges, &mut visited, 1.0)
            } else {
                -1.0
            }
        })
        .collect()
}
