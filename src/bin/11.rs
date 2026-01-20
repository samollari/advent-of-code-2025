use std::{
    collections::{HashMap, VecDeque},
    hash::RandomState,
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

fn get_graph_routes(graph: Graph, start_node: &str) -> Option<Vec<Vec<String>>> {
    let start_node_string = start_node.to_owned();

    let mut processing_states: HashMap<String, ProcessingState, RandomState> = HashMap::from_iter(
        graph
            .outputs
            .keys()
            .map(|node| (node.clone(), ProcessingState::NotStarted)),
    );
    processing_states.insert(
        "out".to_owned(),
        ProcessingState::Done(vec![vec!["out".to_owned()]]),
    );

    let mut process_queue: VecDeque<String> = VecDeque::new();
    process_queue.push_back(start_node_string.clone());

    loop {
        // println!("{:?}, {:?}", process_queue, processing_states);
        match process_queue.pop_front() {
            Some(node_to_process) => match processing_states.get(&node_to_process) {
                Some(state) => match state {
                    ProcessingState::NotStarted | ProcessingState::InProgress => {
                        let node_children = graph.outputs.get(&node_to_process).unwrap();

                        let childs_paths = node_children
                            .iter()
                            .filter_map(|child| match processing_states.get(child) {
                                Some(ProcessingState::Done(paths)) => Some(
                                    paths
                                        .iter()
                                        .map(|path| {
                                            let mut path = path.clone();
                                            path.insert(0, node_to_process.clone());
                                            path
                                        })
                                        .collect_vec(),
                                ),
                                _ => None,
                            })
                            .collect_vec();

                        let all_done_result = (childs_paths.len() == node_children.len())
                            .then(|| childs_paths.into_iter().flatten().collect_vec());

                        let my_new_state = match all_done_result {
                            Some(paths) => {
                                let mut new_process_queue = VecDeque::from(
                                    graph
                                        .inputs
                                        .get(&node_to_process)
                                        .unwrap_or(&vec![])
                                        .clone(),
                                );
                                new_process_queue.append(&mut process_queue);
                                process_queue = new_process_queue;
                                ProcessingState::Done(paths)
                            }
                            None => {
                                process_queue.append(&mut VecDeque::from(node_children.clone()));
                                ProcessingState::InProgress
                            }
                        };

                        processing_states.insert(node_to_process.clone(), my_new_state.clone());

                        match (node_to_process == start_node_string, my_new_state) {
                            (true, ProcessingState::Done(_)) => break,
                            _ => {}
                        };
                    }
                    ProcessingState::Done(_) => {
                        if node_to_process == start_node_string {
                            break;
                        }
                    }
                },
                None => {
                    println!("{} queued but doesn't exist!", node_to_process);
                    panic!("Node was queued that doesn't exist!")
                }
            },
            None => break,
        }
    }

    let final_result = processing_states.get(start_node);

    assert!(matches!(final_result, Some(ProcessingState::Done(_))));

    match final_result {
        Some(ProcessingState::Done(paths)) => Some(paths.to_owned()),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    get_graph_routes(parse_input(input), "you").map(|paths| paths.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let fft_string = "fft".to_owned();
    let dac_string = "dac".to_owned();

    get_graph_routes(parse_input(input), "svr").map(|paths| {
        paths
            .iter()
            .filter(|path| path.contains(&fft_string) && path.contains(&dac_string))
            .count() as u64
    })
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
