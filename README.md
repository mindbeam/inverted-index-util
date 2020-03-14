# inverted-index-util
This crate provides utilities for implementing inverted indexes

## Usage
```rust
    use inverted_index_util::document_list::insert_document_mut;
    let mut document_list: Vec<u8> = Vec::new();

    insert_document_mut(&mut document_list, b"aaaaaaaaaaaaaaaa");
    insert_document_mut(&mut document_list, b"cccccccccccccccc");
    insert_document_mut(&mut document_list, b"aaaaaaaaaaaaaaaa");
    insert_document_mut(&mut document_list, b"bbbbbbbbbbbbbbbb");

    assert_eq!(
        &document_list[..],
        &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]
    );
```
