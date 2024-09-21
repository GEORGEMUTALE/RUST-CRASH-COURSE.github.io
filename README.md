
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
