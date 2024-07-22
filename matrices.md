<h1 align="center">
  <br>
  <a href="YOUR_REPO_LINK_HERE"><img src="https://github.com/drllama07/MKA_Programming_language/blob/master/MKA.png" alt="YOUR_APP_NAME" width="200"></a>
  <br>
  MKA Programming Language
  <br>
</h1>

<h4 align="center">A simple programming language for math-based applications.</h4>


## Introduction

- Now you can manipulate 2D matrices in MKA, opening up even more use scenarios.

***NOTE:*** This new update is not fully documented and tested. Therefore, this new features might contain bugs.

## Syntax

- An example of declaring a matrix:
  > ```rust 
  > # matrix_name = {
  >  [1, 1, 1]
  >  [1, 1, 1]
  > }
  > ```
- Now with `#` symbol you can use it to distinguish matrix assignment.
- To make it as close to as how mathematical representation of matrices MKA uses this syntax

## Matrix Operations
- Unlike other data types, matrices can only be used within matrix functions.
- Furthermore, matrices cannot be indexed or modified like vectors. But, using special functions you can access them.

- ***MATRIX FUNCTIONS:***
- > **Matrix Multiplication:**
  > 
  > Example: 
  > ```rust 
  > m_mult(matrix_name, matrix_name, output_matrix)
  > ```
  > - first and second parameters are the matrices that MKA will use.
  > - For matrix functions you have to provide a output name.
  > - This is because, with the new functionality in MKA, the results of matrix operations are stored in a new matrix.
- > **Element-wise Multiplication:**
  > 
  > Example: 
  > ```rust 
  > m_star(matrix_name, 1+ 1, output_name)
  > ``` 
  > This time every elemnt in matrix_name will be multiplied by the second parameters value.
- > **Element-wise Addition:**
  > 
  > Example: 
  > ```rust 
  > m_plus(matrix_name, 1+ 1, output_name)
  > ``` 
  > This time every elemnt in matrix_name will be added by the second parameters value.
- > **Printing Matrices** 
  > ```rust 
  > m_print(matrix_name)
  > 
  > OUTPUT:
  > -- [] -- 
  > -- [] -- 
  > ```

- > **Accessing Elements**
  > 
  > ```rust 
  > m_get(matrix_name, first_index, second_index)
  > OUTPUT:
  > f32
  > m_change(matrix_name, first_index, second_index, new_value)
  > ```
  > These functions will allow you to access values and change them by indexes


## Conclusion 
- For error guide about matrices MKA will output what is the error in the code.
- This will be mainly used in my next example program, writing a XOR neural network in MKA.


