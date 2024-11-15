use chumsky::prelude::*;
use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(From, Into, Clone, Copy)]
pub struct Ident(usize);

#[derive(From, Into, Clone, Copy)]
pub struct Abstr(usize);

#[derive(From, Into, Clone, Copy)]
pub struct Apply(usize);

#[derive(Clone, Copy)]
pub enum Term {
    Ident(Ident),
    Abstr(Abstr),
    Apply(Apply),
}

pub struct Parsed {
    pub idents: TiVec<Ident, String>,
    pub abstrs: TiVec<Abstr, (String, Term)>,
    pub applys: TiVec<Apply, [Term; 2]>,
    pub object: Term,
}

impl TryFrom<String> for Parsed {
    type Error = Vec<Simple<char>>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use core::cell::RefCell;

        let idents = RefCell::new(TiVec::new());
        let abstrs = RefCell::new(TiVec::new());
        let applys = RefCell::new(TiVec::new());

        let total = recursive(|recur| {
            let arrow = text::ident().padded().then_ignore(just("=>"));

            let abstr = arrow.then(recur.clone()).map(|(arw, rec)| {
                let idx = abstrs.borrow_mut().push_and_get_key((arw, rec));
                Term::Abstr(idx)
            });

            let ident = text::ident().map(|raw| {
                let idx = idents.borrow_mut().push_and_get_key(raw);
                Term::Ident(idx)
            });

            let paren = recur.delimited_by(just("("), just(")")).or(ident).padded();

            let apply = paren.clone().then(paren.repeated()).foldl(|lhs, rhs| {
                let idx = applys.borrow_mut().push_and_get_key([lhs, rhs]);
                Term::Apply(idx)
            });

            abstr.or(apply)
        });

        let object = total.then_ignore(end()).parse(value)?;

        Ok(Self {
            idents: idents.into_inner(),
            abstrs: abstrs.into_inner(),
            applys: applys.into_inner(),
            object,
        })
    }
}
