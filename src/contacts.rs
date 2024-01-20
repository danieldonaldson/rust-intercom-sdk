use std::collections::HashMap;

use crate::client::Client;
use crate::errors::Error;
use crate::util::{ListResponse, Tags};
use fnv::FnvHashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub async fn get_contacts(client: &Client) -> Result<ListResponse<Contact>, Error> {
    let res = client.get("contacts".to_string(), &vec![()]).await?;
    Ok(res.json::<ListResponse<Contact>>().await?)
}

pub async fn get_all_contacts(client: &Client) -> Result<ListResponse<Contact>, Error> {
    let mut all_contacts = ListResponse::default();
    let mut params = FnvHashMap::default();

    params.insert("per_page", "150".to_string());
    let mut more_pages = true;
    while more_pages {
        let res = client
            .get("contacts".to_string(), &params)
            .await?
            .json::<ListResponse<Contact>>()
            .await?;
        println!(
            "Adding page: {} of {}",
            res.pages.page, res.pages.total_pages
        );
        all_contacts.data.extend(res.data);
        if let Some(next) = res.pages.next {
            params.insert("starting_after", next.starting_after);
        } else {
            all_contacts.pages = res.pages;
            all_contacts.total_count = res.total_count;
            all_contacts.extra = res.extra;
            more_pages = false;
        }
    }
    Ok(all_contacts)
    //TODO: handle pagination and allow user to select if they want to paginate
}

pub async fn create_contact(
    client: &Client,
    contact: ContactForCreation,
) -> Result<Contact, Error> {
    let res = client.post("contacts".to_string(), &contact).await?;
    Ok(res.json::<Contact>().await?)
}

// Supports creation by email, external id, or role
#[derive(Serialize, Deserialize, Debug)]
pub struct ContactForCreation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_up_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed_from_emails: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<HashMap<String, String>>,
}

impl ContactForCreation {
    pub fn new_from_email(email: String, role: Role) -> Self {
        Self {
            role: role.into(),
            external_id: None,
            email: Some(email),
            phone: None,
            name: None,
            avatar: None,
            signed_up_at: None,
            last_seen_at: None,
            owner_id: None,
            unsubscribed_from_emails: None,
            custom_attributes: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Contact {
    id: String,
    #[serde(rename = "type")]
    contact_type: String,
    role: Role,
    email: Option<String>,
    external_id: Option<String>,
    phone: Option<String>,
    name: Option<String>,
    owner_id: Option<i64>,
    has_hard_bounced: bool,
    marked_email_as_spam: bool,
    unsubscribed_from_emails: bool,
    created_at: i64,
    updated_at: i64,
    signed_up_at: Option<i64>,
    last_seen_at: Option<i64>,
    last_replied_at: Option<i64>,
    last_contacted_at: Option<i64>,
    last_email_opened_at: Option<i64>,
    last_email_clicked_at: Option<i64>,
    language_override: Option<String>,
    browser: Option<String>,
    browser_version: Option<String>,
    browser_language: Option<String>,
    os: Option<String>,
    android_app_name: Option<String>,
    android_app_version: Option<String>,
    android_device: Option<String>,
    android_os_version: Option<String>,
    android_sdk_version: Option<String>,
    android_last_seen_at: Option<Value>, //why intercom. Y u do dis
    ios_app_name: Option<String>,
    ios_app_version: Option<String>,
    ios_device: Option<String>,
    ios_os_version: Option<String>,
    ios_sdk_version: Option<String>,
    ios_last_seen_at: Option<Value>, //tell me its an integer. give me a string. Y u do dis
    tags: Tags,
    custom_attributes: HashMap<String, Value>,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Role {
    #[serde(rename = "lead")]
    Lead,
    #[serde(rename = "user")]
    User,
    #[default]
    #[serde(rename = "visitor")]
    Visitor,
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    fn setup_client() -> Client {
        dotenv::dotenv().ok();
        let access_token = std::env::var("INTERCOM_ACCESS_TOKEN").unwrap();
        Client::new(access_token)
    }

    #[tokio::test]
    async fn test_get_contacts() {
        let client = setup_client();
        let list = get_contacts(&client).await.unwrap();
        println!("{:?}", list.total_count);
        assert!(list.total_count > 6320);
    }

    #[tokio::test]
    async fn test_get_all_contacts() {
        let client = setup_client();
        let list = get_all_contacts(&client).await.unwrap();
        println!("{:?}", list.total_count);
        assert_eq!(list.total_count, list.data.len());
    }

    #[tokio::test]
    async fn test_create_contact() {
        let client = setup_client();
        let email = std::env::var("INTERCOM_TEST_EMAIL").unwrap();
        let domain = std::env::var("INTERCOM_TEST_DOMAIN").unwrap();
        let test_email = format!("{}+{}@{}", email, Uuid::new_v4(), domain);
        dbg!(&test_email);
        let contact = ContactForCreation::new_from_email(test_email.clone(), Role::User);
        dbg!(serde_json::to_string(&contact).unwrap());
        let created = create_contact(&client, contact).await.unwrap();
        println!("{:?}", created.id);
        assert_eq!(created.email, Some(test_email));
    }
}
