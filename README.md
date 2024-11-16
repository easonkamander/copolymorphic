## CoPolymorphic Type Inference

This project is still in the early stages of development.

Enter a file path containing one closed lambda term.
<br />
For example, the **s** combinator can be written as:
```
x => y => z => x z (y z)
```

Which would result in the following principal type:
```
Forest (A) {
        A = F -> B
        B = I -> C
        C = G & J -> D
        D = free()
        E = H -> D
        F = G -> E
        G = free()
        H = free()
        I = J -> H
        J = free()
}
```

Expanding these definitions into a single expression yields:
```
A =
F -> B =
(G -> E) -> I -> C =
(G -> H -> D) -> (J -> H) -> (G & J) -> D
```
