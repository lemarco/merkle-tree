mod merkle_tree;

use merkle_tree::{Hashable, MerkleTreeBuilder};
// #[allow(unconditional_recursion)]
impl Hashable for i32 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
// impl Hashable for Vec<i32> {
//     fn as_bytes(&self) -> Vec<u8> {
//         let mut output = vec![];
//         output.extend(self.iter().flat_map(|item| item.to_le_bytes()));

//         output
//     }
// }
// impl Hashable for [i32] {
//     fn as_bytes(&self) -> Vec<u8> {
//         let mut output = vec![];
//         output.extend(self.iter().flat_map(|item| item.to_le_bytes()));

//         output
//     }
// }
fn main() {
    // let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let a = data.as_slice();
    let tree = MerkleTreeBuilder::new().add(&[1, 2, 3, 4, 5, 6, 7]);

    println!("tree: {}", tree);
}
