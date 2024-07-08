use std::{cmp, collections::HashMap};

use crate::graph::VertexId;

use super::algorithm::Component;

pub struct Stack {
    stack: Vec<VertexId>,
    vertices: HashMap<VertexId, Vertex>,
    next_vertex_index: VertexIndex,
}
impl Stack {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            stack: Vec::new(),
            next_vertex_index: VertexIndex(0),
        }
    }
    // is only called when vertex does not yet exist
    pub fn push(&mut self, vertex: VertexId) {
        self.vertices.insert(
            vertex.clone(),
            Vertex::from(self.next_vertex_index, self.next_vertex_index),
        );
        self.next_vertex_index = VertexIndex(self.next_vertex_index.0 + 1);
        self.stack.push(vertex);
    }
    // is only called when vertex exists
    pub fn update_with_minimum(&mut self, vertex_id: VertexId, update_id: VertexId) {
        if let Some(update) = self.vertices.get(&update_id) {
            let vertex = self.vertices.get(&vertex_id).unwrap();
            self.vertices.insert(
                vertex_id,
                Vertex::from(vertex.index, cmp::min(vertex.low_link, update.low_link)),
            );
        }
    }
    // is only called when vertex exists
    pub fn is_root(&self, vertex_id: &VertexId) -> bool {
        self.vertices.get(vertex_id).unwrap().is_root()
    }
    // is only called when vertex exists
    pub fn pop_until(&mut self, vertex: VertexId) -> Component {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct VertexIndex(usize);
