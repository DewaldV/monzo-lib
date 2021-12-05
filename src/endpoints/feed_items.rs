//! Create items in your account feed

pub use basic::Request as Basic;

pub(crate) mod basic {
    use serde::Serialize;

    use crate::{client, client::send_and_resolve_request, endpoints::Endpoint, Result};

    /// A request to create a new basic feed item.
    ///
    /// Currently a 'basic' feed item is the only kind
    /// of feed item which is supported
    ///
    /// Use the builder methods to set optional fields
    #[derive(Debug)]
    #[must_use]
    pub struct Request<'a> {
        client: &'a dyn client::Inner,
        payload: Payload<'a>,
    }

    impl<'a> Request<'a> {
        pub(crate) fn new(
            client: &'a dyn client::Inner,
            account_id: &'a str,
            title: &'a str,
            image_url: &'a str,
        ) -> Self {
            let params = Params {
                title,
                image_url,
                background_color: None,
                body_color: None,
                title_color: None,
                body: None,
            };

            let payload = Payload {
                account_id,
                url: None,
                r#type: "basic",
                params,
            };

            Self { client, payload }
        }

        /// Set the url of the feed item.
        ///
        /// This is the url the user will be redirected to after
        /// tapping on the feed item
        pub fn url(mut self, url: &'a str) -> Self {
            self.payload.url = Some(url);
            self
        }

        /// Set the title of the feed item.
        pub fn title(mut self, title: &'a str) -> Self {
            self.payload.params.title = title;
            self
        }

        /// Set the image of the feed item.
        ///
        /// # Note
        /// *This doesn't currently seem to do anything*
        pub fn image_url(mut self, image_url: &'a str) -> Self {
            self.payload.params.image_url = image_url;
            self
        }

        /// Set the background colour of the feed item
        pub fn background_color(mut self, background_color: &'a str) -> Self {
            self.payload.params.background_color = Some(background_color);
            self
        }

        /// Set the body colour of the feed item
        pub fn body_color(mut self, body_color: &'a str) -> Self {
            self.payload.params.body_color = Some(body_color);
            self
        }

        /// Set the title colour of the feed item
        pub fn title_color(mut self, title_color: &'a str) -> Self {
            self.payload.params.title_color = Some(title_color);
            self
        }

        /// Set the body text of the feed item
        pub fn body(mut self, body: &'a str) -> Self {
            self.payload.params.body = Some(body);
            self
        }

        /// Consume and send the [`Request`].
        pub async fn send(self) -> Result<()> {
            send_and_resolve_request(self.client, &self).await
        }
    }

    impl<'a> Endpoint for Request<'a> {
        fn method(&self) -> reqwest::Method {
            reqwest::Method::POST
        }

        fn endpoint(&self) -> &str {
            "https://api.monzo.com/feed"
        }

        fn json(&self) -> Option<&dyn erased_serde::Serialize> {
            Some(&self.payload)
        }
    }

    #[derive(Debug, Serialize)]
    struct Params<'a> {
        #[serde(rename = "params[title]")]
        title: &'a str,

        #[serde(rename = "params[image_url]")]
        image_url: &'a str,

        #[serde(rename = "params[background_color]")]
        #[serde(skip_serializing_if = "Option::is_none")]
        background_color: Option<&'a str>,

        #[serde(rename = "params[body_color]")]
        #[serde(skip_serializing_if = "Option::is_none")]
        body_color: Option<&'a str>,

        #[serde(rename = "params[title_color]")]
        #[serde(skip_serializing_if = "Option::is_none")]
        title_color: Option<&'a str>,

        #[serde(rename = "params[body]")]
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<&'a str>,
    }

    #[derive(Debug, Serialize)]
    struct Payload<'a> {
        // required for all feed item requests
        account_id: &'a str,
        r#type: &'static str,

        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<&'a str>,

        #[serde(flatten)]
        params: Params<'a>,
    }
}
