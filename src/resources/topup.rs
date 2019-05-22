// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::TopupId;
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{BalanceTransaction, Currency, Source};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Topup".
///
/// For more details see [https://stripe.com/docs/api/topups/object](https://stripe.com/docs/api/topups/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Topup {
    /// Unique identifier for the object.
    pub id: TopupId,

    /// Amount transferred.
    pub amount: i64,

    /// ID of the balance transaction that describes the impact of this top-up on your account balance.
    ///
    /// May not be specified depending on status of top-up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Date the funds are expected to arrive in your Stripe account for payouts.
    ///
    /// This factors in delays like weekends or bank holidays.
    /// May not be specified depending on status of top-up.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_availability_date: Option<Timestamp>,

    /// Error code explaining reason for top-up failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for top-up failure if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    pub source: Source,

    /// Extra information about a top-up.
    ///
    /// This will appear on your source's bank statement.
    /// It must contain at least one letter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The status of the top-up is either `canceled`, `failed`, `pending`, `reversed`, or `succeeded`.
    pub status: TopupStatus,

    /// A string that identifies this top-up as part of a group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl Topup {
    /// Returns a list of top-ups.
    pub fn list(client: &Client, params: ListTopups<'_>) -> Response<List<Topup>> {
        client.get_query("/topups", &params)
    }

    /// Retrieves the details of a top-up that has previously been created.
    ///
    /// Supply the unique top-up ID that was returned from your previous request, and Stripe will return the corresponding top-up information.
    pub fn retrieve(client: &Client, id: &TopupId, expand: &[&str]) -> Response<Topup> {
        client.get_query(&format!("/topups/{}", id), &Expand { expand })
    }
}

impl Object for Topup {
    type Id = TopupId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "topup"
    }
}

/// The parameters for `Topup::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListTopups<'a> {
    /// A positive integer representing how much to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<RangeQuery<Timestamp>>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a TopupId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a TopupId>,

    /// Only return top-ups that have the given status.
    ///
    /// One of `canceled`, `failed`, `pending` or `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<TopupStatusFilter>,
}

impl<'a> ListTopups<'a> {
    pub fn new() -> Self {
        ListTopups {
            amount: Default::default(),
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}

/// An enum representing the possible values of an `Topup`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TopupStatus {
    Canceled,
    Failed,
    Pending,
    Reversed,
    Succeeded,
}

/// An enum representing the possible values of an `ListTopups`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TopupStatusFilter {
    Canceled,
    Failed,
    Pending,
    Succeeded,
}
