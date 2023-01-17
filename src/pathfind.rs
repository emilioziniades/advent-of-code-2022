use std::{collections::HashMap, hash::Hash};

use crate::queue::PriorityQueue;

pub enum PathAlgo {
    BreadthFirst,
    Djikstra,
    AStar,
}

pub trait Graph<T: Sized> {
    fn neighbours(&self, current: T) -> &Vec<T>;
    fn nodes(&self) -> Vec<T>;
    fn cost(&self, current: &T, next: &T) -> usize;
}

pub struct CameFrom<T>(HashMap<T, Option<T>>);

pub fn breadth_first_search<G, T>(graph: &G, start: T) -> CameFrom<T>
where
    G: Graph<T>,
    T: Eq + Hash + Copy,
{
    let mut frontier: Vec<T> = Vec::new();
    frontier.push(start);

    let mut came_from: CameFrom<T> = CameFrom(HashMap::new());
    came_from.0.insert(start, None);

    while let Some(current) = frontier.pop() {
        let neighbours = graph.neighbours(current);

        for neighbour in neighbours {
            if !came_from.0.contains_key(&neighbour) {
                frontier.push(*neighbour);
                came_from.0.insert(*neighbour, Some(current));
            }
        }
    }

    came_from
}

pub fn djikstra<G, T>(graph: &G, start: T) -> CameFrom<T>
where
    G: Graph<T>,
    T: Eq + Hash + Copy,
{
    let mut frontier = PriorityQueue::new();
    frontier.push(start, 0);

    let mut came_from: CameFrom<T> = CameFrom(HashMap::new());
    came_from.0.insert(start, None);

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some(current) = frontier.pop() {
        let neighbours = graph.neighbours(current);

        for neighbour in neighbours {
            let current_cost = cost_so_far.get(&current).unwrap();
            let new_cost = current_cost + graph.cost(&current, neighbour);
            let neighbour_existing_cost = cost_so_far.get(neighbour).copied();
            if !came_from.0.contains_key(&neighbour) || Some(new_cost) < neighbour_existing_cost {
                frontier.push(*neighbour, new_cost);
                came_from.0.insert(*neighbour, Some(current));
                cost_so_far.insert(*neighbour, new_cost);
            }
        }
    }

    came_from
}

pub fn recreate_path<T: Eq + Hash + Copy>(came_from: &CameFrom<T>, start: T, goal: T) -> Vec<T> {
    let mut path: Vec<T> = Vec::new();
    let mut current = goal;

    while current != start {
        path.push(current);
        current = came_from.0.get(&current).unwrap().unwrap();
    }

    path.reverse();
    path
}

pub fn recreate_all_paths<T: Eq + Hash + Copy, G>(
    graph: &G,
    start: T,
    algo: PathAlgo,
) -> HashMap<T, Vec<T>>
where
    G: Graph<T>,
{
    let mut all_paths: HashMap<T, Vec<T>> = HashMap::new();

    let came_from = match algo {
        PathAlgo::BreadthFirst => breadth_first_search(graph, start),
        PathAlgo::Djikstra => djikstra(graph, start),
        PathAlgo::AStar => todo!(),
    };

    for goal in graph.nodes() {
        let path = recreate_path(&came_from, start, goal);
        all_paths.insert(goal, path);
    }

    all_paths
}
