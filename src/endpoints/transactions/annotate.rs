use std::collections::HashMap;

use super::{get::Response, Transaction};
use crate::{client, endpoints::Endpoint, Result};


/// A request to annotate a specific transaction vias via the Monzo API
///
/// Form params are a hashmap to allow for setting Monzo's specific syntax for metadata.
#[derive(Debug)]
#[must_use]
pub struct Request<'a, C>
where
    C: client::Inner,
{
    client: &'a C,
    endpoint: String,
    form: HashMap<String, String>,
}

impl<'a, C> Endpoint for Request<'a, C>
where
    C: client::Inner,
{
    const METHOD: reqwest::Method = reqwest::Method::PATCH;

    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn form(&self) -> Option<&dyn erased_serde::Serialize> {
        Some(&self.form)
    }
}

impl<'a, C> Request<'a, C>
where
    C: client::Inner,
{
    pub(crate) fn new(client: &'a C, transaction_id: &'a str, metadata: HashMap<String, String>) -> Self {
        let endpoint = format!("/transactions/{}", &transaction_id);

        let form_i = metadata
            .iter()
            .map(|(k, v)| (format!("metadata[{}]", k), v.clone()));

        let form = HashMap::from_iter(form_i);

        Self { client, endpoint, form }
    }

    /// Consume the request and return the [`Transaction`]
    pub async fn send(self) -> Result<Transaction> {
        let response: Response = self.client.handle_request(&self).await?;

        Ok(response.into())
    }
}
