mod blueprint;
mod ui;

use re_chunk_store::ColumnDescriptor;
use re_types::blueprint::archetypes;
use re_viewer_context::{ViewId, ViewSystemExecutionError, ViewerContext};
use re_viewport_blueprint::ViewProperty;

/// Wrapper over the `DataframeQuery` blueprint archetype that can also display some UI.
pub(crate) struct Query {
    query_property: ViewProperty,
}

impl Query {
    /// Create a query object from the blueprint store.
    ///
    /// See the `blueprint_io` module for more related accessors.
    pub(crate) fn from_blueprint(ctx: &ViewerContext<'_>, view_id: ViewId) -> Self {
        Self {
            query_property: ViewProperty::from_archetype::<archetypes::DataframeQuery>(
                ctx.blueprint_db(),
                ctx.blueprint_query,
                view_id,
            ),
        }
    }

    /// Display the selection panel ui for this query.
    ///
    /// Implementation is in the `ui` module.
    pub(crate) fn selection_panel_ui(
        &self,
        ctx: &ViewerContext<'_>,
        ui: &mut egui::Ui,
        view_id: ViewId,
        view_columns: &[ColumnDescriptor],
    ) -> Result<(), ViewSystemExecutionError> {
        let timeline = self.timeline(ctx)?;

        self.timeline_ui(ctx, ui, &timeline)?;
        ui.separator();
        self.filter_range_ui(ctx, ui, &timeline)?;
        ui.separator();
        self.filter_is_not_null_ui(ctx, ui, &timeline, view_id)?;
        ui.separator();
        self.column_visibility_ui(ctx, ui, &timeline, view_columns)?;
        ui.separator();
        self.latest_at_ui(ctx, ui)?;

        Ok(())
    }
}