use crate::{Component, Graph};

use super::union_find::UnionFind;

pub struct WCC<'a> {
    graph: &'a Graph,
    union_find: UnionFind,
}

impl<'a> WCC<'a> {
    pub fn on(graph: &'a Graph) -> Self {
        Self {
            graph,
            union_find: UnionFind::new(graph.vertices().map(|v| v.clone())),
        }
    }
    pub fn get(self) -> impl Iterator<Item = Component> {
        let mut union_find = self.union_find;
        self.graph
            .edges()
            .map(|e| (e.clone().0, e.1.clone()))
            .for_each(|(from, to)| union_find.union(from, to).unwrap());
        union_find.all_components()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Graph, VertexId};

    use super::*;

    #[test]
    fn empty_graph_has_no_component() {
        let graph = Graph::from(0, vec![]).unwrap();
        assert_eq!(
            WCC::on(&graph).get().collect::<Vec<_>>(),
            Vec::<Component>::new()
        );
    }

    #[test]
    fn single_vertex_is_a_weak_component() {
        let graph = Graph::from(1, vec![]).unwrap();
        assert_eq!(
            WCC::on(&graph).get().collect::<Vec<_>>(),
            vec![Component::from(vec![VertexId(0)])]
        );
    }

    #[test]
    fn unconnected_vertices_are_in_separate_components() {
        let graph = Graph::from(2, vec![]).unwrap();
        let wcc = WCC::on(&graph).get().collect::<Vec<_>>();
        assert_eq!(wcc.len(), 2);
        assert!(wcc.contains(&Component::from(vec![VertexId(0)])));
        assert!(wcc.contains(&Component::from(vec![VertexId(1)])));
    }

    #[test]
    fn connected_vertices_are_in_same_components_indepent_of_edge_direction() {
        let graph = Graph::from(2, vec![(0, 1)]).unwrap();
        assert_eq!(
            WCC::on(&graph).get().collect::<Vec<_>>(),
            vec![Component::from(vec![VertexId(0), VertexId(1)]),]
        );

        let graph = Graph::from(2, vec![(1, 0)]).unwrap();
        assert_eq!(
            WCC::on(&graph).get().collect::<Vec<_>>(),
            vec![Component::from(vec![VertexId(1), VertexId(0)]),]
        );
    }

    #[test]
    fn finds_components_on_some_random_graph() {
        let graph = Graph::from(6, vec![(0, 1), (1, 0), (2, 3), (3, 4), (5, 2), (2, 5)]).unwrap();
        let wcc = WCC::on(&graph).get().collect::<Vec<_>>();
        assert_eq!(wcc.len(), 2);
        assert!(wcc.contains(&Component::from(vec![VertexId(0), VertexId(1)])));
        assert!(wcc.contains(&Component::from(vec![
            VertexId(2),
            VertexId(3),
            VertexId(4),
            VertexId(5)
        ])));
    }
}
