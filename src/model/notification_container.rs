pub struct NotificationContainer {
    counter: usize,
    notifications: Vec<(String, String, String)>,
}

impl NotificationContainer {
    pub fn new() -> Self {
        Self {
            counter: 0,
            notifications: vec![],
        }
    }

    pub fn add(&mut self, text: String) {
        self.counter += 1;

        let id = self.counter.to_string();

        self.notifications.push((id, text, String::from("error")));
    }

    pub fn remove(&mut self, notification_id: String) {
        self.notifications = self
            .notifications
            .iter()
            .cloned()
            .filter(|notification| notification.0 != notification_id)
            .collect();
    }

    pub fn list(&self) -> Vec<(String, String, String)> {
        self.notifications.clone()
    }
}
