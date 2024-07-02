use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use crate::graph::{Edge, Graph, VertexId};

struct Vertex<'a> {
    id: VertexId,
    current_neighbour: Option<VertexId>,
    neighbours: Box<dyn Iterator<Item = VertexId> + 'a>,
    dropped: bool,
}
impl<'a> Vertex<'a> {
    fn from(vertex: VertexId, graph: &'a Graph) -> Self {
        Self {
            id: vertex.clone(),
            current_neighbour: None,
            neighbours: Box::new(graph.out_neighbors(vertex)),
            dropped: false,
        }
    }
    fn next_neighbour(mut self) -> Self {
        Self {
            current_neighbour: self.neighbours.next(),
            ..self
        }
    }
    fn drop(self) -> Self {
        Self {
            dropped: true,
            ..self
        }
    }
}
impl<'a> fmt::Debug for Vertex<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vertex")
            .field("id", &self.id)
            .field("current_neighbour", &self.current_neighbour)
            .field("dropped", &self.dropped)
            .finish()
    }
}

// Generalization to enumeration that covers both depth first and breadth first
// requires two changes:
// - When vertex is not ended yet, it has to be restored to buffer,
//   which can be a stack or a queue, such that it goes back to the same location
//   it was before it was popped.
// - Handle double edges (having same source and sink). Currently the sink vertex
//   is pushed twice to the buffer, which is fine for edges but also results in
//   returning EndVertex twice for the sink.
pub struct DepthFirst<'a> {
    graph: &'a Graph,
    stack: Vec<Vertex<'a>>,
    explored: HashSet<VertexId>,
    output_queue: VecDeque<DFSEntry>,
}
impl<'a> DepthFirst<'a> {
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
    pub fn explored(self) -> HashSet<VertexId> {
        self.explored
    }
    pub fn drop_current_vertex(&mut self) {
        if let Some(v) = self.stack.pop() {
            self.stack.push(v.drop());
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
        if let None = vertex.current_neighbour {
            if !vertex.dropped {
                let id = vertex.id.clone();
                return Some(DFSEntry::EndVertex(id));
            }
        }
        None
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

#[derive(Debug, PartialEq, Clone)]
pub enum DFSEntry {
    BeginVertex(VertexId),
    BeginEdge(Edge),
    EndVertex(VertexId),
    EndEdge(Edge),
}

impl<'a> Iterator for DepthFirst<'a> {
    type Item = DFSEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.output_queue.is_empty() {
            return self.output_queue.pop_back();
        }
        match self.stack.pop() {
            None => None,
            Some(vertex) => {
                if let Some(entry) = self.begin_vertex(vertex.id.clone()) {
                    self.stack.push(vertex);
                    // directly return here to make sure that the returned started vertex
                    // can be dropped by calling drop_current_vertex afterwards
                    return Some(entry);
                }

                self.end_previous_edge(&vertex)
                    .and_then(|x| Some(self.output_queue.push_front(x)));

                let updated_vertex = vertex.next_neighbour();
                let updated_vertex_id = updated_vertex.id.clone();
                let updated_current_neighbour = updated_vertex.current_neighbour.clone();
                let updated_dropped = updated_vertex.dropped;

                self.end_vertex(&updated_vertex)
                    .and_then(|x| Some(self.output_queue.push_front(x)))
                    .or_else(|| {
                        if !updated_vertex.dropped {
                            self.stack.push(updated_vertex);
                        }
                        None
                    });

                if !updated_dropped {
                    self.begin_next_edge(updated_vertex_id, updated_current_neighbour)
                        .and_then(|x| Some(self.output_queue.push_front(x)));
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
            DepthFirst::on(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            Vec::<DFSEntry>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph, VertexId(0))
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
            DepthFirst::on(&graph, VertexId(0))
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
            DepthFirst::on(&graph, VertexId(0))
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
            DepthFirst::on(&graph, VertexId(0))
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
            DepthFirst::on(&graph, VertexId(1))
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
            DepthFirst::on(&graph, VertexId(0))
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
            DepthFirst::on(&graph, VertexId(0))
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
            DepthFirst::on(&graph, VertexId(0))
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
