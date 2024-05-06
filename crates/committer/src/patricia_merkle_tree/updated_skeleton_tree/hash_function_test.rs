use crate::felt::Felt;
use crate::hash::hash_trait::{HashFunction, HashInputPair, HashOutput};
use crate::hash::pedersen::PedersenHashFunction;
use crate::patricia_merkle_tree::filled_tree::node::ClassHash;
use crate::patricia_merkle_tree::filled_tree::node::Nonce;
use crate::patricia_merkle_tree::node_data::inner_node::{
    BinaryData, EdgeData, EdgePath, EdgePathLength, NodeData, PathToBottom,
};
use crate::patricia_merkle_tree::node_data::leaf::ContractState;
use crate::patricia_merkle_tree::node_data::leaf::LeafDataImpl;
use crate::patricia_merkle_tree::updated_skeleton_tree::hash_function::TreeHashFunction;
use crate::patricia_merkle_tree::updated_skeleton_tree::hash_function::TreeHashFunctionImpl;
use rstest::rstest;

#[rstest]
#[case(Felt::ONE, Felt::TWO, Felt::from_hex("0x5bb9440e27889a364bcb678b1f679ecd1347acdedcbf36e83494f857cc58026").unwrap())]
#[case(Felt::from(0xBE_u128), Felt::from(0xA0BEE_u128), Felt::from_hex("0x4e8f149d7d5adb77a8c85b631a3acb6fb9aa5ecb06ea4ec105753629243e380").unwrap())]
#[case(Felt::from(0x1234_u128), Felt::from(0xABCD_u128), Felt::from_hex("0x615bb8d47888d2987ad0c63fc06e9e771930986a4dd8adc55617febfcf3639e").unwrap())]
fn test_tree_hash_function_impl_binary_node(
    #[case] left_hash: Felt,
    #[case] right_hash: Felt,
    #[case] expected_hash: Felt,
) {
    let hash_output = TreeHashFunctionImpl::<PedersenHashFunction>::compute_node_hash(
        &NodeData::Binary(BinaryData {
            left_hash: HashOutput(left_hash),
            right_hash: HashOutput(right_hash),
        }),
    );
    assert_eq!(
        hash_output,
        PedersenHashFunction::compute_hash(HashInputPair(left_hash, right_hash))
    );
    assert_eq!(hash_output, HashOutput(expected_hash));
}

#[rstest]
#[case(Felt::ONE, Felt::TWO, 3,  Felt::from_hex("0x5bb9440e27889a364bcb678b1f679ecd1347acdedcbf36e83494f857cc58029").unwrap())]
#[case(Felt::from(0xBE_u128), Felt::from(0xA0BEE_u128), 0xBB,  Felt::from_hex("0x4e8f149d7d5adb77a8c85b631a3acb6fb9aa5ecb06ea4ec105753629243e43b").unwrap())]
#[case(Felt::from(0x1234ABCD_u128),Felt::from(42_u128),6, Felt::from_hex("0x1d937094c09b5f8e26a662d21911871e3cbc6858d55cc49af9848ea6fed4e9").unwrap())]
fn test_tree_hash_function_impl_edge_node(
    #[case] bottom_hash: Felt,
    #[case] edge_path: Felt,
    #[case] length: u8,
    #[case] expected_hash: Felt,
) {
    let hash_output = TreeHashFunctionImpl::<PedersenHashFunction>::compute_node_hash(
        &NodeData::Edge(EdgeData {
            bottom_hash: HashOutput(bottom_hash),
            path_to_bottom: PathToBottom {
                path: EdgePath(edge_path),
                length: EdgePathLength(length),
            },
        }),
    );
    let direct_hash_computation = HashOutput(
        PedersenHashFunction::compute_hash(HashInputPair(bottom_hash, edge_path)).0
            + Felt::from(length),
    );
    assert_eq!(hash_output, HashOutput(expected_hash));
    assert_eq!(hash_output, direct_hash_computation);
}

#[rstest]
#[case(NodeData::Leaf(LeafDataImpl::CompiledClassHash(ClassHash(Felt::from_hex("0xACDC").unwrap()))), Felt::from_hex("0xACDC").unwrap())]
#[case(NodeData::Leaf(LeafDataImpl::StorageValue(Felt::from_hex("0xDEAFBEEF").unwrap())), Felt::from_hex("0xDEAFBEEF").unwrap())]
// Random StateTreeTuples and the expected hash results were generated and computed elsewhere.
#[case(
    NodeData::Leaf(
        LeafDataImpl::ContractState(ContractState {
            class_hash: ClassHash(Felt::from_hex("0x150917f3bba17e3c0be685981d6cf8874098a857f354d374688e76eb69f44ab").unwrap()), 
            storage_root_hash: HashOutput(Felt::from_hex("0x5d6b3a46ac855f3cea9af60e13cc6c81bf237b901f49f73981a521d4cb6c50c").unwrap()), 
            nonce: Nonce(Felt::from_hex("0x38").unwrap()) 

        }
        )
    ),
    Felt::from_hex("0x3f27688d56740e5e238acde5b408154563c4f6e05514b8029e86ad51c388f8b").unwrap()
)]
#[case(
    NodeData::Leaf(
        LeafDataImpl::ContractState(ContractState{
            class_hash: ClassHash(Felt::from_hex("0x2c9982b9bd36f16e409c98616e43dbc9e4b47db8a8e8edc3b915bc4dab3c61c").unwrap()), 
            storage_root_hash: HashOutput(Felt::from_hex("0x63d7c2a04df01e361238cf4bf07b7cee2e3c5ee58b4ec37cf279aed9ac8d013").unwrap()), 
            nonce: Nonce(Felt::from_hex("0x1b").unwrap()) 
        })
    ),
    Felt::from_hex("0x26e95377fcfa70a9882dc08653b7dc77165befe23971aa2fa5b926ff81af6cc").unwrap()
)]
#[case(
    NodeData::Leaf(
        LeafDataImpl::ContractState(ContractState{
            class_hash: ClassHash(Felt::from_hex("0x314bd50b446c4e8c3a82b0eb9326e532432fbd925ab7afea236f7f8618848a2").unwrap()), 
            storage_root_hash: HashOutput(Felt::from_hex("0x1118159298289b372808e6b008b6bc9d68f5ebc060962ea1331fdc70cf3d047").unwrap()), 
            nonce: Nonce(Felt::from_hex("0x47").unwrap()) 
        })
    ),
    Felt::from_hex("0x1b20bbb35009bf03f86fb092b56a9c44deedbcca6addf8f7640f54a48ba5bbc").unwrap()
)]
fn test_tree_hash_function_impl_leaf_node(
    #[case] node_data: NodeData<LeafDataImpl>,
    #[case] expected_hash: Felt,
) {
    let hash_output = TreeHashFunctionImpl::<PedersenHashFunction>::compute_node_hash(&node_data);
    assert_eq!(hash_output, HashOutput(expected_hash));
}
