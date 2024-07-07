# Bug Language

Bug is a stack-based programming language with a functional paradigm, developed for educational purposes.

The program written in bug language is compiled by the bug compiler (`bugc`) to an intermediate bytecode and serialized to a file, then the Bug Virtual Machine (`bvm`) loads the file containing the bytecode and execute.

### Hello, world!

`hello.bug`
```code
f main -> "Hello, world!" .write;
```
### More examples
1. `sum two integers`
```
f sum(int lhs, int rhs) int ->
  lhs rhs +;
f main ->
  34 35 .sum .write;
```

2. `print Fibonacci sequence`
```
fib(int x, int y, int stop) ->
  y stop > if -> return;
  y .write
  y x y + stop .fib;

f main -> 0 1 150 .fib;
```

## Try it now 

To use this language your can download a release for you platform or go through the source code.

### From Release
1. Download a release for you platform [here](https://github.com/edilson258/bug/releases)
2. Unpack and give execution permissions
```shell
unzip <your_platform>-bug-toolkit.zip
chmod +x <your_platform>-bugc
chmod +x <your_platform>-bvm
```
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
