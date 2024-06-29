#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct VertexId(pub usize);

#[derive(Debug, PartialEq)]
struct Edge(VertexId, VertexId);

#[derive(Debug, PartialEq)]
pub struct Graph {
    vertices: Vec<VertexId>,
    edges: Vec<Edge>,
    out_index: Vec<Vec<VertexId>>,
}
impl Graph {
    pub fn from(vertex_count: usize, edges: Vec<(usize, usize)>) -> Result<Self, String> {
        let mut out_index: Vec<Vec<VertexId>> = vec![vec![]; vertex_count];
        let edges = edges
            .into_iter()
            .map(|(from, to)| {
                if vertex_count > from && vertex_count > to {
                    let (from, to) = (VertexId(from), VertexId(to));
                    out_index[from.0].push(to.clone());
                    Ok(Edge(from, to))
                } else {
                    Err("Dangling edges are not allowed".to_string())
                }
            })
            .collect::<Result<Vec<Edge>, String>>();
        edges.and_then(|edges| {
            Ok(Self {
                vertices: (0..vertex_count).map(|i| VertexId(i)).collect(),
                edges,
                out_index,
            })
        })
    }

    pub fn out_neighbors_ref<'a>(
        &'a self,
        vertex: &'a VertexId,
    ) -> impl Iterator<Item = &'a VertexId> {
        self.out_index.get(vertex.0).unwrap().iter()
    }

    pub fn out_neighbors<'a>(&'a self, vertex: VertexId) -> impl Iterator<Item = VertexId> + 'a {
        self.out_index
            .get(vertex.0)
            .unwrap()
            .iter()
            .map(|v| v.clone())
    }

    pub fn contains(&self, vertex: &VertexId) -> bool {
        self.vertices.len() > vertex.0
    }

    pub fn vertices(&self) -> impl Iterator<Item = &VertexId> {
        self.vertices.iter()
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
                vertices: vec![
                    VertexId(0),
                    VertexId(1),
                    VertexId(2),
                    VertexId(3),
                    VertexId(4),
                    VertexId(5)
                ],
                edges: vec![
                    Edge(VertexId(0), VertexId(1)),
                    Edge(VertexId(4), VertexId(5)),
                    Edge(VertexId(1), VertexId(1))
                ],
                out_index: vec![
                    vec![VertexId(1)],
                    vec![VertexId(1)],
                    vec![],
                    vec![],
                    vec![VertexId(5)],
                    vec![]
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
                .out_neighbors_ref(&VertexId(0))
                .collect::<Vec<&VertexId>>(),
            vec![&VertexId(0), &VertexId(1), &VertexId(1), &VertexId(2)]
        );
    }
}
