
#[macro_export]
macro_rules! impl_sem_val {
    (
        $key_ty:ident,
        $val_ty:ident,
        $key_ty_name:ident { $($key_field:ident: self.$($field_ident:ident$($(::<$field_ty:ty>)?($($field_tt:tt)*))?).+),* }
    ) => {
        impl PartialEq for $val_ty {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for $val_ty {}

        impl std::hash::Hash for $val_ty {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }

        impl crate::data::semantics::Sem for $val_ty {
            fn logical_name(&self) -> String {
                crate::data::semantics::SemVal::to_key(self).logical_name()
            }

            fn description(&self) -> String {
                crate::data::semantics::SemVal::to_key(self).description()
            }
        }

        impl crate::data::semantics::SemVal<$key_ty> for $val_ty {
            fn to_key(&self) -> $key_ty {
                $key_ty_name {
                    $($key_field: self.$($field_ident$($(::<$field_ty>)?($($field_tt)*))?).+),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_sem_key {
    (
        $key_ty:ident,
        $val_ty:ident,
        $store_name:ident
    ) => {
        impl crate::data::semantics::SemKey<$val_ty> for $key_ty {
            fn get_val(
                &self, ctx: &crate::data::context::SemContext,
            ) -> anyhow::Result<std::rc::Rc<$val_ty>> {
                ctx.$store_name.get(self)
            }
        }
    };
}
