
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
# Module 4: Advanced Data Types in Rust

## 1. Structs and Enums

### Structs

Structs are custom data types that let you create complex types by grouping together different data types. They are similar to classes in other programming languages.

```rust
// Define a struct
struct Person {
    name: String,
    age: u32,
}

// Create an instance of the struct
fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Name: {}, Age: {}", person.name, person.age);
}
```
### Enums
Enums allow you to define a type that can be one of several variants. This is useful for representing data that can take on a limited set of values.

```rust
// Define an enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Use the enum
fn move_player(direction: Direction) {
    match direction {
        Direction::Up => println!("Moving up!"),
        Direction::Down => println!("Moving down!"),
        Direction::Left => println!("Moving left!"),
        Direction::Right => println!("Moving right!"),
    }
}

fn main() {
    let direction = Direction::Up;
    move_player(direction);
}
```
## 2. Option<T>
The Option<T> type is used to represent a value that may or may not be present. It is a powerful way to handle nullable values.

```rust
// Define a function that returns an Option
fn find_item(index: usize) -> Option<&'static str> {
    let items = ["apple", "banana", "cherry"];
    if index < items.len() {
        Some(items[index])
    } else {
        None
    }
}

fn main() {
    match find_item(1) {
        Some(item) => println!("Found: {}", item),
        None => println!("Item not found."),
    }
}
```
## 3. Arrays
Arrays in Rust are fixed-size collections of elements of the same type. The size of an array is determined at compile time.

```rust
// Define an array
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First number: {}", numbers[0]);
}
```
## 4. Vectors (Variable Length)
Vectors are similar to arrays, but they can change size at runtime. They are more flexible than arrays and are a common choice for collections of data.
```rust 
// Define a vector
fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    
    for number in &numbers {
        println!("{}", number);
    }
}
```
Conclusion
In this module, we've covered advanced data types in Rust, including `structs`, `enums`, `Option<T>`, `arrays`, and `vectors`. Understanding these concepts is crucial for effective Rust programming.
# Module 5: Working with Strings and Functions

## 1. String Slicing

String slicing allows you to reference a part of a string. This is useful when you only need a portion of the string.

```rust
fn main() {
    let my_string = String::from("Hello, Rust!");
    let slice = &my_string[0..5]; // Slicing the first 5 characters
    println!("Slice: {}", slice); // Output: Hello
}
```
## 2. String Manipulation
You can change and manipulate strings in various ways, like adding or removing characters.

```rust
fn main() {
    let mut my_string = String::from("Hello");
    my_string.push_str(", Rust!"); // Adding text
    println!("{}", my_string); // Output: Hello, Rust!

    let new_string = my_string.replace("Rust", "World"); // Replacing text
    println!("{}", new_string); // Output: Hello, World!
}
```
## 3. Tuples and Structs
### Tuples
Tuples are a way to group different types of data together. They can hold a fixed number of values.

```rust
Copy code
fn main() {
    let my_tuple = (10, 20.5, "Hello");
    println!("First: {}, Second: {}, Third: {}", my_tuple.0, my_tuple.1, my_tuple.2);
}
````
### Structs
Structs, as mentioned in Module 4, are custom data types that allow you to group related data together.

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point is at ({}, {})", p.x, p.y);
}
```

4. Functions and Closures
### Functions
Functions are blocks of code that perform a specific task. You can reuse them throughout your program.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice");
}
```
### Closures
Closures are similar to functions but can capture variables from their surrounding environment.

```rust
fn main() {
    let x = 5;
    let add = |y| y + x; // Closure that captures `x`
    
    println!("5 + 5 = {}", add(5)); // Output: 10
}
```
Conclusion
In this module, we've covered string slicing, string manipulation, tuples, structs, functions, and closures. These concepts are fundamental for writing effective Rust programs.
