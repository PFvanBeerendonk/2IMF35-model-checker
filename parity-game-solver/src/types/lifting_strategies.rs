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


#[derive(Debug)]// Algorithm 4.4 Focus List Lifting Strategy
pub struct FocusListLiftingStrategy {
    phase: i64,
    num_attempts: i64,
    num_failed: i64,
    next_vertex: i64,
    focus_list: VecDeque<(Vertex, i64)>,
}

#[derive(Debug)]// Algorithm 4.4 Focus List Lifting Strategy
pub struct FocusListLiftingStrategy {
    phase: i64,
    num_attempts: i64,
    num_failed: i64,
    next_vertex: i64,
    focus_list: VecDeque<(Vertex, i64)>,
}

impl FocusListLiftingStrategy {
    pub fn new() -> Self {
        Self {
            phase: 1,
            num_attempts: 0,
            num_failed: 0,
            next_vertex: 0,
            focus_list: VecDeque::new(),
        }
    }

    pub fn run(
        &mut self,
        progress_measure: &mut ProgressMeasure,
        vertices: &Vertices,
        max_priority: i64,
        max_size: usize,
        max_attempts: i64,
    ) {
        let v_count = vertices.len() as i64;

        loop {
            self.num_attempts += 1;

            if self.phase == 1 {
                if let (pm, did_update) = progress_measure.clone().lift_v(
                    self.next_vertex,
                    vertices,
                    max_priority,
                ) {
                    *progress_measure = pm;
                    self.num_failed = if did_update { 0 } else { self.num_failed + 1 };

                    if did_update {
                        self.focus_list.push_back((vertices[self.next_vertex as usize].clone().unwrap(), 2));
                    }

                    self.next_vertex = (self.next_vertex + 1) % v_count;

                    if self.num_failed == v_count {
                        break;
                    }

                    if self.num_attempts == v_count || self.focus_list.len() == max_size {
                        self.phase = 2;
                        self.num_attempts = 0;
                    }
                }
            } else {
                if let Some((v, credit)) = self.focus_list.pop_front() {
                    if let (pm, did_update) = progress_measure.clone().lift_v(
                        v.identifier,
                        vertices,
                        max_priority,
                    ) {
                        *progress_measure = pm;
                        if did_update {
                            self.focus_list.push_back((v, credit + 2));
                        } else if credit > 0 {
                            self.focus_list.push_back((v, credit / 2));
                        }
                    }

                    if self.focus_list.is_empty() || self.num_attempts == max_attempts {
                        self.focus_list.clear();
                        self.phase = 1;
                        self.num_attempts = 0;
                    }
                }
            }
        }
    }
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
