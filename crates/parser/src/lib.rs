use chumsky::{prelude::*, text::whitespace};

#[derive(Debug)]
pub enum Tokens {
    Str(String),
    Int(String),
    Var(String, Box<Tokens>),
    Command(String),
    Fn {
        name: String,
        args: Vec<String>,
        body: Vec<Tokens>,
        then: Box<Option<Tokens>>
    }
}

pub fn parser() -> impl Parser<char, Tokens, Error = Simple<char>> {
    let ident = text::ident().padded();

    let cmd = text::ident()
        .repeated()
        .at_least(1)
        .map(|idents| idents.join(" "))
        .map(Tokens::Command);

    let expr = recursive(|_| {
        let int = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(Tokens::Int);

        let str = just('"')
            .ignore_then(filter(|c| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(Tokens::Str);
        
        let inline = just('$')
            .ignore_then(
                text::ident()
                    .padded()
                    .repeated()
                    .delimited_by(just('('), just(')'))
                    .map(|idents| idents.join(" "))
            ).map(Tokens::Command);

        int
            .or(inline)
            .or(str)
            .or(cmd)
    }).padded();

    let decl = recursive(|decl| {

        let assign = text::keyword("let")
            .ignore_then(ident)
            .then_ignore(just('=')).padded()
            .then(expr.clone().padded())
            .map(|(name, expr)| Tokens::Var(name, Box::new(expr)));


        let function = text::keyword("fn")
            .ignore_then(ident)
            .then(
                ident
                    .or_not()
                    .separated_by(just(','))
                    .delimited_by(just('('), just(')'))
            ).padded()
            .then(
                expr.clone()
                    .delimited_by(just('{'), just('}'))
                    .repeated()
                    .or_not()
            ).padded()
            .then(decl.or_not())
            .map(|(((name, args), body), then)| {
                Tokens::Fn {
                    name,
                    args: args.into_iter().flatten().collect(),
                    body: body.into_iter().flatten().collect(),
                    then: Box::new(then),
                }
            });
        function.or(assign)
    });

    decl.or(expr).then_ignore(end())
}

pub fn parse_from_string(input: String) -> Result<Tokens, Vec<Simple<char>>> {
    parser().parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_1() {
        unimplemented!()
    }
}
