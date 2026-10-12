//! Dependency-graph resolution.
//!
//! The resolver operates purely over the [`crate::models`] types and is
//! game-agnostic. See [`DependencyGraph`](crate::models::DependencyGraph) for the algorithm.
//!
//! Dependency version suffixes are ignored: every package resolves to its
//! latest available version unless requested as `Owner-Name-Version` or held by
//! [`DependencyGraph::pins`](crate::models::DependencyGraph::pins). See
//! [`DependencyGraph::resolve`](crate::models::DependencyGraph::resolve) for the
//! full limitation.

#![deny(missing_docs)]

pub use crate::models::{DependencyGraph, Resolution};
