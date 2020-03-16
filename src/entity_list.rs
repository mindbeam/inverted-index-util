use std::cmp::Ordering::{Equal, Greater, Less};
use typenum::uint::Unsigned;

pub fn insert_entity_mut<N>(entity_list: &mut Vec<u8>, entity: &[u8])
where
    N: Unsigned,
{
    let entity_len = N::to_usize();
    assert_eq!(entity_len, entity.len());

    let list_len = entity_list.len();

    assert_eq!(list_len.checked_rem(entity_len), Some(0));
    let mut size = list_len / entity_len;

    if size == 0 {
        entity_list.splice(0..0, entity.iter().copied());
        return;
    }

    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let start = mid * entity_len;
        let end = start + entity_len;
        let cmp = entity_list[start..end].cmp(entity);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }

    let start = base * entity_len;
    let end = start + entity_len;
    let cmp = entity_list[start..end].cmp(entity);

    let offset = match cmp {
        Equal => return, // already present
        Less => (base + 1) * entity_len,
        Greater => base * entity_len,
    };

    entity_list.splice(offset..offset, entity.iter().copied());
}

pub enum ImmutResult {
    Changed(Vec<u8>),
    Unchanged,
}
pub fn insert_entity_immut<N>(entity_list: &[u8], entity: &[u8]) -> ImmutResult
where
    N: Unsigned,
{
    let entity_len = N::to_usize();
    assert_eq!(entity_len, entity.len());

    let list_len = entity_list.len();

    assert_eq!(list_len.checked_rem(entity_len), Some(0));
    let mut size = list_len / entity_len;

    if size == 0 {
        let mut entity_list = entity_list.to_vec();
        entity_list.splice(0..0, entity.iter().copied());
        return ImmutResult::Changed(entity_list);
    }

    let mut base = 0usize;
    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let start = mid * entity_len;
        let end = start + entity_len;
        let cmp = entity_list[start..end].cmp(entity);
        base = if cmp == Greater { base } else { mid };
        size -= half;
    }

    let start = base * entity_len;
    let end = start + entity_len;
    let cmp = entity_list[start..end].cmp(entity);

    let offset = match cmp {
        Equal => return ImmutResult::Unchanged, // already present
        Less => (base + 1) * entity_len,
        Greater => base * entity_len,
    };

    let mut entity_list = entity_list.to_vec();
    entity_list.splice(offset..offset, entity.iter().copied());

    ImmutResult::Changed(entity_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::consts::{U16, U32};

    #[test]
    fn insert_document_mut_16() {
        let mut entity_list: Vec<u8> = Vec::new();

        insert_entity_mut::<U16>(&mut entity_list, b"aaaaaaaaaaaaaaaa");
        insert_entity_mut::<U16>(&mut entity_list, b"cccccccccccccccc");
        insert_entity_mut::<U16>(&mut entity_list, b"aaaaaaaaaaaaaaaa");
        insert_entity_mut::<U16>(&mut entity_list, b"bbbbbbbbbbbbbbbb");

        assert_eq!(&entity_list[..], &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
    }
    #[test]
    fn insert_document_immut_16() {
        let mut entity_list = Vec::new();

        match insert_entity_immut::<U16>(&entity_list, b"aaaaaaaaaaaaaaaa") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut::<U16>(&entity_list, b"cccccccccccccccc") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut::<U16>(&entity_list, b"aaaaaaaaaaaaaaaa") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut::<U16>(&entity_list, b"bbbbbbbbbbbbbbbb") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };

        assert_eq!(entity_list, &b"aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbcccccccccccccccc"[..]);
    }

    #[test]
    fn insert_document_mut_32() {
        let mut entity_list: Vec<u8> = Vec::new();

        insert_entity_mut::<U32>(&mut entity_list, b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        insert_entity_mut::<U32>(&mut entity_list, b"cccccccccccccccccccccccccccccccc");
        insert_entity_mut::<U32>(&mut entity_list, b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        insert_entity_mut::<U32>(&mut entity_list, b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");

        assert_eq!(
            &entity_list[..],
            &b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccccccccccccccccccccccccccccc"[..]
        );
    }
    #[test]
    fn insert_document_immut_32() {
        let mut entity_list = Vec::new();

        match insert_entity_immut::<U32>(&entity_list, b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut::<U32>(&entity_list, b"cccccccccccccccccccccccccccccccc") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut::<U32>(&entity_list, b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };
        match insert_entity_immut::<U32>(&entity_list, b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb") {
            ImmutResult::Changed(l) => entity_list = l,
            ImmutResult::Unchanged => {}
        };

        assert_eq!(
            entity_list,
            &b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcccccccccccccccccccccccccccccccc"[..]
        );
    }
}
