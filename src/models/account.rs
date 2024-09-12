use super::id::ClientID;

#[derive(Debug, PartialEq)]
pub struct Account {
    client_id: ClientID,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}

impl Account {
    pub fn new(client_id: ClientID) -> Self {
        Account {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn client_id(&self) -> ClientID {
        self.client_id
    }

    pub fn available(&self) -> f32 {
        self.available
    }

    pub fn held(&self) -> f32 {
        self.held
    }

    pub fn total(&self) -> f32 {
        self.total
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn update_total(&mut self) {
        self.total = self.available + self.held;
    }

    pub fn add(&mut self, amount: f32) {
        self.available += amount;
        self.update_total();
    }

    pub fn subtract(&mut self, amount: f32) {
        if amount <= self.available {
            self.available -= amount;
            self.update_total();
        }
    }

    pub fn hold(&mut self, amount: f32) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn release(&mut self, amount: f32) {
        self.held -= amount;
        self.available += amount;
    }

    pub fn chargeback(&mut self, amount: f32) {
        self.held -= amount;
        self.locked = true;
        self.update_total();
    }
}
