# show-enum

Parse C enumerations and output a
[show](https://hackage.haskell.org/package/base-4.20.0.1/docs/Prelude.html#v:show)
function to stdout.

For this input file:

```c
typedef enum Flags
{
	RDONLY,
	WRONLY,
	RWRITE,
} Flags;
```

The command:

```bash
show-enum ./test.c
```

Will generate the following code:

```c
const char *show_flags(Flags self) {
    switch (self) {
        case RDONLY: return "RDONLY";
        case WRONLY: return "WRONLY";
        case RWRITE: return "RWRITE";
        default: return "";
    }
}
```

## Features

- Generate function to display your enums
- Blazingly fast 🔥
- Zero-allocation: all allocations come from reading the input file and printing to stdout

## Upcoming features

- [ ] Read input from stdin to be usable in pipelines
- [ ] Specify a prefix for your library or application
- [ ] Specify whether to use snake_case or camelCase

## License

MIT