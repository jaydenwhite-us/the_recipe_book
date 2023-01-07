# Numerical Recipes in Rust

> A translation of Numerical Recipes in C 2nd Edition into Rust

## How to Use

> This package is meant to be read and reimplemented by the reader.
> Use these code examples with the 2nd or 3rd edition of Numerical Recipes in C.

### Inspiration

The authors of Numerical Recipes in C intentionally avoid modules (citation needed ~page 7) because C does not fully support modularization, and simple data structures are easier to translate across language barriers. This repository translates the algorithms from C to Rust with modularization where it makes sense. It does not attempt to be, and does not claim to be, any better for computers than the original implementations. `It does attempt to be clearer for humans to interact with via`:

> 1. Named loops like \_traverse_matrix.
> 2. Named indexing variables e.g. (row, column).
> 3. Reducing nesting where possible.
> 4. Boolean flags provided to toggle some optimizations.

Tools 1,2 and 3. are trivial. They _should_ reduce the human working-memory necessary to understand how a program is traversing the allocated computer memory. The fourth tool requires explanation.

### Recognizing Optimization

An optimization can be additive or subtractive. It either adds code to reduce the number of resources consumed, or it removes code to the same effect.

Many programmers default to searching for subtractive optimizations. But by virtue of being in a heavily edited and widely published book, algorithms in Numerical Recipes in C default to using additive optimizations. These may be 2 or 3 innocuous seeming variable assignments in 50 lines of code, and are easy to miss. When possible, these instructions are surrounded with an if statement, so that a word search for the relevant boolean flag will quickly return the optimizing blocks of code.
