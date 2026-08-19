use askama::Result;
// Import the Values trait from askama
use askama::Values;
use time::OffsetDateTime;
use time::macros::format_description;

// 1. Add this macro attribute
#[askama::filter_fn]
// 2. Add the extra `_: &dyn Values` argument to the signature
pub fn format_date(date: &OffsetDateTime, _: &dyn Values) -> Result<String> {
    let format = format_description!("[year]-[month]-[day]");

    date.format(&format)
        .map_err(|e| askama::Error::Custom(Box::new(e)))
}
