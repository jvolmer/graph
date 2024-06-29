use std::collections::HashSet;

use crate::graph::{Graph, VertexId};

struct Vertex<'a> {
    vertex: &'a VertexId,
    neighbours: Box<dyn Iterator<Item = &'a VertexId> + 'a>,
    current_neighbour: Option<&'a VertexId>,
}
impl<'a> Vertex<'a> {
    fn from(vertex: &'a VertexId, graph: &'a Graph) -> Self {
        Self {
            vertex,
            neighbours: Box::new(graph.out_neighbors(vertex)),
            current_neighbour: None,
        }
    }
}

pub struct DepthFirstWithFunctions<'a, E>
where
    E: FnMut(&VertexId, &VertexId) -> (),
{
    graph: &'a Graph,
    stack: Vec<Vertex<'a>>,
    explored: HashSet<&'a VertexId>,
    end_edge: E,
}
impl<'a, E> DepthFirstWithFunctions<'a, E>
where
    E: FnMut(&VertexId, &VertexId) -> (),
{
    pub fn on(graph: &'a Graph, start: &'a VertexId, end_edge: E) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                stack: vec![Vertex::from(start, graph)],
                explored: HashSet::new(),
                end_edge,
            }
        } else {
            Self {
                graph,
                stack: vec![],
                explored: HashSet::new(),
                end_edge,
            }
        }
    }
}
impl<'a> DepthFirstWithFunctions<'a, fn(&VertexId, &VertexId)> {
    pub fn on_empty(graph: &'a Graph, start: &'a VertexId) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                stack: vec![Vertex::from(start, graph)],
                explored: HashSet::new(),
                end_edge: |_, _| {},
            }
        } else {
            Self {
                graph,
                stack: vec![],
                explored: HashSet::new(),
                end_edge: |_, _| {},
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum VertexInDFS<'a> {
    Begin(&'a VertexId),
    End(&'a VertexId),
}

impl<'a, E> Iterator for DepthFirstWithFunctions<'a, E>
where
    E: FnMut(&VertexId, &VertexId) -> (),
{
    type Item = VertexInDFS<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let stack_lengh = self.stack.len();
        if stack_lengh == 0 {
            return None;
        }
        match self.stack.get_mut(stack_lengh - 1) {
            Some(ref mut vertex) => {
                if !self.explored.contains(vertex.vertex) {
                    self.explored.insert(vertex.vertex);
                    return Some(VertexInDFS::Begin(vertex.vertex));
                }
                match vertex.current_neighbour {
                    Some(neighbour) => {
                        (self.end_edge)(vertex.vertex, neighbour);
                    }
                    None => (),
                }
                vertex.current_neighbour = vertex.neighbours.next();
                match vertex.current_neighbour {
                    Some(neighbour) => {
                        if !self.explored.contains(&neighbour) {
                            self.stack.push(Vertex::from(neighbour, self.graph));
                        }
                    }
                    None => {
                        let end = vertex.vertex;
                        self.stack.pop();
                        return Some(VertexInDFS::End(end));
                    }
                }
                self.next()
            }
            None => None,
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
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            Vec::<VertexInDFS>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::End(&VertexId(0))
            ]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::Begin(&VertexId(1)),
                VertexInDFS::End(&VertexId(1)),
                VertexInDFS::End(&VertexId(0))
            ]
        );
    }

    #[test]
    fn depth_first_enumerates_vertices_depth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::Begin(&VertexId(1)),
                VertexInDFS::Begin(&VertexId(3)),
                VertexInDFS::End(&VertexId(3)),
                VertexInDFS::Begin(&VertexId(4)),
                VertexInDFS::Begin(&VertexId(5)),
                VertexInDFS::End(&VertexId(5)),
                VertexInDFS::End(&VertexId(4)),
                VertexInDFS::End(&VertexId(1)),
                VertexInDFS::Begin(&VertexId(2)),
                VertexInDFS::End(&VertexId(2)),
                VertexInDFS::End(&VertexId(0)),
            ]
        );
    }

    #[test]
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::End(&VertexId(0))
            ]
        );
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(1))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(1)),
                VertexInDFS::End(&VertexId(1))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_only_once() {
        let graph = Graph::from(2, vec![(0, 1), (0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::Begin(&VertexId(1)),
                VertexInDFS::End(&VertexId(1)),
                VertexInDFS::End(&VertexId(0))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::Begin(&VertexId(1)),
                VertexInDFS::End(&VertexId(1)),
                VertexInDFS::End(&VertexId(0))
            ]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, &VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(&VertexId(0)),
                VertexInDFS::Begin(&VertexId(1)),
                VertexInDFS::Begin(&VertexId(2)),
                VertexInDFS::End(&VertexId(2)),
                VertexInDFS::End(&VertexId(1)),
                VertexInDFS::End(&VertexId(0))
            ]
        );
    }

    mod executes_functions {
        use super::*;

        #[test]
        fn finishes_edge_after_target_vertex_finished() {
            let graph = Graph::from(6, vec![(0, 1)]).unwrap();
            let mut edges = Vec::<(VertexId, VertexId)>::new();
            let mut dfs = DepthFirstWithFunctions::on(&graph, &VertexId(0), |from, to| {
                edges.push((from.clone(), to.clone()));
            });
            assert_eq!(dfs.next(), Some(VertexInDFS::Begin(&VertexId(0))));
            assert_eq!(edges, Vec::<(VertexId, VertexId)>::new());

            assert_eq!(dfs.next(), Some(VertexInDFS::Begin(&VertexId(1))));
            assert_eq!(edges, Vec::<(VertexId, VertexId)>::new());

            // assert_eq!(dfs.next(), Some(VertexInDFS::End(&VertexId(1))));
            // assert_eq!(edges, vec![(VertexId(0), VertexId(1))]);
        }

        //     #[test]
        //     fn ends_vertices_after_all_its_neighbours_have_been_visited() {
        //         let graph = Graph::from(6, vec![(0, 1), (0, 2)]).unwrap();
        //         let mut vertices = Vec::<String>::new();
        //         DepthFirstWithFunctions::on(
        //             &graph,
        //             &VertexId(0),
        //             VertexFunctions {
        //                 begin_vertex: |_| {},
        //                 end_vertex: |v| {
        //                     vertices.push(format!("E{:?}", v.0));
        //                 },
        //                 end_edge: |_, _| {},
        //             },
        //         )
        //         .into_iter()
        //         .collect::<Vec<&VertexId>>();
        //         assert_eq!(
        //             vertices,
        //             vec![
        //                 "B0".to_string(),
        //                 "B1".to_string(),
        //                 "E1".to_string(),
        //                 "B2".to_string(),
        //                 "E2".to_string(),
        //                 "E0".to_string(),
        //             ]
        //         );
        //     }

        // #[derive(Debug, PartialEq)]
        // struct SCCTest {
        //     unfinished_components: u64,
        //     next_component: u64,
        // }

        // #[test]
        // fn bla() {
        //     let graph = Graph::from(6, vec![(0, 1), (0, 2)]).unwrap();
        //     let mut scc = SCCTest {
        //         unfinished_components: 1,
        //         next_component: 5,
        //     };
        //     DepthFirstWithFunctions::on(
        //         &graph,
        //         &VertexId(0),
        //         VertexFunctions {
        //             begin_vertex: |_| {},
        //             end_vertex: |v| {
        //                 // if scc.unfinished_components == 1 {
        //                 scc.next_component = 6;
        //                 // }
        //             },
        //             end_edge: |_, _| scc.unfinished_components = 9,
        //         },
        //     )
        //     .into_iter()
        //     .collect::<Vec<&VertexId>>();
        //     assert_eq!(
        //         scc,
        //         SCCTest {
        //             unfinished_components: 1,
        //             next_component: 6
        //         }
        //     );
        // }
    }
}
