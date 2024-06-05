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
        if let Some(next) = self.queue.pop_back() {
            self.graph
                .out_neighbors(&next)
                .for_each(|v| self.queue.push_front(v.clone()));
            Some(next)
        } else {
            None
        }
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

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(vec![(0, 1)]);
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1)]
        );
    }

    #[test]
    fn enumerates_vertices_breadth_first() {
        let graph = Graph::from(vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]);
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![
                VertexId(0),
                VertexId(1),
                VertexId(2),
                VertexId(3),
                VertexId(4),
                VertexId(5)
            ]
        );
    }
}
