use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher, StableHasherResult};

#[derive(Clone)]
pub struct RcSlice<T> {
    data: Rc<Box<[T]>>,
    offset: u32,
    len: u32,
}

impl<T> RcSlice<T> {
    pub fn new(vec: Vec<T>) -> Self {
        RcSlice {
            offset: 0,
            len: vec.len() as u32,
            data: Rc::new(vec.into_boxed_slice()),
        }
    }
}

impl<T> Deref for RcSlice<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        &self.data[self.offset as usize..(self.offset + self.len) as usize]
    }
}

impl<T: fmt::Debug> fmt::Debug for RcSlice<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.deref(), f)
    }
}

impl<CTX, T> HashStable<CTX> for RcSlice<T>
where
    T: HashStable<CTX>,
{
    fn hash_stable<W: StableHasherResult>(&self, hcx: &mut CTX, hasher: &mut StableHasher<W>) {
        (**self).hash_stable(hcx, hasher);
    }
}
