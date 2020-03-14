use std::convert::TryInto;

pub fn insert_entity_mut(entity_list: &mut Vec<u8>, entity_id: &[u8; 16]) {
    let mut chunks: Vec<[u8; 16]> = entity_list.chunks_exact(16).map(|v| v.try_into().unwrap()).collect();

    match chunks.binary_search(&entity_id) {
        Ok(_) => {}
        Err(i) => {
            chunks.insert(i, entity_id.clone());
        }
    }
    *entity_list = chunks.concat()
}

pub fn insert_entity_immut(entity_list: &[u8], entity_id: &[u8; 16]) -> Vec<u8> {
    let mut chunks: Vec<[u8; 16]> = entity_list.chunks_exact(16).map(|v| v.try_into().unwrap()).collect();

    match chunks.binary_search(&entity_id) {
        Ok(_) => {}
        Err(i) => {
            chunks.insert(i, entity_id.clone());
        }
    }
    chunks.concat()
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert_document_mut() {
        let mut entity_list: Vec<u8> = Vec::new();

        super::insert_entity_mut(&mut entity_list, b"aaaaaaaaaaaaaaaa");
        super::insert_entity_mut(&mut entity_list, b"cccccccccccccccc");
        super::insert_entity_mut(&mut entity_list, b"aaaaaaaaaaaaaaaa");
        super::insert_entity_mut(&mut entity_list, b"bbbbbbbbbbbbbbbb");

        assert_eq!(&entity_list[..], &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
    }
    #[test]
    fn insert_document_immut() {
        let document_list = b"";

        let document_list = super::insert_entity_immut(document_list, b"aaaaaaaaaaaaaaaa");
        let document_list = super::insert_entity_immut(&document_list, b"cccccccccccccccc");
        let document_list = super::insert_entity_immut(&document_list, b"aaaaaaaaaaaaaaaa");
        let document_list = super::insert_entity_immut(&document_list, b"bbbbbbbbbbbbbbbb");

        assert_eq!(document_list, &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
    }
}
