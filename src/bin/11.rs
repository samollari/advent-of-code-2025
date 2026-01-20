use std::{
    collections::{HashMap, VecDeque},
    iter::once,
};

use itertools::Itertools;

advent_of_code::solution!(11);

struct Graph {
    outputs: HashMap<String, Vec<String>>,
    inputs: HashMap<String, Vec<String>>,
}

fn parse_input(input: &str) -> Graph {
    let output_list = input.trim().lines().map(|line| {
        let (from, to_devices) = line.trim().split_once(": ").unwrap();
        (
            from.to_string(),
            to_devices
                .trim()
                .split(' ')
                .map(str::to_string)
                .collect_vec(),
        )
    });

    let outputs = output_list.clone().collect::<HashMap<_, _>>();

    let inputs = output_list
        .flat_map(|(from, to_devices)| {
            to_devices
                .iter()
                .map(|to_device| (to_device.clone(), from.clone()))
                .collect_vec()
        })
        .into_group_map();

    Graph { outputs, inputs }
}

#[derive(Debug, Clone)]
enum ProcessingState {
    NotStarted,
    InProgress,
    Done(Vec<Vec<String>>),
}

fn get_graph_routes<'a>(graph: &'a Graph, start_node: &String) -> Vec<VecDeque<String>> {
    if *start_node == "out".to_string() {
        return vec![VecDeque::from([start_node.clone()])];
    }

    let empty_list = vec![];
    let children = graph.outputs.get(start_node).unwrap_or(&empty_list);

    let _a = children
        .iter()
        .flat_map(|child| {
            get_graph_routes(graph, child).into_iter().map(|mut path| {
                path.push_front(start_node.clone());
                path
            })
        })
        .collect();
    _a
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(get_graph_routes(&parse_input(input), &"you".to_string()).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let fft_string = "fft".to_owned();
    let dac_string = "dac".to_owned();

    Some(
        get_graph_routes(&parse_input(input), &"svr".to_string())
            .into_iter()
            .filter(|path| path.contains(&fft_string) && path.contains(&dac_string))
            .count() as u64,
    )
}

const EXAMPLE_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(EXAMPLE_2);
        assert_eq!(result, Some(2));
    }
}
