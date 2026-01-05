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

#[macro_export]
macro_rules! generate_getters {
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
            paste::paste! {
                impl $struct_name {
                    $(
                        fn [<get_ $field_name>](&self) -> &$field_type {
                            &self.$field_name
                        }
                    )*
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! generate_setters {
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
            paste::paste! {
                impl $struct_name {
                    $(
                        fn [<set_ $field_name>](&mut self, value: $field_type) {
                            self.$field_name = value;
                        }
                    )*
                }
            }
        )*
    };
}
