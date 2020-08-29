# java-random-lcg-rs
Java Random lcg implementation in rust that allows for combining

In Cargo.toml:

[dependencies]<br>
java_random_lcg = { git = "https://github.com/v31l-sys/java-random-lcg-rs" }

================================================================

In main.rs:<br><br>
use java_random_lcg::*;<br><br>

Multiple LCGs are already available:<br>
FORWARD[1..=4]<br>
BACKWARD[1..=6]<br><br>

FORWARD1 is equivalent to the first Java Random LCG and so on.

```rust
let mut cr = ChunkRandom::default();
cr.get_random(32141);
cr.random_next(0, &FORWARD3); //equivalent of Java random_next 3 times
```

To create your own combined LCG:

```rust
let mut cr = ChunkRandom::default();
cr.get_random(32141);

//create our arbitrary LCG from Java Random
let arbitrary_lcg_forward = LCG::generate(128, JAVA_MULTIPLIER, LCGType::FORWARD);

//LCG class has members: multiplier, addend, modulus, and LCGType. Currently it is only implemented for Java Random.

cr.random_next(0, &arbitrary_lcg_forward); //equivalent of Java random_next 128 times
```
