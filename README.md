# inverted-index-util
This crate provides utilities for implementing inverted indexes

## Usage
```rust
    let mut document_list: Vec<u8> = Vec::new();

    inverted_index_util::document_list::insert_document_mut(&mut document_list, b"aaaaaaaaaaaaaaaa");
    inverted_index_util::document_list::insert_document_mut(&mut document_list, b"cccccccccccccccc");
    inverted_index_util::document_list::insert_document_mut(&mut document_list, b"aaaaaaaaaaaaaaaa");
    inverted_index_util::document_list::insert_document_mut(&mut document_list, b"bbbbbbbbbbbbbbbb");

    assert_eq!(&document_list[..], &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
```
