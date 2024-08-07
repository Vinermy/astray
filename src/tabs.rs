use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Serialize, Debug, Deserialize)]
pub enum Tabs {
    SystemView,
    Research,
    Colonies,
    ShipModules
}

impl From<Tabs> for String {
    fn from(value: Tabs) -> Self {
        match value {
            Tabs::SystemView => String::from("System View"),
            Tabs::Research => String::from("Research"),
            Tabs::Colonies => String::from("Colonies"),
            Tabs::ShipModules => String::from("Ship modules"),
        }
    }
}