# SukiScript

## Creators

- [Seth Leander L. Caballero] &nbsp;&nbsp;- Sesilyo
- [Justin B. Lauricio] &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;- llaollao902

## Overview

SukiScript is a domain-specific language (DSL) for describing the daily operations of a sari-sari store. It lets an owner define products, record sales and restocking, monitor inventory, record expenses, and produce a simple profit report.

The name uses suki, a familiar Filipino word for a regular customer, while the language itself uses English keywords for clear, consistent syntax.

## Host language and build

- Host language: Rust
- Version metadata: 0.1.0
- Build: `./build.sh`
- Release binary: `./target/release/cmsc124-terminal-js`

## Running it


| Command | What it does |
|---|---|
| `./run <file>` | Executes a program. This is the Lab 4 path; in current Lab 3 it is not used for expression evaluation. |
| `./run --tokenize <file>` | Prints the token stream for the file. |
| `./run --tokenize` | Starts the interactive tokenizer prompt. |
| `./run --parse <file>` | Prints the parsed AST of each expression in the file. |
| `./run --parse` | Starts the interactive expression parser. |
| `./run --eval <file>` | Evaluates each expression in the file and prints its value, one per line. |
| `./run --eval` | Starts the REPL and prints evaluated values. |
| `./run` | Starts the REPL in the standard interactive mode used by the project. |

| Code | Meaning |
|---|---|
| `0` | Clean run |
| `65` | Input rejected before execution started (lexical error, syntax error, unreadable file, bad usage) |
| `70` | Program started and then hit a runtime error |

## File extension

`.suki`

This matches the `ext` field in the Lab test manifests used by the harness, including `tests/lab3/manifest.json`.

## Lexical structure

### Keywords

| Keyword | Purpose |
|---|---|
| `product` | Declares an item sold by the store |
| `price` | Introduces a product's selling price |
| `stock` | Refers to a product's available quantity |
| `sell` | Records a product sale |
| `quantity` | Introduces the number of units involved |
| `restock` | Adds units to an existing product's stock |
| `expense` | Records an operating expense |
| `cost` | Introduces a monetary cost |
| `calculate` | Computes a derived total |
| `revenue` | Refers to total income from sales |
| `profit` | Refers to revenue minus costs and expenses |
| `show` | Prints a value or message |
| `check` | Starts a conditional block |
| `else` | Starts an alternative conditional branch |
| `true` | Boolean literal |
| `false` | Boolean literal |
| `nil` | Nil literal |

### Operators

| Operator | Category | Operands | Associativity | Precedence |
|---|---|---|---|---|
| `<` | comparison | binary | left | 2 |
| `<=` | comparison | binary | left | 2 |
| `>` | comparison | binary | left | 2 |
| `>=` | comparison | binary | left | 2 |
| `==` | comparison | binary | left | 2 |
| `!=` | comparison | binary | left | 2 |
| `+` | arithmetic | binary | left | 3 |
| `-` | arithmetic | binary | left | 3 |
| `*` | arithmetic | binary | left | 4 |
| `/` | arithmetic | binary | left | 4 |
| `!` | logical negation | unary | right | 5 |
| `-` | arithmetic negation | unary | right | 5 |

### Literals

| Kind | Syntax | Produces |
|---|---|---|
| number | `42`, `3.14` | `f64` runtime value |
| string | `"hello"` | `String` runtime value |
| boolean | `true`, `false` | `bool` runtime value |
| nil | `nil` | `Nil` runtime value |

### Identifiers

- Start characters: letters and `_`
- Continue characters: letters, digits, and `_`
- Case-sensitive: yes
- Keywords are reserved and cannot be used as identifiers

### Comments

- Line comments: `//`
- Block comments: not supported in this language
- Nesting: not supported

## Whitespace and termination

- Whitespace is ignored outside of strings.
- Expressions are evaluated one at a time in file mode.
- Parentheses are the grouping delimiters.
- The parser stops at the end of the current expression line.

## Token output format

The scanner prints one token per line in the form:

```text
Token(type=NUMBER, lexeme=42, literal=42, line=1)
```

The fields are:
- `type`: token category such as `NUMBER`, `PLUS`, `STRING`, or `IDENTIFIER`
- `lexeme`: the exact source text that produced the token
- `literal`: the parsed runtime value, when one exists
- `line`: the source line number

## Grammar

```
program     -> statement*
statement   -> product | sell | restock | expense
             | calculate | show | check

product     -> "product" IDENTIFIER "price" expression "stock" expression
sell        -> "sell" IDENTIFIER "quantity" expression
restock     -> "restock" IDENTIFIER "quantity" expression "cost" expression
expense     -> "expense" IDENTIFIER "cost" expression
calculate   -> "calculate" ( "revenue" | "profit" )
show        -> "show" expression
check       -> "check" expression block ( "else" block )?
block       -> "{" statement* "}"

expression  -> equality
equality    -> comparison ( ( "!=" | "==" ) comparison )*
comparison  -> term ( ( ">" | ">=" | "<" | "<=" ) term )*
term        -> factor ( ( "-" | "+" ) factor )*
factor      -> unary ( ( "/" | "*" ) unary )*
unary       -> ( "!" | "-" ) unary | primary
primary     -> NUMBER | STRING | "true" | "false" | "nil"
             | "stock" IDENTIFIER | "price" IDENTIFIER
             | "revenue" | "profit"
             | "(" expression ")"
```

## Parse output format

The parser prints the AST with prefix notation. The implementation in `print_ast.rs` produces output like:

```text
(+ 2 (* 3 4))
```

Grouping is printed as:

```text
(group (...))
```

and unary expressions are printed as:

```text
(- x)
```

## Semantics

### Values and types

A runtime value is stored in a Rust tagged union:

```rust
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}
```

This is the right fit for a dynamically typed language because the evaluator always receives a value in one uniform type and then checks which case it is at runtime. Rust's pattern matching makes this explicit and prevents forgetting a case.

All numbers are stored as 64-bit floating-point values (`f64`).

### Value printing

The evaluator prints values in a consistent format:

- Numbers: integer-like values print without a trailing `.0`; decimals print normally.
  - Example: `5` prints as `5`, `3.14` prints as `3.14`
- Nil: prints as `nil`
- Strings: print without surrounding quotes.
  - Example: `"hello"` prints as `hello`

This matches the implementation in `src/value.rs` and the Lab 3 harness expectations.

### Truthiness

We use the Lox-style rule:

- `false` is falsey
- `nil` is falsey
- every other value is truthy

Examples:

- `!true` -> `false`
- `!false` -> `true`
- `!nil` -> `true`
- `!0` -> `false`
- `!"hello"` -> `false`

This is the rule implemented by `Value::is_truthy()` in `src/value.rs`.

### Operator semantics

- `+` on two numbers adds them
- `+` on two strings concatenates them
- `+` on mixed types is a runtime error
- `-`, `*`, and `/` require numeric operands on both sides
- `/` by zero is a runtime error
- `<`, `<=`, `>`, and `>=` require numbers and return booleans
- `==` and `!=` work across any pair of values and return booleans
- unary `-` requires a number
- unary `!` applies the truthiness rule to its operand

These rules are implemented in `src/evaluator.rs`.

### Runtime errors

Runtime errors are raised only when a value is invalid for the operator being applied. Examples include:

- negating a string
- subtracting a number from a string
- comparing a number to a string
- dividing by zero

A runtime error:

- prints to stderr
- includes the source line number
- uses exit code `70` in file mode
- leaves the REPL alive so the user can continue entering expressions

Example format:

```text
[line 3] Runtime error: Operands must be numbers.
```

## Native functions


| Name | Arguments | Returns | Notes |
|---|---|---|---|
| [name] | [count and types] | [type] | [caveats] |


## Errors and diagnostics

Message format:

```
[one real static error]
[one real runtime error]
```


| Failure | Exit code |
|---|---|
| [lexical error] | 65 |
| [syntax error] | 65 |
| [runtime error] | 70 |


## Testing conventions


| Folder | Activity | Mode | Flag |
|---|---|---|---|
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | `--eval` |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Functions | inline | none |


```
[specific tests]...
```

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
python3 run_tests.py tests/lab1
```

## Sample code

```
[a short program]
```

Output:

```
[its output]
```

## Design rationale

[Why the language is the way it is. Cover the choices that surprised you, the
features you cut, and the decisions you reversed. Specific reasons, not
approval of your own work.]

## Known limitations

- [What doesn't work, what is unimplemented, where behavior is worse than you
  would like.]

## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | [entry] |
