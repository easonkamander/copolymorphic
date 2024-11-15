use crate::{annote::Annote, Term};
use derive_more::{From, Into};
use typed_index_collections::TiVec;

#[derive(From, Into, Clone, Copy)]
pub struct MonoIdx(usize);

#[derive(From, Into, Clone, Copy)]
pub struct PolyIdx(usize);

pub enum Mono {
    Free(),
    Func(PolyIdx, MonoIdx),
}

#[derive(Default)]
pub struct Poly {
    pub factors: Vec<MonoIdx>,
    pub witness: Option<Term>,
}

pub struct Forest {
    pub syntax: Annote<MonoIdx>,
    pub monos: TiVec<MonoIdx, Mono>,
    pub polys: TiVec<PolyIdx, Poly>,
}

impl Forest {
    pub fn check(&mut self, term: Term, mono: MonoIdx) {
        match term {
            Term::Ident(idx) => {
                self.syntax.idents[idx].mono = mono;
                let abs = self.syntax.idents[idx].data;
                let fun = self.syntax.abstrs[abs].mono;
                let Mono::Func(dom, _) = self.monos[fun] else {
                    unreachable!()
                };
                self.polys[dom].factors.push(mono);
                if let Some(witness) = self.polys[dom].witness {
                    self.check(witness, mono);
                }
            }
            Term::Abstr(idx) => {
                self.syntax.abstrs[idx].mono = mono;
                let rec = self.syntax.abstrs[idx].data;
                let cod = match self.monos[mono] {
                    Mono::Func(_, cod) => cod,
                    Mono::Free() => {
                        let dom = self.polys.push_and_get_key(Poly::default());
                        let cod = self.monos.push_and_get_key(Mono::Free());
                        self.monos[mono] = Mono::Func(dom, cod);
                        cod
                    }
                };
                self.check(rec, cod);
            }
            Term::Apply(idx) => {
                self.syntax.applys[idx].mono = mono;
                let [fun, arg] = self.syntax.applys[idx].data;
                let dom = self.polys.push_and_get_key(Poly {
                    factors: Vec::new(),
                    witness: Some(arg),
                });
                let arw = self.monos.push_and_get_key(Mono::Func(dom, mono));
                self.check(fun, arw);
            }
        };
    }
}

impl From<Annote<()>> for Forest {
    fn from(value: Annote<()>) -> Self {
        let mut forest = Self {
            syntax: Annote::from(value).fmap(&mut |()| MonoIdx::from(usize::MAX)),
            monos: TiVec::new(),
            polys: TiVec::new(),
        };

        let object = forest.syntax.object;
        let freest = forest.monos.push_and_get_key(Mono::Free());
        forest.check(object, freest);

        forest
    }
}
