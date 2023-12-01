*Group Name: Hendies*

*Group Members: Daniel Philipov, Sohum Sharma*


# Rust Regex Compiler
## Overview
A simple regex compiler for UIUC CS 128 Honors project. It will not have all the intricacies of the standard regex compiler, but will have the following features:
* Capture groups ```"(ab)"``` - lets quantifiers operate on the entire group (capturing not supported)
* Character classes ```"[abc]"``` - matches any character in the class
* Character ranges ```"[a-zA-Z]"``` - matches any character in the range
* Alternation ```"a|b"``` - matches either a or b (can be stacked or nested in capture groups)
* Quantifiers:
  * ```"*"``` - 0 or more
  * ```"+"``` - 1 or more
  * ```"?"``` - 0 or 1
  * ```"{n}"``` - exactly n
  * ```"{n,}"``` - n or more
  * ```"{n,m}"``` - between n and m (inclusive)
  * ```"{,m}"``` - at most m

We chose this project because it is a challenging task with a very graphical implementation, which is easy to visualize but hard to implement.

## Technical Stuff

We will implement this by using [Deterministic Finite Automata](https://en.wikipedia.org/wiki/Deterministic_finite_automaton).

![](https://www.tutorialspoint.com/automata_theory/images/dfa_graphical_representation.jpg)

Deterministic finite automata are essentially like graphs that are traversed until the end of the string is reached, whereupon we check if the current node is a terminal node. If it is, we return true, else we return false.

They are made with a four step process, starting with making a nondeterministic finite automata (NFA) that has empty connections (named ε).

1. Turn string into a NFA with ε.
2. Rework nodes and add connections in order to remove the empty connections.
3. Collapse connections so that there are no duplicate paths leaving any node.
4. Remove any nodes that cannot be reached throughout normal use.

### Timeline

By checkpoint 1, we hope to finish Step 1 (the hardest one by far) and be a long way along with Step 2.

By checkpoint 2, we hope to be finished with Steps 1, 2, 3, and 4. From that point on, we will just be testing it for any edge cases we haven't thought of.

## Possible Challenges

Working with traced graphs can cause many, many infinite loops that need to be avoided. A recursive approach would also exceed the recursion depth if such an issue isn't resolved. 

Another challenge will be to collapse nodes and connections into a different set of nodes.

The biggest challenge will be managing the large complex datastructure that is this graph, especially through transitions from NFA to DFA.

## Why we chose this

I (Daniel) have had mild success making a more basic regex compiler in python, but it ended up being horrendously complicated and unreadable, which hopefully I can improve upon, while learning a lot about managing datastructures in rust.
