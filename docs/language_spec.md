# 📖 Owlang Language Specification – v0.1

1. **Overview**

Owlang is a human-readable programming language designed to be intuitive even for non-programmers.
The goal is to make code look like structured natural language while remaining unambiguous for the compiler.

2. **Tokens**

| Category    | Token(s)                                                                             | Regex / Rule                                   | Examples                               |
| ----------- | ------------------------------------------------------------------------------------ | ---------------------------------------------- | -------------------------------------- |
| Identifiers | User-defined names (variables, functions, etc.)                                      | `[a-zA-Z_][a-zA-Z0-9_]*`                       | `myVar`, `calculateSum`, `_privateVar` |
| Keywords    | `is`, `if`, `then`, `otherwise`, `while`, `not`, `and`, `or`, `true`, `false`, `say` | Reserved words, case-sensitive                 | if name = "x" then say y               |
| Integers    | Whole numbers                                                                        | `[0-9][0-9_]*`                                 | `0`, `42`, `1_000`                     |
| Floats      | Numbers with decimal/exponent                                                        | `[0-9][0-9_]*\.[0-9][0-9_]*([eE][-+]?[0-9]+)?` | `3.14`, `2.71828`, `1.0e-10`           |
| Strings     | Text literals                                                                        | `"([^"\]                                       | \.)\*"or"""..."""`                     |
| Booleans    | Boolean literals                                                                     | Keywords: `true`, `false`                      | `if flag = true then ...`              |
| Operators   | `is`, `=`, `not`, `and`, `or`, `+`, `-`, `*`, `/`, `%`, `<`, `>`, `<=`, `>=`         | Context-dependent                              | `x is 5`, `if x = 10`                  |
| Punctuation | Structural characters                                                                | `(`, `)`, `{`, `}`, `[`, `]`, `,`, `;`         | `say(x, y)`                            |
| Comments    | Single-line comment                                                                  | `#.*`                                          | `# this is a comment`                  |
| Whitespace  | Space, tab, newline                                                                  | `[ \t\n]+`                                     | -                                      |

3. **Example Program**

```ow
# Greeting example
name is "Grato"
say "Hello " + name

if name = "Grato" then
    say "You are the special guest"
otherwise
    say "You are a normal guest"

x is 0
while x < 3 then
    say "x is " + x
    x is x + 1
```

4. **Design Philosophy**

- Owlang is designed with the following principles in mind:
  - **Readable by humans**: Code should resemble simple English sentences.
  - **Minimal symbolic noise**: Replace symbols (`=`, `!`, `&&`, `||`) with words (`is`, `not`, `and`, `or`).
  - **Universal math operators remain**: `+`, `-`, `<`, `>` are kept for familiarity.
  - **Education-first**: A non-programmer should be able to guess what a line does.
