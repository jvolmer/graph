use std::collections::HashSet;

use crate::graph::{Graph, VertexId};

struct Vertex<'a> {
    vertex: VertexId,
    neighbours: Box<dyn Iterator<Item = VertexId> + 'a>,
    current_neighbour: Option<VertexId>,
}
impl<'a> Vertex<'a> {
    fn from(vertex: VertexId, graph: &'a Graph) -> Self {
        Self {
            vertex: vertex.clone(),
            neighbours: Box::new(graph.out_neighbors(vertex)),
            current_neighbour: None,
        }
    }
}

pub struct DepthFirstWithFunctions<'a, E>
where
    E: FnMut(VertexId, VertexId) -> (),
{
    graph: &'a Graph,
    stack: Vec<Vertex<'a>>,
    explored: HashSet<VertexId>,
    end_edge: E,
}
impl<'a, E> DepthFirstWithFunctions<'a, E>
where
    E: FnMut(VertexId, VertexId) -> (),
{
    pub fn on(graph: &'a Graph, start: VertexId, end_edge: E) -> Self {
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
impl<'a> DepthFirstWithFunctions<'a, fn(VertexId, VertexId)> {
    pub fn on_empty(graph: &'a Graph, start: VertexId) -> Self {
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
pub enum VertexInDFS {
    Begin(VertexId),
    End(VertexId),
}

impl<'a, E> Iterator for DepthFirstWithFunctions<'a, E>
where
    E: FnMut(VertexId, VertexId) -> (),
{
    type Item = VertexInDFS;

    fn next(&mut self) -> Option<Self::Item> {
        let stack_lengh = self.stack.len();
        if stack_lengh == 0 {
            return None;
        }
        match self.stack.get_mut(stack_lengh - 1) {
            Some(ref mut vertex) => {
                if !self.explored.contains(&vertex.vertex) {
                    self.explored.insert(vertex.vertex.clone());
                    return Some(VertexInDFS::Begin(vertex.vertex.clone()));
                }
                match vertex.current_neighbour.clone() {
                    Some(neighbour) => {
                        (self.end_edge)(vertex.vertex.clone(), neighbour);
                    }
                    None => (),
                }
                vertex.current_neighbour = vertex.neighbours.next();
                match vertex.current_neighbour.clone() {
                    Some(neighbour) => {
                        if !self.explored.contains(&neighbour) {
                            self.stack.push(Vertex::from(neighbour, self.graph));
                        }
                    }
                    None => {
                        let end = vertex.vertex.clone();
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
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            Vec::<VertexInDFS>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::End(VertexId(0))
            ]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::Begin(VertexId(1)),
                VertexInDFS::End(VertexId(1)),
                VertexInDFS::End(VertexId(0))
            ]
        );
    }

    #[test]
    fn depth_first_enumerates_vertices_depth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::Begin(VertexId(1)),
                VertexInDFS::Begin(VertexId(3)),
                VertexInDFS::End(VertexId(3)),
                VertexInDFS::Begin(VertexId(4)),
                VertexInDFS::Begin(VertexId(5)),
                VertexInDFS::End(VertexId(5)),
                VertexInDFS::End(VertexId(4)),
                VertexInDFS::End(VertexId(1)),
                VertexInDFS::Begin(VertexId(2)),
                VertexInDFS::End(VertexId(2)),
                VertexInDFS::End(VertexId(0)),
            ]
        );
    }

    #[test]
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::End(VertexId(0))
            ]
        );
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(1))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(1)),
                VertexInDFS::End(VertexId(1))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_only_once() {
        let graph = Graph::from(2, vec![(0, 1), (0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::Begin(VertexId(1)),
                VertexInDFS::End(VertexId(1)),
                VertexInDFS::End(VertexId(0))
            ]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::Begin(VertexId(1)),
                VertexInDFS::End(VertexId(1)),
                VertexInDFS::End(VertexId(0))
            ]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on_empty(&graph, VertexId(0))
                .into_iter()
                .collect::<Vec<VertexInDFS>>(),
            vec![
                VertexInDFS::Begin(VertexId(0)),
                VertexInDFS::Begin(VertexId(1)),
                VertexInDFS::Begin(VertexId(2)),
                VertexInDFS::End(VertexId(2)),
                VertexInDFS::End(VertexId(1)),
                VertexInDFS::End(VertexId(0))
            ]
        );
    }

    // #[test]
    // fn finishes_edge_after_target_vertex_finished() {
    //     let graph = Graph::from(6, vec![(0, 1)]).unwrap();
    //     let mut edges = Vec::<(VertexId, VertexId)>::new();
    //     let mut dfs = DepthFirstWithFunctions::on(&graph, VertexId(0), |from, to| {
    //         edges.push((from.clone(), to.clone()));
    //     });
    //     assert_eq!(dfs.next(), Some(VertexInDFS::Begin(VertexId(0))));
    //     assert_eq!(edges, Vec::<(VertexId, VertexId)>::new());

    //     assert_eq!(dfs.next(), Some(VertexInDFS::Begin(VertexId(1))));
    //     assert_eq!(edges, Vec::<(VertexId, VertexId)>::new());

    //     // assert_eq!(dfs.next(), Some(VertexInDFS::End(VertexId(1))));
    //     // assert_eq!(edges, vec![(VertexId(0), VertexId(1))]);
    // }
}
