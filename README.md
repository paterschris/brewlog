# brewlog

A small command-line calculator for brewing coffee. Tell it how much water or
coffee you have and it works out the rest.

```sh
$ brewlog v60 --water 320
Pour-over
  coffee: 20 g
  water:  320 g
  ratio:  1:16.0
  steep:  3:30
```

Water can also be given in cups or fluid ounces:

```sh
$ brewlog press --ounces 12
French press
  coffee: 23 g
  water:  340 g
  ratio:  1:15.0
  steep:  4:00
```

## Supported methods

| Method       | Aliases                 | Ratio |
| ------------ | ----------------------- | ----- |
| Pour-over    | `pour-over`, `v60`      | 1:16  |
| French press | `french-press`, `press` | 1:15  |
| AeroPress    | `aeropress`             | 1:14  |
| Cold brew    | `cold-brew`, `cold`     | 1:8   |

## Building

```sh
cargo build --release
cargo test
```

## License

MIT
