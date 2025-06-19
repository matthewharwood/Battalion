use surrealdb::sql::Thing;

/// Trait to convert Option<Thing> fields to string
use std::ops::Deref;

pub trait IdToString {
    fn id_string(&self) -> String;
}

/// Works for anything that derefs to `Option<Thing>`
/// (`&Option<Thing>`, `Box<Option<Thing>>`, etc.)
pub fn impl_id_to_string<T>(obj: T) -> String
where
    T: Deref<Target = Option<Thing>>,
{
    match obj.deref() {
        Some(thing) => thing.to_string(),
        None => String::new(),
    }
}

#[macro_export]
macro_rules! impl_id_to_string_for {
    // variant 1: field is literally `id`
    ($ty:ty) => {
        impl $crate::IdToString for $ty {
            fn id_string(&self) -> String {
                $crate::impl_id_to_string(&self.id)
            }
        }
    };
    // variant 2: caller specifies the field name
    ($ty:ty, $field:ident) => {
        impl $crate::IdToString for $ty {
            fn id_string(&self) -> String {
                $crate::impl_id_to_string(&self.$field)
            }
        }
    };
}