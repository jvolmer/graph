struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

struct Vertex {}

struct VertexId {
    id: usize,
}
impl VertexId {
    pub fn index(&self) -> usize {
        self.id
    }
}

struct Edge {
    from: usize,
    to: usize,
}
