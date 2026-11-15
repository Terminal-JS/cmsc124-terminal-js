# What I did for Lab5
## **11/16/2026**
- Added `while`, `and`, `or`, `return`
- Updated **README.md**
- Updated **token_types.rs**
  - `TokenType` enum
  - `from_keyword` impl
  - `Display` impl
- Rewired `expression` method precedebce chaining in **parser.rs**:
  - prev:
  ```md
  expression -> equality ->  comparison -> term -> factor -> unary -> primary -> ...
  ```
  - new:
  ```md
  expression -> or -> and -> equality -> comparison -> ...
  ```