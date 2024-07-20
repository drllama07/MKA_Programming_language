<h1 align="center">
  <br>
  <a href="YOUR_REPO_LINK_HERE"><img src="https://raw.githubusercontent.com/YOUR_GITHUB_USERNAME/YOUR_REPO_NAME/main/PATH_TO_YOUR_LOGO.png" alt="YOUR_APP_NAME" width="200"></a>
  <br>
  MKA Programming Language
  <br>
</h1>

<h4 align="center">A simple programming language for math-based applications.</h4>

<p align="left">
  • <a href="#introduction">Introduction</a> <br>
  • <a href="#basic-mathematical-operations">Basic Mathematical Operations</a> <br>
  • <a href="#variables-and-data-types">Variables and Data Types</a> <br>
  • <a href="#assignment-rules">Assignment Rules</a> <br>
  • <a href="#vectors">Vectors</a> <br>
  • <a href="#for-loops">For Loops</a> <br>
  • <a href="#builtin-functions">Builtin Functions</a> <br>
  • <a href="#functions">Functions</a> <br>
  • <a href="#memory-and-extra-notes">Memory and Extra Notes</a> <br>
  • <a href="#future-topics">Future Topics</a> <br>
  • <a href="#examples">Examples</a>
</p>

## Introduction
### Who This Documentation Is For:

This documentation is intended for anyone who has a basic understanding of mathematical concepts such as functions, logarithms, and exponents, as well as for those who are interested in learning these topics. Whether you are a student, educator, or enthusiast, this guide will help you leverage the MKA Programming Language to create and explore math-based applications and models.

> *SIDE NOTE:*
> You don’t necessarily need to use these concepts. I strongly believe that you will find things that you can leverage for your own needs or even learn new concepts. After all, we are here to learn, right?

### How This Documentation Will Go:
- **Syntax and Rules:** Detailed explanation of the syntax and rules for utilizing this feature.
- **Programming Examples:** Demonstrations of how this feature can be used in programming contexts.
- > *SIDE NOTE:*
  > This could be usefull for the ones who are familiar with coding.
- **Mathematical Functionality:** Examples showcasing how this feature operates within mathematical functions.
- - > *SIDE NOTE:*
  > This could be usefull for the ones who are familiar with mathematical concepts.
- **Error Guide:** A comprehensive guide to errors, which will be printed in the terminal, associated with this feature and their solutions.

## Basic Mathematical Operations

MKA supports fundamental mathematical operations, including addition, subtraction, multiplication, division, and exponentiation. These operations follow standard mathematical precedence rules:

### ADDITION " + ": 
*Example use:* 
```rust
1 + 2
1 + (2 + 3)
```
- **NOTE:** Just like in mathematics or other programming languages, you can use `()` to manipulate the precedence of operations. However, in MKA, parentheses do not affect the order of operations in the same way. The standard operator precedence rules apply, and using parentheses will not change the evaluation order of operations.
### SUBSTRACTION " - ": 
*Example use:* 
```rust
1 - 2
1 - (2 + 3) = -4
1 - 2 + 3 = 2
```
- **NOTE:** This time "()" changes the solution of this code so try to keep this in mind.

### MULTIPLICATION " * ": 
*Example use:* 
```rust
1 * 2
2 * (2 + 3) = 10
2 * 2 + 3 = 7
```
- **NOTE:** Operations such as `*` and `/` are prioritized according to standard mathematical rules. This means that multiplication and division are evaluated before addition and subtraction, just as in traditional mathematics.

### DIVISION " / ": 
*Example use:* 
```rust
1 / 2
10 / (2 + 3) = 2
2 / 2 + 3 = 4
```


### EXPONENTIATION " ^ ": 
*Example use:* 
```rust
1 ^ 2
2 ^ (2 * 3) = 32
2 ^ 2 * 3  = 12
(2 ^ 2) * 3  = 12
```
- **NOTE:** Exponentiation (`^`) has the same priority in the order of operations as multiplication (`*`), division (`/`). Therefore, be carefull with them

## Variables and Data Types

### Data Types: 
MKA uses f32 as its value storage type. This means that every thing will be used as a float.

You can use `1` or `1.0` while using numbers and thye will all be used as f32 without a problem.
```rust
1.3 + 2
2.34 + 33.0
```
*Precision Requirements*:
For most mathematical applications and typical use cases, the precision offered by f32 is sufficient. It provides up to 7 decimal digits of precision, which is adequate for many calculations and ensures a balance between performance and precision.

This should be enough for most of the simple classes on mathematics.

Overall, f32 was chosen for its balance of precision and efficiency, ensuring that MKA performs well while meeting the needs of most typical mathematical applications.

**SIDE NOTE:** If you need f64, feel free to modify the code.

*Negation:*

Negation is the process of changing the sign of a number. For example, applying negation to -1 results in 1. It is commonly used in mathematical expressions and programming to reverse the sign of a value.

```rust
-1 + 2
1 * (-1)
-(1 * 2) + 2
```

### Variables

In MKA, variables are fundamental for storing and manipulating data. They represent a named storage location in memory where values can be assigned and retrieved.

**Declaring Variables**

Variables are declared by simply using the variable name followed by an assignment operator (=) and the value to be assigned.

```rust
foo = 1.22
a_b = 1 + 2 ^ 2
```
- **NOTE:**
- Use Only Lowercase Letters and Underscores: MKA does not support uppercase letters or numbers in variable names.
- `e` and `pi` will be assigned to thier mathematical values so do not try to declare or change them.

After declaring variables you can use them by their names
```rust
a = 12
b = 2
c = (a / b) * 2
```

CODE:

[parser.rs](src/parser.rs)

[environmnets.rs](src/environments.rs)



## Assignment Rules

### Compound Assignment Operators
If you have assigned a value and try to change it again like this: 
```rust
a = 0
a = a + 1
a + 1 = 2
```
**OR**
```rust
a = 0
a += 1
b = 1
b -= 1
a + b = 1
```
MKA supports compound assignment operators to simplify common operations involving variables. These operators combine a binary operation with an assignment, making code more concise and readable.

- **NOTE** You can only use `+=` and `-=` for now.
- They are particularly useful in loops and iterative processes.



## Vectors

If you are familiar with programming languages you know that they have various data types like lists, tuples, arrays, and vectors(mainly in Rust).

The term "vector" comes from Rust, the underlying language, and multidimensional mathematical operations(the cool stuff everyone loves).

These types allows developers to store multiple values under one variable name.

In MKA vectors are just like those data types.

- **VECTORS IN MKA**
1. You can only store f32 just like variables, but now you can only use variable names or numbers to assign values not operations.
```rust
a = 0
<0, a , 12>

< 1 + 1, 2> => Error
```
2. To assign vectors just like variables you can do this:
```rust
foo = <0 , 1 ,2> 
```
3. To call vector values you can use indexes
```rust
foo = <0 , 21 ,23>
foo<0> = 0
foo<1> = 21
foo<2> = 23
foo<3> => Error -> Out of index bounds !!!
```
*NOTE FOR NON-PROGRAMMERS:* Indexing is a fundamental concept in programming that allows you to access individual elements within a data structure, such as arrays or lists. In many programming languages, including Rust, lists are referred to as "vectors." A vector is a dynamic array that can grow or shrink in size. Indexing involves specifying a position within this vector to retrieve or modify the value stored at that position. Typically, indexing starts from 0, meaning the first element is accessed with an index of 0, the second with an index of 1, and so on. This allows for efficient data manipulation and retrieval within the vector.

> ***SUPER IMPORTANT***
> So as you can imagine you cannot access a value with an index that is float `vec<1.1>`. Therefore, MKA will see it as index 1 not 1.1.
> You will see this in for loops and other things that involves using only integers.



## For Loops
*How to use for loops in MKA.*

## Builtin Functions
*List and explanation of builtin functions in MKA.*

## Functions
*How to define and use functions in MKA.*

## Memory and Extra Notes
*Information about memory usage and additional notes.*

## Future Topics
*Topics that are planned for future versions of MKA.*

## Examples
*Examples demonstrating various features of MKA.*

### Example 1: Basic Calculation
**Explanation**: This example demonstrates a simple arithmetic operation in MKA.
```mka
// Example code
let result = 3 + 5;
print(result);

