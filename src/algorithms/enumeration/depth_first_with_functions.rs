use std::collections::{HashSet, VecDeque};

use crate::graph::{Edge, Graph, VertexId};

struct Vertex<'a> {
    id: VertexId,
    current_neighbour: Option<VertexId>,
    neighbours: Box<dyn Iterator<Item = VertexId> + 'a>,
}
impl<'a> Vertex<'a> {
    fn from(vertex: VertexId, graph: &'a Graph) -> Self {
        Self {
            id: vertex.clone(),
            current_neighbour: None,
            neighbours: Box::new(graph.out_neighbors(vertex)),
        }
    }
    fn next_neighbour(mut self) -> Self {
        Self {
            id: self.id,
            current_neighbour: self.neighbours.next(),
            neighbours: self.neighbours,
        }
    }
}

pub struct DepthFirstAdvanced<'a> {
    graph: &'a Graph,
    stack: Vec<Vertex<'a>>,
    explored: HashSet<VertexId>,
    output_queue: VecDeque<VertexInDFS>,
}
impl<'a> DepthFirstAdvanced<'a> {
    pub fn on(graph: &'a Graph, start: VertexId) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                stack: vec![Vertex::from(start, graph)],
                explored: HashSet::new(),
                output_queue: VecDeque::new(),
            }
        } else {
            Self {
                graph,
                stack: vec![],
                explored: HashSet::new(),
                output_queue: VecDeque::new(),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VertexInDFS {
    BeginVertex(VertexId),
    BeginEdge(Edge),
    EndVertex(VertexId),
    EndEdge(Edge),
}

impl<'a> Iterator for DepthFirstAdvanced<'a> {
    type Item = VertexInDFS;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.output_queue.is_empty() {
            return self.output_queue.pop_back();
        }
        match self.stack.pop() {
            None => None,
            Some(vertex) => {
                // begin vertex
                if !self.explored.contains(&vertex.id) {
                    self.explored.insert(vertex.id.clone());
                    self.output_queue
                        .push_front(VertexInDFS::BeginVertex(vertex.id.clone()));
                }

                // end previous_edge
                if let Some(old_neighbour) = vertex.current_neighbour.clone() {
                    self.output_queue
                        .push_front(VertexInDFS::EndEdge(Edge(vertex.id.clone(), old_neighbour)));
                }

                // next edge for current vertex
                let updated_vertex = vertex.next_neighbour();
                match updated_vertex.current_neighbour.clone() {
                    Some(new_neighbour) => {
                        // begin edge
                        let from = updated_vertex.id.clone();
                        self.stack.push(updated_vertex);
                        if !self.explored.contains(&new_neighbour) {
                            self.stack
                                .push(Vertex::from(new_neighbour.clone(), self.graph));
                        }
                        self.output_queue
                            .push_front(VertexInDFS::BeginEdge(Edge(from, new_neighbour)));
                    }
                    None => {
                        // end vertex
                        self.output_queue
                            .push_front(VertexInDFS::EndVertex(updated_vertex.id));
                    }
                }
                self.next()
            }
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
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::EndVertex(VertexId(0))
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
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginVertex(VertexId(1)),
                VertexInDFS::EndVertex(VertexId(1)),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::EndVertex(VertexId(0))
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
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginVertex(VertexId(1)),
                VertexInDFS::BeginEdge(Edge(VertexId(1), VertexId(3))),
                VertexInDFS::BeginVertex(VertexId(3)),
                VertexInDFS::EndVertex(VertexId(3)),
                VertexInDFS::EndEdge(Edge(VertexId(1), VertexId(3))),
                VertexInDFS::BeginEdge(Edge(VertexId(1), VertexId(4))),
                VertexInDFS::BeginVertex(VertexId(4)),
                VertexInDFS::BeginEdge(Edge(VertexId(4), VertexId(5))),
                VertexInDFS::BeginVertex(VertexId(5)),
                VertexInDFS::EndVertex(VertexId(5)),
                VertexInDFS::EndEdge(Edge(VertexId(4), VertexId(5))),
                VertexInDFS::EndVertex(VertexId(4)),
                VertexInDFS::EndEdge(Edge(VertexId(1), VertexId(4))),
                VertexInDFS::EndVertex(VertexId(1)),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(2))),
                VertexInDFS::BeginVertex(VertexId(2)),
                VertexInDFS::EndVertex(VertexId(2)),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(2))),
                VertexInDFS::EndVertex(VertexId(0))
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
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::EndVertex(VertexId(0))
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
                VertexInDFS::BeginVertex(VertexId(1)),
                VertexInDFS::EndVertex(VertexId(1))
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
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginVertex(VertexId(1)),
                VertexInDFS::EndVertex(VertexId(1)),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::EndVertex(VertexId(0))
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
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginVertex(VertexId(1)),
                VertexInDFS::BeginEdge(Edge(VertexId(1), VertexId(0))),
                VertexInDFS::EndEdge(Edge(VertexId(1), VertexId(0))),
                VertexInDFS::EndVertex(VertexId(1)),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::EndVertex(VertexId(0))
            ]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::BeginVertex(VertexId(0)),
                VertexInDFS::BeginEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::BeginVertex(VertexId(1)),
                VertexInDFS::BeginEdge(Edge(VertexId(1), VertexId(2))),
                VertexInDFS::BeginVertex(VertexId(2)),
                VertexInDFS::BeginEdge(Edge(VertexId(2), VertexId(1))),
                VertexInDFS::EndEdge(Edge(VertexId(2), VertexId(1))),
                VertexInDFS::EndVertex(VertexId(2)),
                VertexInDFS::EndEdge(Edge(VertexId(1), VertexId(2))),
                VertexInDFS::EndVertex(VertexId(1)),
                VertexInDFS::EndEdge(Edge(VertexId(0), VertexId(1))),
                VertexInDFS::EndVertex(VertexId(0))
            ]
        );
    }
}
