// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{PlanId, PriceId, SubscriptionId, SubscriptionItemId};
use crate::params::{Deleted, Expand, List, Metadata, Object, Paginable, Timestamp};
use crate::resources::{Currency, Plan, Price, SubscriptionItemBillingThresholds, TaxRate};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionItem".
///
/// For more details see <https://stripe.com/docs/api/subscription_items/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItem {
    /// Unique identifier for the object.
    pub id: SubscriptionItemId,

    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The `subscription` this `subscription_item` belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// The tax rates which apply to this `subscription_item`.
    ///
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}

impl SubscriptionItem {
    /// Returns a list of your subscription items for a given subscription.
    pub fn list(
        client: &Client,
        params: &ListSubscriptionItems<'_>,
    ) -> Response<List<SubscriptionItem>> {
        client.get_query("/subscription_items", params)
    }

    /// Adds a new item to an existing subscription.
    ///
    /// No existing items will be changed or replaced.
    pub fn create(
        client: &Client,
        params: CreateSubscriptionItem<'_>,
    ) -> Response<SubscriptionItem> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/subscription_items", &params)
    }

    /// Retrieves the subscription item with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionItemId,
        expand: &[&str],
    ) -> Response<SubscriptionItem> {
        client.get_query(&format!("/subscription_items/{}", id), Expand { expand })
    }

    /// Updates the plan or quantity of an item on a current subscription.
    pub fn update(
        client: &Client,
        id: &SubscriptionItemId,
        params: UpdateSubscriptionItem<'_>,
    ) -> Response<SubscriptionItem> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/subscription_items/{}", id), &params)
    }

    /// Deletes an item from the subscription.
    ///
    /// Removing a subscription item from a subscription will not cancel the subscription.
    pub fn delete(
        client: &Client,
        id: &SubscriptionItemId,
    ) -> Response<Deleted<SubscriptionItemId>> {
        client.delete(&format!("/subscription_items/{}", id))
    }
}

impl Object for SubscriptionItem {
    type Id = SubscriptionItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_item"
    }
}

/// The parameters for `SubscriptionItem::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateSubscriptionItem<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
    ///
    /// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.  Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.  Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
    /// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).  Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<SubscriptionPaymentBehavior>,

    /// The identifier of the plan to add to the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceId>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<SubscriptionItemPriceData>,

    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,

    /// The quantity you'd like to apply to the subscription item you're creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The identifier of the subscription to modify.
    pub subscription: SubscriptionId,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

impl<'a> CreateSubscriptionItem<'a> {
    pub fn new(subscription: SubscriptionId) -> Self {
        CreateSubscriptionItem {
            billing_thresholds: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            payment_behavior: Default::default(),
            plan: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            proration_behavior: Default::default(),
            proration_date: Default::default(),
            quantity: Default::default(),
            subscription,
            tax_rates: Default::default(),
        }
    }
}

/// The parameters for `SubscriptionItem::list`.
#[derive(Clone, Debug, Serialize)]
pub struct ListSubscriptionItems<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SubscriptionItemId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionItemId>,

    /// The ID of the subscription whose items will be retrieved.
    pub subscription: SubscriptionId,
}

impl<'a> ListSubscriptionItems<'a> {
    pub fn new(subscription: SubscriptionId) -> Self {
        ListSubscriptionItems {
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            subscription,
        }
    }
}
impl Paginable for ListSubscriptionItems<'_> {
    type O = SubscriptionItem;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `SubscriptionItem::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscriptionItem<'a> {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,

    /// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
    ///
    /// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.  Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.  Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
    /// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).  Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<SubscriptionPaymentBehavior>,

    /// The identifier of the new plan for this subscription item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,

    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceId>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<SubscriptionItemPriceData>,

    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply the same proration that was previewed with the [upcoming invoice](https://stripe.com/docs/api#retrieve_customer_invoice) endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,

    /// The quantity you'd like to apply to the subscription item you're creating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

impl<'a> UpdateSubscriptionItem<'a> {
    pub fn new() -> Self {
        UpdateSubscriptionItem {
            billing_thresholds: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
            payment_behavior: Default::default(),
            plan: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            proration_behavior: Default::default(),
            proration_date: Default::default(),
            quantity: Default::default(),
            tax_rates: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: SubscriptionItemPriceDataRecurring,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<SubscriptionItemPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItemPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PlanInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

/// An enum representing the possible values of an `SubscriptionItemPriceDataRecurring`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            PlanInterval::Day => "day",
            PlanInterval::Month => "month",
            PlanInterval::Week => "week",
            PlanInterval::Year => "year",
        }
    }
}

impl AsRef<str> for PlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PlanInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `SubscriptionItemPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl SubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionItemPriceDataTaxBehavior::Exclusive => "exclusive",
            SubscriptionItemPriceDataTaxBehavior::Inclusive => "inclusive",
            SubscriptionItemPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for SubscriptionItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionItemPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateSubscriptionItem`'s `payment_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl SubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionPaymentBehavior::AllowIncomplete => "allow_incomplete",
            SubscriptionPaymentBehavior::DefaultIncomplete => "default_incomplete",
            SubscriptionPaymentBehavior::ErrorIfIncomplete => "error_if_incomplete",
            SubscriptionPaymentBehavior::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for SubscriptionPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionPaymentBehavior {
    fn default() -> Self {
        Self::AllowIncomplete
    }
}

/// An enum representing the possible values of an `CreateSubscriptionItem`'s `proration_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionProrationBehavior::AlwaysInvoice => "always_invoice",
            SubscriptionProrationBehavior::CreateProrations => "create_prorations",
            SubscriptionProrationBehavior::None => "none",
        }
    }
}

impl AsRef<str> for SubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
