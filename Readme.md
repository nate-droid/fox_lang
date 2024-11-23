# FoxLang and Theorem Prover

# Why the Name?

I was looking for a short and sweet name, and asked my 4-year-old what it should be, and here we are.

# What is FoxLang?

At it's core, FoxLang is an attempt at both creating a programming language and a theorem prover all at once.

Instead of "just" creating a theorem prover, I wanted to write a language that would serve as the core. In addition to 
enjoying language design, I wanted to write my own "math" language, since I found it a hassle to always type the symbols ∀ and ∃. 

I am currently in the process of writing a parser and lexer that will allow fox_lang to work with existing theorem provers
like MetaMath and Lean. Initial progress has been made with MetaMath, and several axioms can already be parsed and reduced.

This is a work and progress, and I imagine the structure will change quite significantly over time.

# Examples

I am aiming to add a few tests for every new features and axiom that I work on. Some more "specific" examples can be found
in 'src/metamath_parser.rs'. There, I am aiming to start adding more and more of the axioms that they use to build up
their system. When I get a bit more time, and the structure stabilizes a bit more, I will add more focused examples in 
an examples folder.

# Coming Up

After stabilizing the overall structure, I aim to get a number of the "classic" axioms from MetaMath working (ie Modus Ponens).
This is a bit of a long stretch, but afterwards, I am curious to dive into some more "full-featured" systems like Lean and Coq.
