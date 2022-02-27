/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#[cfg(test)]
mod parser_test {
    use colored::Styles;

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
    fn parse_background_color_test() {
        let span = Span::new("*[#FF00FF]");
        let r = parse_background_color(span).unwrap();
        assert_eq!(
            r.1.clone().background_color().unwrap().background_color,
            Some(Color {
                red: 255,
                green: 0,
                blue: 255
            })
        );
        assert_eq!(
            r.1.background_color().unwrap().end_pos.location_offset(),
            10
        );
    }

    #[test]
    fn parse_no_bg_color_test() {
        let span = Span::new("*[]");
        let r = parse_no_bg_color(span).unwrap();
        assert_eq!(
            r.1.clone().background_color().unwrap().background_color,
            None
        );
        assert_eq!(
            r.1.background_color().unwrap().end_pos.location_offset(),
            10
        );
    }

    #[test]
    fn parse_style_test() {
        let span = Span::new("$[b]");
        let r = parse_style(span).unwrap();
        assert_eq!(r.1.clone().style().unwrap().style, Styles::Bold);
        assert_eq!(r.1.style().unwrap().end_pos.location_offset(), 4);
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
        let span = Span::new("`hello world `");
        let r = parse_literal(span).unwrap();
        assert_eq!(r.1.clone().literal().unwrap().literal, "hello world ");
        assert_eq!(r.1.literal().unwrap().end_pos.location_offset(), 14);
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
        let t = parse_theme(span).unwrap();
        let r = t.1.get_vec();
        assert_eq!(
            r[0].color().unwrap().color,
            Color {
                red: 255,
                green: 0,
                blue: 255
            }
        );
        assert_eq!(r[1].literal().unwrap().literal, "hello world");
        assert_eq!(r[2].var().unwrap().var_name, "my_var");
    }
}
