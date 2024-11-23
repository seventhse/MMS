use crate::_entities::sea_orm_active_enums::{Status, TeamUserRoles, TeamUserStatus};
use serde::{Deserialize, Serialize};

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Status::Inactive => "inactive",
            Status::Active => "active",
        })
    }
}
impl Serialize for TeamUserStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            TeamUserStatus::Joined => "joined",
            TeamUserStatus::Lefted => "lefted",
        })
    }
}

impl<'de> Deserialize<'de> for TeamUserStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "joined" => Ok(TeamUserStatus::Joined),
            "lefted" => Ok(TeamUserStatus::Lefted),
            _ => Err(serde::de::Error::custom("Invalid team user status")),
        }
    }
}

impl Serialize for TeamUserRoles {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            TeamUserRoles::Admin => "Admin",
            TeamUserRoles::Guest => "Guest",
            TeamUserRoles::Manager => "Manager",
            TeamUserRoles::Member => "Member",
            TeamUserRoles::Owner => "Owner",
        })
    }
}

impl<'de> Deserialize<'de> for TeamUserRoles {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Admin" => Ok(TeamUserRoles::Admin),
            "Guest" => Ok(TeamUserRoles::Guest),
            "Manager" => Ok(TeamUserRoles::Manager),
            "Member" => Ok(TeamUserRoles::Member),
            "Owner" => Ok(TeamUserRoles::Owner),
            _ => Err(serde::de::Error::custom("Invalid team role")),
        }
    }
}

impl TeamUserRoles {
    pub fn can_remove_team(&self) -> bool {
        self.eq(&TeamUserRoles::Owner)
    }

    pub fn can_update_team(&self) -> bool {
        [TeamUserRoles::Owner, TeamUserRoles::Admin].contains(&self)
    }

    pub fn can_remove_user_by_team(&self) -> bool {
        [TeamUserRoles::Owner, TeamUserRoles::Admin].contains(&self)
    }
}
