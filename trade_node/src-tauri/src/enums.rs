use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum Direction {
    #[serde(rename = "多")]
    Long,
    #[serde(rename = "空")]
    Short,
}

impl Direction {
    pub(crate) fn value(&self) -> i32 {
        match self {
            Direction::Long => 0,
            Direction::Short => 1,
        }
    }
}

impl From<String> for Direction {
    fn from(direction: String) -> Self {
        match direction {
            x if x == "多" => Direction::Long,
            x if x == "空" => Direction::Short,
            _ => Direction::Long,
        }
    }
}

impl From<i32> for Direction {
    fn from(direction: i32) -> Self {
        match direction {
            0 => Direction::Long,
            1 => Direction::Short,
            _ => Direction::Long,
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Currency {
    #[serde(rename = "人民币")]
    RMB,
    #[serde(rename = "美元")]
    DOLLAR,
}

impl Currency {
    pub(crate) fn value(&self) -> i32 {
        match self {
            Currency::RMB => 0,
            Currency::DOLLAR => 1,
        }
    }
}

impl From<i32> for Currency {
    fn from(currency: i32) -> Self {
        match currency {
            0 => Currency::RMB,
            1 => Currency::DOLLAR,
            _ => Currency::RMB,
        }
    }
}

impl From<String> for Currency {
    fn from(currency: String) -> Self {
        match currency {
            x if x == "人民币" => Currency::RMB,
            x if x == "美元" => Currency::DOLLAR,
            _ => Currency::RMB,
        }
    }
}

#[derive(Debug,PartialEq,Serialize,Deserialize)]
pub enum PositionType {
    #[serde(rename = "open")]
    OPEN,
    #[serde(rename = "close")]
    CLOSE,
}

impl PositionType {
    pub(crate) fn value(&self) -> i32 {
        match self {
            PositionType::OPEN => 0,
            PositionType::CLOSE => 1,
        }
    }
}

impl From<i32> for PositionType {
    fn from(ope: i32) -> Self {
        match ope {
            0 => PositionType::OPEN,
            1 => PositionType::CLOSE,
            _ => PositionType::OPEN,
        }
    }
}
