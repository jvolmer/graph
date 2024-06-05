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
}

#[derive(Debug, PartialEq)]
pub struct VertexId(pub usize);

#[derive(Debug, PartialEq)]
struct Edge(VertexId, VertexId);

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
}
