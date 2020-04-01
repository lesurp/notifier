use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationContent {
    pub id: usize,
    pub content: String,
}

impl Borrow<usize> for NotificationContent {
    fn borrow(&self) -> &usize {
        &self.id
    }
}

impl PartialEq for NotificationContent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for NotificationContent {}
impl Hash for NotificationContent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateNotification {
    pub tag: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub enum CreateNotificationResult {
    Ok(usize),
    Err,
}

#[derive(Serialize, Deserialize)]
pub struct GetNotifications {
    pub tag: String,
}

#[derive(Serialize, Deserialize)]
pub enum GetNotificationsResult {
    Ok(Vec<NotificationContent>),
    TagDoesNotExist,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteNotification {
    pub tag: String,
    pub id: usize,
}

#[derive(Serialize, Deserialize)]
pub enum DeleteNotificationResult {
    Ok,
    TagDoesNotExist,
    IdDoesNotExist,
}
