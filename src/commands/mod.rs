use serde_json::Error;

pub mod projects;
pub mod tickets;
pub(crate) mod login;

pub trait RenderList<T> {
    fn render_list(&self, input: T) -> Result<String, Error>;
}
