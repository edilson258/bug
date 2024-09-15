# Bug Language

Bug is a stack-based programming language with a functional paradigm, developed for educational purposes.

The program written in bug language is compiled by the bug compiler (`bugc`) to an intermediate bytecode and serialized to a file, then the Bug Virtual Machine (`bvm`) loads the file containing the bytecode and execute.

### Hello, world!

`hello.bug`
```code
fn main() void -> "Hello, world!" @write;
```
### More examples
1. `sum two integers`
```
fn sum(int lhs, int rhs) int -> lhs rhs +;
fn main() void -> 34 35 @sum @write;
```
## Try it now 

To use this language your can download a release for you platform or go through the source code.

### From source code
1. clone the repository
```shell
git clone https://github.com/edilson258/bug.git
```
2. Compile and run a bug program
```shell
cargo run --bin bugc <some_program>.bug
cargo run --bin bvm out.bin
```
Now see [Examples](https://github.com/edilson258/bug/tree/main/examples) for help. Happy hacking!

## Contributions
Feel free to fork and play with it. PRs are welcome!💯
