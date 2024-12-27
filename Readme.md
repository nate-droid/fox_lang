# FoxLang and Theorem Prover

# What is FoxLang?

At it's core, FoxLang is an attempt at both creating a programming language and a theorem prover all at once.

Instead of "just" creating a theorem prover, I wanted to write a language that would serve as the core. In addition to
enjoying language design, I wanted to write my own "math" language, since I found it a hassle to always type the symbols ∀ and ∃.

I am currently in the process of writing a parser and lexer that will allow fox_lang to work with existing theorem provers
like MetaMath and Lean. Initial progress has been made with MetaMath, and several axioms can already be parsed and reduced.

This is a work and progress, and I imagine the structure will change quite significantly over time.

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

# Coming Up

I've managed to complete the first 8 axioms from MetaMath, along with things like Modus Ponens and Generalization. 
For next steps, I am looking to create more of an actual language, as opposed to just accepting a proposition as a string. 
In order to keep logic split up I will most likely implement this as a separate parser.

Note! I haven't had a lot of time, so I have decided to continue by stapling things together (20 minutes here, 15 minutes there), and letting the structure 
evolve as the project grows, apologize for the mess!
