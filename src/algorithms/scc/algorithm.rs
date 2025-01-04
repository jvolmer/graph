use super::stack::Stack;
use crate::algorithms::component::Component;
///! Tarjan's Strongly Connected Components Algorithm
///!
///! The algorithm finds all strongly connected components in a graph. A strongly connected component is a set of vertices where each vertex can reach any other vertex in the component via the existing edges. Each vertex belongs to exactly one strongly connected component, therefore the components partition the graph into stronly connected subgraphs.
///!
///! The algorithm is based on a depth first search and is linear in the number of edges and vertices. The algorithm is executed via an iterator over components, each time next() is called on the iterator, the algorithm continues and computes the next component.
use crate::algorithms::enumeration::detailed::{graph::DepthFirst, tree::DFSEntry};
use crate::graph::Graph;

/// Includes the state of the strongly connected components computation of a graph.
///
/// It includes the state of a depth first search and a stack of vertices whose component was not yet fully found.
pub struct SCC<'a> {
    dfs: DepthFirst<'a>,
    unfinished_components: Stack,
}

/// Initializes the stongly connected state
impl<'a> SCC<'a> {
    pub fn on(graph: &'a Graph) -> Self {
        Self {
            dfs: DepthFirst::on(&graph),
            unfinished_components: Stack::new(),
        }
    }
}

// Found vertices are pushed to a stack and are only popped when all vertices of its component
impl<'a> Iterator for SCC<'a> {
    type Item = Component;

    /// Gives the next strongly connected component of the graph.

    /// Internally it iterates over the next vertices in the depth first serach until it finds the next strongly connected component. New vertices are pushed to a stack and are only popped when all vertices of its component are found. Each
    ///When the depth first search is finished processing an edge with the full subgraph it points to,  
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.dfs.next() {
                None => return None,
                Some(next) => match next {
                    DFSEntry::BeginVertex(v) => self.unfinished_components.push(v),
                    DFSEntry::EndEdge(e) => {
                        self.unfinished_components.update_with_minimum(e.0, e.1);
                    }
                    DFSEntry::EndVertex(v) => {
                        if self.unfinished_components.is_root(&v) {
                            return Some(self.unfinished_components.pop_until(v));
                        }
                    }
                    _ => (),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::VertexId;

    #[test]
    fn empty_graph_has_no_components() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            Vec::<Component>::new()
        );
    }

    #[test]
    fn single_vertex_is_a_strong_component() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![VertexId(0)])]
        );
    }

    #[test]
    fn vertices_connected_with_one_edge_are_not_stronly_connected() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(1)]),
                Component::from(vec![VertexId(0)])
            ]
        );
    }

    #[test]
    fn vertices_connected_in_both_directions_are_stronly_connected() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![VertexId(1), VertexId(0)]),]
        );
    }

    #[test]
    fn loop_is_stronly_connected() {
        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 0)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![VertexId(2), VertexId(1), VertexId(0)]),]
        );
    }

    #[test]
    fn finds_component_in_different_ordering() {
        let graph = Graph::from(3, vec![(2, 1), (1, 2)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(0)]),
                Component::from(vec![VertexId(2), VertexId(1)]),
            ]
        );
        let graph = Graph::from(6, vec![(4, 2), (2, 0), (1, 3), (5, 2), (0, 4)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(0), VertexId(2), VertexId(4)]),
                Component::from(vec![VertexId(3)]),
                Component::from(vec![VertexId(1)]),
                Component::from(vec![VertexId(5)]),
            ]
        );
    }

    #[test]
    fn finds_two_stronly_connected_components() {
        let graph = Graph::from(5, vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 4), (4, 3)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(4), VertexId(3)]),
                Component::from(vec![VertexId(2), VertexId(1), VertexId(0)]),
            ]
        );
    }

    #[test]
    fn two_nested_loops_belong_to_same_strong_component() {
        let graph = Graph::from(4, vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 2)]).unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![Component::from(vec![
                VertexId(3),
                VertexId(2),
                VertexId(1),
                VertexId(0)
            ]),]
        );
    }

    #[test]
    fn works_on_knuth_example() {
        let graph = Graph::from(
            10,
            vec![
                (3, 1),
                (4, 1),
                (5, 9),
                (2, 6),
                (5, 3),
                (5, 8),
                (9, 7),
                (9, 3),
                (2, 3),
                (8, 4),
                (6, 2),
                (6, 4),
                (3, 3),
                (8, 3),
                (2, 7),
                (9, 5),
                (0, 2),
                (8, 8),
                (4, 1),
                (9, 7),
                (1, 6),
            ],
        )
        .unwrap();
        assert_eq!(
            SCC::on(&graph).into_iter().collect::<Vec<Component>>(),
            vec![
                Component::from(vec![VertexId(7)]),
                Component::from(vec![
                    VertexId(1),
                    VertexId(2),
                    VertexId(3),
                    VertexId(4),
                    VertexId(6),
                ]),
                Component::from(vec![VertexId(0)]),
                Component::from(vec![VertexId(8)]),
                Component::from(vec![VertexId(5), VertexId(9)]),
            ]
        );
    }
}
