use crate::graph::VertexId;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Component(HashSet<VertexId>);
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
