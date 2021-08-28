use super::SetToSet;

use crate::id::IdLike;

use crate::methods::{SharedAnyToSet, SharedSet};
use crate::methods::{ViewAnyToSet, AnyToSet};
use crate::methods::{ViewSet, Set};

use crate::raw_junctions::set_to_set::RawSetToSet;

use std::collections::BTreeSet;

use crate::moogcell::InteriorVSet;
use crate::iterators::{ToSetKeysIterator, ToSetKeyValueIterator, VSetIterator};

use crate::structures::VSet;

// == type ==
pub struct Fwd<'a, A: IdLike, B: IdLike> { pub(in crate::shared_junctions) me: &'a SetToSet<A, B> }
pub struct FwdSet<'a, A: IdLike, B: IdLike> { 
    pub(in crate::shared_junctions) parent: &'a SetToSet<A, B>, 
    cache: InteriorVSet<'a, RawSetToSet<A, B>, A, B>,
    pub(in crate::shared_junctions) key: A 
}

// == caching ==
impl <'a, A: IdLike, B: IdLike> FwdSet<'a, A, B> {
    pub(in crate::shared_junctions) fn fetch(&self) -> VSet<'a, A, B> {
        return self.cache.get_or_compute_arg(|o| o.fwd().get_short(self.key).0)
    }
}

// == main impl ==
impl <'a, A: IdLike, B: IdLike> SharedAnyToSet<'a, A, B> for Fwd<'a, A, B> {
    type Multi = FwdSet<'a, A, B>;
    type Expunge = BTreeSet<B>;  

    type Iter = impl 'a+DoubleEndedIterator<Item=(A, B)>;
    type Keys = impl 'a+DoubleEndedIterator<Item=A>;
    type Sets = impl 'a+DoubleEndedIterator<Item=(A, Self::Multi)>;
    type Values = impl 'a+DoubleEndedIterator<Item=B>;

    fn get(&self, a: A) -> Self::Multi { FwdSet { 
        parent: self.me, 
        cache: self.me.raw.create_interior_vset::<A, B>(), 
        key: a 
    } }
    fn contains_key(&self, a: A) -> bool { self.me.raw.borrow().fwd().contains_key(a) }

    fn len(&self) -> usize { self.me.raw.borrow().fwd().len() }  
    fn keys_len(&self) -> usize { self.me.raw.borrow().fwd().keys_len() }

    fn iter(&self) -> Self::Iter {
        FwdIterator::<'a, A, B> {
            iter: ToSetKeyValueIterator::new(self.me.raw.create_interior_btreeset_range())
        }
    }
    fn keys(&self) -> Self::Keys {
        FwdKeysIterator::<'a, A, B> { 
            iter: ToSetKeysIterator::new(self.me.raw.create_interior_btreeset_range())
        }
    }
    fn sets(&self) -> Self::Sets { 
        let me = self.me;
        self.keys().map(move |k| (k, me.fwd().get(k))) 
    }
    fn values(&self) -> Self::Values { self.iter().map(|(_, v)| v) }

    fn insert(&self, a: A, b: B) -> Option<B> { self.me.raw.borrow_mut().mut_fwd().insert(a, b) }
    fn expunge(&self, a: A) -> Self::Expunge { self.me.raw.borrow_mut().mut_fwd().expunge(a) }
}

// == Forward (sets) ==
impl <'a, A: IdLike, B: IdLike> SharedSet<'a, B> for FwdSet<'a, A, B> {
    type Iter = impl 'a+DoubleEndedIterator<Item=B>;

    fn contains(&self, b: B) -> bool { self.fetch().contains(b) }
    fn len(&self) -> usize { self.fetch().len() }

    fn iter(&self) -> Self::Iter {
        FwdSetIterator {
            iter: VSetIterator::new(
                self.parent.raw.create_interior_vset(),
                self.parent.raw.create_interior_btreeset_range(),
                self.key,
            )
        }
    }

    fn insert(&self, b: B) -> Option<B>  { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).insert(b) }
    fn remove(&self, b: B) -> Option<B> { self.parent.raw.borrow_mut().mut_fwd().get_mut(self.key).remove(b) }
}

// == iterators ==
struct FwdIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeyValueIterator<'a, RawSetToSet<A, B>, A, B>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdIterator<'a, A, B> {
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        self.iter.next(|p| &p.fwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd)
    }
}

struct FwdKeysIterator<'a, A: IdLike, B: IdLike> {
    iter: ToSetKeysIterator<'a, RawSetToSet<A, B>, A>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdKeysIterator<'a, A, B> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        self.iter.next(|p| &p.fwd)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdKeysIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p| &p.fwd)
    }
}

struct FwdSetIterator<'a, A: IdLike, B: IdLike> {
    iter: VSetIterator<'a, RawSetToSet<A, B>, A, B>,
}

impl<'a, A: IdLike, B: IdLike> Iterator for FwdSetIterator<'a, A, B> {
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next(|p, k| p.fwd().get_short(k).0)
    }
}

impl <'a, A: IdLike, B: IdLike> DoubleEndedIterator for FwdSetIterator<'a, A, B> {
    fn next_back(&mut self) -> Option<Self::Item> { 
        self.iter.next_back(|p, k| p.fwd().get_short(k).0)
    }
}