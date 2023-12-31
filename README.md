# Mono

Mono is an interpreted programming language developed using Rust.

## Supported Features

As Mono is still in its early stages, not all planned features have been implemented. Current supported features include:

- [x] Evaluating arithmetic expressions.
- [x] Evaluating boolean expressions.
- [x] Variables.
- [x] If-Else statements.
- [x] While statements.
- [x] Functions.
- [x] Lists.

## Cli

Mono's command-line interface (CLI) offers various capabilities, including executing files and offering an interactive REPL. There are also several modes available for both functionalities.

### Usage

To launch the REPL:
```Console
> mono <flag>
```

To execute a file:
```Console
> mono <flag> <path/to/file.mono>
```

To execute code:
```Console
> mono -c <flag> <code>
```

### Flags

The following flags are available to customize your experience:

- `-t` : Tokenizes the input and prints each token.
- `-p` : Parses the input and prints a formatted representation of the generated AST.
- `-e` : Evaluates the input and prints the resulting value.

By utilizing these flags, you can gain insights into various stages of Mono's execution process.
