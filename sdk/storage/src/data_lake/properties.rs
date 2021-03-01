use azure_core::AddAsHeader;
use http::request::Builder;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties<'a, 'b>(HashMap<Cow<'a, str>, Cow<'b, str>>);

impl<'a, 'b> Properties<'a, 'b> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<K: Into<Cow<'a, str>>, V: Into<Cow<'b, str>>>(
        &mut self,
        k: K,
        v: V,
    ) -> Option<Cow<'b, str>> {
        self.0.insert(k.into(), v.into())
    }

    pub fn hash_map(&self) -> &HashMap<Cow<'a, str>, Cow<'b, str>> {
        &self.0
    }
}

impl<'a, 'b> AddAsHeader for Properties<'a, 'b> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        // the header is a comma separated list of key=base64(value)
        // see
        // [https://docs.microsoft.com/en-us/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers](https://docs.microsoft.com/en-us/rest/api/storageservices/datalakestoragegen2/filesystem/create#request-headers)
        let mut s = String::new();
        self.0.iter().for_each(|(k, v)| {
            s.push_str(&format!("{}={},", k.as_ref(), base64::encode(v.as_ref())));
        });

        // since we added a comma to the last entry,
        // we will strip it to the exported
        // header:
        builder.header("x-ms-properties", &s[..s.len() - 1])
    }
}

//
//impl From<S> for Properties
//where
//    S: Into<String>,
//{
//    fn from(snapshot: S) -> Self {
//        Self::new(snapshot.into())
//    }
//}
