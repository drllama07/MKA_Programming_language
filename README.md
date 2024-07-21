<h1 align="center">
  <br>
  <a href="YOUR_REPO_LINK_HERE"><img src="https://github.com/drllama07/MKA_Programming_language/blob/master/MKA.png" alt="YOUR_APP_NAME" width="200"></a>
  <br>
  MKA Programming Language
  <br>
</h1>

<h4 align="center"></h4>
<p align="center">
  <a href="#what-is-mka-?">Why </a> •
  <a href="#installation">Installation</a> •
  <a href="#starter-guide">Starter Guide</a> •
  <a href="#error-guide">Error Guide</a> •
  <a href="#code-guide">Code Guide</a> •
  <a href="#credits">More</a> •
  <a href="#license">License</a>
</p>

# What is MKA ? 
MKA is a super simple programming language designed to be a companion for mathematical tasks. 

- Designed to be simple and easy to learn
- Flexible for your specialized needs
- Variety of fields -> Calculus , Machine Learning , Physics and more.
- Prioritized for students and learners

## Who Is This Intended For
> Do you want to start learning about Machine Learning or Calculus in a familiar environment without worrying about programming? 

> If you need a flexible calculator for your tasks

> Or anything that can benefit from MKA.

> ### Personal Note:
> This application is primarily designed for educational purposes rather than industrial use due to its limitations.
> 
> *Tutorials and additional libraries for the mentioned fields will be provided in the future. Stay tuned for updates*

# Installation

1. Download the repository.

> **Note:** If you are not familiar with programming, you can download the pre-built executables from the repository.


2. Build the executable using [Rust's Cargo](https://www.rust-lang.org/tools/install).

```bash
cargo build
```
> For more info on how to build and use executables you can search online

3. ```bash
   cd target\debug
   your_executable_name.exe
   ```

# Starter Guide
## Menu
After running the executable you have to provide a argument like:
```bash
   your_executable_name.exe calc
   your_executable_name.exe menu
```
- calc: This argument will directly proceed to the running mode.
- menu: This argument will open the menu page.

### File name
- For the default file main.mka, you can simply press Enter without typing the file name.
- For other file names, such as calculus.mka, you need to type the file name explicitly (e.g., **calculus.mka**).

# MKA Basics and Error Guide:
To learn about MKA you can read the documentation [MKA-Documentation](documentation.md)

>Programming in MKA Basics
>Learn the fundamentals of programming in MKA.
>Common errors and how to resolve them
>A guide to common errors and their solutions.

# Future Upgrades and Projects

## Upgrade

- A new way to comment in .mka files
- To make debugging and outputting results easier, future upgrades will include enhanced printing and formatting capabilities

## Future Projects

- ***Calculus library*** -> I will review calculus questions and, while doing so, implement integration, differentiation, and series features in MKA.
- ***Machine learning library*** -> I will implement a simple XOR problem-solving neural network in MKA.
- ***Graphing feature and a text editor app for easier access*** -> 
- ***MKA Physical System Simulations*** -> A new and exciting project???

# *Code Guide*

## The Tokenizer/Lexer
**This includes these files:**
> *tokens.rs* and *scanner.rs*
- While working on these parts, I was heavily supported by the [repository](https://github.com/ClementTsang/rustcc/blob/master/src/parser/lexer.rs) by **ClementTsang**. Special thanks to this repository for its valuable contribution.
- I tried to keep my tokenizer as modular as possible by separating tokens to *tokens.rs* and lexing the chars to *scanner.rs*
- So if you need to print the tokens for debugging reasons there is a function commented in *main.rs* file
- Remove the /* */.
```rust
for tok in tokens.iter() {
        println!("Token -> {} Value -> {}", tok.kind, tok.value);
    }
```
## The Parser and AST
**This includes these files:**
> *parser.rs*
### Learning resources
Unlike the other sections learning AST and parsing was a bit more challenging 
> Here are the things that taught me the basics:
> - [Compilers by Stanford](https://web.stanford.edu/class/cs143/)
> - [Crafting interpreters](https://craftinginterpreters.com/the-lox-language.html)
> - [A tutorial by Codescope](https://www.youtube.com/watch?v=6D-ieqQignQ&list=PLj_VrUwyDuXRizi2nJjP8uP9Tmh91OLwA&index=3)

### My Context-Free Grammar
> ```rust
> expression: assigment
> assigment: (Variable "=")? expression | (Id "(" Id ")" "=")? expression | term | for_loop
> term: factor (("+" | "-") factor)*
> factor: unary (("*" | "/" ) unary)* | unary "^" unary
> unary: - primary | primary
> primary: variable | number | vector | fncall | "(" expression ")" | for_loop
> arguments: expr ("," expr)*
> fncall: variable* "(" expression ")"
> for_loop: "for" Variable "in" variable | number "to" variable | number     "{" expression "}"
> vector: "<" variable "," ... ">"
> ```

### Implementation
- Initially, I followed the tutorial by Codescope to guide the development of the tokenizer. However, as the project progressed, I began using my own methods to achieve the desired functionality.
- This is a recursive descent parser (top-down) designed without left recursion.
- Due to my level of experience:
  
> **Note** During this process, some parts of the code may not be optimized for efficiency or may not fully adhere to Rust best practices. However, I have learned a lot and improved as a programmer.


## The Interpreter and the Runtime Environment
**This includes these files:**
> *interpreter.rs* and *environments.rs*
> This is the area where I received the least amount of external help. However, the Codescope tutorial was incredibly helpful, so special thanks to them.



### TREE-WALK Interpreter
> **Note:** I’m not certain if this is an exact Tree-walk interpreter as described in Crafting Interpreters, but I’ll name to it by that name.
>

### Environments for storing values, functions, and more
- Storing complex values like vectors, functions, and import names.
- After reviewing several repositories, I concluded that using hash maps was the simplest approach.
- This implementation may not be optimal, but the primary factor is my current level of knowledge.

# Credits
> - [lexer](https://github.com/ClementTsang/rustcc/blob/master/src/parser/lexer.rs)
> - [Compilers by Stanford](https://web.stanford.edu/class/cs143/)
> - [Crafting interpreters](https://craftinginterpreters.com/the-lox-language.html)
> - [A tutorial by Codescope](https://www.youtube.com/watch?v=6D-ieqQignQ&list=PLj_VrUwyDuXRizi2nJjP8uP9Tmh91OLwA&index=3)

# Personal Code Review
## Project Goal
Over the past year, I’ve been experimenting with Desmos and TI-84 calculators for my exams. As a competitive programmer, I wanted to create an alternative or a different yet equally useful mathematical tool. Additionally, my interest in machine learning has influenced this project. I've designed a tool that allows users to create math-based programs and facilitates their journey. By this, I mean that many people enjoy math but may not programming. I want to ensure that this tool does not limit them, enabling them to explore basic machine learning tasks and even design complex mathematical programs. This language is crafted for learners who seek a simple, accessible tool for their mathematical needs.

## Programming Review
- > **Got even better at Rust**
- > - Rc and Refcells for borrowing and memory
  > - hashmaps in rust
  > - matching a lot
  > - Recursion
  > - enums and structs

- > **Computer Science**
  > - Lexers 
  > - Parsing
  > - > AST
  > - > Recursive descend parsing
  > - Interpreters and environments



# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

