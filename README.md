# unit-enum

From an existing enum, derive a unit `enum` where none of the
variants have any fields.

```rust
use unit_enum::UnitEnum;

#[derive(UnitEnum)]
#[unit_enum(name = "AllType", derive(Clone, Copy, PartialEq, Debug))]
enum All {
    Unit,
    Tuple(bool, i32),
    Struct { x: i32, y: i32 },
}
```

Will generate:

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
enum AllType {
    Unit,
    Tuple,
    Struct,
}
```

It will also derive `From`/`Into` traits so you can do:

```rust
let all_struct: All = All::Struct { x: 12, y: -4 };
let all_type_struct: AllType = AllType::from(&all_struct);

assert_eq!(all_type_struct, AllType::Struct);
```