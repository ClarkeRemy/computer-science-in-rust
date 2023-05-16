// this s a comment
// anything after a "//" is a comment

/* this is an inline comment */
// anything between "/*" and "*/"" is a comment

// Despite a desire to be explicit, if we take nothing for granted,
// it will be dificult to get off of the ground.
// The following compiler lints are in place
#![allow(dead_code)]            // this is not production code
#![allow(redundant_semicolons)] // the code is not idiomatically formatted

// The following compiler declarations are in place
#![no_implicit_prelude] // Access to the Standard library will be made more explicit
extern crate core;  // this assumes little to nothing,
extern crate alloc; //  "    "      an allocator is present,
extern crate std;   //  "    "      we are in an operating system.
// `extern` and `crate` are keywords.
// Keywords are reserved by the compiler, so your names must not be keywords 


// This is a trivial function.
// it has no parameters, so it cannot take arguments
// The `{}` is a code block. The behavior of the function is here.
fn noop( /* where parameters go */) -> () /* Return type */ {/* where computation goes */}

// a function can be called by following its name it with `()`
fn calls_noop() { noop() }

// Functions have return values.
// `trivial_function` does in fact have a return value, but it was omitted.
// In Rust we need to be explicit about what those return values should be,
// This is done with types. The most basic type is the unit type `()`.
// the unit type has one variant value.

// Here is a type alias for the unit type.
type Unit = (); // with this I can write Unit any where a `()` type declaration is expected

fn noop_implicit()         {} // implicitly returns `()`
fn noop_too()      -> ()   {} // explicitly returns `()`
fn also_noop()     -> Unit {} // using an alias is the same

// we could  define a function that takes an argument now that we have types.
fn takes_unit( u : Unit ) -> Unit { u }
fn uses_takes_unit() {takes_unit( noop() )}

// a more interesting argument to take is a function as a value
fn takes_procedure( p : fn() ) { p()}
fn takes_procedure_explicit(p : fn() -> () ) { p() } // equivalent


// note how we pass noop into takes_procedure
fn uses_takes_procedure() { takes_procedure( noop ) }

// we can use `;` to sequence 
fn sequence_noops() { noop() ; takes_procedure( noop ) }

// we can organize code into modules
// to use the code from outside the module, it must be preceeded by the pub keyword.
// the name can then be found with a path, separated by `::`
mod my_module
{ pub fn noop() {} }
fn uses_module() { my_module::noop() }

// Sometimes nested module names can get long
mod this { pub mod is { pub mod long { pub fn nested_noop() {} } } }
// a `use` can mke this more managable
fn uses_use() 
{ use this::is::long::nested_noop
; use this::is::long::nested_noop as n_loop
; nested_noop()
; n_loop()
}

// A function that returns `()` only communicates to the caller that it finished.
// if the function actualy executes a process in the real world,
// it can start to make sense why we would care, take this for example.
fn call_twice_delay( f : fn() ) 
{ f() 
; std::thread::park()
; f() 
}
// Note how we did not pass the operating system as an argument
// We took the operating system for granted, as though it was globally accessible.
// This means we are depending on side effects.
// All we can conclude is that each proceedure will complete one after another.
// If the program is to be treated like a process,
// this is fine
fn fire_the_missiles() {/* firing missiles */}
fn use_call_twice_delay_procedure() { call_twice_delay( fire_the_missiles ) }

// but if it is math, it does not represent mathematical functions well, which are a mapping of sets
fn increment_all_values_in_matrix_by_1() 
{/* somehow this function has a way to change a matrix you care about */}
fn use_call_twice_delay_math() 
{ call_twice_delay( increment_all_values_in_matrix_by_1 ) }

// In general a prcedure is sequence of operations.
// A function is a mapping from one set of values to another.
// Functions can be implemented with procedures.
// Rust functions are actually procedures.
// Informally, if we talk about procedures, we mean functions that return `()`
// or never return.

// But what is function that can never return?
// here is a function that never returns
fn loops() { loop {} }

// Technically I could declare a return value.
// this type checks, but not for the reason you might expect.
fn loops2() -> () { loop {} } 

// What is really going is that `loop {}` is an expression of type `!`, the never type.
fn loops3() -> ! { loop {} }
// you can't type alias the never type as of writing this
// type Never = !;

// since it will never return, 
// it type checks for any type that is waiting for the return value

// if we try to return it will not compile
// fn loops3() -> ! { loop {return} }

// functions that abort the program without returning from `main`
// also return `!`
fn aborts() -> ! {std::process::abort()}

// What is important is that the never type is uninhabited;
// This means you can't actually create a value of type never.
// It is possible to define a type similar to the never type
// with unimhabited sum type. Rust can do this with the enum declaration
enum Never {} // this is a user defined type
fn never() -> Never { loop{} }

// note however that it is not the same as the `!` type
fn returns_unit1() { takes_unit({ never(); }) } // this will type check only if we use `;`, 
fn returns_unit2() { takes_unit( loops()) }     // this one just works

// uninhabited types might seem silly, but they can actually be very useful.
// they will be expanded upon later

// Another useful type is `bool`
// `bool` can only be `true` or `false`
// as `bool` is a built-in type, `true and `false` are keywords.
type Boolean = bool;

// inside of a code block we can place a `bool` expression
fn returns_true()  -> bool { true  /* the expression resolving to `true */ }
fn returns_false() -> bool { false /*  ...  */ }

// `bool` has many language items, that expect boolean values
// such as if
fn uses_if(b : bool) -> bool { if b { true } else { false } }
fn not(b : bool) -> bool { if b { false } else { true } }

// `if` is an expression so the `else` is required, 
// to ensure that some value of the correct type made regardless
// an exception is when the type of the `if` expression is `()`
// then the else can be omitted
fn run_it(r : fn(), answer : bool ) { if answer { r() } }

// another exception is an early `return`, 
// this combination is called a guard clause
fn it_runs(r : fn()->bool , answer : bool ) -> bool
{ if answer { return r() } 
; false
}

// All the code we have written so far is compiling but not actually running.
// a program needs an entry point, `main` is the default entry point when 
// running inside of an operating system.
fn main() { fire_the_missiles() }
// when we talk about running a usable binary, we will use a `main` procedure.

// But, for describing isolated functions, 
// having test functions is more condusive to a declarative explaination.

// here is a test procedure
#[test] fn a_test_function() {}

// This `#[test]` atrribute tells the `cargo` build system that this is a test
// to be run.

// code may also be given the `#[cfg(test)]` attribute.
// This is only an optimization. 
// When the code does not run tests, 
// the compilation of the attributed code is skipped, keeping compilation fast.
// It is safe to ignore.
#[test] #[cfg(test)] fn a_test_only_function() {}

// a test function expects to either finish a code block returning unit, 
// or to panic on purpose, or by accident


// We will make use of some the standard library macros.
#[test] #[cfg(test)] fn an_actual_test() 
{ if false { core::panic!() } 
; core::assert!(true)
; core::assert!(true, "using these macros we can format messages to describe what went wrong")
; core::assert_eq!(true, true)
; core::assert_eq!(true, true,  "Rust functions have a fixed number of arguments")
; core::assert_eq!(true, false)
; core::assert_eq!(true, false, "Rust Macros can have a variable number, {}ly", true)
}

// tests that are meant to panic can only be done in 
// documentation comments like so
// note that it cannot be done with `#[cfg(test)]` attribute
mod will_panic 
{
//! ```should_panic
//! core::panic!()
//! ```
// internal documetation
}