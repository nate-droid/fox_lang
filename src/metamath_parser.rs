#[cfg(test)]
mod tests {
    use crate::cut::{reduce, Axiom};
    use crate::metamath_lexer::MetaMathLexer;
    use crate::parser::{Node, Parser};
    use super::*;

    #[test]
    fn test_node_print() {
        let input = "(𝜑 → (𝜓 → 𝜑))";
        let mut parser = Parser::new_mm(input.to_string());
        let node = parser.parse().unwrap();
        assert_eq!(node.to_string(), input);
    }
    
    #[test]
    fn test_ax_1() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";

        let mut parser = Parser::new_mm(input.to_string());

        let node = parser.parse().unwrap();
        
        let result = reduce(node);

        let (left, right) = result.unwrap();
        if let Node::Identifier { value } = left {
            assert_eq!(value, "𝜑");
        } else {
            panic!("Expected Node::Identifier");
        }

        if let Node::BinaryExpression { left, operator: _, right } = right {
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
        axiom.solve().expect("TODO: panic message");
        println!("{:?}", axiom.steps);

    }

    #[test]
    fn test_recursive_step() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";

        let parser = Parser::new_mm(input.to_string());
        
        let mut axiom = Axiom::new("ax-1".to_string(), vec![], input.to_string(), parser);
        axiom.solve().expect("TODO: panic message");
        println!("{:?}", axiom.steps);
    }
    
    #[test]
    fn test_ax_2() {
        let input = "⊢ ((𝜑 → (𝜓 → 𝜒)) → ((𝜑 → 𝜓) → (𝜑 → 𝜒)))";
        
        let parser = Parser::new_mm(input.to_string());
        
        let mut axiom = Axiom::new("ax-2".to_string(), vec![], input.to_string(), parser);
        axiom.solve().expect("TODO: panic message");
        axiom.print_steps();
    }
    
    #[test] 
    fn test_ax_3() {
        let input = "⊢ ((¬ 𝜑 → ¬ 𝜓) → (𝜓 → 𝜑))";
        // let input = "⊢ (𝜑 → ¬ 𝜓)";
        // let input = "⊢ (¬ 𝜑 → ¬ 𝜓)";
        // let input = "((¬ 𝜑) → (¬ 𝜓))";
        // let input = "⊢ (¬ 𝜓)";

        let parser = Parser::new_mm(input.to_string());
        
        let mut axiom = Axiom::new("ax-3".to_string(), vec![], input.to_string(), parser);
        // assert!(axiom.solve().is_ok(), "Axiom solve resulted in an error");
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }
}