use crate::graph::{Graph, VertexId};
use std::{collections::HashSet, mem};

use super::{buffer, tree::TreeEnumeration};

pub struct GraphEnumeration<'a, N> {
    graph: &'a Graph,
    enumeration: TreeEnumeration<'a, N>,
    // TODO maybe get rid of Box dyn
    vertices: Box<dyn Iterator<Item = &'a VertexId> + 'a>,
    explored: HashSet<&'a VertexId>,
}
impl<'a, N> GraphEnumeration<'a, N>
where
    N: buffer::Buffer<'a>,
{
    pub fn on(graph: &'a Graph) -> Self {
        let mut vertices = graph.vertices();
        match vertices.next() {
            None => Self {
                graph,
                enumeration: TreeEnumeration::on(graph, &VertexId(0)),
                vertices: Box::new(vertices),
                explored: HashSet::new(),
            },
            Some(v) => Self {
                graph,
                enumeration: TreeEnumeration::on(graph, &v),
                vertices: Box::new(vertices),
                explored: HashSet::new(),
            },
        }
    }

    fn start_new_tree(&mut self, vertex: &'a VertexId) {
        let old_enumeration = mem::replace(
            &mut self.enumeration,
            TreeEnumeration::on(self.graph, vertex),
        );
        self.explored.extend(old_enumeration.explored());
    }
}

impl<'a, N> Iterator for GraphEnumeration<'a, N>
where
    N: buffer::Buffer<'a>,
{
    type Item = &'a VertexId;

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
                    self.start_new_tree(&v);
                    self.next()
                }
                None => None,
            },
        }
    }
}
pub type GraphBreadthFirst<'a> = GraphEnumeration<'a, buffer::Queue<'a>>;
pub type GraphDepthFirst<'a> = GraphEnumeration<'a, buffer::Stack<'a>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_find_anything_on_empty_graph() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            Vec::<&VertexId>::new()
        );
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            Vec::<&VertexId>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0)]
        );
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0)]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1)]
        );
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1)]
        );
    }

    #[test]
    fn iterates_over_unconnected_components() {
        let graph = Graph::from(4, vec![]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(2), &VertexId(3)]
        );
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(2), &VertexId(3)]
        );
    }

    #[test]
    fn breadth_first_iterates_over_each_component_breadth_first() {
        let graph = Graph::from(5, vec![(0, 1), (0, 2), (3, 4)]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![
                &VertexId(0),
                &VertexId(1),
                &VertexId(2),
                &VertexId(3),
                &VertexId(4)
            ]
        );
    }

    #[test]
    fn depth_first_iterates_over_each_component_depth_first() {
        let graph = Graph::from(8, vec![(0, 1), (0, 2), (1, 3), (4, 5), (4, 6), (5, 7)]).unwrap();
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![
                &VertexId(0),
                &VertexId(2),
                &VertexId(1),
                &VertexId(3),
                &VertexId(4),
                &VertexId(6),
                &VertexId(5),
                &VertexId(7),
            ]
        );
    }

    #[test]
    fn iterates_over_each_component_in_edge_direction_first() {
        let graph = Graph::from(3, vec![(0, 1), (2, 0)]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(2),]
        );
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(2),]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            GraphBreadthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1),]
        );
        assert_eq!(
            GraphDepthFirst::on(&graph)
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1),]
        );
    }
}
