pub enum SimulationResultValidity {
    Valid,
    Invalid,
}

impl SimulationResultValidity {
    pub fn from_str(s: &str) -> Option<SimulationResultValidity> {
        match s {
            "valid" => Some(SimulationResultValidity::Valid),
            "invalid" => Some(SimulationResultValidity::Invalid),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SimulationResultValidity::Valid => "valid",
            SimulationResultValidity::Invalid => "invalid",
        }
    }
}

pub enum SimulationResultStatus {
    Success,
    Failed,
    Error,
    Unknown,
}

impl SimulationResultStatus {
    pub fn from_str(s: &str) -> Option<SimulationResultStatus> {
        match s {
            "success" => Some(SimulationResultStatus::Success),
            "failed" => Some(SimulationResultStatus::Failed),
            "error" => Some(SimulationResultStatus::Error),
            "unknown" => Some(SimulationResultStatus::Unknown),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SimulationResultStatus::Success => "success",
            SimulationResultStatus::Failed => "failed",
            SimulationResultStatus::Error => "error",
            SimulationResultStatus::Unknown => "unknown",
        }
    }
}

pub enum StatisticValidity {
    Valid,
    Invalid,
}

impl StatisticValidity {
    pub fn from_str(s: &str) -> Option<StatisticValidity> {
        match s {
            "valid" => Some(StatisticValidity::Valid),
            "invalid" => Some(StatisticValidity::Invalid),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StatisticValidity::Valid => "valid",
            StatisticValidity::Invalid => "invalid",
        }
    }
}

pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub fn from_str(s: &str) -> Option<UserRole> {
        match s {
            "admin" => Some(UserRole::Admin),
            "user" => Some(UserRole::User),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }
}