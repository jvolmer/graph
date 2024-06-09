use crate::graph::{Graph, VertexId};
use std::collections::{HashSet, VecDeque};

struct Enumeration<'a, E> {
    graph: &'a Graph,
    next: E,
    explored: HashSet<VertexId>,
}
impl<'a, E> Enumeration<'a, E>
where
    E: Next,
{
    fn on(graph: &'a Graph, start: VertexId) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                next: E::start(start.clone()),
                explored: HashSet::new(),
            }
        } else {
            Self {
                graph,
                next: E::new(),
                explored: HashSet::new(),
            }
        }
    }
}

impl<'a, E> Iterator for Enumeration<'a, E>
where
    E: Next,
{
    type Item = VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.pop() {
            if self.explored.contains(&next) {
                return self.next();
            }
            self.explored.insert(next.clone());
            self.graph
                .out_neighbors(&next)
                .for_each(|v| self.next.push(v.clone()));
            Some(next)
        } else {
            None
        }
    }
}

trait Next {
    fn new() -> Self;
    fn start(vertex: VertexId) -> Self;
    fn push(&mut self, vertex: VertexId);
    fn pop(&mut self) -> Option<VertexId>;
}
struct Queue(VecDeque<VertexId>);
impl Next for Queue {
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
type BreadthFirst<'a> = Enumeration<'a, Queue>;
struct Stack(Vec<VertexId>);
impl Next for Stack {
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
type DepthFirst<'a> = Enumeration<'a, Stack>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_find_non_existend_vertex() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![]
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1)]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1)]
        );
    }

    #[test]
    fn breadth_first_enumerates_vertices_breadth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
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

    #[test]
    fn depth_first_enumerates_vertices_depth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![
                VertexId(0),
                VertexId(2),
                VertexId(1),
                VertexId(4),
                VertexId(5),
                VertexId(3),
            ]
        );
    }

    #[test]
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        );
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(1))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(1)]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(1))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(1)]
        );
    }

    #[test]
    fn finds_each_vertex_only_once() {
        let graph = Graph::from(2, vec![(0, 1), (0, 1)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1),]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1),]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1),]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1),]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1), VertexId(2)]
        );
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1), VertexId(2)]
        );
    }
}
