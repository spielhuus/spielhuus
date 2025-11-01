+++
title = 'Longest Common Subsequence'
description = 'Exploring the Longest Common Subsequence algorithm, a key concept in diffing.'
date = 2025-10-30T16:35:00+02:00
draft = false
tags = ['graph']
script = "lcs/js/main.ts"
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
  #solutions { display: grid; grid-template-columns: repeat(4, 1fr); }
</style>


Displaying the difference between two sequences of text is a common problem in computer science. These "diffing" algorithms are essential for tools like version control systems (e.g., Git) and are integrated into most code editors to show changes.
Many of these algorithms are based on a fundamental concept: finding the Longest Common Subsequence.
Longest Common Subsequence (LCS)
Before diving into complex diffs, it's helpful to understand the {{< wikipedia "Longest common subsequence problem" >}}. The goal is to find the longest subsequence of characters that appears in the same order within two different strings.

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
  <input type="text" id="lcs_input_a" name="a" value="CBDA">
  <label for="b">Input A:</label>
  <input type="text" id="lcs_input_b" name="b", value="ACADB">
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


