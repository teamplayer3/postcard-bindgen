declare type u8 = number
declare type u16 = number
declare type u32 = number
declare type u64 = number
declare type u128 = number
declare type usize = number
declare type i8 = number
declare type i16 = number
declare type i32 = number
declare type i64 = number
declare type i128 = number
declare type isize = number

export type Test = { field: u8 }

export type Type = "Test"
declare type ValueType<T extends Type> = T extends "Test" ? Test : void

export function serialize<T extends Type>(type: T, value: ValueType<T>): u8[]
export function deserialize<T extends Type>(type: T, bytes: u8[]): ValueType<T>
