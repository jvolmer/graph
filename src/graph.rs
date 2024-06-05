#[derive(Debug, PartialEq, Clone)]
pub struct VertexId(pub usize);

#[derive(Debug, PartialEq)]
struct Edge(VertexId, VertexId);

#[derive(Debug, PartialEq)]
pub struct Graph {
    edges: Vec<Edge>,
}
impl Graph {
    pub fn from(edges: Vec<(usize, usize)>) -> Self {
        Self {
            edges: edges
                .into_iter()
                .map(|(from, to)| Edge(VertexId(from), VertexId(to)))
                .collect(),
        }
    }

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
            Graph::from(vec![(0, 1), (4, 5), (1, 1)]),
            Graph {
                edges: vec![
                    Edge(VertexId(0), VertexId(1)),
                    Edge(VertexId(4), VertexId(5)),
                    Edge(VertexId(1), VertexId(1))
                ]
            }
        );
    }

    #[test]
    fn gets_out_neighbors() {
        let graph = Graph::from(vec![(0, 0), (0, 1), (0, 1), (0, 2), (1, 4)]);
        assert_eq!(
            graph
                .out_neighbors(&VertexId(0))
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(1), &VertexId(2)]
        );
    }
}
