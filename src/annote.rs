use crate::{syntax::Syntax, Abstr, Apply, Ident, Term};
use typed_index_collections::TiVec;

pub struct Typed<T, X> {
    pub data: X,
    pub mono: T,
}

impl<T: Default, X> From<X> for Typed<T, X> {
    fn from(value: X) -> Self {
        Self {
            data: value,
            mono: T::default(),
        }
    }
}

impl<T, X> Typed<T, X> {
    fn fmap<U>(self, func: &mut impl FnMut(T) -> U) -> Typed<U, X> {
        Typed {
            data: self.data,
            mono: func(self.mono),
        }
    }
}

pub struct Annote<T> {
    pub idents: TiVec<Ident, Typed<T, Abstr>>,
    pub abstrs: TiVec<Abstr, Typed<T, Term>>,
    pub applys: TiVec<Apply, Typed<T, [Term; 2]>>,
    pub object: Term,
}

impl<T: Default> From<Syntax> for Annote<T> {
    fn from(value: Syntax) -> Self {
        Self {
            idents: value.idents.into_iter().map(Typed::from).collect(),
            abstrs: value.abstrs.into_iter().map(Typed::from).collect(),
            applys: value.applys.into_iter().map(Typed::from).collect(),
            object: value.object,
        }
    }
}

impl<T> Annote<T> {
    pub fn fmap<U>(self, func: &mut impl FnMut(T) -> U) -> Annote<U> {
        Annote {
            idents: self.idents.into_iter().map(|n| n.fmap(func)).collect(),
            abstrs: self.abstrs.into_iter().map(|n| n.fmap(func)).collect(),
            applys: self.applys.into_iter().map(|n| n.fmap(func)).collect(),
            object: self.object,
        }
    }
}
