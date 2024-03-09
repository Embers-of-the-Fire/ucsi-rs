#![feature(type_alias_impl_trait)]
#![allow(unused_variables)]

use std::error::Error;

use ucsi::{
    core::{
        units::{
            base::{kg, m, s},
            exported::force::Newton,
        },
        value::Value,
    },
    macros::cast::{cast_si_value, infer_cast_si_value},
    unit, units::any::{SiOpsUnit, CastFrom}, val,
};

#[test]
fn test_ucsi_ops() {
    let speed = val!(10.0 * (m / s));
    let time = val!(1.0 * s);
    let acc = speed / time;
    let mass = val!(1.0 * kg);
    let force = acc * mass;
    let checked_force = cast_si_value!(force => unit!(((m / s) / s) * kg) as Newton);
    // println!("{}", checked_force.display_si().unwrap());
    let checked_force = infer_cast_si_value!(force => Newton);
    // println!("{}", checked_force.display_si().unwrap());
    let checked_force = force.cast_const::<Newton>();
    // println!("{}", checked_force.display_si().unwrap());
    // println!("{}", checked_force.display_si_latex().unwrap());
    // println!("{}", checked_force.display_si_symbol_latex().unwrap());
}

#[test]
fn test_cast_ops() {
    fn can_cast<T: SiOpsUnit, B: SiOpsUnit + CastFrom<T>, V>(val: Value<V, T>) -> Value<V, B> {
        Value::new(val.value)
    }

    let speed: Value<f64, unit!(m / s)> = Value::new(10.0);
    let time: Value<f64, s> = Value::new(1.0);
    let acc = speed / time;
    let mass: Value<f64, kg> = Value::new(1.0);
    let force = acc * mass;
    let checked_force: Value<f64, s> = can_cast(force);
    // println!("{}", checked_force.display_si().unwrap());
}

#[test]
fn test_ops_display() -> Result<(), Box<dyn Error>> {
    let speed: Value<f64, unit!(m / s)> = Value::new(10.0);
    let time: Value<f64, s> = Value::new(1.0);
    let acc = speed / time;
    let mass: Value<f64, kg> = Value::new(1.0);
    let force = acc * mass;
    // let dis = force.display_symbol()?;
    // println!("{}", dis);
    Ok(())
}
