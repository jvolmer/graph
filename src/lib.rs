pub mod algorithms;
pub mod graph;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub use crate::algorithms::component::Component;
pub use crate::algorithms::enumeration::basic::graph::BreadthFirst as BreadthFirstOnGraph;
pub use crate::algorithms::enumeration::basic::graph::DepthFirst as DepthFirstOnGraph;
pub use crate::algorithms::enumeration::basic::tree::BreadthFirst as BreadthFirstOnTree;
pub use crate::algorithms::enumeration::basic::tree::DepthFirst as DepthFirstOnTree;
pub use crate::algorithms::enumeration::detailed::graph::DepthFirst as DetailedDepthFirstOnGraph;
pub use crate::algorithms::enumeration::detailed::tree::{
    DFSEntry, DepthFirst as DetailedDepthFirstOnTree,
};
pub use crate::algorithms::scc::algorithm::SCC;
pub use crate::graph::{Edge, Graph, VertexId};
