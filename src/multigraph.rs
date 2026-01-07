use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use ureq::Agent;

use crate::{Edge, EdgeId, Node, NodeId, PlotConfig, PlotSVG, Subgraph, SubgraphId};

/// Represents a [Multigraph](https://en.wikipedia.org/wiki/Multigraph) (supports multiple egdes between same pair of nodes).
/// Supports both directed and undirected edges specified by [`Edge`] builder methods.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Multigraph {
    edges: BTreeMap<EdgeId, Edge>,
    nodes: BTreeMap<NodeId, Node>,
    highlighted_edges: BTreeSet<EdgeId>,
    highlighted_nodes: BTreeSet<NodeId>,
    nodecolumns: BTreeSet<Vec<NodeId>>,
    nodelayers: BTreeSet<Vec<NodeId>>,
    subgraphs: BTreeMap<SubgraphId, Subgraph>,
    title: String,
}
impl Multigraph {
    /// Creates a new, empty [Multigraph](https://en.wikipedia.org/wiki/Multigraph).
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    /// Creates a new, empty [Multigraph](https://en.wikipedia.org/wiki/Multigraph) with title.
    pub fn from<T: Display>(title: T) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }
}
impl Multigraph {
    /// Creates a new [`Edge`] in the [`Multigraph`], between two [`Nodes`](Node) specified by their Label.
    ///
    /// # Arguments
    /// * `from` - A displayable type representing the Label of the source Node.
    /// * `to` - A displayable type representing the Label of the target Node.
    pub fn add<S, T>(&mut self, from: S, to: T) -> EdgeId
    where
        S: Display,
        T: Display,
    {
        // 1. add Nodes (from & to) to Graph
        let from_id = self.add_node(Node::from(from), false);
        let to_id = self.add_node(Node::from(to), false);

        // 2. add Edge to Graph
        self.add_edge(Edge::from(from_id, to_id), true)
    }
    /// Adds an [`Edge`] to the [`Multigraph`].
    ///
    /// # Arguments
    /// * `accept_multiedge` - Whether to create a new Edge when it already exists an Edge between same pair of Nodes,
    ///   allowing for multiple Edges, or replacing existing one.
    pub fn add_edge(&mut self, edge: Edge, accept_multiedge: bool) -> EdgeId {
        let new_edge_id = self.edges.len();

        // check: accept dublicates?
        if accept_multiedge {
            // insert Edge as unique (whether it exist or not)
            self.edges.insert(new_edge_id, edge);
        } else {
            // else: replace Edge (inserts if Edge don't exist)
            match self.edges.iter().find(|(_, existing_edge)| *existing_edge == &edge) {
                Some((existing_id, _)) => return *existing_id, // return existing edge-id
                None => self.edges.insert(new_edge_id, edge),  // insert new Edge
            };
        }

        // return new edge-id
        new_edge_id
    }
    /// Adds a [`Node`] to the [`Multigraph`].
    ///
    /// # Arguments
    /// * `accept_duplicates` - Whether to handle Nodes with identical Label and/or Object (SVG) uniquely, or not.
    pub fn add_node(&mut self, node: Node, accept_duplicated: bool) -> NodeId {
        let new_node_id = self.nodes.len();

        // check: accept duplicates?
        if accept_duplicated {
            // save as new node
            self.nodes.insert(new_node_id, node);
            return new_node_id;
        }

        // else: do Node already exist?
        match self.nodes.iter().find(|(_, existing_node)| *existing_node == &node) {
            Some((existing_id, _)) => return *existing_id, // return existing node-id
            None => self.nodes.insert(new_node_id, node),  // insert new Node
        };

        // return new node-id
        new_node_id
    }
    /// Adds a [`Subgraph`] to the [`Multigraph`]. Subgraphs can't overlap, and can not contain any nodes already in a
    /// Nodelayer.
    ///
    /// # Return
    /// Error when a Node is already a member in an existing Subgraph or Nodelayer.
    pub fn add_subgraph(&mut self, subgraph: Subgraph) -> Result<SubgraphId> {
        let new_subgraph_id = self.subgraphs.len();

        // check: empty?
        if subgraph.get_nodes().is_empty() {
            bail!("Subgraph is empty");
        }

        // check: not overlapping with another Subgraph?
        for (existing_id, existing_subgraph) in self.subgraphs.iter() {
            for new_node_id in subgraph.get_nodes() {
                if existing_subgraph.get_nodes().contains(new_node_id) {
                    bail!("Subgraph {existing_id} already contains Node {new_node_id} (overlap is not supported)");
                }
            }
        }

        // check: not overlapping with another Nodelayer?
        for (nodelayer_index, existing_nodelayer) in self.nodelayers.iter().enumerate() {
            for new_node_id in subgraph.get_nodes() {
                if existing_nodelayer.contains(new_node_id) {
                    bail!("Nodelayer {nodelayer_index} already contains Node {new_node_id} (overlap between Nodelayers and Subgraphs is not supported)");
                }
            }
        }

        // check: do Subgraph already exist?
        match self.subgraphs.iter().find(|(_, existing_subgraph)| *existing_subgraph == &subgraph) {
            Some((existing_id, _)) => return Ok(*existing_id),        // return existing subgraph-id
            None => self.subgraphs.insert(new_subgraph_id, subgraph), // insert new Subgraph
        };

        // return new subgraph-id
        Ok(new_subgraph_id)
    }
    /// Adds nodes that should be vertically aligned. Only supported by `Layout::Layered` and `Layout::Structured`.
    pub fn add_nodecolumn<I: IntoIterator<Item = NodeId>>(&mut self, nodes: I) -> Result<()> {
        let nodes_vec: Vec<_> = nodes.into_iter().collect();

        // check: empty?
        if nodes_vec.is_empty() {
            return Ok(());
        }

        // check: not overlapping with another Node-column?
        for (existing_id, existing_nodecolumn) in self.nodecolumns.iter().enumerate() {
            for new_node_id in nodes_vec.iter() {
                if existing_nodecolumn.contains(new_node_id) {
                    bail!("Node-column {existing_id} already contains Node {new_node_id}. Overlap is not allowed!");
                }
            }
        }

        // 1. insert Node-column
        self.nodecolumns.insert(nodes_vec);

        // return
        Ok(())
    }
    /// Adds a list of Nodes to same layer in the Graph. Only supported by `Layout::Layered` and `Layout::Structured`.
    /// Nodes in a Nodelayer can not occur in any existing Subgraph og Nodelayer.
    ///
    /// # Return
    /// Error when a Node is already a member in an existing Subgraph or Nodelayer.
    pub fn add_nodelayer<I: IntoIterator<Item = NodeId>>(&mut self, nodes: I) -> Result<()> {
        let nodes_vec: Vec<_> = nodes.into_iter().collect();

        // check: empty?
        if nodes_vec.is_empty() {
            return Ok(());
        }

        // check: not overlapping with another Subgraph?
        for (existing_id, existing_subgraph) in self.subgraphs.iter() {
            for new_node_id in nodes_vec.iter() {
                if existing_subgraph.get_nodes().contains(new_node_id) {
                    bail!("Subgraph {existing_id} already contains Node {new_node_id} (overlap between Nodelayers and Subgraphs is not supported)");
                }
            }
        }

        // 1. insert Nodelayer
        self.nodelayers.insert(nodes_vec);

        // return
        Ok(())
    }
    /// Highlights an [`Edge`] by its id.
    pub fn highlight_edge(&mut self, id: EdgeId) {
        self.highlighted_edges.insert(id);
    }
    /// Highlights a list of [`Edges`](Edge) (by id).
    pub fn highlight_edges<I: IntoIterator<Item = EdgeId>>(&mut self, ids: I) {
        self.highlighted_edges.extend(ids);
    }
    /// Highlights a [`Node`] by its id.
    pub fn highlight_node(&mut self, id: NodeId) {
        self.highlighted_nodes.insert(id);
    }
    /// Highlights a list of [`Nodes`](Node) (by id).
    pub fn highlight_nodes<I: IntoIterator<Item = NodeId>>(&mut self, ids: I) {
        self.highlighted_nodes.extend(ids);
    }

    // -- getters
    /// Returns a reference to the title.
    pub fn get_title(&self) -> &str {
        &self.title
    }
    /// Returns a reference to an [`Edge`] by its id.
    pub fn get_edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(&id)
    }
    /// Returns a reference to a [`Node`] by its id.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }
    /// Returns an reference to all nodecolumns in the graph.
    pub fn get_nodecolumns(&self) -> &BTreeSet<Vec<NodeId>> {
        &self.nodecolumns
    }
    /// Returns an reference to all nodelayers in the graph.
    pub fn get_nodelayers(&self) -> &BTreeSet<Vec<NodeId>> {
        &self.nodelayers
    }
    /// Return the number of Edges in the Graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    /// Return the number of Nodes in the Graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // -- iterators
    /// Returns an iterator over all [`Edges`](Edge) in the graph.
    pub fn edges(&self) -> impl DoubleEndedIterator<Item = (&EdgeId, &Edge)> {
        self.edges.iter()
    }
    /// **Danger!** Returns a mutable iterator over all [`Edges`](Edge) in the graph.
    pub fn edges_mut(&mut self) -> impl DoubleEndedIterator<Item = (&EdgeId, &mut Edge)> {
        self.edges.iter_mut()
    }
    /// Returns an iterator over all higlighted [`Edges`](Edge) in the graph.
    pub fn edges_highlighted(&self) -> impl DoubleEndedIterator<Item = &EdgeId> {
        self.highlighted_edges.iter()
    }
    /// Returns an iterator over all [`Nodes`](Node) in the graph.
    pub fn nodes(&self) -> impl DoubleEndedIterator<Item = (&NodeId, &Node)> {
        self.nodes.iter()
    }
    /// **Danger!** Returns a mutable iterator over all [`Nodes`](Node) in the graph.
    pub fn nodes_mut(&mut self) -> impl DoubleEndedIterator<Item = (&NodeId, &mut Node)> {
        self.nodes.iter_mut()
    }
    /// Returns an iterator over all higlighted [`Nodes`](Node) in the graph.
    pub fn nodes_highlighted(&self) -> impl DoubleEndedIterator<Item = &NodeId> {
        self.highlighted_nodes.iter()
    }
    /// Returns an iterator over all [`Subgraphs`](Node) in the graph.
    pub fn subgraphs(&self) -> impl DoubleEndedIterator<Item = (&SubgraphId, &Subgraph)> {
        self.subgraphs.iter()
    }

    // -- checkers
    /// Checks if an [`Edge`] is highlighted.
    pub fn is_edge_highlighted(&self, edge_id: &EdgeId) -> bool {
        self.highlighted_edges.contains(edge_id)
    }
    /// Checks if a [`Node`] is highlighted.
    pub fn is_node_highlighted(&self, node_id: &NodeId) -> bool {
        self.highlighted_nodes.contains(node_id)
    }
}
impl Multigraph {
    /// Plots a Multigraph and exports it to file.
    ///
    /// # Arguments
    /// * `filename` - name of the exported SVG plot: "plot.svg", or full path: "plots/test/plot.svg".
    /// * `api_key` - valid api_key to access Graphplot API.
    /// * `config` - optional argument spesifying user defined plot [`Config`].
    pub fn save<F: Display, T: Display>(&self, filename: F, api_key: T, config: Option<PlotConfig>) -> Result<()> {
        // 1. plot the Multigraph
        let plot = self.plot(api_key, config).context("Error when plotting Multigraph")?;

        // 2. save Plot as svg
        plot.save(filename).context("Error exporting Plot to svg")
    }
    /// Plots the Multigraph and returns a `PlotSVG`. Useful when nesting Multigraphs.
    ///
    /// # Arguments
    /// * `api_key` - valid API-key to access Graphplot API.
    /// * `config` - optional argument spesifying user defined plot [`Config`].
    pub fn plot<T: Display>(&self, api_key: T, config: Option<PlotConfig>) -> Result<PlotSVG> {
        #[derive(Clone, Debug, Serialize)]
        struct MultigraphPlotRequest<'a> {
            multigraph: &'a Multigraph,
            api_key: String,
            config: &'a PlotConfig,
        }
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct MultigraphPlotResponse {
            content: String,
            width: f32,
            height: f32,
        }
        #[derive(Debug, Clone, Deserialize)]
        struct ApiErrorResponse {
            error: String,
        }

        // check: user defined theme?
        let config = match config {
            Some(userdefined) => userdefined,
            None => PlotConfig::default(),
        };

        // 1. send request to server
        let agent_config = Agent::config_builder().http_status_as_error(false).build();
        let agent = Agent::new_with_config(agent_config);
        let endpoint = format!("{url}/multigraph", url = config.get_url());
        let mut response = agent
            .post(endpoint)
            .header("Authorization", &format!("Bearer {api_key}"))
            .send_json(&MultigraphPlotRequest {
                multigraph: &self,
                config: &config,
                api_key: api_key.to_string(),
            })
            .context("Error with request to Graphplot API")?;

        // 2. parse response (SVG plot)
        let plot: MultigraphPlotResponse = match response.status() == 200 {
            true => response.body_mut().read_json().context("Error deserializing Graphplot API response")?,
            false => {
                let status = response.status();
                match response.body_mut().read_json::<ApiErrorResponse>() {
                    Ok(api_error) => bail!("{status:?}: {}", api_error.error),
                    Err(_) => bail!("{status}: Failed to parse error-response from server (JSON)"),
                }
            }
        };

        // return
        Ok(PlotSVG::from(plot.content, plot.width, plot.height, config.get_style()))
    }
}
