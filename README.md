# MLOG Elevated transpiler

This is a transpiler of my custom programming language called MLOG Elevated into MLOG (Mindustry's Logic).

For syntax, see examples.

## Current stage

Right now, the transpiler can succesfully transpile two examples.

However, in many cases (especially when calling builtin functions), excess temporary variables are created. I'll try to optimize it soon.

In future, I'm going to add syntax sugar for arithmetic operations - infix notation. So you could write `(a + b) * c` instead of `mul(add(a, b), c)`.

Currently user-defined functions aren't much developed.

