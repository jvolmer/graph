use crate::{
    algorithms::enumeration::BreadthFirst,
    graph::{Graph, VertexId},
};
use std::{collections::HashSet, mem};

struct FullEnumeration<'a> {
    graph: &'a Graph,
    enumeration: BreadthFirst<'a>,
    vertices: Box<dyn Iterator<Item = VertexId>>,
    explored: HashSet<VertexId>,
}
impl<'a> FullEnumeration<'a> {
    fn on(graph: &'a Graph) -> Self {
        let mut vertices = graph.vertices();
        match vertices.next() {
            None => Self {
                graph,
                enumeration: BreadthFirst::on(graph, VertexId(0)),
                vertices: Box::new(vertices),
                explored: HashSet::new(),
            },
            Some(v) => Self {
                graph,
                enumeration: BreadthFirst::on(graph, v),
                vertices: Box::new(vertices),
                explored: HashSet::new(),
            },
        }
    }

    fn start_new_tree(&mut self, vertex: VertexId) {
        let old_enumeration =
            mem::replace(&mut self.enumeration, BreadthFirst::on(self.graph, vertex));
        self.explored.extend(old_enumeration.explored());
    }
}

impl<'a> Iterator for FullEnumeration<'a> {
    type Item = VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        match self.enumeration.next() {
            Some(v) => {
                if self.explored.contains(&v) {
                    self.next()
                } else {
                    Some(v)
                }
            }
            None => match self.vertices.next() {
                Some(v) => {
                    self.start_new_tree(v);
                    self.next()
                }
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_find_anything_on_empty_graph() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![]
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0)]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1)]
        );
    }

    #[test]
    fn iterates_over_unconnected_components() {
        let graph = Graph::from(4, vec![]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1), VertexId(2), VertexId(3)]
        );
    }

    #[test]
    fn iterates_over_each_component_breadth_first() {
        let graph = Graph::from(5, vec![(0, 1), (0, 2), (3, 4)]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![
                VertexId(0),
                VertexId(1),
                VertexId(2),
                VertexId(3),
                VertexId(4)
            ]
        );
    }

    #[test]
    fn iterates_over_each_component_in_edge_direction_first() {
        let graph = Graph::from(3, vec![(0, 1), (2, 0)]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1), VertexId(2),]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            FullEnumeration::on(&graph)
                .into_iter()
                .collect::<Vec<VertexId>>(),
            vec![VertexId(0), VertexId(1),]
        );
    }
}
