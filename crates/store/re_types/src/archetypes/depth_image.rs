// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/depth_image.fbs".

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

/// **Archetype**: A depth image, i.e. as captured by a depth camera.
///
/// Each pixel corresponds to a depth value in units specified by [`components::DepthMeter`][crate::components::DepthMeter].
///
/// ## Example
///
/// ### Depth to 3D example
/// ```ignore
/// use ndarray::{s, Array, ShapeBuilder};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_depth_image_3d").spawn()?;
///
///     let width = 300;
///     let height = 200;
///     let mut image = Array::<u16, _>::from_elem((height, width).f(), 65535);
///     image.slice_mut(s![50..150, 50..150]).fill(20000);
///     image.slice_mut(s![130..180, 100..280]).fill(45000);
///
///     let depth_image = rerun::DepthImage::try_from(image)?
///         .with_meter(10000.0)
///         .with_colormap(rerun::components::Colormap::Viridis);
///
///     // If we log a pinhole camera model, the depth gets automatically back-projected to 3D
///     rec.log(
///         "world/camera",
///         &rerun::Pinhole::from_focal_length_and_resolution(
///             [200.0, 200.0],
///             [width as f32, height as f32],
///         ),
///     )?;
///
///     rec.log("world/camera/depth", &depth_image)?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/1200w.png">
///   <img src="https://static.rerun.io/depth_image_3d/924e9d4d6a39d63d4fdece82582855fdaa62d15e/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DepthImage {
    /// The raw depth image data.
    pub buffer: Option<SerializedComponentBatch>,

    /// The format of the image.
    pub format: Option<SerializedComponentBatch>,

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    ///
    /// Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    /// In 3D views on the other hand, this affects where the points of the point cloud are placed.
    pub meter: Option<SerializedComponentBatch>,

    /// Colormap to use for rendering the depth image.
    ///
    /// If not set, the depth image will be rendered using the Turbo colormap.
    pub colormap: Option<SerializedComponentBatch>,

    /// The expected range of depth values.
    ///
    /// This is typically the expected range of valid values.
    /// Everything outside of the range is clamped to the range for the purpose of colormpaping.
    /// Note that point clouds generated from this image will still display all points, regardless of this range.
    ///
    /// If not specified, the range will be automatically estimated from the data.
    /// Note that the Viewer may try to guess a wider range than the minimum/maximum of values
    /// in the contents of the depth image.
    /// E.g. if all values are positive, some bigger than 1.0 and all smaller than 255.0,
    /// the Viewer will guess that the data likely came from an 8bit image, thus assuming a range of 0-255.
    pub depth_range: Option<SerializedComponentBatch>,

    /// Scale the radii of the points in the point cloud generated from this image.
    ///
    /// A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    /// if it is at the same depth, leaving no gaps.
    /// A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    ///
    /// TODO(#6744): This applies only to 3D views!
    pub point_fill_ratio: Option<SerializedComponentBatch>,

    /// An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
    ///
    /// Objects with higher values are drawn on top of those with lower values.
    pub draw_order: Option<SerializedComponentBatch>,
}

impl DepthImage {
    /// Returns the [`ComponentDescriptor`] for [`Self::buffer`].
    #[inline]
    pub fn descriptor_buffer() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.ImageBuffer".into(),
            archetype_field_name: Some("buffer".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::format`].
    #[inline]
    pub fn descriptor_format() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.ImageFormat".into(),
            archetype_field_name: Some("format".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::meter`].
    #[inline]
    pub fn descriptor_meter() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.DepthMeter".into(),
            archetype_field_name: Some("meter".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::colormap`].
    #[inline]
    pub fn descriptor_colormap() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.Colormap".into(),
            archetype_field_name: Some("colormap".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::depth_range`].
    #[inline]
    pub fn descriptor_depth_range() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.ValueRange".into(),
            archetype_field_name: Some("depth_range".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::point_fill_ratio`].
    #[inline]
    pub fn descriptor_point_fill_ratio() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.FillRatio".into(),
            archetype_field_name: Some("point_fill_ratio".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for [`Self::draw_order`].
    #[inline]
    pub fn descriptor_draw_order() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.DrawOrder".into(),
            archetype_field_name: Some("draw_order".into()),
        }
    }

    /// Returns the [`ComponentDescriptor`] for the associated indicator component.
    #[inline]
    pub fn descriptor_indicator() -> ComponentDescriptor {
        ComponentDescriptor {
            archetype_name: Some("rerun.archetypes.DepthImage".into()),
            component_name: "rerun.components.DepthImageIndicator".into(),
            archetype_field_name: None,
        }
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 2usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            DepthImage::descriptor_buffer(),
            DepthImage::descriptor_format(),
        ]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| [DepthImage::descriptor_indicator()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 5usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            DepthImage::descriptor_meter(),
            DepthImage::descriptor_colormap(),
            DepthImage::descriptor_depth_range(),
            DepthImage::descriptor_point_fill_ratio(),
            DepthImage::descriptor_draw_order(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            DepthImage::descriptor_buffer(),
            DepthImage::descriptor_format(),
            DepthImage::descriptor_indicator(),
            DepthImage::descriptor_meter(),
            DepthImage::descriptor_colormap(),
            DepthImage::descriptor_depth_range(),
            DepthImage::descriptor_point_fill_ratio(),
            DepthImage::descriptor_draw_order(),
        ]
    });

impl DepthImage {
    /// The total number of components in the archetype: 2 required, 1 recommended, 5 optional
    pub const NUM_COMPONENTS: usize = 8usize;
}

/// Indicator component for the [`DepthImage`] [`::re_types_core::Archetype`]
pub type DepthImageIndicator = ::re_types_core::GenericIndicatorComponent<DepthImage>;

impl ::re_types_core::Archetype for DepthImage {
    type Indicator = DepthImageIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.DepthImage".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Depth image"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: DepthImageIndicator = DepthImageIndicator::DEFAULT;
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
        let buffer = arrays_by_descr
            .get(&Self::descriptor_buffer())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_buffer()));
        let format = arrays_by_descr
            .get(&Self::descriptor_format())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_format()));
        let meter = arrays_by_descr
            .get(&Self::descriptor_meter())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_meter()));
        let colormap = arrays_by_descr
            .get(&Self::descriptor_colormap())
            .map(|array| SerializedComponentBatch::new(array.clone(), Self::descriptor_colormap()));
        let depth_range = arrays_by_descr
            .get(&Self::descriptor_depth_range())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_depth_range())
            });
        let point_fill_ratio = arrays_by_descr
            .get(&Self::descriptor_point_fill_ratio())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_point_fill_ratio())
            });
        let draw_order = arrays_by_descr
            .get(&Self::descriptor_draw_order())
            .map(|array| {
                SerializedComponentBatch::new(array.clone(), Self::descriptor_draw_order())
            });
        Ok(Self {
            buffer,
            format,
            meter,
            colormap,
            depth_range,
            point_fill_ratio,
            draw_order,
        })
    }
}

impl ::re_types_core::AsComponents for DepthImage {
    #[inline]
    fn as_serialized_batches(&self) -> Vec<SerializedComponentBatch> {
        use ::re_types_core::Archetype as _;
        [
            Self::indicator().serialized(),
            self.buffer.clone(),
            self.format.clone(),
            self.meter.clone(),
            self.colormap.clone(),
            self.depth_range.clone(),
            self.point_fill_ratio.clone(),
            self.draw_order.clone(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for DepthImage {}

impl DepthImage {
    /// Create a new `DepthImage`.
    #[inline]
    pub fn new(
        buffer: impl Into<crate::components::ImageBuffer>,
        format: impl Into<crate::components::ImageFormat>,
    ) -> Self {
        Self {
            buffer: try_serialize_field(Self::descriptor_buffer(), [buffer]),
            format: try_serialize_field(Self::descriptor_format(), [format]),
            meter: None,
            colormap: None,
            depth_range: None,
            point_fill_ratio: None,
            draw_order: None,
        }
    }

    /// Update only some specific fields of a `DepthImage`.
    #[inline]
    pub fn update_fields() -> Self {
        Self::default()
    }

    /// Clear all the fields of a `DepthImage`.
    #[inline]
    pub fn clear_fields() -> Self {
        use ::re_types_core::Loggable as _;
        Self {
            buffer: Some(SerializedComponentBatch::new(
                crate::components::ImageBuffer::arrow_empty(),
                Self::descriptor_buffer(),
            )),
            format: Some(SerializedComponentBatch::new(
                crate::components::ImageFormat::arrow_empty(),
                Self::descriptor_format(),
            )),
            meter: Some(SerializedComponentBatch::new(
                crate::components::DepthMeter::arrow_empty(),
                Self::descriptor_meter(),
            )),
            colormap: Some(SerializedComponentBatch::new(
                crate::components::Colormap::arrow_empty(),
                Self::descriptor_colormap(),
            )),
            depth_range: Some(SerializedComponentBatch::new(
                crate::components::ValueRange::arrow_empty(),
                Self::descriptor_depth_range(),
            )),
            point_fill_ratio: Some(SerializedComponentBatch::new(
                crate::components::FillRatio::arrow_empty(),
                Self::descriptor_point_fill_ratio(),
            )),
            draw_order: Some(SerializedComponentBatch::new(
                crate::components::DrawOrder::arrow_empty(),
                Self::descriptor_draw_order(),
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
            self.buffer
                .map(|buffer| buffer.partitioned(_lengths.clone()))
                .transpose()?,
            self.format
                .map(|format| format.partitioned(_lengths.clone()))
                .transpose()?,
            self.meter
                .map(|meter| meter.partitioned(_lengths.clone()))
                .transpose()?,
            self.colormap
                .map(|colormap| colormap.partitioned(_lengths.clone()))
                .transpose()?,
            self.depth_range
                .map(|depth_range| depth_range.partitioned(_lengths.clone()))
                .transpose()?,
            self.point_fill_ratio
                .map(|point_fill_ratio| point_fill_ratio.partitioned(_lengths.clone()))
                .transpose()?,
            self.draw_order
                .map(|draw_order| draw_order.partitioned(_lengths.clone()))
                .transpose()?,
        ];
        let indicator_column =
            ::re_types_core::indicator_column::<Self>(_lengths.into_iter().count())?;
        Ok(columns.into_iter().chain([indicator_column]).flatten())
    }

    /// The raw depth image data.
    #[inline]
    pub fn with_buffer(mut self, buffer: impl Into<crate::components::ImageBuffer>) -> Self {
        self.buffer = try_serialize_field(Self::descriptor_buffer(), [buffer]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ImageBuffer`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_buffer`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_buffer(
        mut self,
        buffer: impl IntoIterator<Item = impl Into<crate::components::ImageBuffer>>,
    ) -> Self {
        self.buffer = try_serialize_field(Self::descriptor_buffer(), buffer);
        self
    }

    /// The format of the image.
    #[inline]
    pub fn with_format(mut self, format: impl Into<crate::components::ImageFormat>) -> Self {
        self.format = try_serialize_field(Self::descriptor_format(), [format]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ImageFormat`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_format`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_format(
        mut self,
        format: impl IntoIterator<Item = impl Into<crate::components::ImageFormat>>,
    ) -> Self {
        self.format = try_serialize_field(Self::descriptor_format(), format);
        self
    }

    /// An optional floating point value that specifies how long a meter is in the native depth units.
    ///
    /// For instance: with uint16, perhaps meter=1000 which would mean you have millimeter precision
    /// and a range of up to ~65 meters (2^16 / 1000).
    ///
    /// Note that the only effect on 2D views is the physical depth values shown when hovering the image.
    /// In 3D views on the other hand, this affects where the points of the point cloud are placed.
    #[inline]
    pub fn with_meter(mut self, meter: impl Into<crate::components::DepthMeter>) -> Self {
        self.meter = try_serialize_field(Self::descriptor_meter(), [meter]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::DepthMeter`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_meter`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_meter(
        mut self,
        meter: impl IntoIterator<Item = impl Into<crate::components::DepthMeter>>,
    ) -> Self {
        self.meter = try_serialize_field(Self::descriptor_meter(), meter);
        self
    }

    /// Colormap to use for rendering the depth image.
    ///
    /// If not set, the depth image will be rendered using the Turbo colormap.
    #[inline]
    pub fn with_colormap(mut self, colormap: impl Into<crate::components::Colormap>) -> Self {
        self.colormap = try_serialize_field(Self::descriptor_colormap(), [colormap]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::Colormap`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_colormap`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_colormap(
        mut self,
        colormap: impl IntoIterator<Item = impl Into<crate::components::Colormap>>,
    ) -> Self {
        self.colormap = try_serialize_field(Self::descriptor_colormap(), colormap);
        self
    }

    /// The expected range of depth values.
    ///
    /// This is typically the expected range of valid values.
    /// Everything outside of the range is clamped to the range for the purpose of colormpaping.
    /// Note that point clouds generated from this image will still display all points, regardless of this range.
    ///
    /// If not specified, the range will be automatically estimated from the data.
    /// Note that the Viewer may try to guess a wider range than the minimum/maximum of values
    /// in the contents of the depth image.
    /// E.g. if all values are positive, some bigger than 1.0 and all smaller than 255.0,
    /// the Viewer will guess that the data likely came from an 8bit image, thus assuming a range of 0-255.
    #[inline]
    pub fn with_depth_range(
        mut self,
        depth_range: impl Into<crate::components::ValueRange>,
    ) -> Self {
        self.depth_range = try_serialize_field(Self::descriptor_depth_range(), [depth_range]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::ValueRange`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_depth_range`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_depth_range(
        mut self,
        depth_range: impl IntoIterator<Item = impl Into<crate::components::ValueRange>>,
    ) -> Self {
        self.depth_range = try_serialize_field(Self::descriptor_depth_range(), depth_range);
        self
    }

    /// Scale the radii of the points in the point cloud generated from this image.
    ///
    /// A fill ratio of 1.0 (the default) means that each point is as big as to touch the center of its neighbor
    /// if it is at the same depth, leaving no gaps.
    /// A fill ratio of 0.5 means that each point touches the edge of its neighbor if it has the same depth.
    ///
    /// TODO(#6744): This applies only to 3D views!
    #[inline]
    pub fn with_point_fill_ratio(
        mut self,
        point_fill_ratio: impl Into<crate::components::FillRatio>,
    ) -> Self {
        self.point_fill_ratio =
            try_serialize_field(Self::descriptor_point_fill_ratio(), [point_fill_ratio]);
        self
    }

    /// This method makes it possible to pack multiple [`crate::components::FillRatio`] in a single component batch.
    ///
    /// This only makes sense when used in conjunction with [`Self::columns`]. [`Self::with_point_fill_ratio`] should
    /// be used when logging a single row's worth of data.
    #[inline]
    pub fn with_many_point_fill_ratio(
        mut self,
        point_fill_ratio: impl IntoIterator<Item = impl Into<crate::components::FillRatio>>,
    ) -> Self {
        self.point_fill_ratio =
            try_serialize_field(Self::descriptor_point_fill_ratio(), point_fill_ratio);
        self
    }

    /// An optional floating point value that specifies the 2D drawing order, used only if the depth image is shown as a 2D image.
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
}

impl ::re_byte_size::SizeBytes for DepthImage {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.buffer.heap_size_bytes()
            + self.format.heap_size_bytes()
            + self.meter.heap_size_bytes()
            + self.colormap.heap_size_bytes()
            + self.depth_range.heap_size_bytes()
            + self.point_fill_ratio.heap_size_bytes()
            + self.draw_order.heap_size_bytes()
    }
}
