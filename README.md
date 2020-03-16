[![Build Status](https://travis-ci.org/mindbeam/inverted-index-util.svg?branch=master)](https://travis-ci.org/mindbeam/inverted-index-util)

# inverted-index-util
This crate provides utilities for implementing inverted indexes

## Usage
```rust
    use inverted_index_util::entity_list::insert_entity_mut;
    use typenum::consts::U16;
    let mut entity_list: Vec<u8> = Vec::new();

    insert_entity_mut::<U16>(&mut entity_list, b"aaaaaaaaaaaaaaaa");
    insert_entity_mut::<U16>(&mut entity_list, b"cccccccccccccccc");
    insert_entity_mut::<U16>(&mut entity_list, b"aaaaaaaaaaaaaaaa");
    insert_entity_mut::<U16>(&mut entity_list, b"bbbbbbbbbbbbbbbb");

    assert_eq!(
        &entity_list[..],
        &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]
    );
```
