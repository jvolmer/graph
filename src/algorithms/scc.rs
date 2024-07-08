use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::graph::{Graph, VertexId};

use super::enumeration::detailed::{graph::DepthFirst, tree::DFSEntry};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct VertexIndex(usize);
#[derive(Debug, PartialEq)]
struct Vertex {
    index: VertexIndex,
    low_link: VertexIndex,
}
impl Vertex {
    fn from(index: VertexIndex, low_link: VertexIndex) -> Self {
        Self { index, low_link }
    }
    fn is_root(&self) -> bool {
        self.index == self.low_link
    }
}
struct SCCStack {
    stack: Vec<VertexId>,
    vertices: HashMap<VertexId, Vertex>,
    next_vertex_index: VertexIndex,
}
impl SCCStack {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            stack: Vec::new(),
            next_vertex_index: VertexIndex(0),
        }
    }
    // is only called when vertex does not yet exist
    fn push(&mut self, vertex: VertexId) {
        self.vertices.insert(
            vertex.clone(),
            Vertex::from(self.next_vertex_index, self.next_vertex_index),
        );
        self.next_vertex_index = VertexIndex(self.next_vertex_index.0 + 1);
        self.stack.push(vertex);
    }
    // is only called when vertex exists
    fn update_with_minimum(&mut self, vertex_id: VertexId, update_id: VertexId) {
        if let Some(update) = self.vertices.get(&update_id) {
            let vertex = self.vertices.get(&vertex_id).unwrap();
            self.vertices.insert(
                vertex_id,
                Vertex::from(vertex.index, cmp::min(vertex.low_link, update.low_link)),
            );
        }
    }
    // is only called when vertex exists
    fn is_root(&self, vertex_id: &VertexId) -> bool {
        self.vertices.get(vertex_id).unwrap().is_root()
    }
    // is only called when vertex exists
    fn pop_until(&mut self, vertex: VertexId) -> Component {
        let mut component = Component::new();
        loop {
            match self.stack.pop() {
                None => panic!("Pop did not find vertex {vertex:?}"),
                Some(v) => {
                    self.vertices.remove(&v);
                    component.add(v.clone());
                    if v == vertex {
                        return component;
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Component(HashSet<VertexId>);
impl Component {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    pub fn from(vertices: Vec<VertexId>) -> Self {
        Self(HashSet::from_iter(vertices))
    }
    pub fn add(&mut self, vertex: VertexId) {
        self.0.insert(vertex);
    }
}

struct SCC<'a> {
    dfs: DepthFirst<'a>,
    unfinished_components: SCCStack,
}

impl<'a> SCC<'a> {
    pub fn on(graph: &'a Graph) -> Self {
        Self {
            dfs: DepthFirst::on(&graph),
            unfinished_components: SCCStack::new(),
        }
    }
}

impl<'a> Iterator for SCC<'a> {
    type Item = Component;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.dfs.next() {
                None => return None,
                Some(next) => match next {
                    DFSEntry::BeginVertex(v) => self.unfinished_components.push(v),
                    DFSEntry::EndEdge(e) => {
                        self.unfinished_components.update_with_minimum(e.0, e.1);
                    }
                    DFSEntry::EndVertex(v) => {
                        if self.unfinished_components.is_root(&v) {
                            return Some(self.unfinished_components.pop_until(v));
                        }
                    }
                    _ => (),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_has_no_components() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            Vec::<Component>::new()
        );
    }

    #[test]
    fn single_vertex_is_a_strong_component() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![VertexId(0)])]
        );
    }

    #[test]
    fn vertices_connected_with_one_edge_are_not_stronly_connected() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(1)]),
                Component::from(vec![VertexId(0)])
            ]
        );
    }

    #[test]
    fn vertices_connected_in_both_directions_are_stronly_connected() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![VertexId(1), VertexId(0)]),]
        );
    }

    #[test]
    fn loop_is_stronly_connected() {
        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 0)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![VertexId(2), VertexId(1), VertexId(0)]),]
        );
    }

    #[test]
    fn finds_component_in_different_ordering() {
        let graph = Graph::from(3, vec![(2, 1), (1, 2)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(0)]),
                Component::from(vec![VertexId(2), VertexId(1)]),
            ]
        );
        let graph = Graph::from(6, vec![(4, 2), (2, 0), (1, 3), (5, 2), (0, 4)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(0), VertexId(2), VertexId(4)]),
                Component::from(vec![VertexId(3)]),
                Component::from(vec![VertexId(1)]),
                Component::from(vec![VertexId(5)]),
            ]
        );
    }

    #[test]
    fn finds_two_stronly_connected_components() {
        let graph = Graph::from(5, vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 3)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(4), VertexId(3)]),
                Component::from(vec![VertexId(2), VertexId(1), VertexId(0)]),
            ]
        );
    }

    #[test]
    fn two_nested_loops_belong_to_same_strong_component() {
        let graph = Graph::from(4, vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 2)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![
                VertexId(3),
                VertexId(2),
                VertexId(1),
                VertexId(0)
            ]),]
        );
    }

    #[test]
    fn works_on_knuth_example() {
        let graph = Graph::from(
            10,
            vec![
                (3, 1),
                (4, 1),
                (5, 9),
                (2, 6),
                (5, 3),
                (5, 8),
                (9, 7),
                (9, 3),
                (2, 3),
                (8, 4),
                (6, 2),
                (6, 4),
                (3, 3),
                (8, 3),
                (2, 7),
                (9, 5),
                (0, 2),
                (8, 8),
                (4, 1),
                (9, 7),
                (1, 6),
            ],
        )
        .unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(7)]),
                Component::from(vec![
                    VertexId(6),
                    VertexId(1),
                    VertexId(4),
                    VertexId(3),
                    VertexId(2),
                ]),
                Component::from(vec![VertexId(0)]),
                Component::from(vec![VertexId(8)]),
                Component::from(vec![VertexId(5), VertexId(9)]),
            ]
        );
    }
}
