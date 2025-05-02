mod xeger;

use regex_syntax::{hir::Hir, parse};

fn main() {

}


#[cfg(test)]
mod tests {
    use regex_syntax::{ast, hir::Hir};

    #[test]
    fn it_works() {
        let mut parser = ast::parse::Parser::new();

        let ast = parser.parse("c|([a-zA-Z0-9]{4,5})*").expect("Error in parsing a|b");

        println!("{:#?}", ast);

        // assert_eq!(hir, Hir::alternation(vec![
        //     Hir::literal("a".as_bytes()),
        //     Hir::literal("b".as_bytes()),
        // ]));
    }
}

