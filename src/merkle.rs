use ring::digest::{SHA256, self};
use hex;

pub struct Node<'a> {
    pub value: Vec<&'a str>, // Let's use something we can grow and append to
    pub children: Vec<Node<'a>>,
    pub hash: Vec<u8>,
}

impl<'a> Node<'a> {
    pub fn new(value: Vec<&'a str>) -> Self {
        Self { value: value.clone(), children: vec![], hash: hash(value) }
    }

    pub fn hash_to_hex(&self) -> String {
        hex::encode(self.hash.clone())
    }
}

fn hash(value: Vec<&str>) -> Vec<u8> {
    let digest = digest::digest(&SHA256, value.join("").as_bytes());
    digest.as_ref().to_vec()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = Node {
            value: vec!["a", "b", "c"],
            children: vec![],
            hash: hash(vec!["a", "b", "c"]),
        };
        println!("{:?}", node.hash_to_hex());
        assert_eq!(node.value, vec!["a", "b", "c"]);
    }
}