+++
title = 'Myers alorirhm'
description = 'Building an edit string with the Myers algorithm'
date = 2025-11-01T16:35:00+02:00
draft = true
tags = ['graph']
script = "myers/js/main.ts"
github = 'diff'
+++

<style>
  .flex-container {
    display: flex;
    padding: 10px;
  }
  .flex-container > div {
    padding: 20px;
    margin: 5px;
  } 
  #diffData .diff-add td,  
  #diffData tr.diff-add {  
      background-color: var(--darkgreen); 
  }
  #diffData .diff-delete td, 
  #diffData tr.diff-delete {  
      background-color: var(--darkred);
  }
  #diffData {
    border: 0;
    border-spacing: 0;
    border-collapse: collapse;
    width: fit-content; /* Add this line */ }
  #solutions { display: grid; grid-template-columns: repeat(4, 1fr); }
</style>


The Myers algorithm is the common algorithm for creating a diff. It is used in the diff tool or git. 
It works differend then LCS. LCS builds the Longest common substring, Myers algorithm searches for 
the shortest edit script. It can be optimized to find the ideal represantation of the difference.

it is important that we are traversing nodes and not cells in the grid. we can move either left, right or 
diagonal. When reaching a node with a diagnoal connection we directly slip to the end of the diagonal
line. This connection is called snake in the original paper, refering to the game snakes & Ladders where
you slide down to the snakes tail when you reach the head.

when we search the path we keep track of the moves with `k` and `d`. Where `d` is the depth in the 
tree and `k` is the depth in the vertical axis. `k` can be calculated with `k = x - y`.

When we move to the right `x` increases and so does `k`, moving down increases `y` so `k` decreases.
when sliding down the sname `x` and `y` increases, so `k` keeps the same.

when we start traversing the tree we select the best move. the best move is the one with the highest 
`x` value. This will prefere deletions over insertions.










For example, let's take two strings:

```
String A: ABCDEFG
String B: ABDCEFG
```

The longest common subsequence is `ABCEFG`. Notice that the "D" from String B is skipped to maintain the common sequence, as it does not appear in the correct position relative to the "C".

To calculate the LCS, we can use a dynamic programming approach by filling a two-dimensional grid. The rules for filling each cell in the grid are as follows:

1) If the characters for the current row and column match, take the value from the top-left diagonal cell and add one.
1) If the characters do not match, take the maximum value from either the cell directly above or the cell directly to the left.

Below is an interactive demonstration. You can input two strings and see how the grid is constructed.

<form id="lcsForm">
  <label for="a">Input A:</label>
  <input type="text" id="lcs_input_a" name="a" value="ABCABBA">
  <label for="b">Input B:</label>
  <input type="text" id="lcs_input_b" name="b", value="CBABAC">
  <button type="button" id="lcs_button">Calculate</button>
</form>

<figure>
  <canvas id="lcs_canvas_1" width="400" height="400"></canvas>
</figure>

Once the grid is filled, we can find the LCS by backtracking from the bottom-right cell to the top-left, following the path that led to the final length.

<form id="lcsForm">
  <label for="result">Result:</label>
  <input type="text" id="lcs_result" name="result">
</form>

the diff datas:

<table id="diffData">
    <thead>
        <tr>
            <th width="20px"></th>
            <th width="30px"></th>
            <th width="30px"></th>
            <th width="30px"></th>
        </tr>
    </thead>
    <tbody>
</tbody>
</table>

## diff on source code


to create a diff on source code we want to change the code to work with lines in the text.


<form id="codeForm">
  <label for="a">Code A:</label>
  <input type="text" id="source_code_input_a" name="a" value="" width="100%" heigh="50px">
  <textarea id="code-editor-1" name="code-editor" rows="10" cols="50" spellcheck="false">
  function greet() {
    console.log("Hello, world!");
  }
  </textarea>
  <label for="b">Input B:</label>
  <textarea id="code-editor-2" name="code-editor" rows="10" cols="50" spellcheck="false">
  function greet() {
    console.log("Hello, world!");
  }
  </textarea>
  <button type="button" id="lcs_button">Calculate</button>
</form>



