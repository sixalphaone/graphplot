use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{GraphStyle, NodeId};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Subgraph {
    title: Option<String>,
    pub(crate) nodes: Vec<NodeId>,
    style: Option<GraphStyle>,
}
impl Subgraph {
    // -- builders
    /// Creates a new, empty [`Subgraph`].
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    /// Creates a new, empty [`Subgraph`] with title.
    pub fn from<T: Display>(title: T) -> Self {
        Self {
            title: Some(title.to_string()),
            ..Default::default()
        }
    }
    /// Creates a new [`Subgraph`] from Nodes.
    pub fn from_nodes<I: IntoIterator<Item = NodeId>>(nodes: I) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
            ..Default::default()
        }
    }
    /// Overwrites the Title for the [`Subgraph`].
    pub fn title<T: Display>(mut self, title: T) -> Self {
        self.title = Some(title.to_string());
        self
    }
    /// Adds Nodes to the [`Subgraph`].
    pub fn nodes<I: IntoIterator<Item = NodeId>>(mut self, nodes: I) -> Self {
        let mut nodes: Vec<_> = nodes.into_iter().collect();
        nodes.reverse();
        self.nodes = nodes;
        self
    }
    /// Set custom style for [`Subgraph`].
    pub fn style(mut self, graphstyle: GraphStyle) -> Self {
        self.style = Some(graphstyle);
        self
    }

    // -- setters
    /// Adds a [`Node`] to the [`Subgraph`] represented by its `NodeId`.
    pub fn add<I: IntoIterator<Item = NodeId>>(&mut self, nodes: I) {
        self.nodes.extend(nodes);
    }

    // -- getters
    /// Returns a reference to the Subgraph's Title.
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }
    /// Returns every `NodeId`'s in this Subgraph.
    pub fn get_nodes(&self) -> &Vec<NodeId> {
        &self.nodes
    }
    /// Returns a reference to custom style for the Subgraph, if any.
    pub fn get_style(&self) -> Option<&GraphStyle> {
        self.style.as_ref()
    }
}
// -- traits: custom defined Eq, PartialEq, Ord and PartialOrd
impl Eq for Subgraph {}
impl PartialEq for Subgraph {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.nodes == other.nodes
    }
}
impl Ord for Subgraph {
    fn cmp(&self, other: &Self) -> Ordering {
        self.title.cmp(&other.title).then_with(|| self.nodes.cmp(&other.nodes))
    }
}
impl PartialOrd for Subgraph {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
