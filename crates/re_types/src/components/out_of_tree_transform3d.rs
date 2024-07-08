// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/out_of_tree_transform3d.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: An out-of-tree affine transform between two 3D spaces, represented in a given direction.
///
/// "Out-of-tree" means that the transform only affects its own entity: children don't inherit from it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutOfTreeTransform3D(
    /// Representation of the transform.
    pub crate::datatypes::Transform3D,
);

impl ::re_types_core::SizeBytes for OutOfTreeTransform3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Transform3D>::is_pod()
    }
}

impl<T: Into<crate::datatypes::Transform3D>> From<T> for OutOfTreeTransform3D {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Transform3D> for OutOfTreeTransform3D {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Transform3D {
        &self.0
    }
}

impl std::ops::Deref for OutOfTreeTransform3D {
    type Target = crate::datatypes::Transform3D;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Transform3D {
        &self.0
    }
}

impl std::ops::DerefMut for OutOfTreeTransform3D {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Transform3D {
        &mut self.0
    }
}

::re_types_core::macros::impl_into_cow!(OutOfTreeTransform3D);

impl ::re_types_core::Loggable for OutOfTreeTransform3D {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.OutOfTreeTransform3D".into()
    }

    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        crate::datatypes::Transform3D::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Transform3D::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::datatypes::Transform3D::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}
