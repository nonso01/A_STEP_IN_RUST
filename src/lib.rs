#![allow(dead_code)]

// Hello here are brief notes of what
// i learn daily in  RUST
// this file is not intended to be ran or compiled, 
// but could be tested so as to be error free
// for now the compiler will yield se resolvable errors

const WHAT_IS_RUST: str = "Rust is a statically typed language like C, it is also used for the creation of a large spectrum of softwares from servers, database, embeded sys, to AI";

// Basic types in Rust are
// Scalar and Compund types

//  ------- Scalar Types -----
//
// Numbers, boolean, String 
//
// i8, i16, i32, i64, i128, isize ( smae for unsigned )
// bool for ( true/ false )
// String, str for string characters

// just like Js we make use of 
// ( let ) to declare an immutable variable
let x: i8 = 0;

// to make a variable mutable use
// ( mut )
let mut y: i16 = 0;
y = 200; // will  work just fine
