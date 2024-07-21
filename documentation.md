<h1 align="center">
  <br>
  <a href="YOUR_REPO_LINK_HERE"><img src="https://github.com/drllama07/MKA_Programming_language/blob/master/MKA.png" alt="YOUR_APP_NAME" width="200"></a>
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
  • <a href="#built-in-functions">Builtin Functions</a> <br>
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

You can use `1` or `1.0` while using numbers and they will all be used as f32 without a problem.
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
- `e` and `pi` will be assigned to their mathematical values so do not try to declare or change them.

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
### Definition:

A for loop allows you to execute a block of code repeatedly for a specified number of iterations. It is commonly used to iterate over a range of values or elements in a collection.

1. Syntax:
```rust
for i in 0 to 10 {
    1 * i
}
```
2. After saying `for` you can declare a variable name which will be your loop variable, but choose a name so that it doesnt interrrupt with previous variables.
   - This variable will be only avaliable while for loop is active then it will be removed from memory.
   - You can only use it in the for loop.
3. After loop variable you have to say `in` which refers to the next things starting index.
   - Starting index should be a integer like vector indexes.
   - ( >= 0) and not float.
   - Loop variable will start from that value.
4. After `to` the ending index will be declared and it has the same rules as starting index.
   - The ending point is exclusive, meaning the loop will stop before reaching this value.

5. `{ code }` this part contains your loop body and this is the part that will run for n times.

***FOR LOOP RULES***
```rust
for i in 0 to 3 {
    1 * i
}
OUTPUT:
0
1
2
```
- The loop variable i takes on each value from the starting point (0) to the ending point (10), one at a time.
- The ending point is not included, so if you set it to 5, it will stop at 4.
- **NOTE:**
- Starting and ending indexes should be a variable or number not an operation:
```rust
for i in 0 to 1 + 1 {
    ERROR
}
```
### Mathematics related use cases
1. [**Sigma:**](https://en.wikipedia.org/wiki/Summation)
<h1>
&#931;<sub>i=1</sub><sup>5</sup> i
</h1>

***MKA version:***

```rust
 for i in 0 to 6 {
   sum += i
 }
```

*NOTE:* You have to add 1 to the ending index because it ends before reaching the end value.

### More examples:

```rust
 for i in 0 to 6 {
   vec<i>
   12 ^ i
 }
```

## Built-in Functions

In MKA, built-in functions provide essential mathematical operations and utilities that simplify calculations and enhance programming efficiency. These functions are pre-defined and ready to use, helping you avoid having to implement common tasks from scratch. Below is an overview of the built-in functions available in MKA.

1. > `print()` for printing out results
   > The print() function in MKA is used to output values to the console. It can handle multiple arguments, printing each argument on a new line. This function is versatile for displaying results, debugging, and general output.
   > ```rust
   > a = 0
   > vec = <0>
   > print(1, a, vec<0>, 1 + 1>)
   > OUTPUT:
   > -> 1
   > -> 0
   > -> 0
   > -> 2

2. > **Trigonometric Functions**
   > MKA includes several built-in trigonometric functions to facilitate computations involving angles.
   > They all take one argument -> `sin(a)`
   > ***SUPER IMPORTANT*** They all work in `radians` not degrees
   > List:
   > sin()
   > cos()
   > tan()
   > cot()
   > csc()
   > sec()
   > 
   > **Logarithmic Functions**
   > `ln(a)` has only one argument
   >
   > `log(a, base)` a is the input and the second argument will be the base of log.
   > 
   > **Factorial**
   > `factorial(a)` = a! which returns the factorial of a.
   >
3. >  **Vector Manipulation Functions**
   > MKA provides several built-in functions for managing and manipulating vectors, essential for handling sequential data. Here’s an overview of these vector functions:
   >
   > `push(vec_name, index, value)`
   >  Replaces the value at a specific index in the vector vec_name with a new value. This function allows you to update elements at a given position.
   > ```rust
   > vec = <1,2>
   > push(vec, 0, 2)
   > OUTPUT:
   > <2,2>
   > ```
   >
   > `pop(vec_name, index)`
   > Removes the value at that index from the vector vec_name and returns it. This function is useful for removing elements from a vector.
   > ```rust
   > vec = <1,2>
   > pop(vec, 0)
   > OUTPUT:
   > <2>
   > ```
   >
   > `snap(vec_name, value)`
   >  Adds a new value to the end of the vector vec_name. This function allows you to expand the vector by appending elements.
   > ```rust
   > vec = <1,2>
   > snap(vec, 3)
   > OUTPUT:
   > <1,2,3>
   > ```
   >
   > `len(vec_name)`
   > Returns the length of the vector vec_name, i.e., the number of elements it contains.
   > ```rust
   > vec = <1,2>
   > len(vec)
   > OUTPUT:
   > 2
   > ```
4. *Later updates will introduce new built-in functions or using libraries to expand the possibilities of MKA.



## Functions


### What is a Function? 

- Think of a function as a recipe. It takes certain ingredients (called parameters), performs some steps (instructions), and gives you a result (output).

### Functions: A Comparison for Mathematicians and Programmers 

- > [***In Mathematics:***](https://www.desmos.com/calculator?lang=en)
  > From [Britannica](https://www.britannica.com/science/function-mathematics):
  > Function, in mathematics, an expression, rule, or law that defines a relationship between one variable (the independent variable) and another variable (the dependent variable). Functions are ubiquitous in mathematics and are essential for formulating physical relationships in the sciences.

- > [***In Programming:***](https://www.geeksforgeeks.org/functions-programming/)
  > In programming, a function is a block of code designed to perform a specific task. It takes inputs (called parameters), executes a sequence of statements, and returns an output.

### Function Assignment In MKA

1. > ***SYNTAX:***
   > 
   > ```rust
   > fn(parameter1, parameter2) = parameter1 * parameter2
   >
   > OR
   > 
   > fn(parameter1, parameter2) = {
   > 
   >      ...CODE....
   > 
   >      print(parameter1 + parameter2)
   > }
   > ```
   >
   > ***Breakdown of the Syntax:***
   >
   > First, type the name of your function -> `bla_bla`
   > Second, open a `(` and start to type the names of your parameters and separate them using commas.
   > Close with a `)` and then `=`
   >
   > *Single-line functions:* write the body of your function -> ` a + b ^ 7`
   >
   > *Multi-line functions:* Open a `{`
   >
   > write the body of your code
   >
   > close with a `}`

2. > ***Function rules and memory***
   >
   > *Parameters*
   > - When you define a function, you specify parameters—these are like empty containers that will hold values when the function is used. For example, in the function fn(a, b) = { print(a + b) }, a and b are the parameters.
   > - When a Function Runs: Every time you call a function with specific values, those values are placed into the function's local memory. For instance, if you call fn(5, 3), the values 5 and 3 are temporarily stored in the local memory for a and b.
   > - After the function completes running, the program losses the access to those parameters and you cannot use them anymore.
   >
   > *Return values*
   > - The [interpreter](src/interpreter.rs) evaluates every single line of a function's body and the last one will be the return value.
   > - ```rust
   >   fn(a) = {
   >       1 + a
   >       2 + 2
   >       3 * 4
   >   }
   >   OUTPUT: 12
   >   ```
   ***SUPER IMPORTANT NOTE:***
   In MKA everything has a return value even `print()` has a return value. For more details you can take a look at the interpreter.rs and see what things are returned from what.
   
3. > ***Calling Functions***
   > - You can basicly call them with thier names and `()`
   > - ` fn(1) `
   > - Each function should be called with the exact number of parameters it expects; otherwise, it may result in errors.
   > - You can input variables, numbers, vector calls, and operations to functions as parameters.
   > - But you cannot input function calls as parameters.
   > - ```rust
   >   print( 1.0 , vec<0>, 1 + 1, a) => Ok
   >
   >   print( fn(1), a , 1) => Error -> Parsing error
   >   ```
- *NOTE:* In MKA functions are only allowed to return f32s.


## Memory and Extra Notes

1. > ***Multi-File Support***
   > - MKA allows you to run other .mka files using import method before your main file.
   > - ```rust
   >   @import blabla
   >   ```
   > - This runs that blabla.mka file and stores the variables, vectors, and functions in that file.
   > - By doing that you can access those things in your main file by just calling them
   > - *NOTE:* import statements should be at the beginning of your main file.

## Error Guide

### The Way MKA Programming Language Outputs Errors:

1. > ***PANIC Statements***
   > `thread 'main' panicked at ... src/code.rs:00:00` This is example of how these statements start and the file path is the place where it paniced.
   > This panic feature relies on Rust's panic macro that ceases the program if something unexpected happens like syntax or logic errors.
   
### Memory Related Errors:

- > The main file for memory -> [environments.rs](src/environments.rs)
- > An example: 
  > ```rust 
  > aa  = 0
  > print(a)
  > 
  > OUTPUT: 
  > 
  > thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/environments.rs:46:41
  > ```
  > This example depicts an error caused by using an undeclared variable, which returns None.
  > 

- ***Vector Errors:***
  > ```rust 
  > aa  = <0>
  > print(a<0>)
  > 
  > OUTPUT: 
  > 
  > thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/environments.rs:62:38
  > ```
  > This time the code is trying to call an undeclared vector. Therefore, Memory panics and returns None.
  > 
 
  > ```rust 
  > aa  = <>
  > print(aa<0>)
  > 
  > OUTPUT: 
  > 
  > thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', src/environments.rs:62:10
  > ```
  > This happens due to index being out of bounds.
  > This will happen to `push() , pop(), and snap()` functions too.

- ***Function Errors:*** 
  > *Note:*
  > Every function returns `1234` as their default value.

  > ```rust 
  > f(a) = a
  > n = ff(1)
  > print(n)
  > 
  > OUTPUT: 
  > 
  > Error -> Fncall no fn named ff is found
  > -> 1234 
  > ```
  > Undeclared function errors will produce a more precise error message and return a default value of 1234.

  > ```rust 
  > f(a, b) = a * b
  > n = f(1)
  > print(n)
  > 
  > OUTPUT: 
  > 
  > thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/environments.rs:118:61
  > ```
  > Missing parameter inputs will give an index error because parameters are stored in vectors.

### Syntax Related Errors:

- ***Matching Errors:***
 > MKA uses a function to match tokens throughout the parsing process. Thus, learning its error output will save us some time.

  > ```rust 
  > for i in 0 10 {  
  > }
  > 
  > OUTPUT: 
  > 
  > thread 'main' panicked at 'match_token error: found: Number(10.0) , expected: To ', src/parser.rs:99:17
  > ```
  > This error indicates that at the position where 10 is located, the program expects to, which is a syntax rule, as you may recall.
  >
  > These errors occur when a syntax rule is breached, causing the program to panic.


- ***Miscellaneous Errors:***
 > `unknown token or syntax ?` means that there is a general error where the program cannot read the code properly. This type of error can be harder to debug.

> For most panic errors, you should follow these steps to resolve them:
> - Read the Error Message: Carefully read the error message provided. It often includes clues about what went wrong and where.
> - Check Syntax: Refer to the syntax guide to ensure your code adheres to the language rules. Verify that all keywords, operators, and punctuation are used correctly.
> - Consult Documentation: Look up relevant sections in the documentation for more details on the error and suggested solutions.
> - Test Incrementally: Simplify and test smaller portions of your code to isolate the problem and verify its source.

## Conclusion

MKA is a relatively small language, but I hope it proves useful to its users. Over time, I plan to expand and refine it for a wider range of scenarios. Thank you for reading this documentation.