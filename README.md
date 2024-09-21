
# Module 1: Getting Started with Rust

## What is Rust?

Rust is a systems programming language focused on safety, speed, and concurrency. It guarantees memory safety without needing a garbage collector, making it ideal for applications that require high performance, such as game engines, operating systems, and web servers. Its modern syntax, coupled with its emphasis on safe code, makes it an increasingly popular choice among developers.

## Installing Rust

To get started with Rust, you need to install it on your machine. Follow these steps:

1. Open your terminal (Command Prompt for Windows, Terminal for macOS/Linux).
2. Run the following command to install Rust using the official installation tool, `rustup`:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Follow the on-screen instructions to complete the installation.

   - On Windows, you can alternatively use the official Rust installer [here](https://rust-lang.org).

   Once installed, you can verify the installation by running:

   ```bash
   rustc --version
   ```

   This should output the version of Rust you have installed.

## Creating and Running a Rust Project

1. **Creating a New Project**
   
   To create a new Rust project, navigate to the folder where you want your project to reside and run:

   ```bash
   cargo new project_name
   cd project_name
   ```

   `cargo` is Rust's build system and package manager, and this command sets up a new project with the necessary files.

2. **Compiling and Running a File**

   Inside your new project folder, you’ll find a `src/main.rs` file. To compile and run your project, use the following command:

   ```bash
   cargo run
   ```

   This will compile the code and run the resulting executable.

3. **Testing Your Project**

   Rust includes a built-in testing framework. To run tests, you can add test functions to your code, then use:

   ```bash
   cargo test
   ```

## Setting Up VS Code for Rust

For this course, we recommend using Visual Studio Code as your code editor. Here’s how to set it up:

1. **Install VS Code**  
   If you don’t already have VS Code installed, you can download it [here](https://code.visualstudio.com/).

2. **Rust Extensions for VS Code**

   To improve your Rust development experience, install the following extensions:

   - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer): This provides autocompletion, code navigation, and more.
   - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb): A debugger for Rust.

After installing these extensions, you'll be ready to develop, debug, and run Rust code directly within VS Code.

# Module 2: Rust Data Types and Memory

## Data Types in Rust

In Rust, every value has a data type. The data type tells Rust what kind of data is being used. Some common data types are:

- **Integers**: Whole numbers like `5`, `-3`. Examples:
  ```rust
  let x: i32 = 10; // 32-bit integer
  let y: u8 = 255; // Unsigned 8-bit integer (positive only)
  ```

- **Floating Points**: Numbers with decimals like `4.2`. Examples:
  ```rust
  let z: f64 = 3.14; // 64-bit floating point
  ```

- **Booleans**: True or false values. Example:
  ```rust
  let is_active: bool = true;
  ```

- **Characters**: Letters or symbols like `a` or `#`. Example:
  ```rust
  let letter: char = 'A';
  ```

## Determining Allocated Memory

Different types use different amounts of memory. For example:
- `i32` uses 32 bits (4 bytes).
- `f64` uses 64 bits (8 bytes).

You can check the size of any type using this code:
```rust
println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
```

## Stack and Heap

Rust stores variables in two places: the **stack** and the **heap**.

- **Stack**: Fast, but stores small, simple data types (like integers).
- **Heap**: Slower, but stores larger data like strings.

Example:
```rust
let x = 5; // Stored on the stack
let s = String::from("Hello"); // Stored on the heap
```

## Variable Scope

Variables in Rust have a **scope**, meaning they only exist in certain parts of the program. When a variable goes out of scope, it’s removed from memory.

Example:
```rust
{
    let x = 5; // x is in scope
    println!("{}", x);
} // x goes out of scope here
```

## Shadowing

**Shadowing** lets you declare a variable with the same name, but change its value or type.

Example:
```rust
let x = 5;
let x = x + 1; // x is now 6
let x = "Hello"; // x is now a string
```

## Arithmetic and Conditional Operators

Rust supports basic math operations:
- `+` (Addition)
- `-` (Subtraction)
- `*` (Multiplication)
- `/` (Division)
- `%` (Modulus)

Example:
```rust
let sum = 5 + 10;
let product = 4 * 3;
let remainder = 10 % 3;
```

### Conditional Operators

Conditional operators allow you to compare values:
- `==` (Equal to)
- `!=` (Not equal to)
- `>` (Greater than)
- `<` (Less than)

Example:
```rust
let x = 5;
if x > 3 {
    println!("x is greater than 3");
} else {
    println!("x is 3 or less");
}
```

---

This module explains the basic data types and memory handling in Rust. Understanding these topics will help you write safe and efficient Rust code!

# Module 3: Control Flow in Rust

## Conditionals

Conditionals in Rust allow you to make decisions in your code using `if`, `else if`, and `else`.

Example:
```rust
let x = 5;

if x > 10 {
    println!("x is greater than 10");
} else if x == 5 {
    println!("x is equal to 5");
} else {
    println!("x is less than 10");
}
```

### Using `if` in a `let` Statement

You can also use `if` in a `let` statement to assign a value based on a condition:
```rust
let number = if x > 5 { 10 } else { 0 };
println!("The value is {}", number);
```

## Loops

Rust has different kinds of loops: `loop`, `while`, and `for`.

### `loop`

The `loop` keyword creates an infinite loop until you explicitly tell it to stop using `break`.

Example:
```rust
let mut counter = 0;

loop {
    counter += 1;
    if counter == 5 {
        break; // Exit the loop when counter is 5
    }
    println!("Counter is at {}", counter);
}
```

### `while` Loop

A `while` loop will run as long as a condition is true.

Example:
```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
println!("Liftoff!");
```

### `for` Loop

A `for` loop is useful for iterating over a collection of values.

Example:
```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("The value is: {}", element);
}
```

## Pattern Matching with `match`

The `match` expression is a powerful tool in Rust that allows you to compare a value against a set of patterns and execute code based on the pattern that matches.

Example:
```rust
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"),
}
```

In this example, the `_` pattern is a catch-all, meaning it will match any value not explicitly matched by the other arms.

### Matching Multiple Values

You can also match multiple patterns with a single arm:

```rust
let number = 1;

match number {
    1 | 2 => println!("One or Two"),
    _ => println!("Something else"),
}
```

### Matching Ranges

You can match a range of values:

```rust
let number = 6;

match number {
    1..=5 => println!("Between 1 and 5"),
    _ => println!("Outside the range"),
}
```

---

This module covers Rust's control flow mechanisms, such as conditionals, loops, and pattern matching using `match`. These tools are essential for handling logic in your programs.


