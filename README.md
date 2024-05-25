# The `ucsi` library

The `ucsi` library provides SI-based unit typing system
and zero-cost unit wrapper for values.

You can use the values of these packages for normal arithmetic
(since they overload the operators),
and then rust's generic engine and ucsi will help you derive their types.

Thanks to rust's powerful type system and static evaluator,
ucsi lets you know at compile time what your arithmetic violates,
and there's perfect support for constant arithmetic.

At the same time, you can easily create your own types
(see [`unit!`] macro and the [`macros`] module)
that work seamlessly with ucsi's type system.

## Quick start

```rust
use ucsi::units::base::{kg, m, s};
use ucsi::unit;
use ucsi::Value;

// create your new unit
// see the `unit` macro or `ucsi::core::ops` for more information.
type Newton = unit!((kg * m) / (s ** { 2 }));

// build some value
// `Value<value_type, value_unit>`
let speed: Value<f64, unit!(m / s)> = Value::new(10.0);
let time: Value<f64, s> = Value::new(1.0);
let acc = speed / time;
let mass: Value<f64, kg> = Value::new(1.0);
// this is `newton` in SI,
// but not now because it's currently typed as `((m / s) / s) * kg`
let force = acc * mass;
// lets cast it to newton
let checked_force: Value<_, Newton> =
    // note the `cast_const` here. `f64` is copy, so this cast is const-able.
    // If your value is not copy-able, use `cast` instead.
    force.cast_const();
```

Thanks to rust's const evaluation, the type-casting is **compile-time checked**!

To clearify the type-casting check, consider the following example:

```rust
let s_can_never_be_newton: Value<_, s> = force.cast_const();
```

The example above cannot compile, and will throw some error like this:

```text
error[E0080]: evaluation of `<Second as CastFrom<ucsi::ops::Mul<ucsi::ops::Div<ucsi::ops::Div<Meter, Second>, Second>, 
Kilogram>>>::CAN_CAST_FROM` failed
--> D:\WBH\rust\ucsi\ucsi\src\core\units\any.rs:25:9
   |
25 |         panic!("cannot cast si type")
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cannot cast si type', D:\WBH\rust\ucsi\ucsi\src\core\units\any.rs:25:9
   |
note: inside `is_same_type_or_panic::<ucsi::ops::Mul<ucsi::ops::Div<ucsi::ops::Div<Meter, Second>, Second>, Kilogram>, 
Second>`
--> D:\WBH\rust\ucsi\ucsi\src\core\units\any.rs:25:9
   |
25 |         panic!("cannot cast si type")
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside `<Second as CastFrom<ucsi::ops::Mul<ucsi::ops::Div<ucsi::ops::Div<Meter, Second>, Second>, Kilogram>>>::CAN_CAST_FROM`
--> D:\WBH\rust\ucsi\ucsi\src\core\units\any.rs:36:9
   |
36 |         is_same_type_or_panic::<T, B>();
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 = note: this error originates in the macro `$crate::panic::panic_2021` which comes from the expansion of the macro `panic` (in Nightly builds, run with -Z macro-backtrace for more info)

note: the above error was encountered while instantiating `fn ucsi::Value::<f64, ucsi::ops::Mul<ucsi::ops::Div<ucsi::ops::Div<ucsi::units::base::Meter, ucsi::units::base::Second>, ucsi::units::base::Second>, ucsi::units::base::Kilogram>>::cast_const::<ucsi::units::base::Second>`
--> ucsi-test\tests\test_ops.rs:26:25
   |
26 |     let s_can_never_be_newton: Value<_, s> = force.cast_const();
   |                                              ^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0080`.
```

Well, there's probably a little bit of a lot of information that's being reported.
Since rust's const evaluation and generic system is still under rapid development,
this could be improved in the future.

Anyway, at least we can check units at compile time.

## Dive into the library

You can find all provided built-in types in the [`units`] module.

If you want to create your well-defined special type or associated unit,
see the [`macros`] module for some code generator.

## Features

### `no_std` / `no_alloc`

This library offers no-std support and no-alloc support.

If `use_std` feature is not enabled, `String` and `format!` will be exported from `::alloc`.

If `use_alloc` feature is not enabled, string-based api like most formatter
cannot be used.

### External crate feature

#### `const_soft_float`

Disabled by default, included in the `full` feature.

This library re-exports the
[`const_soft_float`](https://docs.rs/crate/const_soft_float/0.1.4)
crate's constant float mathematical operations,
and the lib's standard associated units offer constant
conversion method for those types.

## License

This project is licensed under
the [MIT License](./LICENSE-MIT) or the [Apache 2.0 License](./LICENSE-APACHE).

## Contribute

This project is still in early development,
so there is no strict contribution process,
you can open issues or pull requests as you please!
