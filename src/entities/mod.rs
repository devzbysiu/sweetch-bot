use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct WatchedGame {
    title: String,
    acceptable_price: Option<f64>,
}

impl WatchedGame {
    #[cfg(test)]
    pub fn new<S: Into<String>>(title: S) -> Self {
        Self {
            title: title.into(),
            acceptable_price: None,
        }
    }

    #[cfg(test)]
    pub fn with_acceptable_price(mut self, price: f64) -> Self {
        self.acceptable_price = Some(price);
        self
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn acceptable_price(&self) -> Option<f64> {
        self.acceptable_price
    }
}
