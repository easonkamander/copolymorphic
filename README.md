## CoPolymorphic Type Inference

This project is in the very early stages of development.

Provide a file path containing one closed lambda term.
<br />
For example, the **S** combinator can be written as:
```
x => y => z => x z (y z)
```

Which would result in the following "principal type":
```
Forest (A) {
        A = F -> B
        B = [] -> C
        C = [] -> D
        D = free()
        E = [] -> D
        F = [] -> E
}
```

Expanding these definitions into a single expression yields:
```
([] -> [] -> D) -> [] -> [] -> D
```

This is a somewhat unusual type, encoding the minimal path to HNF.
<br />
In fact, type inference terminates for exactly the solvable terms.
