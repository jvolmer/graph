use crate::graph::{Graph, VertexId};
use std::collections::VecDeque;

struct BreadthFirst<'a> {
    graph: &'a Graph,
    queue: VecDeque<VertexId>,
}
impl<'a> BreadthFirst<'a> {
    fn on(graph: &'a Graph, start: VertexId) -> Self {
        Self {
            graph,
            queue: VecDeque::from(vec![start]),
        }
    }
}

impl<'a> Iterator for BreadthFirst<'a> {
    type Item = VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(vec![(0, 0)]);
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        );
    }
}
