# spinifex-unsigned-varint

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

Implementation of the unsigned variable integer type used in multiformats

## Table of Contents

- [Install](#install)
- [Usage](#usage)
- [API](#api)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

```shell
$ git clone git@github.com:jmcph4/spinifex-unsigned-varint.git
$ cd spinifex-unsigned-varint
$ cargo build
```

## Usage

```rust
/* initialise from native integer types */
let some_number: u128 = 128;
let my_uvarint: UVarInt = UVarInt::new(some_number);

/* encode into byte vector */
let my_uvarint_bytes: Vec<u8> = my_uvarint
println!("{:#b}", my_uvarint_bytes); /* "[128, 1]" */

/* decode from byte vector */
let my_other_uvarint_bytes: Vec<u8> = vec![128, 128, 1];
let my_other_uvarint: UVarInt = UVarInt::from_bytes(my_other_uvarint_bytes).unwrap();
println!("{}", my_other_uvarint); /* "uv16384" */
```

## Maintainers

[@jmcph4](https://github.com/jmcph4)

## Contributing

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

MIT © 2020 Jack McPherson
