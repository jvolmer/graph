///! Union-find algorithm to create a disjoint-set data structure
///!
///! Creates a forest (undirected acyclic graph) that shows which nodes belong together. The root node of each tree (component) is used as the identify of the tree. The algorithms needs to know all involved nodes upfront. Then it can do two actions:
///! union two vertices: Add an undirected edge between the two nodes to make them part of the same tree
///! find a node: Give the identity of the including tree.
///!
///! Uses the union by size improvement.
///! Another improvement that could be implemented: path compression in find fn
use std::collections::HashMap;

use crate::graph::VertexId;

#[derive(Debug, PartialEq, Clone)]
enum Node {
    TreeRoot(usize),
    DecendentOf(VertexId),
}

#[derive(Debug, PartialEq)]
struct Component {
    id: VertexId,
    size: usize,
}

#[derive(Debug, PartialEq)]
struct UnionFind {
    list: HashMap<VertexId, Node>,
}
#[derive(Debug, PartialEq)]
enum Error {
    VertexNotIncluded(VertexId),
}

impl UnionFind {
    fn new(vertices: impl Iterator<Item = VertexId>) -> Self {
        Self {
            list: HashMap::from_iter(vertices.map(|v| (v.clone(), Node::TreeRoot(1)))),
        }
    }
    fn find(&self, id: VertexId) -> Result<Component, Error> {
        let x = self
            .list
            .get(&id)
            .ok_or(Error::VertexNotIncluded(id.clone()))?;
        match x.clone() {
            Node::TreeRoot(size) => return Ok(Component { id, size }),
            Node::DecendentOf(id) => return self.find(id),
        }
    }

    fn union(&mut self, x: VertexId, y: VertexId) -> Result<(), Error> {
        match (self.find(x), self.find(y)) {
            (
                Ok(Component {
                    id: xroot,
                    size: xsize,
                }),
                Ok(Component {
                    id: yroot,
                    size: ysize,
                }),
            ) => {
                if xroot != yroot {
                    if xsize < ysize {
                        self.list.insert(xroot, Node::DecendentOf(yroot.clone()));
                        self.list.insert(yroot, Node::TreeRoot(xsize + ysize));
                    } else {
                        self.list.insert(yroot, Node::DecendentOf(xroot.clone()));
                        self.list.insert(xroot, Node::TreeRoot(xsize + ysize));
                    }
                }
                return Ok(());
            }
            (Err(e), _) => return Err(e),
            (_, Err(e)) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_single_entry() {
        let union_find = UnionFind::new(vec![VertexId(1)].into_iter());
        assert_eq!(
            union_find.find(VertexId(1)),
            Ok(Component {
                id: VertexId(1),
                size: 1
            })
        );
    }

    #[test]
    fn finds_an_entry() {
        let union_find =
            UnionFind::new(vec![VertexId(1), VertexId(3), VertexId(5), VertexId(2)].into_iter());
        assert_eq!(
            union_find.find(VertexId(5)),
            Ok(Component {
                id: VertexId(5),
                size: 1
            })
        );
    }

    #[test]
    fn cannot_find_nonexistend_vertex() {
        let union_find = UnionFind::new(vec![VertexId(0)].into_iter());
        assert_eq!(
            union_find.find(VertexId(8)),
            Err(Error::VertexNotIncluded(VertexId(8)))
        );
    }

    #[test]
    fn united_vertices_are_in_same_component() {
        {
            let mut union_find =
                UnionFind::new(vec![VertexId(0), VertexId(1), VertexId(2)].into_iter());
            assert!(union_find.union(VertexId(2), VertexId(0)).is_ok());
            assert_eq!(union_find.find(VertexId(2)), union_find.find(VertexId(0)));
            assert!(union_find.find(VertexId(1)) != union_find.find(VertexId(2)));
        }
        {
            let mut union_find = UnionFind::new(
                vec![
                    VertexId(1),
                    VertexId(3),
                    VertexId(0),
                    VertexId(5),
                    VertexId(2),
                    VertexId(6),
                ]
                .into_iter(),
            );

            assert!(union_find.union(VertexId(1), VertexId(5)).is_ok());
            assert_eq!(union_find.find(VertexId(1)), union_find.find(VertexId(5)));

            assert!(union_find.union(VertexId(5), VertexId(3)).is_ok());
            assert_eq!(union_find.find(VertexId(5)), union_find.find(VertexId(3)));

            assert!(union_find.union(VertexId(2), VertexId(6)).is_ok());
            assert_eq!(union_find.find(VertexId(2)), union_find.find(VertexId(6)));

            assert!(union_find.find(VertexId(0)) != union_find.find(VertexId(1)));
            assert!(union_find.find(VertexId(0)) != union_find.find(VertexId(2)));
        }
    }

    #[test]
    fn vertices_are_added_to_bigger_sized_components() {
        let mut union_find =
            UnionFind::new(vec![VertexId(0), VertexId(1), VertexId(2), VertexId(3)].into_iter());
        assert!(union_find.union(VertexId(1), VertexId(2)).is_ok());
        assert!(union_find.union(VertexId(0), VertexId(1)).is_ok());
        assert!(union_find.union(VertexId(3), VertexId(1)).is_ok());
        assert_eq!(
            union_find,
            UnionFind {
                list: HashMap::from_iter(
                    vec![
                        (VertexId(1), Node::TreeRoot(4)),
                        (VertexId(0), Node::DecendentOf(VertexId(1))),
                        (VertexId(2), Node::DecendentOf(VertexId(1))),
                        (VertexId(3), Node::DecendentOf(VertexId(1)))
                    ]
                    .into_iter()
                )
            }
        );
    }

    #[test]
    fn cannot_union_nonexistend_vertex() {
        let mut union_find = UnionFind::new(vec![VertexId(0)].into_iter());
        assert_eq!(
            union_find.union(VertexId(0), VertexId(8)),
            Err(Error::VertexNotIncluded(VertexId(8)))
        );
        assert_eq!(
            union_find.union(VertexId(7), VertexId(0)),
            Err(Error::VertexNotIncluded(VertexId(7)))
        );
        assert_eq!(
            union_find.union(VertexId(6), VertexId(5)),
            Err(Error::VertexNotIncluded(VertexId(6)))
        );
    }
}
