# FoxLang and Theorem Prover

# What is FoxLang?

At it's core, FoxLang is an attempt at both creating a programming language and a theorem prover all at once.

Instead of "just" creating a theorem prover, I wanted to write a language that would serve as the core. In addition to
enjoying language design, I wanted to write my own "math" language, since I found it a hassle to always type the symbols ∀ and ∃.

I am currently in the process of writing a parser and lexer that will allow fox_lang to work with existing theorem provers
like MetaMath and Lean. Initial progress has been made with MetaMath, and several axioms can already be parsed and reduced.

This is a work and progress, and I imagine the structure will change quite significantly over time.

# What can Fox do?

At the moment, Fox is able to parse the first batch of axioms from MetaMath. Most notably are the axiom of infinity and the
axiom of choice. For the "main" parts of the language, Fox is able to set variables, print variables, perform simple arithmetic,
and I've just recently added extremely simple support for conditionals.

# Design Choices

The first iterations of Fox are using an interpreter. This will allow me to prototype the syntax as I evolve the use cases.
My ultimate goal is to write a bytecode VM that handles the runtime once the syntax settles down a bit.

# Why the Name?

I was looking for a short and succinct name; so naturally, I asked my 4-year-old what it should be, and here we are.

# Examples

I am aiming to add a few tests for every new features and axiom that I work on. Some more "specific" examples can be found
in `src/metamath_parser.rs`. There, I am aiming to start adding more and more of the axioms that they use to build up
their system. When I get a bit more time, and the structure stabilizes a bit more, I will add more focused examples in 
an examples folder. To run all tests at once, you can run `cargo test`. I have been aiming to keep main in a "working" state
as much as possible, meaning that running all tests should always pass.

Initial drafts of the language can be found in 'lang_scratch/'. As implied in the naming, these are only rough initial drafts
and are subject to change.

## Theorem Prover Examples

You can, for example, prove theorems by using the MetaMath format. Here is an example of a proof of the Axiom of Choice:

```rust
#[test]
    fn ax_ac() {
        let input = "⊢ ∃𝑦∀𝑧∀𝑤((𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥) → ∃𝑣∀𝑢(∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))";

        let mut axiom = Axiom::new("ax-ac".to_string(), input.to_string());
        axiom.solve().unwrap_or_else(|e| panic!("Axiom solve resulted in an error: {:?}", e));
        axiom.print_steps();
    }
```

Which will return the following steps:

```
initial assertion: ⊢ ∃𝑦∀𝑧∀𝑤((𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥) → ∃𝑣∀𝑢(∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))
Step { index: 0, expression: "∃𝑦∀𝑧∀𝑤(((𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥) → ∃𝑣∀𝑢((∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))))" }
Step { index: 1, expression: "𝑦" }
Step { index: 2, expression: "∀𝑧∀𝑤(((𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥) → ∃𝑣∀𝑢((∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))))" }
Step { index: 3, expression: "𝑧" }
Step { index: 4, expression: "∀𝑤(((𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥) → ∃𝑣∀𝑢((∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))))" }
Step { index: 5, expression: "𝑤" }
Step { index: 6, expression: "((𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥) → ∃𝑣∀𝑢((∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣)))" }
Step { index: 7, expression: "(𝑧 ∈ 𝑤 ∧ 𝑤 ∈ 𝑥)" }
Step { index: 8, expression: "∃𝑣∀𝑢((∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))" }
Step { index: 9, expression: "𝑧 ∈ 𝑤" }
Step { index: 10, expression: "𝑤 ∈ 𝑥" }
Step { index: 11, expression: "𝑣" }
Step { index: 12, expression: "∀𝑢((∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣))" }
Step { index: 13, expression: "𝑥" }
Step { index: 14, expression: "𝑢" }
Step { index: 15, expression: "(∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)) ↔ 𝑢 = 𝑣)" }
Step { index: 16, expression: "∃𝑡((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦))" }
Step { index: 17, expression: "𝑢 = 𝑣" }
Step { index: 18, expression: "𝑡" }
Step { index: 19, expression: "((𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡) ∧ (𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦))" }
Step { index: 20, expression: "(𝑢 ∈ 𝑤 ∧ 𝑤 ∈ 𝑡)" }
Step { index: 21, expression: "(𝑢 ∈ 𝑡 ∧ 𝑡 ∈ 𝑦)" }
Step { index: 22, expression: "𝑢 ∈ 𝑤" }
Step { index: 23, expression: "𝑤 ∈ 𝑡" }
Step { index: 24, expression: "𝑢 ∈ 𝑡" }
Step { index: 25, expression: "𝑡 ∈ 𝑦" }
```


# Coming Up

The Theorem Prover can now reduce the Axiom of Choice and I am now currently beginning to take a look at support Classes
and Category Theory. It might not make it into the code for some time, but am also exploring various branches of Type Theory (Homotopic Type Theory, etc).

Since previous progress has been heavily focused on the MetaMath portion of the language, I wanted to spend a bit of time 
with the language as a whole. I was reminded of the challenges posed in the "Euler Project" and decided that trying to solve
some of the initial problems in `Fox` would be a good way to figure out what the missing elements of the language are.

Note! I haven't had a lot of time, so I have decided to continue by stapling things together (20 minutes here, 15 minutes there), and letting the structure 
evolve as the project grows, apologize for the mess!
