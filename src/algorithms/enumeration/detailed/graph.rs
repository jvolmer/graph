use crate::graph::{Graph, VertexId};
use std::{collections::HashSet, mem};

use super::tree::{self, DFSEntry};

pub struct DepthFirst<'a> {
    graph: &'a Graph,
    enumeration: tree::DepthFirst<'a>,
    // TODO maybe get rid of Box dyn
    vertices: Box<dyn Iterator<Item = &'a VertexId> + 'a>,
    explored: HashSet<VertexId>,
}
impl<'a> DepthFirst<'a> {
    pub fn on(graph: &'a Graph) -> Self {
        let mut vertices = graph.vertices();
        match vertices.next() {
            None => Self {
                graph,
                enumeration: tree::DepthFirst::on(graph, VertexId(0)),
                vertices: Box::new(vertices),
                explored: HashSet::new(),
            },
            Some(v) => Self {
                graph,
                enumeration: tree::DepthFirst::on(graph, v.clone()),
                vertices: Box::new(vertices),
                explored: HashSet::new(),
            },
        }
    }

    fn start_new_tree(&mut self, vertex: VertexId) {
        let old_enumeration = mem::replace(
            &mut self.enumeration,
            tree::DepthFirst::on(self.graph, vertex),
        );
        self.explored.extend(old_enumeration.explored());
    }
    fn make_sure_that_entry_was_not_already_given(&mut self, entry: DFSEntry) -> Option<DFSEntry> {
        if let DFSEntry::BeginVertex(v) = entry.clone() {
            if self.explored.contains(&v) {
                self.enumeration.drop_current_vertex();
                return None;
            }
        }
        Some(entry)
    }
}

impl<'a> Iterator for DepthFirst<'a> {
    type Item = DFSEntry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.enumeration.next() {
            Some(entry) => self
                .make_sure_that_entry_was_not_already_given(entry)
                .or_else(|| self.next()),
            None => match self.vertices.next() {
                Some(v) => {
                    self.start_new_tree(v.clone());
                    self.next()
                }
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::Edge;

    use super::*;

    #[test]
    fn does_not_find_anything_on_empty_graph() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            Vec::<DFSEntry>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
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
            DepthFirst::on(&graph)
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
    fn iterates_over_unconnected_components() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::EndVertex(VertexId(0)),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::EndVertex(VertexId(1)),
            ]
        );
    }

    #[test]
    fn iterates_over_each_component_depth_first() {
        let graph = Graph::from(8, vec![(0, 1), (0, 2), (1, 3), (4, 5), (4, 6), (5, 7)]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
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
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(2))),
                DFSEntry::BeginVertex(VertexId(2)),
                DFSEntry::EndVertex(VertexId(2)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(2))),
                DFSEntry::EndVertex(VertexId(0)),
                DFSEntry::BeginVertex(VertexId(4)),
                DFSEntry::BeginEdge(Edge(VertexId(4), VertexId(5))),
                DFSEntry::BeginVertex(VertexId(5)),
                DFSEntry::BeginEdge(Edge(VertexId(5), VertexId(7))),
                DFSEntry::BeginVertex(VertexId(7)),
                DFSEntry::EndVertex(VertexId(7)),
                DFSEntry::EndEdge(Edge(VertexId(5), VertexId(7))),
                DFSEntry::EndVertex(VertexId(5)),
                DFSEntry::EndEdge(Edge(VertexId(4), VertexId(5))),
                DFSEntry::BeginEdge(Edge(VertexId(4), VertexId(6))),
                DFSEntry::BeginVertex(VertexId(6)),
                DFSEntry::EndVertex(VertexId(6)),
                DFSEntry::EndEdge(Edge(VertexId(4), VertexId(6))),
                DFSEntry::EndVertex(VertexId(4)),
            ]
        );
    }

    #[test]
    fn iterates_over_each_component_in_edge_direction_first() {
        let graph = Graph::from(3, vec![(0, 1), (2, 0)]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::BeginEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::EndEdge(Edge(VertexId(0), VertexId(1))),
                DFSEntry::EndVertex(VertexId(0)),
                DFSEntry::BeginVertex(VertexId(2)),
                DFSEntry::BeginEdge(Edge(VertexId(2), VertexId(0))),
                DFSEntry::EndEdge(Edge(VertexId(2), VertexId(0))),
                DFSEntry::EndVertex(VertexId(2))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
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
    }

    #[test]
    fn finds_rest_of_tree_when_dfs_does_not_start_at_its_root() {
        let graph = Graph::from(4, vec![(3, 1), (2, 3), (2, 0)]).unwrap();
        assert_eq!(
            DepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<DFSEntry>>(),
            vec![
                DFSEntry::BeginVertex(VertexId(0)),
                DFSEntry::EndVertex(VertexId(0)),
                DFSEntry::BeginVertex(VertexId(1)),
                DFSEntry::EndVertex(VertexId(1)),
                DFSEntry::BeginVertex(VertexId(2)),
                DFSEntry::BeginEdge(Edge(VertexId(2), VertexId(3))),
                DFSEntry::BeginVertex(VertexId(3)),
                DFSEntry::BeginEdge(Edge(VertexId(3), VertexId(1))),
                DFSEntry::EndEdge(Edge(VertexId(3), VertexId(1))),
                DFSEntry::EndVertex(VertexId(3)),
                DFSEntry::EndEdge(Edge(VertexId(2), VertexId(3))),
                DFSEntry::BeginEdge(Edge(VertexId(2), VertexId(0))),
                DFSEntry::EndEdge(Edge(VertexId(2), VertexId(0))),
                DFSEntry::EndVertex(VertexId(2)),
            ]
        );
    }
}
