use std::collections::{HashMap, HashSet};

fn parse_connections(input: &str) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.trim().split('-');
        let node1 = parts.next().unwrap().to_string();
        let node2 = parts.next().unwrap().to_string();

        // Add both directions since its bidirectional
        graph
            .entry(node1.clone())
            .or_default()
            .insert(node2.clone());
        graph.entry(node2).or_default().insert(node1);
    }

    graph
}

fn find_triangles(graph: &HashMap<String, HashSet<String>>) -> Vec<Vec<String>> {
    let mut triangles = Vec::new();
    let nodes: Vec<String> = graph.keys().cloned().collect();

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            // Check if first two nodes are connected
            if !graph[&nodes[i]].contains(&nodes[j]) {
                continue;
            }

            for k in (j + 1)..nodes.len() {
                // Check if third node is connected to both other nodes

                if graph[&nodes[i]].contains(&nodes[k]) && graph[&nodes[j]].contains(&nodes[k]) {
                    let mut triangle = vec![nodes[i].clone(), nodes[j].clone(), nodes[k].clone()];
                    triangle.sort(); // Sort for consistent ordering
                    triangles.push(triangle);
                }
            }
        }
    }

    triangles
}

fn find_largest_network(graph: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut largest_network = Vec::new();
    let nodes: Vec<String> = graph.keys().cloned().collect();

    // Try each node as a starting point
    for start_node in &nodes {
        let mut current_network = vec![start_node.clone()];
        let mut candidates: HashSet<_> = graph[start_node].clone();

        // Keep adding nodes that are connected to all current nodes
        while !candidates.is_empty() {
            let mut next_candidates = HashSet::new();

            'candidate_loop: for candidate in &candidates {
                // Check if this candidate is connected to all nodes in current network
                for network_node in &current_network {
                    if !graph[candidate].contains(network_node) {
                        continue 'candidate_loop;
                    }
                }
                // If we get here, the candidate is connected to all current nodes
                next_candidates.insert(candidate.clone());
            }

            if next_candidates.is_empty() {
                break;
            }

            // Add the first valid candidate (they're all equivalent at this point)
            let next_node = next_candidates.iter().next().unwrap().clone();
            current_network.push(next_node.clone());

            // Update candidates to be the intersection of current candidates and neighbors of new node
            candidates = candidates
                .intersection(&graph[&next_node])
                .cloned()
                .collect();
        }

        if current_network.len() > largest_network.len() {
            largest_network = current_network;
        }
    }

    largest_network.sort(); // Sort alphabetically
    largest_network
}

fn main() {
    // read input file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    // parse connections into graph
    let graph = parse_connections(&input);

    // find all triangles
    let triangles = find_triangles(&graph);

    // Part 1
    println!("\nPart 1:");
    // print results
    println!("Found {} triangles:", triangles.len());

    // Count triangles with 't' nodes.
    let t_triangles = triangles
        .iter()
        .filter(|triangle| triangle.iter().any(|node| node.starts_with('t')))
        .count();

    println!("Triangles with a 't' node: {}", t_triangles);

    // Part 2
    println!("\nPart 2:");
    let largest_network = find_largest_network(&graph);
    println!("Largest network size: {}", largest_network.len());
    println!("Nodes: {}", largest_network.join(","));
}
