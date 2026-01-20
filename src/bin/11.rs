use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::RandomState,
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
    Done(u64),
}

fn get_graph_routes<'a>(
    graph: &'a Graph,
    start_node: &String,
    cache: &mut HashMap<String, Vec<VecDeque<String>>>,
) -> Vec<VecDeque<String>> {
    match cache.get(start_node) {
        Some(cache_entry) => cache_entry.clone(),
        None => {
            let empty_list = vec![];
            let children = graph.outputs.get(start_node).unwrap_or(&empty_list);

            let child_paths: Vec<VecDeque<String>> = children
                .iter()
                .flat_map(|child| {
                    get_graph_routes(graph, child, cache)
                        .into_iter()
                        .map(|mut path| {
                            path.push_front(start_node.clone());
                            path
                        })
                })
                .collect();

            let this_node_has_multiple_inputs = graph
                .inputs
                .get(start_node)
                .map_or(false, |inputs| inputs.len() > 1);
            if this_node_has_multiple_inputs {
                cache.insert(start_node.clone(), child_paths.clone());
            }

            child_paths
        }
    }
}

fn graph_routes(input: &str, start_node: &str) -> Vec<VecDeque<String>> {
    let out_string = "out".to_string();
    let mut cache = HashMap::from_iter(once((
        out_string.clone(),
        vec![VecDeque::from([out_string])],
    )));

    get_graph_routes(&parse_input(input), &start_node.to_string(), &mut cache)
}

fn double_tree_shake(graph: &Graph, start_node: &str, end_node: &str) -> HashSet<String> {
    let mut found_going_down = HashSet::new();
    let mut process_queue = VecDeque::from([start_node.to_string()]);

    loop {
        match process_queue.pop_front() {
            Some(node_to_process) => {
                let node_is_new = found_going_down.insert(node_to_process.clone());
                if !node_is_new {
                    continue;
                }

                process_queue.append(&mut VecDeque::from(
                    graph
                        .outputs
                        .get(&node_to_process)
                        .unwrap_or(&vec![])
                        .clone(),
                ));
            }
            None => break,
        }
    }

    assert!(process_queue.is_empty());
    let found_going_down = found_going_down;

    let mut found_going_up = HashSet::new();

    process_queue.push_back(end_node.to_string());

    loop {
        match process_queue.pop_front() {
            Some(node_to_process) => {
                let node_is_new = found_going_up.insert(node_to_process.clone());
                if !node_is_new {
                    continue;
                }

                process_queue.append(&mut VecDeque::from(
                    graph
                        .inputs
                        .get(&node_to_process)
                        .unwrap_or(&vec![])
                        .clone(),
                ));
            }
            None => break,
        }
    }

    assert!(process_queue.is_empty());
    let found_going_up = found_going_up;

    found_going_down
        .intersection(&found_going_up)
        .map(String::to_owned)
        .collect()
}

fn count_paths(
    graph: &Graph,
    start_node: &str,
    end_node: &str,
    restrict_to_nodes: HashSet<String>,
) -> u64 {
    let mut processing_states: HashMap<String, ProcessingState, RandomState> =
        HashMap::from_iter(graph.outputs.keys().filter_map(|node| {
            restrict_to_nodes
                .contains(node)
                .then(|| (node.clone(), ProcessingState::NotStarted))
        }));
    processing_states.insert(end_node.to_owned(), ProcessingState::Done(1));

    let mut process_queue: VecDeque<String> = VecDeque::new();
    process_queue.push_back(start_node.to_owned());

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
                                    (Some(acc), None) => Some(acc),
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

                        match (node_to_process == end_node.to_owned(), my_new_state) {
                            (true, ProcessingState::Done(_)) => break,
                            _ => {}
                        };
                    }
                    ProcessingState::Done(_) => {
                        if node_to_process == end_node.to_owned() {
                            break;
                        }
                    }
                },
                None => {}
            },
            None => break,
        }
    }

    let final_result = processing_states.get(start_node);

    assert!(matches!(final_result, Some(ProcessingState::Done(_))));

    match final_result {
        Some(ProcessingState::Done(paths)) => *paths,
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(graph_routes(input, "you").len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_input(input);

    let from_svr_to_fft = double_tree_shake(&graph, "svr", "fft");
    let from_fft_to_dac = double_tree_shake(&graph, "fft", "dac");
    let from_dac_to_out = double_tree_shake(&graph, "dac", "out");

    let from_dac_to_fft = double_tree_shake(&graph, "dac", "fft");

    assert!(
        !from_dac_to_fft.is_empty() ^ !from_fft_to_dac.is_empty(),
        "Circular path"
    );

    // println!(
    //     "from_svr_to_fft: {}, from_fft_to_dac: {}, from_dac_to_out: {}",
    //     from_svr_to_fft.len(),
    //     from_fft_to_dac.len(),
    //     from_dac_to_out.len()
    // );

    let num_paths_from_svr_to_fft = count_paths(&graph, "svr", "fft", from_svr_to_fft);
    let num_paths_from_fft_to_dac = count_paths(&graph, "fft", "dac", from_fft_to_dac);
    let num_paths_from_dac_to_out = count_paths(&graph, "dac", "out", from_dac_to_out);

    Some(num_paths_from_svr_to_fft * num_paths_from_fft_to_dac * num_paths_from_dac_to_out)
}

#[cfg(test)]
mod tests {
    use super::*;

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
