use std::collections::VecDeque;

use crate::graph::VertexId;

pub trait Buffer {
    fn new() -> Self;
    fn start(vertex: VertexId) -> Self;
    fn push(&mut self, vertex: VertexId);
    fn pop(&mut self) -> Option<VertexId>;
}
pub struct Queue(VecDeque<VertexId>);
impl Buffer for Queue {
    fn new() -> Self {
        Self(VecDeque::new())
    }
    fn start(vertex: VertexId) -> Self {
        Self(VecDeque::from(vec![vertex]))
    }
    fn push(&mut self, vertex: VertexId) {
        self.0.push_front(vertex);
    }
    fn pop(&mut self) -> Option<VertexId> {
        self.0.pop_back()
    }
}
pub struct Stack(Vec<VertexId>);
impl Buffer for Stack {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn start(vertex: VertexId) -> Self {
        Self(vec![vertex])
    }
    fn push(&mut self, vertex: VertexId) {
        self.0.push(vertex);
    }
    fn pop(&mut self) -> Option<VertexId> {
        self.0.pop()
    }
}
