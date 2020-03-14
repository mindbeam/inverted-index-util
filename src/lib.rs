//! # inverted-index-util
//!
//! This crate provides utilities for implementing inverted indexes
//!
//! ## Usage
//! ```rust
//!     use inverted_index_util::entity_list::insert_entity_mut;
//!     let mut entity_list: Vec<u8> = Vec::new();
//!
//!     insert_entity_mut(&mut entity_list, b"aaaaaaaaaaaaaaaa");
//!     insert_entity_mut(&mut entity_list, b"cccccccccccccccc");
//!     insert_entity_mut(&mut entity_list, b"aaaaaaaaaaaaaaaa");
//!     insert_entity_mut(&mut entity_list, b"bbbbbbbbbbbbbbbb");
//!
//!     assert_eq!(
//!         &entity_list[..],
//!         &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]
//!     );
//! ```

pub mod entity_list;
