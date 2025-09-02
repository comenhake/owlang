# Owlang Grammar (Draft v0.1)

### Operator Precedence (high → low)

1. Parentheses / literals / identifiers
2. Unary `-`
3. `* / %`
4. `+ -`
5. `< <= > >=`
6. `=` (equality)
7. `not`
8. `and`
9. `or`
10. `is` (assignment, right-associative)

### Decision Rules

- `otherwise` binds to the nearest unmatched `if`
- `not x = y` → `not (x = y)`
- `a < b < c` not allowed; use `a < b and b < c`
- `a is b and c` → `a is (b and c)`
- `a is b = c` invalid; require parentheses
- Blocks: indentation (`INDENT/DEDENT`) or single-line
- Keywords are case-sensitive (`is`, `not`, `and`, `or`, etc.)

### Example Programs

```ow
name is "Anisio"
say "Hello " + name

if name = "Anisio" then
    say "You are the creator"
otherwise
    say "Guest"
```

```ow
while x < 10 then x is x + 1
    say x
```

Also check the [EBNF](grammar.ebnf) and [language specification](language_spec.md) for more details.
