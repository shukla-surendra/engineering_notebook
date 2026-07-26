# 3. Evaluate Reverse Polish Notation

**Difficulty:** Medium
**Topic:** Stack
**Pattern:** Stack-based expression evaluation

## Problem
Evaluate an arithmetic expression given in Reverse Polish (postfix) Notation, provided as a
list of tokens (integers and `+ - * /`). Division truncates toward zero.

## Examples
```
Input: tokens = ["2","1","+","3","*"] -> 9   ((2+1)*3)
Input: tokens = ["4","13","5","/","+"] -> 6   (4 + (13/5))
```

## Approach
Postfix notation is naturally evaluated with a stack: push numbers as they come. On an
operator, pop the top two operands (note the order — the second-popped is the left
operand), apply the operator, and push the result back. At the end exactly one value
remains on the stack: the answer.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Stack-based expression evaluation**, which
itself belongs to the broader **Stack** family of techniques. If the specific trick
above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 stack/03_evaluate_reverse_polish_notation/solution.py`):

```python
--8<-- "stack/03_evaluate_reverse_polish_notation/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Definitional framing (the natural opening for this one — there isn't really a
  separate brute force):** "Postfix notation is *defined* by how it's evaluated — operands
  come before their operator, so a stack is basically the direct implementation of the
  definition rather than a clever optimization over some other approach. I'd say that up
  front, since it explains why there's no O(n²) alternative worth naming."
- **Invariant framing (good for explaining the operand-order subtlety precisely):** "The
  invariant is: at any point, the stack holds exactly the operands not yet consumed by an
  operator, in the order they'd appear if fully parenthesized. The subtlety is operand
  order on subtraction and division — the second-popped value is the left operand, not
  the first-popped, since the stack reverses the pop order relative to push order."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  'stack as evaluator' pattern — the same idea extends to evaluating infix expressions
  with operator precedence (via two stacks, or by first converting to postfix), so I'd
  mention this is the simpler of two related expression-evaluation problems."

### Vocabulary Builder

- **postfix / infix notation** (n. phrases) — postfix places operators after their
  operands (`2 1 +`); infix is the conventional in-between placement (`2 + 1`). Naming
  both shows you know why postfix avoids needing operator-precedence rules at all.
- **operand** (n.) — a value an operator acts on, as opposed to the operator itself;
  useful vocabulary for narrating exactly what you're popping and pushing.
- **truncate toward zero** (v. phrase) — integer division that rounds toward zero rather
  than always down; worth calling out explicitly since it differs from Python's default
  `//` behavior on negative numbers.
- **"mirrors how the notation is defined"** — a reusable phrase for problems where the
  data structure choice isn't an optimization trick but a direct match to the problem's
  own structure.
