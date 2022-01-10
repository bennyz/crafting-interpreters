# Chapter 4 - Scanning

## Notes

(Maximal Munch)[https://en.wikipedia.org/wiki/Maximal_munch] when creating a construct, the most possible input should be consumed (greediness in regex for example).

## Questions

> The lexical grammars of Python and Haskell are not regular. What does that mean, and why aren’t they?
They are not recognizable by a finite automaton. Python for example has to keep track of white-spaces.

> Aside from separating tokens—distinguishing print foo from printfoo—spaces aren’t used for much in most languages. However, in a couple of dark corners, a space does affect how code is parsed in CoffeeScript, Ruby, and the C preprocessor. Where and what effect does it have in each of those languages?

Soon

> Our scanner here, like most, discards comments and whitespace since those aren’t needed by the parser. Why might you want to write a scanner that does not discard those? What would it be useful for?
They can be useful for code generation, like Go does.

Soon

> Add support to Lox’s scanner for C-style /* ... */ block comments. Make sure to handle newlines in them. Consider allowing them to nest. Is adding support for nesting more work than you expected? Why?

Soon



