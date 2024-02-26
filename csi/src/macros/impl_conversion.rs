/// **Note:** Use the alias `csi::macros::conversion::unit_conversion`.
/// 
/// ## Syntax
/// 
/// ```rust,ignore
/// unit_conversion! {
///     Type {
///         const? (ty1, ty2, ...) {
///             to: |val| { ... },
///             from: |val| { ... },
///         };
///         ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! __impl_unit_conversion {
    {
        $($ty:ty {
            $($($idt:ident)? ($($(#[$attr:meta])* $ity:ty),+) $block_tt:tt;)*
        };)*
    } => {
        $($(
            $crate::__impl_unit_conversion! {$ty| $($idt)? ($($(#[$attr])* $ity),+) $block_tt}
        )*)*
    };
    
    ($ty:ty| const ($($(#[$attr:meta])* $impl_ty:ty),+) $block_tt:tt) => {
        $($crate::__impl_unit_conversion! {
            $(#[$attr])* 
            @const $impl_ty as $ty $block_tt
        })+
    };
    ($ty:ty| ($($(#[$attr:meta])* $impl_ty:ty),+) $block_tt:tt) => {
        $($crate::__impl_unit_conversion! {
            $(#[$attr])* 
            @ $impl_ty as $ty $block_tt
        })+
    };

    (
        $(#[$attr:meta])* 
        @$($idt:ident)? $vtype:ty as $unit:ty {
            from: |$idtf:ident| $blockf:block,
            to: |$idtt:ident| $blockt:block $(,)?
        }
    ) => {
        $(#[$attr])* 
        impl $crate::core::value::Value<$vtype, $unit> {
            pub $($idt)? fn to_metric(self)
                -> $crate::core::value::Value<
                    $vtype,
                    <$unit as $crate::core::units::associated::SiAssociatedUnit>::BaseUnit
                >
            {
                let $idtt = self.value;
                let value = $blockt;
                $crate::core::value::Value::new(value)
            }

            pub $($idt)? fn from_metric(
                val: $crate::core::value::Value<
                    $vtype,
                    <$unit as $crate::core::units::associated::SiAssociatedUnit>::BaseUnit
                >
            )
                -> $crate::core::value::Value<
                    $vtype,
                    <$unit as $crate::core::units::associated::SiAssociatedUnit>::BaseUnit
                >
            {
                let $idtf = val.value;
                let value = $blockf;
                $crate::core::value::Value::new(value)
            }
        }
    };
    (
        $(#[$attr:meta])* 
        @$($idt:ident)? $vtype:ty as $unit:ty {
            to: |$idtt:ident| $blockt:block,
            from: |$idtf:ident| $blockf:block $(,)?
        }
    ) => {
        $(#[$attr])* 
        impl $crate::core::value::Value<$vtype, $unit> {
            pub $($idt)? fn to_metric(self)
                -> $crate::core::value::Value<
                    $vtype,
                    <$unit as $crate::core::units::associated::SiAssociatedUnit>::BaseUnit
                >
            {
                let $idtt = self.value;
                let value = $blockt;
                $crate::core::value::Value::new(value)
            }

            pub $($idt)? fn from_metric(
                val: $crate::core::value::Value<
                    $vtype,
                    <$unit as $crate::core::units::associated::SiAssociatedUnit>::BaseUnit
                >
            )
                -> $crate::core::value::Value<
                    $vtype,
                    <$unit as $crate::core::units::associated::SiAssociatedUnit>::BaseUnit
                >
            {
                let $idtf = val.value;
                let value = $blockf;
                $crate::core::value::Value::new(value)
            }
        }
    };
}
