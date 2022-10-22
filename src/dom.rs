/*
 * DOM はノードの木構造
 */

// ノードは 0 個以上の子ノードを持つ
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

pub enum NodeType {
    Element(ElementData),
    Text(String),
}

pub type AttrMap = HashMap<String, String>;

pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}
