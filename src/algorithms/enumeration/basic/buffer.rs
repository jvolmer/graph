use std::collections::VecDeque;

use crate::graph::VertexId;

pub trait Buffer<'a> {
    fn new() -> Self;
    fn start(vertex: &'a VertexId) -> Self;
    fn push(&mut self, vertex: &'a VertexId);
    fn pop(&mut self) -> Option<&'a VertexId>;
}
pub struct Queue<'a>(VecDeque<&'a VertexId>);
impl<'a> Buffer<'a> for Queue<'a> {
    fn new() -> Self {
        Self(VecDeque::new())
    }
    fn start(vertex: &'a VertexId) -> Self {
        Self(VecDeque::from(vec![vertex]))
    }
    fn push(&mut self, vertex: &'a VertexId) {
        self.0.push_front(vertex);
    }
    fn pop(&mut self) -> Option<&'a VertexId> {
        self.0.pop_back()
    }
}
pub struct Stack<'a>(Vec<&'a VertexId>);
impl<'a> Buffer<'a> for Stack<'a> {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn start(vertex: &'a VertexId) -> Self {
        Self(vec![vertex])
    }
    fn push(&mut self, vertex: &'a VertexId) {
        self.0.push(vertex);
    }
    fn pop(&mut self) -> Option<&'a VertexId> {
        self.0.pop()
    }
}
