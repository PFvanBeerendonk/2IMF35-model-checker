use crate::types::vertex::{Vertex, Vertices};
use crate::types::progress_measure::ProgressMeasure; // Import ProgressMeasure
use std::collections::{VecDeque, HashMap};

pub fn least_successor_order(vertices: &Vertices) -> Vec<i64> {
    let mut indices: Vec<usize> = vertices
        .iter()
        .enumerate()
        .filter_map(|(index, v)| v.as_ref().map(|_| index))
        .collect();

    indices.sort_by_cached_key(|&index| {
        vertices[index]
            .as_ref()
            .map_or(0, |vertex| vertex.successors.len())
    });

    indices
        .iter()
        .map(|&index| vertices[index].as_ref().unwrap().identifier)
        .collect()
}

pub fn most_successor_order(vertices: &Vertices) -> Vec<i64> {
    let mut indices: Vec<usize> = vertices
        .iter()
        .enumerate()
        .filter_map(|(index, v)| v.as_ref().map(|_| index))
        .collect();

    indices.sort_by_cached_key(|&index| {
        vertices[index]
            .as_ref()
            .map_or(0, |vertex| vertex.successors.len())
    });

    indices.reverse(); // Reverse the sorted indices to get most to least

    indices
        .iter()
        .map(|&index| vertices[index].as_ref().unwrap().identifier)
        .collect()
}

#[derive(Default)]
pub struct PredecessorLiftingStrategy {
    queued: Vec<bool>,
    queue: VecDeque<Vertex>,
}

impl PredecessorLiftingStrategy {
    pub fn new(vertices: &Vertices, top: &HashMap<i64, bool>) -> Self {
        let mut queued = vec![false; vertices.len()];
        let mut queue = VecDeque::new();

        for (v_index, vertex_option) in vertices.iter().enumerate() {
            if let Some(vertex) = vertex_option {
                if !top.contains_key(&vertex.identifier) || !top[&vertex.identifier] {
                    queued[v_index] = true;
                    queue.push_back(vertex.clone());
                }
            }
        }

        PredecessorLiftingStrategy { queued, queue }
    }

    pub fn lifted(&mut self, v: &Vertex, predecessors: &Vertices, top: &HashMap<i64, bool>) {
        for (w_index, w_option) in predecessors.iter().enumerate() {
            if let Some(w) = w_option {
                if !self.queued[w.identifier as usize] && (!top.contains_key(&w.identifier) || !top[&w.identifier]) {
                    self.queued[w.identifier as usize] = true;
                    self.queue.push_back(w.clone());
                }
            }
        }
    }

    pub fn next(&mut self) -> Option<Vertex> {
        if let Some(v) = self.queue.pop_front() {
            self.queued[v.identifier as usize] = false;
            Some(v)
        } else {
            None
        }
    }
}
