+++
title = 'Infinite monkey'
description = 'The Infinite Monkey Theorem states that a monkey typing randomly on a typewriter will eventually type the complete works of Shakespeare. This is a concept in probability theory. However, it would take an infinite amount of time, and the monkey would not live long enough to achieve this goal. Using a generative algorithm, we could significantly speed up the process.'
date = 2025-08-14T18:35:00+02:00
draft = false
tags = ['GA']
+++

The **Infinite Monkey Theorem** posits that a monkey typing randomly on a
typewriter will eventually type the complete works of Shakespeare. This theorem
is rooted in probability theory. However, achieving this goal would require
infinite time, more than the monkey's lifespan.

By using a generative algorithm, the process can be accelerated significantly.

## Steps of the Generative Algorithm

1. **Create an Initial Population**
- Generate a population of solutions (Phenotypes) using random characters.
- Create an array of arrays with randomly selected characters.

2. **Calculate Fitness**
- Assess the fitness of the population by counting how many characters in each
solution match the target text (the solution).

3. **Reproduction**
- Select two genotypes based on their fitness.
- Generate crossover products of the selected genotypes.
- Mutate some characters randomly.
- Repeat the selection, crossover, and mutation process until the population is fully recreated.

4. **Check the Solution**
- Verify if the sentence generated matches the solution.
- If a match is found, end the process.
- If not, repeat the process starting from Step 2.

Using these refined steps, we can effectively simulate an approach that would
be practically feasible even if the theorem in its raw form is impractically
endless.

## Result


<div>
<div id="result-string"> </div>
<div>MaxFitness: <span id="maxFitness"></span></div>
<div>Round: <span id="round"></span></div>
<div>Generation: <span id="count-string"></span></div>

<label for="targetInput">Text to write:</label>
<input type="text" id="target">
<input type="submit" id="targetSubmit" value="Submit">
</div>

Yes, this is a somewhat silly example. But it's purely reliant on genetic
evolution to find a solution. The algorithm does not inherently "know" 
the path to the solution. Perhaps, we can discover better applications 
for this methodology.

## links

- {{< link "infinite_monkey" >}}
- {{< github "monkey" >}}

<style>
  .letter {
    font-family: monospace;
    font-size: 2em;
    font-weight: light;
    display: inline-block;
    width: 1em;
  }
  .letter-good {
    font-family: monospace;
    font-size: 2em;
    font-weight: bold;
    display: inline-block;
    width: 1em;
  }
</style>

{{< bindgen path="/js/monkey/monkey.js" >}}
