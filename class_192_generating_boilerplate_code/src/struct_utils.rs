#[macro_export]
macro_rules! make_struct_with_new {
    (
        $(struct $struct_name:ident {
                $(
                    $field_name:ident : $field_type:ty
                ),*,  $(
                    ,
                )?
            }
        ),*
    ) => {
        $(
            struct $struct_name {
                $(
                    $field_name: $field_type,
                )*
            }

            impl $struct_name {
                fn new($($field_name: $field_type),*) -> Self {
                    Self {
                        $(
                            $field_name,
                        )*
                    }
                }
            }
        )*
    };
}
