//! Receipt data and editable numeric fields. Demo rates match iced_receipts.
#[derive(Debug, Clone)]
pub struct Receipt {
    pub id: u64,
    pub name: String,
    pub items: Vec<Item>,
    pub service: String,
    pub gratuity: String,
    pub totals: Totals,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub quantity: String,
    pub price: String,
    pub tax: Tax,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Tax {
    #[default]
    Food,
    Alcohol,
    Exempt,
    Other,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Totals {
    pub subtotal: u64,
    pub tax: u64,
    pub service: u64,
    pub gratuity: u64,
    pub total: u64,
}

impl Tax {
    pub const ALL: [Self; 4] = [Self::Food, Self::Alcohol, Self::Exempt, Self::Other];

    pub fn key(self) -> &'static str {
        match self {
            Self::Food => "food",
            Self::Alcohol => "alcohol",
            Self::Exempt => "exempt",
            Self::Other => "other",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Food => "Food (8%)",
            Self::Alcohol => "Alcohol (10%)",
            Self::Exempt => "Non-taxable",
            Self::Other => "Other (8%)",
        }
    }

    fn percent(self) -> u64 {
        match self {
            Self::Food | Self::Other => 8,
            Self::Alcohol => 10,
            Self::Exempt => 0,
        }
    }
}

impl Item {
    pub fn blank(id: u64) -> Self {
        Self {
            id,
            name: String::new(),
            quantity: "1".into(),
            price: String::new(),
            tax: Tax::Food,
        }
    }

    pub fn unit_price(&self) -> Option<u64> {
        money(&self.price)
    }

    pub fn amount(&self) -> Option<u64> {
        let quantity = self
            .quantity
            .parse::<u64>()
            .ok()
            .filter(|q| (1..=10_000).contains(q))?;
        self.unit_price()?.checked_mul(quantity)
    }
}

impl Receipt {
    pub fn blank(id: u64) -> Self {
        Self {
            id,
            name: String::new(),
            items: vec![Item::blank(0)],
            service: "0".into(),
            gratuity: "0.00".into(),
            totals: Totals::default(),
        }
    }

    pub fn recalculate(&mut self) {
        let subtotal: u64 = self.items.iter().filter_map(Item::amount).sum();
        let tax = self
            .items
            .iter()
            .map(|item| (item.amount().unwrap_or(0) * item.tax.percent() + 50) / 100)
            .sum();
        let service = (subtotal * money(&self.service).unwrap_or(0).min(10_000) + 5_000) / 10_000;
        let gratuity = money(&self.gratuity).unwrap_or(0);
        self.totals = Totals {
            subtotal,
            tax,
            service,
            gratuity,
            total: subtotal + tax + service + gratuity,
        };
    }

    pub fn error(&self) -> Option<&'static str> {
        if self.name.trim().is_empty() {
            return Some("Give this sale a name.");
        }
        if self.items.is_empty() {
            return Some("Add at least one item.");
        }
        if self.items.iter().any(|item| item.name.trim().is_empty()) {
            return Some("Give each item a name.");
        }
        if self.items.iter().any(|item| item.amount().is_none()) {
            return Some(
                "Use quantities from 1 to 10,000 and non-negative prices with up to two decimal places.",
            );
        }
        if money(&self.service).is_none_or(|value| value > 10_000) {
            return Some("Service charge must be between 0 and 100 percent.");
        }
        if money(&self.gratuity).is_none() {
            return Some("Enter a non-negative gratuity with up to two decimal places.");
        }
        None
    }

    pub fn samples() -> Vec<Self> {
        [
            (
                "Friday lunch",
                [
                    ("Margherita pizza", "2", "18.00", Tax::Food),
                    ("Sparkling water", "2", "4.50", Tax::Food),
                ],
            ),
            (
                "Dinner with friends",
                [
                    ("Pasta", "3", "24.00", Tax::Food),
                    ("House wine", "2", "12.00", Tax::Alcohol),
                ],
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (name, items))| {
            let mut receipt = Self::blank(id as u64 + 1);
            receipt.name = name.into();
            receipt.items = items
                .into_iter()
                .enumerate()
                .map(|(id, (name, quantity, price, tax))| Item {
                    id: id as u64,
                    name: name.into(),
                    quantity: quantity.into(),
                    price: price.into(),
                    tax,
                })
                .collect();
            receipt.gratuity = "5.00".into();
            receipt.recalculate();
            receipt
        })
        .collect()
    }
}

/// Parse decimal currency without binary floating point or accepting partial input.
fn money(value: &str) -> Option<u64> {
    let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
    if whole.is_empty()
        || !whole.bytes().all(|c| c.is_ascii_digit())
        || fraction.len() > 2
        || !fraction.bytes().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    let whole = whole.parse::<u64>().ok().filter(|n| *n <= 1_000_000)?;
    let fraction = match fraction.len() {
        0 => 0,
        1 => fraction.parse::<u64>().ok()? * 10,
        _ => fraction.parse::<u64>().ok()?,
    };
    Some(whole * 100 + fraction)
}

pub fn currency(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}
