use crate::{parser::Parsed, Abstr, Apply, Ident, Term};
use std::collections::HashMap;
use typed_index_collections::TiVec;

enum Instruction {
    Access(Term),
    Rollback(Abstr, Option<Abstr>),
}

pub struct Syntax {
    pub idents: TiVec<Ident, Abstr>,
    pub abstrs: TiVec<Abstr, Term>,
    pub applys: TiVec<Apply, [Term; 2]>,
    pub object: Term,
}

impl TryFrom<Parsed> for Syntax {
    type Error = Vec<(Ident, String)>;

    fn try_from(value: Parsed) -> Result<Self, Self::Error> {
        let mut scope = HashMap::new();
        let mut stack = Vec::from([Instruction::Access(value.object)]);
        let mut refer = TiVec::from_iter(value.idents.iter().map(|_| None));

        while let Some(instruction) = stack.pop() {
            match instruction {
                Instruction::Access(Term::Ident(idx)) => {
                    refer[idx] = scope.get(&value.idents[idx]).copied();
                }
                Instruction::Access(Term::Abstr(idx)) => {
                    let (arw, rec) = value.abstrs[idx].clone();
                    stack.extend([
                        Instruction::Rollback(idx, scope.insert(arw, idx)),
                        Instruction::Access(rec),
                    ]);
                }
                Instruction::Access(Term::Apply(idx)) => {
                    let [lhs, rhs] = value.applys[idx];
                    stack.extend([Instruction::Access(lhs), Instruction::Access(rhs)]);
                }
                Instruction::Rollback(idx, prev) => {
                    let (name, _) = &value.abstrs[idx];
                    if let Some(prev) = prev {
                        *scope.get_mut(name).unwrap() = prev;
                    } else {
                        scope.remove(name);
                    }
                }
            }
        }

        let mut idents = TiVec::new();
        let mut errors = Vec::new();

        for (ident, abstr) in refer.into_iter_enumerated() {
            if let Some(abstr) = abstr {
                idents.push(abstr);
            } else {
                errors.push((ident, value.idents[ident].clone()));
            }
        }

        if errors.is_empty() {
            Ok(Self {
                idents,
                abstrs: value.abstrs.into_iter().map(|(_, rec)| rec).collect(),
                applys: value.applys,
                object: value.object,
            })
        } else {
            Err(errors)
        }
    }
}
