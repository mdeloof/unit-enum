#![allow(dead_code)]

use unit_enum::UnitEnum;

#[derive(UnitEnum)]
#[unit_enum(name = "AllType", derive(Clone, Copy, PartialEq, Debug))]
enum All<T> {
    Unit,
    Tuple(bool, i32),
    Struct { x: T, y: T },
}

const UNIT: AllType = AllType::Unit;
const TUPLE: AllType = AllType::Tuple;
const STRUCT: AllType = AllType::Struct;

fn main() {
    let all_unit: All<i32> = All::Unit;
    let actual: AllType = AllType::from(&all_unit);
    let expected: AllType = AllType::Unit;

    assert_eq!(actual, expected);
}
