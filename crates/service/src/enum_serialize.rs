use crate::_entities::sea_orm_active_enums::Status;
use serde::Serialize;

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
