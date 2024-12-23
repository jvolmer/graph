# Comprehensive Graph Analytics Algorithms

[![Coverage](https://codecov.io/gh/jvolmer/graph/branch/main/graph/badge.svg)](https://codecov.io/gh/jvolmer/graph) [![Build](https://github.com/jvolmer/graph/actions/workflows/ci.yml/badge.svg)](https://github.com/jvolmer/graph/actions/workflows/ci.yml)

Algorithms for analyzing data in a graph. The purpose of this crate is to document algorithms comprehensibly rather than to focus on the best performance possible.

## Motivation

Algorithms and their implementation can often be hard to comprehend. But although an algorithm is complicated does not mean the code has to be unreadable: Abstractions help us to focus on the main ideas and to hide details. I wanted to learn some graph algorithms and implemented them in such a way that I can (hopefully) still understand them in some months / years. The implementations focus on readability and the core algorithm ideas rather than performance. I also tried to create an easy user interface. There will probably be more algorithms to come.

## Overview

Algorithms:
- [x] Depth First Search (Basic and Detailed variant) on a single tree and on full graph
- [x] Breadth First Search on a single tree and on full graph
- [x] Strongly connected components
- [ ] Weakly connected components
- [ ] Shortest Path
- [ ] K-Shortest Paths
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
