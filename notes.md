[The Dragon Book](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)

## Dictionary

### Lexeme
A raw substring that is taken directly from the source code.
- In the `uint256 number = 1;` all the individual parts are lexemes.

It is the smallest part of the language. It does not mean anything on its own, 
but when grouped they form **Tokens**.

---

### Token

When encountering a **lexeme** we want to classify it. 
