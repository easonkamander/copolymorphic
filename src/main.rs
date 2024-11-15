mod annote;
mod forest;
mod parser;
mod syntax;
mod output;

pub use parser::{Abstr, Apply, Ident, Term};

fn main() -> Result<(), String> {
    let path = std::env::args().nth(1).ok_or("Please enter a file path")?;

    let text = std::fs::read_to_string(path).map_err(|err| {
        return format!("Unable to read file: {}", err.to_string());
    })?;

    let parsed = parser::Parsed::try_from(text).map_err(|errs| {
        return format!("Invalid program syntax: {}", errs[0]);
    })?;

    let syntax = syntax::Syntax::try_from(parsed).map_err(|errs| {
        return format!("Unbound variable detected: {}", errs[0].1);
    })?;

    let annote = annote::Annote::from(syntax);

    let forest = forest::Forest::from(annote);

    println!("{}", forest);

    Ok(())
}
