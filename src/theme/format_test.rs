#[cfg(test)]
mod parser_test {
    use colored::Colorize;

    use crate::theme::format::*;
    use crate::theme::parser::*;

    #[test]
    fn construct_colored_test() {
        let span = Span::new("&[#FF00FF]`hello world`&[#0000FF]{{my_var}}");
        let r = parse_theme(span).unwrap();
        let mut ctx = Context::new();
        ctx.set_var("my_var", " from vsh");

        let hello_world = String::from("hello world");
        let vsh = String::from(" from vsh");

        assert_eq!(
            construct_colored(r.1, ctx),
            format!(
                "{}{}",
                hello_world.truecolor(255, 0, 255),
                vsh.truecolor(0, 0, 255)
            )
        );
    }
}
