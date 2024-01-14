use hex;
use ring::digest::{self, SHA256};

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub value: Vec<&'a str>, // Let's use something we can grow and append to
    pub children: Vec<Node<'a>>,
    pub hash: Vec<u8>,
}

impl<'a> Node<'a> {
    pub fn new(value: Vec<&'a str>) -> Self {
        Self {
            value: value.clone(),
            children: vec![],
            hash: hash(value),
        }
    }

    pub fn hash_to_hex(&self) -> String {
        hex::encode(self.hash.clone())
    }

    pub fn add_child(&mut self, child: Node<'a>) {
        self.children.push(child);
    }
}

fn hash(value: Vec<&str>) -> Vec<u8> {
    let digest = digest::digest(&SHA256, value.join("").as_bytes());
    digest.as_ref().to_vec()
}

#[derive(Debug, Clone)]
pub struct Tree<'a> {
    pub root: Node<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node() {
        let node_value = vec!["a", "b", "c"];
        let node = Node {
            value: node_value.clone(),
            children: vec![],
            hash: hash(node_value),
        };
        println!("{:?}", node.hash_to_hex());
        assert_eq!(node.value, vec!["a", "b", "c"]);
    }

    #[test]
    fn tree() {
        let root_value = vec!["a"];
        let root = Node {
            value: root_value.clone(),
            children: vec![],
            hash: hash(root_value),
        };
        let mut tree = Tree { root: root };
        println!("{:?}", tree.root.hash_to_hex());

        let child_value = vec!["b"];
        let child = Node {
            value: child_value.clone(),
            children: vec![],
            hash: hash(child_value),
        };
        tree.root.add_child(child);

        println!("{:?}", tree);
    }
}
