use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{NodeStyle, TYPST_PREFIX};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub enum NodeShape {
    Circle,
    #[default]
    Rectangle,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Node {
    label: String,
    identifier: Option<String>,
    shape: NodeShape,
    object: Option<String>,
    position: Option<(f32, f32)>,
    style: Option<NodeStyle>,
}
impl Node {
    /// Creates a new [`Node`] without Label.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a new [`Node`] from a Label.
    pub fn from<L: Display>(label: L) -> Self {
        Self {
            label: label.to_string(),
            ..Default::default()
        }
    }
    /// Creates a new [`Node`] from a Label.
    pub fn from_object<SVG: Display>(object: SVG) -> Self {
        Self {
            object: Some(object.to_string()),
            ..Default::default()
        }
    }
}
impl Node {
    /// Changes the shape of the [`Node`] to a circle.
    pub fn circle(mut self) -> Self {
        self.shape = NodeShape::Circle;
        self
    }
    /// Gives an unique identifier to the Node. This is helpful when two or more Nodes need to have same Label.
    pub fn identifier<S: Display>(mut self, identifier: S) -> Self {
        self.identifier = Some(identifier.to_string());
        self
    }
    /// Overwrites the Label for the [`Node`].
    pub fn label<L: Display>(mut self, label: L) -> Self {
        self.label = label.to_string();
        self
    }
    /// Adds an Object (SVG) to the [`Node`]. SVG-element must be without surroundings <svg>-tags, only the element-tag
    /// like <rect>, <group>, <text> etc.
    ///
    /// # Example
    /// ```
    /// let node = Node::from("A").object("<rect width='40' height='20' fill='black'></rect>");
    /// ```
    pub fn object<SVG: Display>(mut self, object: SVG) -> Self {
        self.object = Some(object.to_string());
        self
    }
    /// Adds [typst](https://typst.app) as the content of the [`Node`]. Typst [symbols](https://typst.app/docs/reference/symbols/sym/).
    ///
    /// # Example
    /// ```
    /// let node = Node::from("Expression").typst("$ F(b) - F(a) = integral_a^b f(x) d x $");
    /// ```
    pub fn typst<T: Display>(mut self, typst: T) -> Self {
        self.object = Some(format!("{TYPST_PREFIX}\n{typst}"));
        self
    }
    /// Sets the position of the [`Node`].
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.position = Some((x, y));
        self
    }
    /// Set custom style for [`Node`].
    pub fn style(mut self, nodestyle: NodeStyle) -> Self {
        self.style = Some(nodestyle);
        self
    }

    // -- getters
    /// Returns a reference to the Node's identifier.
    pub fn get_identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }
    /// Returns a reference to the Node's Label.
    pub fn get_label(&self) -> &str {
        &self.label
    }
    /// Returns a reference to the Node's shape (`NodeShape`).
    pub fn get_shape(&self) -> &NodeShape {
        &self.shape
    }
    /// Returns a reference to the Node's Object (SVG), if any.
    pub fn get_object(&self) -> Option<&str> {
        self.object.as_deref()
    }
    /// Returns the Node's position, if any.
    pub fn get_position(&self) -> Option<(f32, f32)> {
        self.position
    }
    /// Returns a reference to custom style for the Node, if any.
    pub fn get_style(&self) -> Option<&NodeStyle> {
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
}
// -- traits: custom defined Eq, PartialEq, Ord and PartialOrd
impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.identifier == other.identifier
            && self.shape == other.shape
            && self.object == other.object
            && self.position == other.position
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label
            .cmp(&other.label)
            .then_with(|| self.identifier.cmp(&other.identifier))
            .then_with(|| self.shape.cmp(&other.shape))
            .then_with(|| self.object.cmp(&other.object))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
