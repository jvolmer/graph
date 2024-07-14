# Comprehensive Graph Analytics Algorithms

Algorithms for analyzing data in a graph. The purpose of this crate is to document algorithms comprehensibly rather than to focus on the best performance possible.

## Motivation

Algorithms are often hard to understand and their implementation is most of the time even harder to comprehend. Performance plays a significant role when implementing algorithms, which often results in adding additional tweaks to the implementation, making it even harder to read and obsuring the main algorithm idea.
But although an algorithm is complicated does not mean the code has to be unreadable. Code can abstract away details, show the main idea of an algorithm first and hide the details in abstractions. I wanted to learn some graph algorithms and implement them in a way that I can still understand them in some months / years and hopefully these implementations can help others as well. The implementations are therefore not focused on the best performance but on readability. I also tried to create an easy user interface. There will probably be more algorithms to come.

## Overview

Algorithms:
- [x] Depth First Search (Basic and Detailed variant) on a single tree and on full graph
- [x] Breadth First Search on a single tree and on full graph
- [x] Strongly connected components
- [ ] Weakly connected components
- [ ] ...

## Examples

### Strongly connected components
```rust

  // 3   ->  0   ->  5  <->  6
  //
  // ^                       ^
  // |   /   |       |       |
  //    L    v       v       v
  //
  // 1   ->  7  <->  2  <-   4

  let graph = graph::Graph::from(8, vec![
                (1, 3),
                (3, 0),
                (0, 1),
                (0, 7),
                (1, 7),
                (7, 2),
                (2, 7),
                (0, 5),
                (5, 2),
                (5, 6),
                (6, 5),
                (6, 4),
                (4, 6),
                (4, 2),
            ],
        )
        .unwrap();

  let mut scc = graph::SCC::on(&graph).into_iter();
  assert_eq!(
    scc.next(),
    Some(graph::Component::from(vec![graph::VertexId(2), graph::VertexId(7)]))
  );
  assert_eq!(
    scc.next(),
    Some(graph::Component::from(vec![graph::VertexId(4), graph::VertexId(5), graph::VertexId(6)]))
  );
  assert_eq!(
    scc.next(),
    Some(graph::Component::from(vec![graph::VertexId(0), graph::VertexId(1), graph::VertexId(3)]))
  );
  assert_eq!(scc.next(), None);

```

### Breadth First
```rust

// 1  <-   0  <->  2
//
// |  \    |
// v   J   v
//
  // 3       4

  let graph = graph::Graph::from(5, vec![(1,3), (1,4), (0,1), (0,4), (0,2), (2,0)]).unwrap();
  
  let mut bfs = graph::BreadthFirstOnGraph::on(&graph).into_iter();
  assert_eq!(bfs.next(), Some(&graph::VertexId(0)));
  assert_eq!(bfs.next(), Some(&graph::VertexId(1)));
  assert_eq!(bfs.next(), Some(&graph::VertexId(4)));
  assert_eq!(bfs.next(), Some(&graph::VertexId(2)));
  assert_eq!(bfs.next(), Some(&graph::VertexId(3)));
  assert_eq!(bfs.next(), None);

```

### Detailed Depth First
```rust

  // 1  <-  0  ->  2
  //
  // |
  // v
  // 
  // 3
  
  let graph = graph::Graph::from(4, vec![(1,3), (0,1), (0,2)]).unwrap();

  let mut dfs = graph::DetailedDepthFirstOnGraph::on(&graph).into_iter();
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginVertex(graph::VertexId(0))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginEdge(graph::Edge(graph::VertexId(0), graph::VertexId(1)))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginVertex(graph::VertexId(1))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginEdge(graph::Edge(graph::VertexId(1), graph::VertexId(3)))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginVertex(graph::VertexId(3))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndVertex(graph::VertexId(3))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndEdge(graph::Edge(graph::VertexId(1), graph::VertexId(3)))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndVertex(graph::VertexId(1))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndEdge(graph::Edge(graph::VertexId(0), graph::VertexId(1)))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginEdge(graph::Edge(graph::VertexId(0), graph::VertexId(2)))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::BeginVertex(graph::VertexId(2))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndVertex(graph::VertexId(2))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndEdge(graph::Edge(graph::VertexId(0), graph::VertexId(2)))));
  assert_eq!(dfs.next(), Some(graph::DFSEntry::EndVertex(graph::VertexId(0))));
  assert_eq!(dfs.next(), None);
  
  ```
