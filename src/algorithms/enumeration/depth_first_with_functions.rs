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

pub struct VertexFunctions<VB, VE, E>
where
    VB: FnMut(&VertexId) -> (),
    VE: FnMut(&VertexId) -> (),
    E: FnMut(&VertexId, &VertexId) -> (),
{
    pub begin_vertex: VB,
    pub end_vertex: VE,
    pub end_edge: E,
}
impl VertexFunctions<fn(&VertexId), fn(&VertexId), fn(&VertexId, &VertexId)> {
    pub fn empty() -> Self {
        Self {
            begin_vertex: |_| {},
            end_vertex: |_| {},
            end_edge: |_, _| {},
        }
    }
}

pub struct DepthFirstWithFunctions<'a, VB, VE, E>
where
    VB: FnMut(&VertexId) -> (),
    VE: FnMut(&VertexId) -> (),
    E: FnMut(&VertexId, &VertexId) -> (),
{
    graph: &'a Graph,
    stack: Vec<Vertex<'a>>,
    explored: HashSet<&'a VertexId>,
    vertex_functions: VertexFunctions<VB, VE, E>,
}
impl<'a, VB, VE, E> DepthFirstWithFunctions<'a, VB, VE, E>
where
    VB: FnMut(&VertexId) -> (),
    VE: FnMut(&VertexId) -> (),
    E: FnMut(&VertexId, &VertexId) -> (),
{
    pub fn on(
        graph: &'a Graph,
        start: &'a VertexId,
        vertex_functions: VertexFunctions<VB, VE, E>,
    ) -> Self {
        if graph.contains(&start) {
            Self {
                graph,
                stack: vec![Vertex::from(start, graph)],
                explored: HashSet::new(),
                vertex_functions,
            }
        } else {
            Self {
                graph,
                stack: vec![],
                explored: HashSet::new(),
                vertex_functions,
            }
        }
    }
}

impl<'a, VB, VE, E> Iterator for DepthFirstWithFunctions<'a, VB, VE, E>
where
    VB: FnMut(&VertexId) -> (),
    VE: FnMut(&VertexId) -> (),
    E: FnMut(&VertexId, &VertexId) -> (),
{
    type Item = &'a VertexId;

    fn next(&mut self) -> Option<Self::Item> {
        let stack_lengh = self.stack.len();
        if stack_lengh == 0 {
            return None;
        }
        match self.stack.get_mut(stack_lengh - 1) {
            Some(ref mut vertex) => {
                if !self.explored.contains(vertex.vertex) {
                    self.explored.insert(vertex.vertex);
                    (self.vertex_functions.begin_vertex)(vertex.vertex);
                    return Some(vertex.vertex);
                }
                match vertex.current_neighbour {
                    Some(neighbour) => {
                        (self.vertex_functions.end_edge)(vertex.vertex, neighbour);
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
                        (self.vertex_functions.end_vertex)(vertex.vertex);
                        self.stack.pop();
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
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            Vec::<&VertexId>::new()
        );
    }

    #[test]
    fn finds_sole_vertex() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0)]
        );
    }

    #[test]
    fn iterates_vertices() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1)]
        );
    }

    #[test]
    fn depth_first_enumerates_vertices_depth_first() {
        let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![
                &VertexId(0),
                &VertexId(1),
                &VertexId(3),
                &VertexId(4),
                &VertexId(5),
                &VertexId(2),
            ]
        );
        assert!(false);
    }

    #[test]
    fn only_finds_connected_vertices() {
        let graph = Graph::from(2, vec![]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0)]
        );
    }

    #[test]
    fn only_searches_in_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(1), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(1)]
        );
    }

    #[test]
    fn finds_each_vertex_only_once() {
        let graph = Graph::from(2, vec![(0, 1), (0, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1),]
        );
    }

    #[test]
    fn finds_each_vertex_in_a_loop_once() {
        let graph = Graph::from(2, vec![(0, 1), (1, 0)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1),]
        );

        let graph = Graph::from(3, vec![(0, 1), (1, 2), (2, 1)]).unwrap();
        assert_eq!(
            DepthFirstWithFunctions::on(&graph, &VertexId(0), VertexFunctions::empty())
                .into_iter()
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(2)]
        );
    }

    mod executes_functions {
        use super::*;

        #[test]
        fn begins_vertices_in_depth_first_order() {
            let graph = Graph::from(6, vec![(0, 1), (0, 2), (4, 5), (1, 3), (1, 4)]).unwrap();
            let mut vertices = Vec::<String>::new();
            DepthFirstWithFunctions::on(
                &graph,
                &VertexId(0),
                VertexFunctions {
                    begin_vertex: |v| {
                        vertices.push(format!("B{:?}", v.0));
                    },
                    end_vertex: |_| {},
                    end_edge: |_, _| {},
                },
            )
            .into_iter()
            .collect::<Vec<&VertexId>>();
            assert_eq!(
                vertices,
                vec![
                    "B0".to_string(),
                    "B1".to_string(),
                    "B3".to_string(),
                    "B4".to_string(),
                    "B5".to_string(),
                    "B2".to_string(),
                ]
            );
        }

        #[test]
        fn ends_vertices_after_all_its_neighbours_have_been_visited() {
            let graph = Graph::from(6, vec![(0, 1), (0, 2)]).unwrap();
            let mut vertices = Vec::<String>::new();
            DepthFirstWithFunctions::on(
                &graph,
                &VertexId(0),
                VertexFunctions {
                    begin_vertex: |v| {
                        vertices.push(format!("B{:?}", v.0));
                    },
                    end_vertex: |v| {
                        vertices.push(format!("E{:?}", v.0));
                    },
                    end_edge: |_, _| {},
                },
            )
            .into_iter()
            .collect::<Vec<&VertexId>>();
            assert_eq!(
                vertices,
                vec![
                    "B0".to_string(),
                    "B1".to_string(),
                    "E1".to_string(),
                    "B2".to_string(),
                    "E2".to_string(),
                    "E0".to_string(),
                ]
            );
        }
    }
}
