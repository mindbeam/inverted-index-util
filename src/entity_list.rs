use std::cmp::Ordering::{Equal, Greater, Less};
const ENTITY_ID_LEN: usize = 16;

pub fn insert_entity_mut(entity_list: &mut Vec<u8>, entity: &[u8; ENTITY_ID_LEN]) {
    let len = entity_list.len();
    assert_eq!(len.checked_rem(ENTITY_ID_LEN), Some(0));

    let mut size = len / ENTITY_ID_LEN;

    if size == 0 {
        entity_list.splice(0..0, entity.iter().copied());
        return;
    }

    let mut base = 0usize;
    while size > 1 {
        let half: usize = size / 2;
        let mid: usize = base + half;
        let start: usize = mid * ENTITY_ID_LEN;
        let end: usize = start + ENTITY_ID_LEN;

        // start >= 0
        // entity_list is a multiple of ENTITY_ID_LEN
        // end : (0 + (len / ENTITY_ID_LEN) / 2) <= len
        let cmp = unsafe { entity_list.get_unchecked(start..end) }.cmp(entity);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }

    let start = base * ENTITY_ID_LEN;
    let end = start + ENTITY_ID_LEN;
    let cmp = entity_list[start..end].cmp(entity);

    let offset = match cmp {
        Equal => return, // already present
        Less => (base + 1) * ENTITY_ID_LEN,
        Greater => base * ENTITY_ID_LEN,
    };

    entity_list.splice(offset..offset, entity.iter().copied());
}

pub enum ImmutResult {
    Changed(Vec<u8>),
    Unchanged,
}
pub fn insert_entity_immut(entity_list: &[u8], entity: &[u8; ENTITY_ID_LEN]) -> ImmutResult {
    let len = entity_list.len();
    assert_eq!(len.checked_rem(ENTITY_ID_LEN), Some(0));

    let mut size = len / ENTITY_ID_LEN;

    if size == 0 {
        let mut entity_list = Vec::with_capacity(1);
        entity_list.splice(0..0, entity.iter().copied());
        return ImmutResult::Changed(entity_list);
    }

    let mut base = 0usize;
    while size > 1 {
        let half: usize = size / 2;
        let mid: usize = base + half;
        let start: usize = mid * ENTITY_ID_LEN;
        let end: usize = start + ENTITY_ID_LEN;

        // start >= 0
        // entity_list is a multiple of ENTITY_ID_LEN
        // end : (0 + (len / ENTITY_ID_LEN) / 2) <= len
        let cmp = unsafe { entity_list.get_unchecked(start..end) }.cmp(entity);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }

    let start = base * ENTITY_ID_LEN;
    let end = start + ENTITY_ID_LEN;
    let cmp = entity_list[start..end].cmp(entity);

    let offset = match cmp {
        Equal => return ImmutResult::Unchanged, // already present
        Less => (base + 1) * ENTITY_ID_LEN,
        Greater => base * ENTITY_ID_LEN,
    };

    let mut entity_list = entity_list.to_vec();
    entity_list.splice(offset..offset, entity.iter().copied());

    ImmutResult::Changed(entity_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_document_mut() {
        let mut entity_list: Vec<u8> = Vec::new();

        insert_entity_mut(&mut entity_list, b"aaaaaaaaaaaaaaaa");
        insert_entity_mut(&mut entity_list, b"cccccccccccccccc");
        insert_entity_mut(&mut entity_list, b"aaaaaaaaaaaaaaaa");
        insert_entity_mut(&mut entity_list, b"bbbbbbbbbbbbbbbb");

        assert_eq!(&entity_list[..], &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
    }
    #[test]
    fn insert_document_immut() {
        let mut entity_list = Vec::new();

        match insert_entity_immut(&entity_list, b"aaaaaaaaaaaaaaaa") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut(&entity_list, b"cccccccccccccccc") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut(&entity_list, b"aaaaaaaaaaaaaaaa") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut(&entity_list, b"bbbbbbbbbbbbbbbb") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };

        assert_eq!(entity_list, &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
    }
}
