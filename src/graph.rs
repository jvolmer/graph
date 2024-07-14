/// A unique identifier for a vertex

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub struct VertexId(pub usize);

/// A directed edge between two vertices

#[derive(Debug, PartialEq, Clone)]
pub struct Edge(
    /// from
    pub VertexId,
    /// to
    pub VertexId,
);

/// An immutable graph structure for analytics
///
/// A graph is defined via its vertices and edges. A vertex is uniquely defined via its VertexId. An edge connects two vertices in a specified direction. In this implementation the vertices and edges do not contain any data.
/// The same vertices can be connected by several edges pointing in the same direction. Dangling edges (edges whos start or end point do not exist) are forbidden and cannot be created with the given implementation. Edges can also have the same start and end vertex.
/// A graph is immutable, once created it cannot be changed.

/// The graph also includes an out index for faster lookups of out neighbours.

#[derive(Debug, PartialEq)]
pub struct Graph {
    vertices: Vec<VertexId>,
    edges: Vec<Edge>,
    out_index: Vec<Vec<VertexId>>,
}
impl Graph {
    /// Creates a graph with vertex_count vertices and the given edges
    ///
    /// Unique vertex ids are created by a simple counter.
    ///
    /// # Errors
    ///
    /// Returns `Err` if one of the given edges is dangling: This happens if a given edge references a vertex that does not exist (happens if the referenced vertex id is larger than the given vertex_count).
    ///
    /// # Examples
    ///
    /// Creates a graph with three vertices (with ids 0, 1 and 2) and one edge that goes from vertex 0 to vertex 1:
    /// ```
    /// use graph::Graph;
    ///
    /// Graph::from(3, vec![(0,1)]).unwrap();
    /// ```
    ///
    /// A dangling edge results in an error:
    /// ```
    /// use graph::Graph;
    ///
    /// assert!(Graph::from(1, vec![(0,1)]).is_err());
    /// ```

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

    // TODO returning refs does not make sense because this is a ref to out_index, not vertices
    pub fn out_neighbors_ref<'a>(
        &'a self,
        vertex: &'a VertexId,
    ) -> impl Iterator<Item = &'a VertexId> {
        self.out_index.get(vertex.0).unwrap().iter()
    }

    /// Gives an iterator over all out neighbors for the given `vertex`
    ///
    /// Out neighbors of `vertex` are all vertices v for which an edge from the given `vertex` to v exists. The order in which the neighbours are given is not predefined.
    ///
    /// # Panics
    ///
    /// Will panic if `vertex` is not part of the graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use graph::{Graph, VertexId};
    ///
    /// let graph = Graph::from(4, vec![(1,3), (2,1), (0,0), (1,0)]).unwrap();
    ///
    /// let mut out_neighbors = graph.out_neighbors(VertexId(1));
    ///
    /// assert_eq!(out_neighbors.next(), Some(VertexId(3)));
    /// assert_eq!(out_neighbors.next(), Some(VertexId(0)));
    /// assert_eq!(out_neighbors.next(), None);
    /// ```

    pub fn out_neighbors<'a>(&'a self, vertex: VertexId) -> impl Iterator<Item = VertexId> + 'a {
        self.out_index
            .get(vertex.0)
            .unwrap() // TODO get rid of panic
            .iter()
            .map(|v| v.clone())
    }

    /// Checks if the graph contains a given `vertex`
    ///
    /// # Examples
    ///
    /// ```
    /// use graph::{Graph, VertexId};
    ///
    /// let graph = Graph::from(2, vec![]).unwrap();
    ///
    /// assert!(graph.contains(&VertexId(0)));
    /// assert!(graph.contains(&VertexId(1)));
    /// assert!(!graph.contains(&VertexId(2)));
    /// ```

    pub fn contains(&self, vertex: &VertexId) -> bool {
        self.vertices.len() > vertex.0
    }

    /// Gives an iterator over all vertices in the graph
    ///
    /// # Examples
    ///
    /// ```
    /// use graph::{Graph, VertexId};
    ///
    /// let graph = Graph::from(2, vec![]).unwrap();
    ///
    /// let mut vertices = graph.vertices();
    ///
    /// assert_eq!(vertices.next(), Some(&VertexId(0)));
    /// assert_eq!(vertices.next(), Some(&VertexId(1)));
    /// assert_eq!(vertices.next(), None);
    /// ```

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
