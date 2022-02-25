#[cfg(test)]
mod parser_test {
    use crate::theme::parser::*;

    #[test]
    fn parse_color_test() {
        let span = Span::new("&[#FF00FF]");
        let r = parse_color(span).unwrap();
        assert_eq!(
            r.1.clone().color().unwrap().color,
            Color {
                red: 255,
                green: 0,
                blue: 255
            }
        );
        assert_eq!(r.1.color().unwrap().end_pos.location_offset(), 10);
    }

    #[test]
    fn parse_var_test() {
        let span = Span::new("{{my_var}}");
        let r = parse_var(span).unwrap();
        assert_eq!(r.1.clone().var().unwrap().var_name, "my_var");
        assert_eq!(r.1.var().unwrap().end_pos.location_offset(), 10);
    }

    #[test]
    fn parse_literal_test() {
        let span = Span::new("`hello world`");
        let r = parse_literal(span).unwrap();
        assert_eq!(r.1.clone().literal().unwrap().literal, "hello world");
        assert_eq!(r.1.literal().unwrap().end_pos.location_offset(), 13);
    }

    #[test]
    fn parse_frag_test() {
        let span = Span::new("{{my_var}}");
        let r = parse_frag(span).unwrap();
        assert_eq!(r.1.clone().var().unwrap().var_name, "my_var");
        assert_eq!(r.1.var().unwrap().end_pos.location_offset(), 10);
    }

    #[test]
    fn parse_theme_test() {
        let span = Span::new("&[#FF00FF]`hello world`{{my_var}}");
        let r = parse_theme(span).unwrap();
        assert_eq!(
            r.1[0].color().unwrap().color,
            Color {
                red: 255,
                green: 0,
                blue: 255
            }
        );
        assert_eq!(r.1[1].literal().unwrap().literal, "hello world");
        assert_eq!(r.1[2].var().unwrap().var_name, "my_var");
    }
}
