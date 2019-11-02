//! Balance endpoint

use crate::{endpoints::handle_response, Result};
use serde::Deserialize;

/// The balance of a Monzo Account
#[derive(Deserialize, Debug)]
pub struct Balance {
    /// The account balance, in the minor units of the listed currency. ie for
    /// GBP, the balance is in pence.
    balance: i64,

    /// The total account balance. I haven't figured out what the difference is
    /// yet
    total_balance: i64,

    /// three-letter currency code for the account
    currency: String,

    /// total expenditure so far this calendar day
    spend_today: i64,
}

impl Balance {
    /// The account balance, in the minor units of the listed currency. ie for
    /// GBP, the balance is in pence.
    #[must_use]
    pub fn balance(&self) -> i64 {
        self.balance
    }

    /// The total account balance. I haven't figured out what the difference is
    /// yet
    #[must_use]
    pub fn total_balance(&self) -> i64 {
        self.total_balance
    }

    /// three-letter currency code for the account
    #[must_use]
    pub fn currency(&self) -> &String {
        &self.currency
    }

    /// total expenditure so far this calendar day
    #[must_use]
    pub fn spend_today(&self) -> i64 {
        self.spend_today
    }
}

/// An object representing a request to the Monzo API for a list of accounts
pub struct GetBalance<'a> {
    reqwest_builder: reqwest::RequestBuilder,
    account_id: &'a str,
}

impl<'a> GetBalance<'a> {
    pub(crate) fn new(
        http_client: &reqwest::Client,
        access_token: impl AsRef<str>,
        account_id: &'a str,
    ) -> Self {
        let reqwest_builder = http_client
            .get("https://api.monzo.com/balance")
            .bearer_auth(access_token.as_ref());

        Self {
            reqwest_builder,
            account_id,
        }
    }

    /// Consume the request and return a future that will resolve to the balance
    /// of the given account
    pub async fn send(self) -> Result<Balance> {
        handle_response(
            self.reqwest_builder
                .query(&[("account_id", self.account_id)]),
        )
        .await
    }
}

/*
use std::future::Future;
use crate::into_future::IntoFuture;

impl<'a> IntoFuture for GetBalance<'a> {
    type Output = Result<Balance>;
    type Future = Pin<Box<dyn Future<Output = Self::Output> + 'a>>;

    fn into_future(self) -> Self::Future {
        Box::pin(self.send())
    }
}

} */
