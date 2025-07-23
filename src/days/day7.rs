use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Add;

pub fn part1(input: &str) -> String {
    let parsed_nodes = parse(input);
    let tree = build_tree(parsed_nodes);

    tree.name
}

fn parse(input: &str) -> Vec<ParsedNode> {
    let regex = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.+))?$").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = regex
                .captures(line)
                .unwrap_or_else(|| panic!("Invalid input: {line}"));

            let name = captures
                .get(1)
                .unwrap_or_else(|| panic!("Missing name: {line}"))
                .as_str()
                .to_string();

            let weight = captures
                .get(2)
                .unwrap_or_else(|| panic!("Missing weight: {line}"))
                .as_str()
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Invalid weight: {line}"));

            let children = captures.get(3).map(|m| m.as_str()).map_or(vec![], |s| {
                s.split(", ").map(|s| s.to_string()).collect_vec()
            });

            ParsedNode {
                name,
                weight,
                children,
            }
        })
        .collect_vec()
}

struct ParsedNode {
    name: String,
    weight: u32,
    children: Vec<String>,
}

fn build_tree(parsed_nodes: Vec<ParsedNode>) -> Node {
    let node_map = parsed_nodes
        .into_iter()
        .map(|node| (node.name.clone(), node))
        .collect::<HashMap<_, _>>();

    let root_name = find_root(&node_map);

    build_tree_rec(&node_map, &root_name)
}

fn find_root(node_map: &HashMap<String, ParsedNode>) -> String {
    let nodes = node_map.values().collect_vec();

    let mut potential_roots = node_map
        .keys()
        .map(|k| (k, true))
        .collect::<HashMap<_, _>>();

    for node in nodes {
        for child in &node.children {
            potential_roots.remove(child);
        }
    }

    potential_roots
        .keys()
        .exactly_one()
        .unwrap_or_else(|_| {
            panic!(
                "Expected exactly one root node, found {}",
                potential_roots.len()
            )
        })
        .to_string()
}

fn build_tree_rec(parsed_nodes: &HashMap<String, ParsedNode>, name: &str) -> Node {
    let parsed_node = &parsed_nodes[name];

    Node {
        name: parsed_node.name.clone(),
        weight: parsed_node.weight,
        children: parsed_node
            .children
            .iter()
            .map(|child_name| build_tree_rec(parsed_nodes, child_name))
            .collect(),
    }
}

struct Node {
    name: String,
    weight: u32,
    children: Vec<Node>,
}

impl Node {
    fn total_weight(&self) -> u32 {
        if self.children.is_empty() {
            return self.weight;
        }

        self.children
            .iter()
            .map(|child| child.total_weight())
            .sum::<u32>()
            .add(self.weight)
    }
}

pub fn part2(input: &str) -> String {
    let parsed_nodes = parse(input);
    let tree = build_tree(parsed_nodes);

    if let Some(result) = find_imbalance(&tree) {
        result.to_string()
    } else {
        panic!("No imbalance found in the tree");
    }
}

fn find_imbalance(tree: &Node) -> Option<u32> {
    if tree.children.is_empty() {
        return None;
    }

    let nodes_by_total_weight = tree
        .children
        .iter()
        .into_group_map_by(|node| node.total_weight());

    if nodes_by_total_weight.len() <= 1 {
        return None;
    }

    let unbalanced = nodes_by_total_weight
        .values()
        .find(|group| group.len() == 1)
        .and_then(|group| group.first())?;

    // Try going deeper into the tree to find an imbalance.
    if let Some(value) = find_imbalance(unbalanced) {
        return Some(value);
    }

    let balanced_weight = nodes_by_total_weight
        .iter()
        .find(|(_, group)| group.len() > 1)
        .map(|(total_weight, _)| total_weight)?;

    let difference = unbalanced.total_weight().abs_diff(*balanced_weight);

    Some(unbalanced.weight - difference)
}

pub fn run(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "pbga (66)\n\
            xhth (57)\n\
            ebii (61)\n\
            havc (66)\n\
            ktlj (57)\n\
            fwft (72) -> ktlj, cntj, xhth\n\
            qoyq (66)\n\
            padx (45) -> pbga, havc, qoyq\n\
            tknk (41) -> ugml, padx, fwft\n\
            jptl (61)\n\
            ugml (68) -> gyxo, ebii, jptl\n\
            gyxo (61)\n\
            cntj (57)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "tknk");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "60");
    }
}
