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
    Done(u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);

    let mut processing_states: HashMap<String, ProcessingState, RandomState> = HashMap::from_iter(
        graph
            .outputs
            .keys()
            .map(|node| (node.clone(), ProcessingState::NotStarted)),
    );
    processing_states.insert("out".to_owned(), ProcessingState::Done(1));

    let mut process_queue: VecDeque<String> = VecDeque::new();
    process_queue.push_back("you".to_owned());

    loop {
        // println!("{:?}, {:?}", process_queue, processing_states);
        match process_queue.pop_front() {
            Some(node_to_process) => match processing_states.get(&node_to_process) {
                Some(state) => match state {
                    ProcessingState::NotStarted | ProcessingState::InProgress => {
                        let node_children = graph.outputs.get(&node_to_process).unwrap();

                        let all_done_result =
                            node_children.iter().fold(Some(0), |acc, child| {
                                match (acc, processing_states.get(child)) {
                                    (Some(acc), Some(ProcessingState::Done(x))) => Some(acc + x),
                                    _ => None,
                                }
                            });

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

                        match (node_to_process == "you".to_owned(), my_new_state) {
                            (true, ProcessingState::Done(_)) => break,
                            _ => {}
                        };
                    }
                    ProcessingState::Done(_) => {
                        if node_to_process == "you".to_owned() {
                            break;
                        }
                    }
                },
                None => panic!("Node was queued that doesn't exist!"),
            },
            None => break,
        }
    }

    let final_result = processing_states.get("you");

    assert!(matches!(final_result, Some(ProcessingState::Done(_))));

    match final_result {
        Some(ProcessingState::Done(paths)) => Some(*paths),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
