use loco_rs::prelude::*;

use crate::models::_entities::todos;

/// Render a list view of `todos`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<todos::Model>) -> Result<Response> {
    format::render().view(v, "todo/list.html", data!({"items": items}))
}

/// Render a single `todo` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &todos::Model) -> Result<Response> {
    format::render().view(v, "todo/show.html", data!({"item": item}))
}

/// Render a `todo` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "todo/create.html", data!({}))
}

/// Render a `todo` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &todos::Model) -> Result<Response> {
    format::render().view(v, "todo/edit.html", data!({"item": item}))
}
