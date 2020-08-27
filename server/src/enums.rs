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

// #[derive(Serialize, Deserialize, Debug)]
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

// use postgres_types::private::BytesMut;
// use bytes::BytesMut;
// use postgres_protocol::types;
// use std::error::Error;
//
// use postgres_types::{FromSql,ToSql,IsNull,Type};
//
// impl<'a> FromSql<'a> for UserRole {
//     fn from_sql(_: &Type, raw: &[u8]) -> Result<Option<UserRole>, Box<dyn Error + Sync + Send>> {
//         let str = types::text_from_sql(raw)?;
//         Ok(UserRole::from_str(str))
//     }
//
//     fn from_sql_null(ty: &Type) -> Result<Self, Box<dyn Error + Sync + Send>> {
//         Ok(UserRole::User) //TODO
//     }
//
//     fn from_sql_nullable(ty: &Type, raw: Option<&'a [u8]>) -> Result<Self, Box<dyn Error + Sync + Send>> {
//         Ok(UserRole)
//     }
//
//     fn accepts(ty: &Type) -> bool {
//         true
//     }
//
//     // accepts!(User);
// }
//
// impl ToSql for UserRole {
//     fn to_sql(&self, _: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
//         types::text_to_sql(self.as_str(), w);
//         Ok(IsNull::No)
//     }
//
//     fn accepts(ty: &Type) -> bool where
//         Self: Sized {
//         true
//     }
//
//     fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
//         unimplemented!()
//     }
//
//     // accepts!(UUID);
//     // to_sql_checked!();
// }