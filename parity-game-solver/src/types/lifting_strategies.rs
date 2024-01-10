use crate::types::vertex::Vertices;

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
