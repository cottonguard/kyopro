pub mod binary_trie;
pub mod dsu;
pub mod fenwick_tree;
pub mod lazy_seg_tree;
mod segment_tree;
pub use segment_tree::*;
pub mod sparse_table;
pub mod splay_tree;
pub mod swag;
pub mod wavelet_matrix;

#[cfg(test)]
mod tests;
