#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct VertexId(pub usize);

#[derive(Debug, PartialEq)]
struct Edge(VertexId, VertexId);

#[derive(Debug, PartialEq)]
pub struct Graph {
    vertex_count: usize,
    edges: Vec<Edge>,
}
impl Graph {
    pub fn from(vertex_count: usize, edges: Vec<(usize, usize)>) -> Result<Self, String> {
        edges
            .into_iter()
            .map(|(from, to)| {
                if vertex_count > from && vertex_count > to {
                    Ok(Edge(VertexId(from), VertexId(to)))
                } else {
                    Err("Dangling edges are not allowed".to_string())
                }
            })
            .collect::<Result<Vec<Edge>, String>>()
            .and_then(|edges| {
                Ok(Self {
                    vertex_count,
                    edges,
                })
            })
    }

    // TODO can be done more efficiently when using an index (has to only be computed once)
    pub fn out_neighbors<'a>(&'a self, vertex: &'a VertexId) -> impl Iterator<Item = &'a VertexId> {
        self.edges.iter().filter(|e| e.0 == *vertex).map(|e| &e.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_graph_from_edge_topology() {
        assert_eq!(
            Graph::from(6, vec![(0, 1), (4, 5), (1, 1)]).unwrap(),
            Graph {
                vertex_count: 6,
                edges: vec![
                    Edge(VertexId(0), VertexId(1)),
                    Edge(VertexId(4), VertexId(5)),
                    Edge(VertexId(1), VertexId(1))
                ]
            }
        );
    }

    #[test]
    fn does_not_create_graph_with_dangling_edges() {
        assert!(Graph::from(0, vec![(0, 0)]).is_err());
    }

    #[test]
    fn gets_out_neighbors() {
        let graph = Graph::from(5, vec![(0, 0), (0, 1), (0, 1), (0, 2), (1, 4)]).unwrap();
        assert_eq!(
            graph
                .out_neighbors(&VertexId(0))
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(1), &VertexId(2)]
        );
    }
}
