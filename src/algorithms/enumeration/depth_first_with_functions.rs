use std::collections::HashSet;

use crate::graph::{Edge, Graph, VertexId};

struct Vertex<'a> {
    vertex: VertexId,
    entered_via: Option<Edge>,
    neighbours: Box<dyn Iterator<Item = VertexId> + 'a>,
}
impl<'a> Vertex<'a> {
    fn from(vertex: VertexId, entered_via: Option<Edge>, graph: &'a Graph) -> Self {
        Self {
            vertex: vertex.clone(),
            entered_via,
            neighbours: Box::new(graph.out_neighbors(vertex)),
        }
    }
}

pub struct DepthFirstAdvanced<'a> {
    graph: &'a Graph,
    stack: Vec<Vertex<'a>>,
    explored: HashSet<VertexId>,
}
impl<'a> DepthFirstAdvanced<'a> {
    pub fn on(graph: &'a Graph, start: VertexId) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                stack: vec![Vertex::from(start, None, graph)],
                explored: HashSet::new(),
            }
        } else {
            Self {
                graph,
                stack: vec![],
                explored: HashSet::new(),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VertexInDFS {
    Begin(VertexId, Option<Edge>),
    End(VertexId, Option<Edge>),
}

impl<'a> Iterator for DepthFirstAdvanced<'a> {
    type Item = VertexInDFS;

    fn next(&mut self) -> Option<Self::Item> {
        let stack_lengh = self.stack.len();
        if stack_lengh == 0 {
            return None;
        }
        if let Some(ref mut vertex) = self.stack.get_mut(stack_lengh - 1) {
            if !self.explored.contains(&vertex.vertex) {
                self.explored.insert(vertex.vertex.clone());
                return Some(VertexInDFS::Begin(
                    vertex.vertex.clone(),
                    vertex.entered_via.clone(),
                ));
            }
            match vertex.neighbours.next() {
                Some(neighbour) => {
                    if !self.explored.contains(&neighbour) {
                        let entered_via = Edge(vertex.vertex.clone(), neighbour.clone());
                        self.stack
                            .push(Vertex::from(neighbour, Some(entered_via), self.graph));
                    }
                }
                None => {
                    let end = vertex.vertex.clone();
                    let from = vertex.entered_via.clone();
                    self.stack.pop();
                    return Some(VertexInDFS::End(end, from));
                }
            }
        }
        return self.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_find_non_existend_vertex() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            Vec::<VertexInDFS>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::End(VertexId(0), None)
            ]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::Begin(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(0), None)
            ]
        );
    }

    #[test]
    fn depth_first_enumerates_vertices_depth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::Begin(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::Begin(VertexId(3), Some(Edge(VertexId(1), VertexId(3)))),
                VertexInDFS::End(VertexId(3), Some(Edge(VertexId(1), VertexId(3)))),
                VertexInDFS::Begin(VertexId(4), Some(Edge(VertexId(1), VertexId(4)))),
                VertexInDFS::Begin(VertexId(5), Some(Edge(VertexId(4), VertexId(5)))),
                VertexInDFS::End(VertexId(5), Some(Edge(VertexId(4), VertexId(5)))),
                VertexInDFS::End(VertexId(4), Some(Edge(VertexId(1), VertexId(4)))),
                VertexInDFS::End(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::Begin(VertexId(2), Some(Edge(VertexId(0), VertexId(2)))),
                VertexInDFS::End(VertexId(2), Some(Edge(VertexId(0), VertexId(2)))),
                VertexInDFS::End(VertexId(0), None),
            ]
        );
    }

    #[test]
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::End(VertexId(0), None)
            ]
        );
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(1))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(1), None),
                VertexInDFS::End(VertexId(1), None)
            ]
        );
    }

    #[test]
    fn finds_each_vertex_only_once() {
        let graph = Graph::from(2, vec![(0, 1), (0, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::Begin(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(0), None)
            ]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::Begin(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(0), None)
            ]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0), None),
                VertexInDFS::Begin(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::Begin(VertexId(2), Some(Edge(VertexId(1), VertexId(2)))),
                VertexInDFS::End(VertexId(2), Some(Edge(VertexId(1), VertexId(2)))),
                VertexInDFS::End(VertexId(1), Some(Edge(VertexId(0), VertexId(1)))),
                VertexInDFS::End(VertexId(0), None)
            ]
        );
    }
}
