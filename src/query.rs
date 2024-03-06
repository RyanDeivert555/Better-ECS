use std::cell::{Ref, RefMut};
use crate::{
    component::{Component, EntityId},
    world::World,
};

// TODO: add QueryWithoutTrait
pub trait Query {
    type Output<'lt>;
    type OutputMut<'lt>;

    fn query(world: &World, key: EntityId) -> Option<Self::Output<'_>>;
    fn query_mut(world: &World, key: EntityId) -> Option<Self::OutputMut<'_>>;
}

impl<T> Query for T
where
    T: Component + 'static,
{
    type Output<'lt> = Ref<'lt, T>;
    type OutputMut<'lt> = RefMut<'lt, T>;

    fn query(world: &World, key: EntityId) -> Option<Self::Output<'_>> {
        world.get_component::<T>(key)
    }

    fn query_mut(world: &World, key: EntityId) -> Option<Self::OutputMut<'_>> {
        world.get_component_mut::<T>(key)
    }
}

#[macro_export]
macro_rules! __impl_query {
	($($generic_type:ident),+) => {
		impl<$($generic_type),*> $crate::query::Query for ($($generic_type,)*)
		where
		$(
			$generic_type: $crate::component::Component + 'static,
		)*
		{
			type Output<'lt> = ($(::core::cell::Ref<'lt, $generic_type>,)*);
			type OutputMut<'lt> = ($(::core::cell::RefMut<'lt, $generic_type>,)*);

			fn query(world: &$crate::world::World, key: $crate::component::EntityId) -> ::core::option::Option<Self::Output<'_>> {
				Some(
					(
						$(world.get_component::<$generic_type>(key)?, )*
					)
				)
			}

			fn query_mut(world: &$crate::world::World, key: $crate::component::EntityId) -> ::core::option::Option<Self::OutputMut<'_>> {
				Some(
					(
						$(world.get_component_mut::<$generic_type>(key)?, )*
					)
				)
			}
		}
	};
}

__impl_query!(T1, T2);
__impl_query!(T1, T2, T3);
__impl_query!(T1, T2, T3, T4);
__impl_query!(T1, T2, T3, T4, T5);
__impl_query!(T1, T2, T3, T4, T5, T6);
__impl_query!(T1, T2, T3, T4, T5, T6, T7);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
__impl_query!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
