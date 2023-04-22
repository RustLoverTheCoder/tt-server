use super::{bot::ApiWebDocument, message::{ApiDocument, ApiMessageEntity, ApiPaymentCredentials}};

pub struct ApiShippingAddress {
    pub street_line_1: String,
    pub street_line_2: String,
    pub city: String,
    pub state: String,
    pub country_iso2: String,
    pub post_code: String,
}

pub struct ApiPaymentSavedInfo {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<ApiShippingAddress>,
}

pub struct ApiPaymentFormNativeParams {
    pub need_cardholder_name: Option<bool>,
    pub need_country: Option<bool>,
    pub need_zip: Option<bool>,
    pub publishable_key: Option<String>,
    pub public_token: Option<String>,
}

pub struct ApiPaymentForm {
    pub can_save_credentials: Option<bool>,
    pub is_password_missing: Option<bool>,
    pub form_id: String,
    pub provider_id: String,
    pub native_provider: Option<String>,
    pub saved_info: Option<ApiPaymentSavedInfo>,
    pub saved_credentials: Option<Vec<ApiPaymentCredentials>>,
    pub invoice_container: ApiInvoiceContainer,
    pub native_params: ApiPaymentFormNativeParams,
}

pub struct ApiLabeledPrice {
    pub label: String,
    pub amount: i32,
}

pub struct ApiReceipt {
    pub photo: Option<ApiWebDocument>,
    pub text: Option<String>,
    pub title: Option<String>,
    pub currency: String,
    pub prices: Vec<ApiLabeledPrice>,
    pub info: Option<ApiReceiptInfo>,
    pub tip_amount: i32,
    pub total_amount: i32,
    pub credentials_title: String,
    pub shipping_prices: Option<Vec<ApiLabeledPrice>>,
    pub shipping_method: Option<String>,
}

pub struct ApiReceiptInfo {
    pub shipping_address: Option<ApiShippingAddress>,
    pub phone: Option<String>,
    pub name: Option<String>,
}

pub struct ApiPremiumPromo {
    pub video_sections: Vec<String>,
    pub videos: Vec<ApiDocument>,
    pub status_text: String,
    pub status_entities: Vec<ApiMessageEntity>,
    pub options: Vec<ApiPremiumSubscriptionOption>,
}

pub struct ApiPremiumSubscriptionOption {
    pub is_current: Option<bool>,
    pub can_purchase_upgrade: Option<bool>,
    pub months: i32,
    pub currency: String,
    pub amount: String,
    pub bot_url: String,
}

pub struct Price {
    label: String,
    amount: i32,
}

pub struct ApiInvoiceContainer {
    is_test: Option<bool>,
    is_name_requested: Option<bool>,
    is_phone_requested: Option<bool>,
    is_email_requested: Option<bool>,
    is_shipping_address_requested: Option<bool>,
    is_flexible: Option<bool>,
    should_send_phone_to_provider: Option<bool>,
    should_send_email_to_provider: Option<bool>,
    currency: Option<String>,
    prices: Option<Vec<Price>>,
}