#[cfg(test)]
mod tests {
    use crate::cut::{reduce, Axiom};
    use crate::parser::{Node, Parser};

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

        let mut axiom = Axiom::new("ax-1".to_string(), input.to_string());
        axiom.solve().expect("TODO: panic message");
        println!("{:?}", axiom.steps);

    }

    #[test]
    fn test_recursive_step() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";
        
        let mut axiom = Axiom::new("ax-1".to_string(), input.to_string());
        axiom.solve().expect("TODO: panic message");
        println!("{:?}", axiom.steps);
    }
    
    #[test]
    fn test_ax_2() {
        let input = "⊢ ((𝜑 → (𝜓 → 𝜒)) → ((𝜑 → 𝜓) → (𝜑 → 𝜒)))";
        
        let mut axiom = Axiom::new("ax-2".to_string(), input.to_string());
        axiom.solve().expect("TODO: panic message");
        axiom.print_steps();
    }
    
    #[test] 
    fn test_ax_3() {
        let input = "⊢ ((¬ 𝜑 → ¬ 𝜓) → (𝜓 → 𝜑))";
        
        let mut axiom = Axiom::new("ax-3".to_string(), input.to_string());

        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();

        assert_eq!(axiom.steps.len(), 7);
    }

    #[test]
    fn test_ax_mp() {
        // test for modus ponens

        let input = "⊢ 𝜑 & ⊢ (𝜑 → 𝜓) ⇒ ⊢ 𝜓";

        let mut axiom = Axiom::new("ax-mp".to_string(), input.to_string());

        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }

    #[test]
    fn test_ax_gen() {
        let input = "∀𝑥𝜑";
        let mut axiom = Axiom::new("ax-gen".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }

    #[test]
    fn test_ax_4() {
        let input = "⊢ (∀𝑥(𝜑 → 𝜓) → (∀𝑥𝜑 → ∀𝑥𝜓))";
        let mut axiom = Axiom::new("ax-4".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }

    #[test]
    fn test_ax_5() {
        let input = "⊢ (𝜑 → ∀𝑥𝜑)";
        let mut axiom = Axiom::new("ax-5".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }

    #[test]
    fn test_ax_6() {
        let input = "⊢ ¬ ∀𝑥 ¬ 𝑥 = 𝑦";

        let mut axiom = Axiom::new("ax-6".to_string(), input.to_string());
        
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();

        assert_eq!(axiom.steps.len(), 6);
    }

    #[test]
    fn test_ax_7() {
        let input = "⊢ (𝑥 = 𝑦 → (𝑥 = 𝑧 → 𝑦 = 𝑧))";
        let mut axiom = Axiom::new("ax-7".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }

    #[test]
    fn test_ax_8() {
        let input = "⊢ (𝑥 = 𝑦 → (𝑥 ∈ 𝑧 → 𝑦 ∈ 𝑧))";
        let mut axiom = Axiom::new("ax-8".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
        assert_eq!(axiom.steps.len(), 8);
    }

    #[test]
    fn test_ax_9() {
        let input = "⊢ (𝑥 = 𝑦 → (𝑧 ∈ 𝑥 → 𝑧 ∈ 𝑦))";
        // let input = "⊢ (𝑥 = 𝑦 → 𝑧)";
        let mut axiom = Axiom::new("ax-9".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
        assert_eq!(axiom.steps.len(), 8);
    }

    #[test]
    fn test_ax_10() {
        let input = "⊢ (¬ ∀𝑥𝜑 → ∀𝑥 ¬ ∀𝑥𝜑)";
        let mut axiom = Axiom::new("ax-10".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();

        assert_eq!(axiom.steps.len(), 6);
    }

    #[test]
    fn test_ax_11() {
        let input = "⊢ (∀𝑥∀𝑦𝜑 → ∀𝑦∀𝑥𝜑)";
        // let input = "⊢ (∀𝑥∀𝑦𝜑)";
        let mut axiom = Axiom::new("ax-11".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
        assert_eq!(axiom.steps.len(), 8);
    }

    #[test]
    fn test_ax_12() {
        let input = "⊢ (𝑥 = 𝑦 → (∀𝑦𝜑 → ∀𝑥(𝑥 = 𝑦 → 𝜑)))";
        let mut axiom = Axiom::new("ax-12".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
        // assert_eq!(axiom.steps.len(), 8);
        // TODO: Validate Length
    }

    #[test]
    fn test_ax_13() {
        let input = "⊢ (¬ 𝑥 = 𝑦 → (𝑦 = 𝑧 → ∀𝑥 𝑦 = 𝑧))";
        let mut axiom = Axiom::new("ax-13".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
        // assert_eq!(axiom.steps.len(), 8);
        // TODO: Validate length
    }

    // The axioms in the next section are related to Zermelo-Fraenkel set theory (ZFC)
    #[test]
    fn test_ax_ext() {
        // Axiom of Extensionality
        let input = "⊢ (∀𝑧(𝑧 ∈ 𝑥 ↔ 𝑧 ∈ 𝑦) → 𝑥 = 𝑦)";
        // let input = "⊢ ¬ (((𝜑 ↔ 𝜓) → ¬ ((𝜑 → 𝜓) → ¬ (𝜓 → 𝜑))) → ¬ (¬ ((𝜑 → 𝜓) → ¬ (𝜓 → 𝜑)) → (𝜑 ↔ 𝜓)))";
        let mut axiom = Axiom::new("ax-ext".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();

        // TODO: Pickup. Need to investigate how biconditional operators are being parsed.
        // seems like another case of incorrect levels of precedence
        
        let axiom = "Detailed syntax breakdown of Axiom ax-ext
Step	Hyp	Ref	Expression
1	 	vz	. . . . 5 setvar 𝑧
2	 	vx	. . . . 5 setvar 𝑥
3	1, 2	wel 2105	. . . 4 wff 𝑧 ∈ 𝑥
4	 	vy	. . . . 5 setvar 𝑦
5	1, 4	wel 2105	. . . 4 wff 𝑧 ∈ 𝑦
6	3, 5	wb 205	. . 3 wff (𝑧 ∈ 𝑥 ↔ 𝑧 ∈ 𝑦)
7	6, 1	wal 1537	. 2 wff ∀𝑧(𝑧 ∈ 𝑥 ↔ 𝑧 ∈ 𝑦)
8	2, 4	weq 1964	. 2 wff 𝑥 = 𝑦
9	7, 8	wi 4	1 wff (∀𝑧(𝑧 ∈ 𝑥 ↔ 𝑧 ∈ 𝑦) → 𝑥 = 𝑦)
";
        for line in axiom.lines() {
            let columns: Vec<&str> = line.split_whitespace().collect();
            let step = columns[0];
            let hyp = columns[1];
            let r#ref = columns[2];
            let expression = columns[3..].join(" ");
            println!("Step: {}, Hyp: {}, Ref: {}, Expression: {}", step, hyp, r#ref, expression);   
        }
        
    }
}