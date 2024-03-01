/// **Note:** Use the alias `csi::macros::unit_def::si_associated_unit_def`.
/// 
/// ## Syntax
/// 
/// ```rust,ignore
/// si_associated_unit_def! {
///     alias visibility? mod alias_mod_name;
/// 
///     /// add docs here...
///     unit UnitName
///     based on BaseUnitType {
///         // unit static configuration
///         // see `csi::core::units::associated::SiAssociatedUnitDefinition`.
///     }
///     alias alias1, alias2, ... ?
///     conversion {
///         const? (#[attr]* ty1, #[attr]* ty2, ...) {
///             to: |val| { ... },
///             from: |val| { ... },
///         };
///         ...
///     };
/// 
///     ...
/// }
/// ```
#[macro_export]
macro_rules! __impl_si_associated_unit_def {
    (
        alias $vis:vis mod $mod:ident;
        $(
            $(#[$attr:meta])*
            unit $name:ident
            based on $ty:ty {
                $($key:ident: $val:expr),*
                $(,)?
            }
            $(alias $($al:ident),+)?
            $(conversion $block_tt:tt)?;
        )+
    ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $(#[$attr])*
            pub struct $name;
    
            impl $crate::core::units::associated::SiAssociatedUnit for $name {
                type BaseUnit = $ty;
                const DEF: $crate::core::units::associated::SiAssociatedUnitDefinition =
                    $crate::core::units::associated::SiAssociatedUnitDefinition {
                        $($key: $val),*
                    };
            }
    
            impl $crate::core::units::any::SiDefinedUnit for $name {
                const DEF: ::core::option::Option<$crate::core::units::any::SiDefinedUnitDefinition> =
                    ::core::option::Option::Some($crate::core::units::any::SiDefinedUnitDefinition {
                        full_name: <$name as $crate::core::units::associated::SiAssociatedUnit>::DEF.full_name,
                        short_name: <$name as $crate::core::units::associated::SiAssociatedUnit>::DEF.short_name,
                        unit_symbol: <$name as $crate::core::units::associated::SiAssociatedUnit>::DEF.unit_symbol,
                    });
            }
    
            impl $crate::core::units::any::SiAnyUnit for $name {}

            $($crate::__impl_unit_conversion! { $name $block_tt; })?
        )+
        
        $($(
            $vis mod $mod {
                #[allow(non_camel_case_types)]
                $(pub type $al = super::$name;)+
            }
        )?)+
    };
}

/// **Note:** Use the alias `csi::macros::unit_def::si_exported_unit_def`.
/// 
/// ## Syntax
/// 
/// ```rust,ignore
/// si_exported_unit_def! {
///     alias visibility? mod alias_mod_name;
/// 
///     /// add docs here...
///     unit UnitName
///     based on BaseUnitType {
///         // unit static configuration
///         // see `csi::core::units::associated::SiAssociatedUnitDefinition`.
///     }
///     alias alias1, alias2, ... ?;
/// 
///     ...
/// }
/// ```
#[macro_export]
macro_rules! __impl_si_exported_unit_def {
    (
        alias $vis:vis mod $mod:ident;
        $(
            $(#[$attr:meta])*
            unit $name:ident based on $ty:ty {
                $($key:ident: $val:expr),*
                $(,)?
            } $(alias $($al:ident),+ $(,)?)?;
        )+
    ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $(#[$attr])*
            pub struct $name;

            impl $crate::core::units::exported::SiExportedUnit for $name {
                type BaseUnit = $ty;
                const DEF: $crate::core::units::exported::SiExportedUnitDefinition =
                    $crate::core::units::exported::SiExportedUnitDefinition {
                        $($key: $val),*
                    };
            }

            impl $crate::core::units::any::SiDefinedUnit for $name {
                const DEF: ::core::option::Option<$crate::core::units::any::SiDefinedUnitDefinition> =
                    ::core::option::Option::Some($crate::core::units::any::SiDefinedUnitDefinition {
                        full_name: <$name as $crate::core::units::exported::SiExportedUnit>::DEF.full_name,
                        short_name: <$name as $crate::core::units::exported::SiExportedUnit>::DEF.short_name,
                        unit_symbol: <$name as $crate::core::units::exported::SiExportedUnit>::DEF.unit_symbol,
                    });
            }

            impl $crate::core::units::any::SiOpsUnit for $name {
                const UNIT_MAP: $crate::core::units::base::BaseUnitMap =
                    <$name as $crate::core::units::exported::SiExportedUnit>::BaseUnit::UNIT_MAP;
            }

            impl $crate::core::units::any::SiAnyUnit for $name {}
        )+
        
        $($(
            $vis mod $mod {
                #[allow(non_camel_case_types)]
                $(pub type $al = super::$name;)+
            }
        )?)+
    };
}