use azure_core::AppendToUrlQuery;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Select<'a>(Cow<'a, str>);

impl<'a> Select<'a> {
    pub fn new(s: impl Into<Cow<'a, str>>) -> Self {
        Self(s.into())
    }
}

impl<'a> AppendToUrlQuery for Select<'a> {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("$select", self.0.as_ref());
    }
}

impl<'a, S> From<S> for Select<'a>
where
    S: Into<Cow<'a, str>>,
{
    fn from(s: S) -> Self {
        Self::new(s)
    }
}
