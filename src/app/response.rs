#[derive(serde::Serialize)]
pub struct RedirectResponse<'a> {
    pub redirect_url: &'a str,
}
