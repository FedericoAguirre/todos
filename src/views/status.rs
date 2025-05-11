use loco_rs::prelude::*;

use crate::models::_entities::statuses;

/// Render a list view of `statuses`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<statuses::Model>) -> Result<Response> {
    format::render().view(v, "status/list.html", data!({"items": items}))
}

/// Render a single `status` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &statuses::Model) -> Result<Response> {
    format::render().view(v, "status/show.html", data!({"item": item}))
}

/// Render a `status` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "status/create.html", data!({}))
}

/// Render a `status` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &statuses::Model) -> Result<Response> {
    format::render().view(v, "status/edit.html", data!({"item": item}))
}
