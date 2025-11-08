+++ 
title = 'Myers Algorithm' 
description = 'Building an edit script with the Myers algorithm' 
date = 2025-11-01T16:35:00+02:00 
draft = true 
tags = ['graph', 'algorithm', 'diff'] 
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
  #codeForm2 {  
    display: flex;  
    flex-direction: column;  
    gap: 10px;
    margin-bottom: 25px;
    width: fit-content;
  }  
  #codeForm textarea, #codeForm2 textarea {  
    min-width: 400px; /* Ensure textareas have enough width */  
  }  
  #diff-table .diff-add td,    
  #diff-table tr.diff-add, 
  #diffSourceData .diff-add td,
  #diffSourceData tr.diff-add, 
  #diffSourceData2 .diff-add td,    
  #diffSourceData2 tr.diff-add {    
      color: var(--green);   
      background-color: var(--darkgreen);   
  }  
  #diff-table .diff-delete td,   
  #diff-table tr.diff-delete, 
  #diffSourceData .diff-delete td,
  #diffSourceData tr.diff-delete, 
  #diffSourceData2 .diff-delete td,   
  #diffSourceData2 tr.diff-delete {    
      color: var(--red);  
      background-color: var(--darkred);  
  }  
  #diff-table, #diffSourceData, #diffSourceData2 {
    border: 0;  
    border-spacing: 0;  
    border-collapse: collapse;  
    width: fit-content; 
  } 
  #solutions { display: grid; grid-template-columns: repeat(4, 1fr); } 
</style> 
 
The Myers algorithm is the foundational method for computing differences, widely employed in tools like `diff` and Git. Unlike algorithms that focus on the Longest Common Subsequence (LCS), Myers aims to find the *shortest edit script (SES)*. This script details the precise insertions and deletions needed to transform one sequence into another, representing the most efficient set of changes. The algorithm's efficiency is remarkable, scaling with the sum of string lengths and the minimum edit distance `D`. 
 
### The Edit Graph 
 
Understanding Myers begins with visualizing the problem on an edit graph. Nodes in this graph are coordinates `(x, y)`, where `x` represents the index in string A and `y` in string B. A path from `(0, 0)` to `(N, M)` (where `N` is length of A, `M` is length of B) represents a sequence of edits. The goal is to find the shortest such path. 
 
Movement rules define the edits: 
*   **Right:** `(x, y) → (x+1, y)` signifies a deletion of `A[x]`. 
*   **Down:** `(x, y) → (x, y+1)` signifies an insertion of `B[y]`. 
*   **Diagonal:** `(x, y) → (x+1, y+1)` signifies a match, occurring when `A[x]` equals `B[y]`. 
 
### Key Variables: `d` and `k` 
 
To navigate this graph efficiently, Myers uses two key variables during its breadth-first search: 
*   `d`: Represents the current *edit distance* or "depth" of traversal. Each non-diagonal move (insertion or deletion) increments `d`. The algorithm systematically explores paths for increasing `d` until it finds one that reaches the end node `(N, M)`. The smallest `d` for which `(N, M)` is reachable is the minimum edit distance. 
*   `k`: The diagonal index, calculated as `k = x - y`. 
    *   A right move (deletion) increments `x` but keeps `y` constant, so `k` increases by 1. 
    *   A down move (insertion) increments `y` but keeps `x` constant, so `k` decreases by 1. 
    *   A diagonal move (match) increments both `x` and `y` equally, so `k` remains constant. 
 
### Snakes: The Greedy Match 
 
A critical optimization in Myers' algorithm is the concept of a "snake." A snake begins with a single non-diagonal move (an insertion or deletion) and is immediately followed by zero or more diagonal moves (matches). When the algorithm makes an insertion or deletion, it then greedily extends this path along any subsequent matching characters as far as possible. This effectively "slides" along the diagonal, bypassing a series of explicit diagonal steps and optimizing the search. This is akin to sliding down a ladder in Snakes & Ladders, moving directly to the end of a sequence of matches. 
 
### The Furthest Reach (`V` array) 
 
The core of Myers' efficiency comes from tracking the `V` array. For each `d`, `V[k]` stores the *furthest `x` coordinate reached* on diagonal `k`. 
At each `d`, the algorithm computes `V[k]` for all relevant `k` values (from `-d` to `d`, stepping by 2, as `k` parity flips with each `d`). For a given `k` at depth `d`, the algorithm can arrive from: 
1.  A deletion from diagonal `k-1` at depth `d-1`: The previous `x` was `V[k-1]`. The new `x` becomes `V[k-1] + 1`. 
2.  An insertion from diagonal `k+1` at depth `d-1`: The previous `x` was `V[k+1]`. The new `x` remains `V[k+1]`. 
The algorithm chooses the path that yields the greatest `x` value for `d`. After determining this starting `x` for the current `d` and `k`, it extends `x` along any matching characters (the snake) until a mismatch or boundary is hit. This new, maximized `x` is then stored in `V[k]`. The process continues for increasing `d` until `V[N-M]` reaches `N`, indicating a complete path from `(0,0)` to `(N,M)` has been found with `d` edits. 
 
### Path Reconstruction 
 
While the `V` array determines the length of the shortest edit script, reconstructing the actual sequence of operations requires storing additional information (e.g., the predecessor `k` or `d` values) during the forward pass, or by re-running the algorithm in reverse. This allows tracing back the specific insertions and deletions that form the SES. 
 
### Interactive Demonstration 
 

Visualize the Myers algorithm's progress on an edit graph. Observe how `d` increases, `k` varies, and snakes are utilized to find the shortest edit path. 
 
<form id="diffForm"> 
  <label for="a">Input A:</label> 
  <input type="text" id="diff_input_a" name="a" value="ABCABBA"> 
  <label for="b">Input B:</label> 
  <input type="text" id="diff_input_b" name="b" value="CBABAC"> 
  <button type="button" id="step_button">Step</button> 
</form> 
 
<figure> 
  <canvas id="diff_grid" width="400" height="400"></canvas> 
</figure> 

Once the shortest edit script is found, the result can be displayed. 

and the final diff

<table id="diff-table" class="diff-table">
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
 
### Line-based Diff 
 
To create a diff for source code, the Myers algorithm works with lines of text rather than individual characters. Each line is treated as an element, and the algorithm identifies which lines have been added, deleted, or remained unchanged. 
 
<form id="codeForm2"> 
  <label for="a">Code A:</label> 
  <textarea id="source_code_input_a2" name="code-editor" rows="10" cols="50" spellcheck="false"> 
  void func1() { 
   x += 1 
  } 
  void func2() { 
   x += 2 
  } 
  </textarea> 
  <label for="b">Input B:</label> 
  <textarea id="source_code_input_b2" name="code-editor" rows="10" cols="50" spellcheck="false"> 
  void func1() { 
   x += 1 
  } 
  void functhreehalves() { 
   x += 1.5 
  } 
  void func2() { 
   x += 2 
  } 
  </textarea> 
</form> 
 
<table id="diffSourceData2" class="diff-table"> 
    <thead> 
        <tr> 
            <th width="20px"></th> 
            <th width="30px"></th> 
            <th width="30px"></th> 
            <th width="500px"></th> 
        </tr> 
    </thead> 
    <tbody> 
</tbody> 
</table> 

