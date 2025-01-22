// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/arrows2d.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: 2D arrows with optional colors, radii, labels, etc.
///
/// ## Example
///
/// ### Simple batch of 2D arrows
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_arrow2d").spawn()?;
///
///     rec.log(
///         "arrows",
///         &rerun::Arrows2D::from_vectors([[1.0, 0.0], [0.0, -1.0], [-0.7, 0.7]])
///             .with_radii([0.025])
///             .with_origins([[0.25, 0.0], [0.25, 0.0], [-0.1, -0.1]])
///             .with_colors([[255, 0, 0], [0, 255, 0], [127, 0, 255]])
///             .with_labels(["right", "up", "left-down"]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/1200w.png">
///   <img src="https://static.rerun.io/arrow2d_simple/59f044ccc03f7bc66ee802288f75706618b29a6e/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Arrows2D {
    /// All the vectors for each arrow in the batch.
    pub vectors: Option<SerializedComponentBatch>,

    /// All the origin (base) positions for each arrow in the batch.
    ///
    /// If no origins are set, (0, 0) is used as the origin for each arrow.
    pub origins: Option<SerializedComponentBatch>,

    /// Optional radii for the arrows.
    ///
    /// The shaft is rendered as a line with `radius = 0.5 * radius`.
    /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
    pub radii: Option<SerializedComponentBatch>,

    /// Optional colors for the points.
    pub colors: Option<SerializedComponentBatch>,

    /// Optional text labels for the arrows.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    pub labels: Option<SerializedComponentBatch>,

    /// Optional choice of whether the text labels should be shown by default.
    pub show_labels: Option<SerializedComponentBatch>,

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<SerializedComponentBatch>,

    /// Optional class Ids for the points.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    pub class_ids: Option<SerializedComponentBatch>,
}

impl Arrows2D {
    /// Returns the [`ComponentDescriptor`] for [`Self::vectors`].
    #[inline]
    pub fn descriptor_vectors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Vector2D".into(),
            archetype_field_name: Some("vectors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::origins`].
    #[inline]
    pub fn descriptor_origins() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Position2D".into(),
            archetype_field_name: Some("origins".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::radii`].
    #[inline]
    pub fn descriptor_radii() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Radius".into(),
            archetype_field_name: Some("radii".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::colors`].
    #[inline]
    pub fn descriptor_colors() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Color".into(),
            archetype_field_name: Some("colors".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::labels`].
    #[inline]
    pub fn descriptor_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Text".into(),
            archetype_field_name: Some("labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::show_labels`].
    #[inline]
    pub fn descriptor_show_labels() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.ShowLabels".into(),
            archetype_field_name: Some("show_labels".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::draw_order`].
    #[inline]
    pub fn descriptor_draw_order() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.DrawOrder".into(),
            archetype_field_name: Some("draw_order".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::class_ids`].
    #[inline]
    pub fn descriptor_class_ids() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.ClassId".into(),
            archetype_field_name: Some("class_ids".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.Arrows2D".into()),
            component_name: "rerun.components.Arrows2DIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [Arrows2D::descriptor_vectors()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Arrows2D::descriptor_origins(),
            Arrows2D::descriptor_indicator(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Arrows2D::descriptor_radii(),
            Arrows2D::descriptor_colors(),
            Arrows2D::descriptor_labels(),
            Arrows2D::descriptor_show_labels(),
            Arrows2D::descriptor_draw_order(),
            Arrows2D::descriptor_class_ids(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            Arrows2D::descriptor_vectors(),
            Arrows2D::descriptor_origins(),
            Arrows2D::descriptor_indicator(),
            Arrows2D::descriptor_radii(),
            Arrows2D::descriptor_colors(),
            Arrows2D::descriptor_labels(),
            Arrows2D::descriptor_show_labels(),
            Arrows2D::descriptor_draw_order(),
            Arrows2D::descriptor_class_ids(),
        ]
    });

impl Arrows2D {
    /// The total number of components in the archetype: 1 required, 2 recommended, 6 optional
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`Arrows2D`] [`::re_types_core::Archetype`]
pub type Arrows2DIndicator = ::re_types_core::GenericIndicatorComponent<Arrows2D>;

impl ::re_types_core::Archetype for Arrows2D {
    type Indicator = Arrows2DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Arrows2D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Arrows 2D"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: Arrows2DIndicator = Arrows2DIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentDescriptor, arrow::array::ArrayRef)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_descr: ::nohash_hasher::IntMap<_, _> = arrow_data.into_iter().collect();
        let vectors = arrays_by_descr
            .get(&Self::descriptor_vectors())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_vectors()));
        let origins = arrays_by_descr
            .get(&Self::descriptor_origins())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_origins()));
        let radii = arrays_by_descr
            .get(&Self::descriptor_radii())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_radii()));
        let colors = arrays_by_descr
            .get(&Self::descriptor_colors())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_colors()));
        let labels = arrays_by_descr
            .get(&Self::descriptor_labels())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_labels()));
        let show_labels = arrays_by_descr
            .get(&Self::descriptor_show_labels())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_show_labels())
            });
        let draw_order = arrays_by_descr
            .get(&Self::descriptor_draw_order())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_draw_order())
            });
        let class_ids = arrays_by_descr
            .get(&Self::descriptor_class_ids())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_class_ids())
            });
        Ok(Self {
            vectors,
            origins,
            radii,
            colors,
            labels,
            show_labels,
            draw_order,
            class_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Arrows2D {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.vectors.clone(),
            self.origins.clone(),
            self.radii.clone(),
            self.colors.clone(),
            self.labels.clone(),
            self.show_labels.clone(),
            self.draw_order.clone(),
            self.class_ids.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for Arrows2D {}

impl Arrows2D {
    /// Create a new `Arrows2D`.
    #[inline]
    pub(crate) fn new(
        vectors: impl IntoIterator<Item = impl Into<crate::components::Vector2D>>,
    ) -> Self {
        Self {
            vectors: try_serialize_field(Self::descriptor_vectors(), vectors),
            origins: None,
            radii: None,
            colors: None,
            labels: None,
            show_labels: None,
            draw_order: None,
            class_ids: None,
        }
    }

    /// Update only some specific fields of a `Arrows2D`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `Arrows2D`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            vectors: Some(SerializedComponentBatch::new(
                crate::components::Vector2D::arrow_empty(),
                Self::descriptor_vectors(),
            )),
            origins: Some(SerializedComponentBatch::new(
                crate::components::Position2D::arrow_empty(),
                Self::descriptor_origins(),
            )),
            radii: Some(SerializedComponentBatch::new(
                crate::components::Radius::arrow_empty(),
                Self::descriptor_radii(),
            )),
            colors: Some(SerializedComponentBatch::new(
                crate::components::Color::arrow_empty(),
                Self::descriptor_colors(),
            )),
            labels: Some(SerializedComponentBatch::new(
                crate::components::Text::arrow_empty(),
                Self::descriptor_labels(),
            )),
            show_labels: Some(SerializedComponentBatch::new(
                crate::components::ShowLabels::arrow_empty(),
                Self::descriptor_show_labels(),
            )),
            draw_order: Some(SerializedComponentBatch::new(
                crate::components::DrawOrder::arrow_empty(),
                Self::descriptor_draw_order(),
            )),
            class_ids: Some(SerializedComponentBatch::new(
                crate::components::ClassId::arrow_empty(),
                Self::descriptor_class_ids(),
            )),
        }
    }

    /// Partitions the component data into multiple sub-batches.
    ///
    /// Specifically, this transforms the existing [`SerializedComponentBatch`]es data into [`SerializedComponentColumn`]s
    /// instead, via [`SerializedComponentBatch::partitioned`].
    ///
    /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
    ///
    /// The specified `lengths` must sum to the total length of the component batch.
    ///
    /// [`SerializedComponentColumn`]: [::re_types_core::SerializedComponentColumn]
    #[inline]
    pub fn columns<I>(
        self,
        _lengths: I,
    ) -> SerializationResult<impl Iterator<Item = ::re_types_core::SerializedComponentColumn>>
    where
        I: IntoIterator<Item = usize> + Clone,
    {
        let columns = [
            self.vectors
                .map(|vectors| vectors.partitioned(_lengths.clone()))
                .transpose()?,
            self.origins
                .map(|origins| origins.partitioned(_lengths.clone()))
                .transpose()?,
            self.radii
                .map(|radii| radii.partitioned(_lengths.clone()))
                .transpose()?,
            self.colors
                .map(|colors| colors.partitioned(_lengths.clone()))
                .transpose()?,
            self.labels
                .map(|labels| labels.partitioned(_lengths.clone()))
                .transpose()?,
            self.show_labels
                .map(|show_labels| show_labels.partitioned(_lengths.clone()))
                .transpose()?,
            self.draw_order
                .map(|draw_order| draw_order.partitioned(_lengths.clone()))
                .transpose()?,
            self.class_ids
                .map(|class_ids| class_ids.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        let indicator_column =
            ::re_types_core::indicator_column::<Self>(_lengths.into_iter().count())?;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// All the vectors for each arrow in the batch.
    #[inline]
    pub fn with_vectors(
        mut self,
        vectors: impl IntoIterator<Item = impl Into<crate::components::Vector2D>>,
    ) -> Self {
        self.vectors = try_serialize_field(Self::descriptor_vectors(), vectors);
        self
    }

    /// All the origin (base) positions for each arrow in the batch.
    ///
    /// If no origins are set, (0, 0) is used as the origin for each arrow.
    #[inline]
    pub fn with_origins(
        mut self,
        origins: impl IntoIterator<Item = impl Into<crate::components::Position2D>>,
    ) -> Self {
        self.origins = try_serialize_field(Self::descriptor_origins(), origins);
        self
    }

    /// Optional radii for the arrows.
    ///
    /// The shaft is rendered as a line with `radius = 0.5 * radius`.
    /// The tip is rendered with `height = 2.0 * radius` and `radius = 1.0 * radius`.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = try_serialize_field(Self::descriptor_radii(), radii);
        self
    }

    /// Optional colors for the points.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = try_serialize_field(Self::descriptor_colors(), colors);
        self
    }

    /// Optional text labels for the arrows.
    ///
    /// If there's a single label present, it will be placed at the center of the entity.
    /// Otherwise, each instance will have its own label.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = try_serialize_field(Self::descriptor_labels(), labels);
        self
    }

    /// Optional choice of whether the text labels should be shown by default.
    #[inline]
    pub fn with_show_labels(
        mut self,
        show_labels: impl Into<crate::components::ShowLabels>,
    ) -> Self {
        self.show_labels = try_serialize_field(Self::descriptor_show_labels(), [show_labels]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ShowLabels`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_show_labels`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_show_labels(
        mut self,
        show_labels: impl IntoIterator<Item = impl Into<crate::components::ShowLabels>>,
    ) -> Self {
        self.show_labels = try_serialize_field(Self::descriptor_show_labels(), show_labels);
        self
    }

    /// An optional floating point value that specifies the 2D drawing order.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    #[inline]
    pub fn with_draw_order(mut self, draw_order: impl Into<crate::components::DrawOrder>) -> Self {
        self.draw_order = try_serialize_field(Self::descriptor_draw_order(), [draw_order]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::DrawOrder`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_draw_order`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_draw_order(
        mut self,
        draw_order: impl IntoIterator<Item = impl Into<crate::components::DrawOrder>>,
    ) -> Self {
        self.draw_order = try_serialize_field(Self::descriptor_draw_order(), draw_order);
        self
    }

    /// Optional class Ids for the points.
    ///
    /// The [`components::ClassId`][crate::components::ClassId] provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = try_serialize_field(Self::descriptor_class_ids(), class_ids);
        self
    }
}

impl ::re_byte_size::SizeBytes for Arrows2D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.vectors.heap_size_bytes()
            + self.origins.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.show_labels.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
    }
}
