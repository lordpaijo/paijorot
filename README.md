# Paijorot Programming Language

Paijorot is a simple, dynamically-typed, interpreted programming language built with Rust. It was created as a fun hobby project with a unique "brainrot" syntax.

## Syntax Overview

### Printing Output (`yap`)
Use `yap` to print to stdout (equivalent to Rust's `println!`):
```
yap "Hello, World!";
```

### Variables (`ts` and `pmo`)
- `ts` declares a variable (like `let` in Rust)
- `pmo` assigns a value (like `=` in Rust)
```
ts x pmo 42;
```

### Arrays (`gyat` or `gyatt`)
Create arrays with `gyat` or `gyatt` followed by a name and elements in curly braces:
```
gyat numbers {1, 2, 3, 4, 5};
```

### Functions (`hawk` and `tuah`)
Use `hawk` to define functions and `tuah` to specify the return value:
```
hawk sum(a, b) tuah a + b;
```

### Loops (`goon`, `goon(n)`, and `edge`)
- `goon` is an infinite loop (like `loop` in Rust)
- `goon(n)` loops n times (like a for loop)
- `edge` marks the end of a loop block
```
goon(5)
    yap "This will print 5 times";
edge

ts counter pmo 0;
goon
    yap counter;
    ts counter pmo counter + 1;
    yo counter > 5
        sybau;
    gurt
        yap "Continuing...";
edge
```

### User Input (`yeet`)
Read user input with `yeet` and store it in a variable:
```
ts user_input pmo yeet;
```

### Breaking Loops (`sybau`)
Use `sybau` to break out of a loop:
```
goon
    yo condition == true
        sybau;
    gurt
        // continue looping
edge
```

### Conditionals (`yo` and `gurt`)
Use `yo` for if statements and `gurt` for else:
```
yo x > 10
    yap "x is greater than 10";
gurt
    yap "x is not greater than 10";
```

## Building & Running

### Prerequisites
- Rust and Cargo

### Building
```
cargo build --release
```

### Running a Paijorot program
```
./paijorot example.paijorot
```

### Running the REPL
```
./paijorot
```

## Example Program

```
// Print a welcome message
yap "skibidi";

// Define a variable
ts x pmo 5;

// Define a function
hawk sum(y) tuah y+y;

// Call the function and store result
ts z pmo sum(x);

// Conditional statement
yo z pmo 10
    yap z;
gurt
    yap z;
```

## Language Features
- Dynamic typing
- First-class functions
- Arrays support
- Conditional statements
- Loops with break support
- User input handling
- Memory safety inherited from Rust
