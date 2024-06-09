use crate::graph::{Graph, VertexId};
use std::collections::{HashSet, VecDeque};

struct BreadthFirst<'a> {
    graph: &'a Graph,
    queue: VecDeque<VertexId>,
    explored: HashSet<VertexId>,
}
impl<'a> BreadthFirst<'a> {
    fn on(graph: &'a Graph, start: VertexId) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                queue: VecDeque::from(vec![start.clone()]),
                explored: HashSet::new(),
            }
        } else {
            Self {
                graph,
                queue: VecDeque::new(),
                explored: HashSet::new(),
            }
        }
    }
}

impl<'a> Iterator for BreadthFirst<'a> {
    type Item = VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.queue.pop_back() {
            if self.explored.contains(&next) {
                return self.next();
            }
            self.explored.insert(next.clone());
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
    fn does_not_find_non_existend_vertex() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
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
    }

    #[test]
    fn enumerates_vertices_breadth_first() {
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
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        )
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(1))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(1)]
        )
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

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            BreadthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1), VertexId(2)]
        );
    }
}
