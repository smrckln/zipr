use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

const FULL_BYTE: u8 = 255u8;

struct HuffTree {
    weight: usize,
    node: Node
}

enum Node {
    Tree(HuffTreeData),
    Leaf(u8)
}

struct HuffTreeData{
    left: Box<HuffTree>,
    right: Box<HuffTree>
}
impl Eq for HuffTree {}
impl PartialEq for HuffTree {
    fn eq(&self, other: &HuffTree) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for HuffTree {
    fn partial_cmp(&self, other: &HuffTree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffTree {
    fn cmp(&self, other: &HuffTree) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}
pub fn compress(bytes: &[u8], out_name: String) {
    let mut buffer: u8 = 0u8;
    let mut progress: u8 = 0u8;

    let mut output: Vec<u8> = Vec::new();

    let tree = make_tree(bytes);
    let mut encoding_table = HashMap::<u8, String>::new();

    make_encoding_table(&tree, &mut encoding_table, "".to_string());

    for byte in bytes {
        let code = encoding_table.get(byte).unwrap();
        for ch in code.as_str().chars() {
            let bit = if ch == '0' { 0 } else { 1 };
            buffer = (buffer << 1) | bit;
            progress = (progress << 1) | 1;

            if progress == FULL_BYTE {
                let byte_push = buffer;
                output.push(byte_push);
                progress = 0u8;
            }
        }
    }

}

pub fn decompress() {

}

fn make_tree(bytes: &[u8]) -> HuffTree {
    let byte_counts = byte_frequencies(bytes);
    let trees = freq_to_tree(byte_counts);

    huff_reduce(trees)
}

fn byte_frequencies(bytes: &[u8]) -> HashMap<u8, usize> {
    let mut byte_counts = HashMap::<u8, usize>::new();
    for byte in 0..255u8 {
        byte_counts.insert(byte, 0);
    }

    for byte in bytes.iter() {
        match byte_counts.get_mut(byte) {
            None => {},
            Some(count) => { *count += 1 }
        }
    }

    byte_counts
}

fn freq_to_tree(byte_counts: HashMap<u8, usize>) -> Vec<HuffTree> {
    byte_counts.iter().map(|(byte, count)| HuffTree {
        weight: *count,
        node: Node::Leaf(*byte)
    }).collect::<Vec<HuffTree>>()
}

fn huff_reduce(trees: Vec<HuffTree>) -> HuffTree {
    let mut queue = BinaryHeap::from(trees);

    loop {
        if queue.len() <= 1 {
            break;
        }

        let a = queue.pop().unwrap();
        let b = queue.pop().unwrap();

        queue.push(HuffTree {
            weight: a.weight + b.weight,
            node: Node::Tree(HuffTreeData{
                left: Box::<HuffTree>::new(a) ,
                right: Box::<HuffTree>::new(b)
            })
        });
    }

    queue.pop().unwrap()
}

fn make_encoding_table (tree: &HuffTree, table: &mut HashMap<u8, String>, prefix: String) {
    match tree.node {
        Node::Tree(ref tree_data) => {
            make_encoding_table(&*tree_data.left, table,
                                format!("{}0", prefix));
            make_encoding_table(&*tree_data.right, table,
                                format!("{}1", prefix));
        },

        Node::Leaf(byte) => {
            table.insert(byte, prefix.to_string());
        }
    }
}
