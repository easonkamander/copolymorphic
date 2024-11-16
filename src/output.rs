use crate::{
    forest::{Forest, Mono, MonoIdx, Poly},
    Term,
};

impl std::fmt::Display for MonoIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LENGTH: usize = 26;
        const LETTER: [char; LENGTH] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        if let Some(mut num) = usize::from(*self).checked_add(1) {
            while num > 0 {
                write!(f, "{}", LETTER[(num - 1) % LENGTH])?;
                num /= LENGTH;
            }
            write!(f, "")
        } else {
            write!(f, "_")
        }
    }
}

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(first) = self.factors.first() {
            write!(f, "{}", first)?;
            for mono in &self.factors[1..] {
                write!(f, " & {}", mono)?;
            }
            write!(f, "")
        } else {
            write!(f, "[]")
        }
    }
}

impl std::fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let freest = match self.syntax.object {
            Term::Ident(idx) => self.syntax.idents[idx].mono,
            Term::Abstr(idx) => self.syntax.abstrs[idx].mono,
            Term::Apply(idx) => self.syntax.applys[idx].mono,
        };
        write!(f, "Forest ({}) {{", freest)?;
        for (mono, defn) in self.monos.iter_enumerated() {
            let expr = match *defn {
                Mono::Free() => format!("free()"),
                Mono::Func(dom, cod) => format!("{} -> {}", self.polys[dom], cod),
            };
            write!(f, "\n\t{} = {}", mono, expr)?;
        }
        write!(f, "\n}}")
    }
}
