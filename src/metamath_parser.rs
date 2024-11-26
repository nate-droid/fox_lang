#[cfg(test)]
mod tests {
    use crate::cut::{reduce, Axiom};
    use crate::metamath_lexer::MetaMathLexer;
    use crate::parser::{Node, Parser};
    use super::*;

    #[test]
    fn test_ax_1() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";

        let mut parser = Parser::new_mm(input.to_string());

        let node = parser.parse();

        let result = reduce(node.unwrap());

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

        let parser = Parser::new_mm(input.to_string());

        let mut axiom = Axiom::new("ax-1".to_string(), vec![], input.to_string(), parser);
        axiom.solve();
        println!("{:?}", axiom.steps);

    }

    #[test]
    fn test_recursive_step() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";

        let parser = Parser::new_mm(input.to_string());
        
        let mut axiom = Axiom::new("ax-1".to_string(), vec![], input.to_string(), parser);
        axiom.solve();
        println!("{:?}", axiom.steps);
    }
    
    #[test]
    fn test_ax_2() {
        let input = "⊢ ((𝜑 → (𝜓 → 𝜒)) → ((𝜑 → 𝜓) → (𝜑 → 𝜒)))";
        
        let parser = Parser::new_mm(input.to_string());
        
        let mut axiom = Axiom::new("ax-2".to_string(), vec![], input.to_string(), parser);
        axiom.solve();
        axiom.print_steps();
    }
    
    #[test]
    fn test_ax_3() {
        let input = "⊢ ((¬ 𝜑 → ¬ 𝜓) → (𝜓 → 𝜑))";
        // let input = "⊢ (¬ 𝜑)";

        let parser = Parser::new_mm(input.to_string());
        
        let mut axiom = Axiom::new("ax-3".to_string(), vec![], input.to_string(), parser);
        axiom.solve().expect("TODO: panic message");
        axiom.print_steps();
    }
}