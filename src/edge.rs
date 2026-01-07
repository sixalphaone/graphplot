use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{EdgeStyle, NodeId, Port, SubgraphId, TYPST_PREFIX};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum ArrowKind {
    #[default]
    Single,
    DoubleEnded,
    None,
}
impl ArrowKind {
    pub fn is_double_ended(&self) -> bool {
        matches!(self, ArrowKind::DoubleEnded)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Edge {
    from: NodeId,
    to: NodeId,
    label: Option<String>,
    object: Option<String>,
    weight: Option<f64>,
    options: EdgeOptions,
    style: Option<EdgeStyle>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub struct EdgeOptions {
    arrowkind: ArrowKind,
    dashed: bool,
    floating: bool,
    headport: Option<Port>,
    tailport: Option<Port>,
    from_subgraph: Option<SubgraphId>,
    to_subgraph: Option<SubgraphId>,
}
impl Edge {
    /// Creates a new [`Edge`] from a source [`Node`](crate::Node) to a target [`Node`](crate::Node).
    pub fn from(from: NodeId, to: NodeId) -> Self {
        Self {
            from,
            to,
            ..Default::default()
        }
    }
}
impl Edge {
    // -- builders
    /// Sets the Label for the [`Edge`].
    pub fn label<L: Display>(mut self, label: L) -> Self {
        self.label = Some(label.to_string());
        self
    }
    /// Adds an Object (SVG) to the [`Edge`]. SVG-element must be without surroundings <svg>-tags, only the element-tag
    /// like <rect>, <group>, <text> etc.
    ///
    /// # Example
    /// ```
    /// let edge = Edge::from(source, target)
    ///     .label("Hi")
    ///     .object("<rect width='40' height='20' fill='black'></rect>");
    /// ```
    pub fn object<SVG: Display>(mut self, object: SVG) -> Self {
        self.object = Some(object.to_string());
        self
    }
    /// Adds [typst](https://typst.app) as the content of the [`Edge`]. Typst [symbols](https://typst.app/docs/reference/symbols/sym/).
    ///
    /// # Example
    /// ```
    /// let edge = Edge::from(a, b).typst("$ (partial g)/(partial x) = 0 $");
    /// ```
    pub fn typst<T: Display>(mut self, typst: T) -> Self {
        self.object = Some(format!("{TYPST_PREFIX}\n{typst}"));
        self
    }
    /// Makes the Edge dashed.
    pub fn dashed(mut self) -> Self {
        self.options.dashed = true;
        self
    }
    /// Makes the [`Edge`] a double-ended arrow.
    pub fn double_ended(mut self) -> Self {
        self.options.arrowkind = ArrowKind::DoubleEnded;
        self
    }
    /// Makes the [`Edge`] have no arrow.
    pub fn no_arrow(mut self) -> Self {
        self.options.arrowkind = ArrowKind::None;
        self
    }
    /// Whether or not Edge-label should be "floating" above the line, meaning no Node or Edge should care about the
    /// Edge-label in the overall layout.
    pub fn floating(mut self) -> Self {
        self.options.floating = true;
        self
    }
    /// Sets a port for head-Node.
    pub fn headport(mut self, port: Port) -> Self {
        self.options.headport = Some(port);
        self
    }
    /// Sets a port for tail-Node.
    pub fn tailport(mut self, port: Port) -> Self {
        self.options.tailport = Some(port);
        self
    }
    /// Sets a Subgraph as the source.
    pub fn from_subgraph(mut self, subgraph: SubgraphId) -> Self {
        self.options.from_subgraph = Some(subgraph);
        self
    }
    /// Sets a Subgraph as the target.
    pub fn to_subgraph(mut self, subgraph: SubgraphId) -> Self {
        self.options.to_subgraph = Some(subgraph);
        self
    }
    /// Set custom style for [`Edge`].
    pub fn style(mut self, edgestyle: EdgeStyle) -> Self {
        self.style = Some(edgestyle);
        self
    }
    /// Adds a weight to the [`Edge`].
    pub fn weight<W: Into<f64>>(mut self, weight: W) -> Self {
        self.weight = Some(weight.into());
        self
    }

    // -- getters
    /// Returns the ID of the source Node.
    pub fn get_from(&self) -> NodeId {
        self.from
    }
    /// Returns the ID of the target Node.
    pub fn get_to(&self) -> NodeId {
        self.to
    }
    /// Returns a reference to the Label of the Edge, if any.
    pub fn get_label(&self) -> Option<&str> {
        self.label.as_deref()
    }
    /// Returns a reference to the Object (SVG) of the Edge, if any.
    pub fn get_object(&self) -> Option<&str> {
        self.object.as_deref()
    }
    /// Returns a reference to custom style for the Edge, if any.
    pub fn get_style(&self) -> Option<&EdgeStyle> {
        self.style.as_ref()
    }
    /// Returns typst content, if any.
    pub fn get_typst(&self) -> Option<String> {
        if let Some(content) = &self.object {
            if content.starts_with(TYPST_PREFIX) {
                return Some(content.replace(TYPST_PREFIX, ""));
            }
        }
        None
    }
    /// Returns the weight of the Edge, if any.
    pub fn get_weight(&self) -> Option<f64> {
        self.weight
    }

    // -- getters: EdgeOptions
    /// Returns the kind of arrow for the Edge.
    pub fn get_arrowkind(&self) -> ArrowKind {
        self.options.arrowkind
    }
    /// Returns wether the Edge is dashed.
    pub fn get_dashed(&self) -> bool {
        self.options.dashed
    }
    /// Return whether Edge-label is "floating".
    pub fn get_floating(&self) -> bool {
        self.options.floating
    }
    /// Returns the port of the head-Node.
    pub fn get_headport(&self) -> Option<Port> {
        self.options.headport
    }
    /// Returns the port of the tail-Node.
    pub fn get_tailport(&self) -> Option<Port> {
        self.options.tailport
    }
    /// Returns the ID of the source Subgraph.
    pub fn get_from_subgraph(&self) -> Option<SubgraphId> {
        self.options.from_subgraph
    }
    /// Returns the ID of the target Subgraph.
    pub fn get_to_subgraph(&self) -> Option<SubgraphId> {
        self.options.to_subgraph
    }
}
// -- traits: custom defined Eq, PartialEq, Ord and PartialOrd
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.label == other.label && self.object == other.object && self.weight == other.weight
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Sammenlign 'from'-feltet først
        match self.from.partial_cmp(&other.from) {
            // Hvis de er like, fortsett til neste felt
            Some(Ordering::Equal) => {}
            // Hvis de IKKE er like, har vi resultatet vårt. Returner det.
            result => return result,
        }

        // Sammenlign 'to'-feltet
        match self.to.partial_cmp(&other.to) {
            Some(Ordering::Equal) => {}
            result => return result,
        }

        // ...og så videre for de andre feltene...
        match self.label.partial_cmp(&other.label) {
            Some(Ordering::Equal) => {}
            result => return result,
        }

        // Her bruker vi partial_cmp for vekten!
        match self.weight.partial_cmp(&other.weight) {
            Some(Ordering::Equal) => {}
            result => return result,
        }

        // Hvis alt annet var likt, sammenlign det siste feltet
        self.object.partial_cmp(&other.object)
    }
}
