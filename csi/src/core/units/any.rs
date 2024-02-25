use super::base::BaseUnitMap;

pub struct SiDefinedUnitDefinition {
    pub full_name: &'static str,
    pub short_name: &'static str,
    pub unit_symbol: &'static str,
}

pub trait SiDefinedUnit: SiAnyUnit {
    const DEF: SiDefinedUnitDefinition;
}

pub trait SiOpsUnit: SiAnyUnit {
    const UNIT_MAP: BaseUnitMap;
}

pub trait SiAnyUnit {}

pub const fn is_same_type<T: SiOpsUnit, U: SiOpsUnit>() -> bool {
    T::UNIT_MAP.simplify().eq(&U::UNIT_MAP.simplify())
}

pub const fn is_same_type_or_panic<T: SiOpsUnit, U: SiOpsUnit>() {
    if !is_same_type::<T, U>() {
        panic!("cannot cast si type")
    }
}

pub trait CastFrom<T: SiOpsUnit> {
    /// This should be theoretically `true` and should never be `false`.
    const CAN_CAST_FROM: bool;
}

impl<T: SiOpsUnit, B: SiOpsUnit> CastFrom<T> for B {
    const CAN_CAST_FROM: bool = {
        is_same_type_or_panic::<T, B>();
        true
    };
}

pub trait CastTo<B: SiOpsUnit> {
    /// This should be theoretically `true` and should never be `false`.
    const CAN_CAST_TO: bool;
}

impl<T: SiOpsUnit, B: SiOpsUnit + CastFrom<T>> CastTo<B> for T {
    const CAN_CAST_TO: bool = <B as CastFrom<T>>::CAN_CAST_FROM;
}
