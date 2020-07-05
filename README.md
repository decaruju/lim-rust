# GOALS
- Orthogonality of features
- No function declaration statement
- No complex scoping, everything is function scoped, no globals
- Only BigInts and doubles, no surprising number behaviour
- Small grammar
  - no for/while constructs
  - no if/else 
- First class enum
  - A pattern matching construct
- Few reserved keywords
  - boolean is an enum
- Failing functions are async functions
- No exceptions, every function returns an error monad
- No limitations between language features and user code
  - Operator overloading
  - callable structs
  - natives monkey patching
- Implicit type coercion when obvious
- smallest minimal program
  - no arcane `public static void main` construct
  
# Undecided
+=, -=, /=, *=, %=
&& and || or & and |
Bitwise operators (probably not)
pattern matching syntax
import syntax
class and enum definition syntax


# Mindset
- A few aspects make a language good or bad
1. How well it manages Arrays, HashMaps and optionally sets
2. How well it manages imports
3. How terse it is
4. How active is its ecosystem
5. How extensible it is
6. How fast it is
6. How safe it is

# Examples
See the example folder for more examples
