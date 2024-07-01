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
    output_queue: VecDeque<DFSEntry>,
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
    fn begin_vertex(&mut self, vertex: VertexId) -> Option<DFSEntry> {
        match self.explored.contains(&vertex) {
            true => None,
            false => {
                self.explored.insert(vertex.clone());
                Some(DFSEntry::BeginVertex(vertex))
            }
        }
    }
    fn end_previous_edge(&self, vertex: &Vertex) -> Option<DFSEntry> {
        vertex
            .current_neighbour
            .clone()
            .and_then(|neighbour| Some(DFSEntry::EndEdge(Edge(vertex.id.clone(), neighbour))))
    }
    fn end_vertex(&self, vertex: &Vertex) -> Option<DFSEntry> {
        match vertex.current_neighbour {
            Some(_) => None,
            None => Some(DFSEntry::EndVertex(vertex.id.clone())),
        }
    }
    // TODO if Vertex<'a> is cloneable: give clone to this function instead of current 2 arguments
    fn begin_next_edge(
        &mut self,
        vertex: VertexId,
        current_neighbour: Option<VertexId>,
    ) -> Option<DFSEntry> {
        current_neighbour.clone().and_then(|neighbour| {
            if !self.explored.contains(&neighbour) {
                self.stack.push(Vertex::from(neighbour.clone(), self.graph));
            }
            Some(DFSEntry::BeginEdge(Edge(vertex, neighbour)))
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum DFSEntry {
    BeginVertex(VertexId),
    BeginEdge(Edge),
    EndVertex(VertexId),
    EndEdge(Edge),
}

impl<'a> Iterator for DepthFirstAdvanced<'a> {
    type Item = DFSEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.output_queue.is_empty() {
            return self.output_queue.pop_back();
        }
        match self.stack.pop() {
            None => None,
            Some(vertex) => {
                self.begin_vertex(vertex.id.clone())
                    .and_then(|x| Some(self.output_queue.push_front(x)));

                self.end_previous_edge(&vertex)
                    .and_then(|x| Some(self.output_queue.push_front(x)));

                let updated_vertex = vertex.next_neighbour();
                let updated_vertex_id = updated_vertex.id.clone();
                let updated_current_neighbour = updated_vertex.current_neighbour.clone();

                self.end_vertex(&updated_vertex)
                    .and_then(|x| Some(self.output_queue.push_front(x)))
                    .or_else(|| Some(self.stack.push(updated_vertex)));

                self.begin_next_edge(updated_vertex_id, updated_current_neighbour)
                    .and_then(|x| Some(self.output_queue.push_front(x)));

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
                .collect::<Vec<DFSEntry>>(),
            Vec::<DFSEntry>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );
    }

    #[test]
    fn depth_first_enumerates_vertices_depth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::BeginEdge(Edge(VertexId(1), VertexId(3))),
                DFSEntry::BeginVertex(VertexId(3)),
                DFSEntry::EndVertex(VertexId(3)),
                DFSEntry::EndEdge(Edge(VertexId(1), VertexId(3))),
                DFSEntry::BeginEdge(Edge(VertexId(1), VertexId(4))),
                DFSEntry::BeginVertex(VertexId(4)),
                DFSEntry::BeginEdge(Edge(VertexId(4), VertexId(5))),
                DFSEntry::BeginVertex(VertexId(5)),
                DFSEntry::EndVertex(VertexId(5)),
                DFSEntry::EndEdge(Edge(VertexId(4), VertexId(5))),
                DFSEntry::EndVertex(VertexId(4)),
                DFSEntry::EndEdge(Edge(VertexId(1), VertexId(4))),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(2))),
                DFSEntry::BeginVertex(VertexId(2)),
                DFSEntry::EndVertex(VertexId(2)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(2))),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );
    }

    #[test]
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(1))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::EndVertex(VertexId(1))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_only_once() {
        let graph = Graph::from(2, vec![(0, 1), (0, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::BeginEdge(Edge(VertexId(1), VertexId(0))),
                DFSEntry::EndEdge(Edge(VertexId(1), VertexId(0))),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            DepthFirstAdvanced::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::BeginEdge(Edge(VertexId(1), VertexId(2))),
                DFSEntry::BeginVertex(VertexId(2)),
                DFSEntry::BeginEdge(Edge(VertexId(2), VertexId(1))),
                DFSEntry::EndEdge(Edge(VertexId(2), VertexId(1))),
                DFSEntry::EndVertex(VertexId(2)),
                DFSEntry::EndEdge(Edge(VertexId(1), VertexId(2))),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::EndVertex(VertexId(0))
            ]
        );
    }
}
