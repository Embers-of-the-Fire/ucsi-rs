use datastruct::DataStruct;
use ucsi::{
    core::format::ufmt::{
        SiDefault, SiDefaultOption, SiFormatter, SiFormatterOption, SiLatex, SiLatexOption,
    },
    units::{
        base::{kg, m, s},
        exported::force::Newton,
    },
    val,
};

#[test]
fn test_format_unit() {
    let speed = val!(10.0 * (m / s));
    let time = val!(1.0 * s);
    let acc = speed / time;
    let mass = val!(1.0 * kg);
    let force = acc * mass;
    let checked_force = force.cast_const::<Newton>();

    let unit = checked_force.fmt_unit::<SiDefault>(SiDefaultOption::data_default());
    println!("{}", unit);
    let unit = checked_force.fmt_unit::<SiLatex>(
        SiLatexOption::default()
            .with_show_all_exponent(true)
            .with_ignore_zero(false),
    );
    println!("{}", unit);
    let unit = checked_force.fmt_unit::<SiFormatter>(
        SiFormatterOption::data_default()
            .with_exponent_seperator("**")
            .with_exponent_symbol_wrapper(("{", "}")),
    );
    println!("{}", unit);
}
