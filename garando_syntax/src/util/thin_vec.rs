use serde::{Deserialize, Serialize};

/// A vector type optimized for cases where this size is usually 0 (c.f. `SmallVector`).
/// The `Option<Box<..>>` wrapping allows us to represent a zero sized vector with `None`,
/// which uses only a single (null) pointer.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Debug)]
pub struct ThinVec<T>(Option<Box<Vec<T>>>);

impl<T> ThinVec<T> {
    pub fn new() -> Self {
        ThinVec(None)
    }
}

impl<T> From<Vec<T>> for ThinVec<T> {
    fn from(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            ThinVec(None)
        } else {
            ThinVec(Some(Box::new(vec)))
        }
    }
}

impl<T> Into<Vec<T>> for ThinVec<T> {
    fn into(self) -> Vec<T> {
        match self {
            ThinVec(None) => Vec::new(),
            ThinVec(Some(vec)) => *vec,
        }
    }
}

impl<T> ::std::ops::Deref for ThinVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        match *self {
            ThinVec(None) => &[],
            ThinVec(Some(ref vec)) => vec,
        }
    }
}

impl<T> Extend<T> for ThinVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        match *self {
            ThinVec(Some(ref mut vec)) => vec.extend(iter),
            ThinVec(None) => *self = iter.into_iter().collect::<Vec<_>>().into(),
        }
    }
}
