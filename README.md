# vitalik_example

Here is risczero solution for simple problem defined in Vitalik post.

- https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649



# Why use zkVM to run this?

Our goal is to run code locally without revealing that our solution but verifier on their side will know we know solution for math problem: x^3 + x + 5 = 35

# Project organization

The main program that calls a method in the guest ZKVM is in [cli/src/main.rs](cli/src/main.rs). The code that runs inside the ZKVM is in [methods/guest/src/bin/vitalik_example_calc.rs](methods/guest/src/bin/vitalik_example_calc.rs). The rest of the project is build support.

For the main RISC Zero project, see [here](https://github.com/risc0/risc0)

# Run this example

To build and run this example, use:

```
cargo run --release
```

you will be asked for solution. When you etner your solution if valid proof will be generated. Later we can send proof to verifier but in our case we verify it on same machine where we genrated it. 
