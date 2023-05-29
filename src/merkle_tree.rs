use std::io::Write;

use crypto_hash::Hasher;

type Hash = Vec<u8>;

pub trait Hashable {
    fn as_bytes(&self) -> Vec<u8>;
    fn hash(&self, hasher: &mut Hasher) -> Vec<u8> {
        hasher
            .write_all(&self.as_bytes())
            .expect("cannot hash data");
        let data = hasher.finish();
        hasher.flush().ok();
        data
    }
}
impl Hashable for Vec<u8> {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

pub fn next_power_of_2(n: usize) -> usize {
    let mut v = n;
    println!("1: {}", v);
    v -= 1;
    println!("2: {}", v);
    v |= v >> 1;
    println!("3: {}", v);
    v |= v >> 2;
    println!("4: {}", v);
    v |= v >> 4;
    println!("5: {}", v);
    v |= v >> 8;
    println!("6: {}", v);
    v |= v >> 16;
    println!("7: {}", v);
    v += 1;
    println!("8: {}", v);
    v
}

const INTERNAL_SIG: u8 = 1u8;

pub struct MerkleTreeBuilder {
    hasher: Hasher,
    // nodes: Vec<Hash>,
    // count_internal_nodes: usize,
    // count_leaves: usize,
}
pub struct MerkleTree {
    pub nodes: Vec<Hash>,
    pub count_internal_nodes: usize,
    pub count_leaves: usize,
}
impl std::fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let nodes = self
            .nodes
            .iter()
            .map(hex::encode)
            .reduce(|init, curr| init + ",\n" + &curr)
            .unwrap();

        writeln!(
            f,
            "count_internal_nodes: {},\ncount_leaves: {},\nnodes:\n{} ",
            self.count_internal_nodes, self.count_leaves, nodes
        )
    }
}

impl MerkleTreeBuilder {
    pub fn new() -> Self {
        Self {
            hasher: Hasher::new(crypto_hash::Algorithm::SHA256),
        }
    }

    pub fn add<T: Hashable>(&mut self, values: &[T]) -> MerkleTree {
        self.build_with_hasher(values)
    }

    fn build_with_hasher<T>(&mut self, values: &[T]) -> MerkleTree
    where
        T: Hashable,
    {
        println!("Func: build_with_hasher()");
        let count_leaves = values.len();
        assert!(
            count_leaves > 1,
            "expected more then 1 value, received {}",
            count_leaves
        );

        let leaves: Vec<Hash> = values.iter().map(|v| v.hash(&mut self.hasher)).collect();

        self.build_from_leaves_with_hasher(leaves.as_slice())
    }
    fn calculate_internal_nodes_count(&self, count_leaves: usize) -> usize {
        next_power_of_2(count_leaves) - 1
    }
    fn hash_internal_node(&mut self, left: &Hash, right: Option<&Hash>) -> Hash {
        let mut input: Vec<u8> = vec![];
        input.extend(&[INTERNAL_SIG]);
        input.extend(left);
        if let Some(r) = right {
            input.extend(r);
        } else {
            input.extend(left);
        }

        input.hash(&mut self.hasher)
    }
    fn build_upper_level(&mut self, nodes: &[Hash]) -> Vec<Hash> {
        let mut row = Vec::with_capacity((nodes.len() + 1) / 2);
        let mut i = 0;
        while i < nodes.len() {
            if i + 1 < nodes.len() {
                row.push(self.hash_internal_node(&nodes[i], Some(&nodes[i + 1])));
                i += 2;
            } else {
                row.push(self.hash_internal_node(&nodes[i], None));
                i += 1;
            }
        }

        if row.len() > 1 && row.len() % 2 != 0 {
            let last_node = row.last().unwrap().clone();
            row.push(last_node);
        }

        row
    }
    fn build_internal_nodes(&mut self, nodes: &mut [Hash], count_internal_nodes: usize) {
        let mut parents = self.build_upper_level(&nodes[count_internal_nodes..]);

        let mut upper_level_start = count_internal_nodes - parents.len();
        let mut upper_level_end = upper_level_start + parents.len();
        nodes[upper_level_start..upper_level_end].clone_from_slice(&parents);

        while parents.len() > 1 {
            parents = self.build_upper_level(parents.as_slice());

            upper_level_start -= parents.len();
            upper_level_end = upper_level_start + parents.len();
            nodes[upper_level_start..upper_level_end].clone_from_slice(&parents);
        }

        nodes[0] = parents.remove(0);
    }

    fn build_from_leaves_with_hasher(&mut self, leaves: &[Hash]) -> MerkleTree {
        let count_leaves = leaves.len();
        let count_internal_nodes = self.calculate_internal_nodes_count(count_leaves);
        let mut nodes = vec![Vec::new(); count_internal_nodes + count_leaves];

        // copy leafs
        nodes[count_internal_nodes..].clone_from_slice(leaves);

        self.build_internal_nodes(&mut nodes, count_internal_nodes);
        MerkleTree {
            nodes,
            count_internal_nodes,
            count_leaves,
        }
    }
}
