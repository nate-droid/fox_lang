#[cfg(test)]
mod tests {
    use crate::cut::{reduce, Axiom};
    use crate::metamath_lexer::MetaMathLexer;
    use crate::parser::{Node, Parser};
    use super::*;

    #[test]
    fn test_ax_1() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";
        let lexer = MetaMathLexer::new(input.to_string());
        let mut parser = Parser::new_mm(input.to_string());

        let node = parser.parse();
        println!("{:?}", node);

        let result = reduce(node.unwrap());

        println!("{:?}", result);

        let (left, right) = result.unwrap();
        if let Node::Identifier { value } = left {
            assert_eq!(value, "𝜑");
        } else {
            panic!("Expected Node::Identifier");
        }

        if let Node::BinaryExpression { left, operator, right } = right {
            if let Node::Identifier { value } = *left {
                assert_eq!(value, "𝜓");
            } else {
                panic!("Expected Node::Identifier");
            }

            if let Node::Identifier { value } = *right {
                assert_eq!(value, "𝜑");
            } else {
                panic!("Expected Node::Identifier");
            }
        } else {
            panic!("Expected Node::BinaryExpression");
        }
    }

    #[test]
    fn test_step_reduce() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";

        let mut parser = Parser::new_mm(input.to_string());

        let node = parser.parse().unwrap();
        println!("{:?}", node);

        let mut axiom = Axiom::new("ax-1".to_string(), vec![], node.to_string(), parser);
        axiom.solve();
        println!("{:?}", axiom.steps);

    }

    #[test]
    fn test_recursive_step() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";

        let mut parser = Parser::new_mm(input.to_string());

        let node = parser.parse().unwrap();
        let mut axiom = Axiom::new("ax-2".to_string(), vec![], node.to_string(), parser);
        axiom.solve();
        println!("{:?}", axiom.steps);
    }
}