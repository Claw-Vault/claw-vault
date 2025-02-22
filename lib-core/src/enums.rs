use serde::{Deserialize, Serialize};

pub enum ValidDuration {
    Minute = 60,
    QuarterHour = 900,
    HalfHour = 1800,
}

impl ValidDuration {
    pub fn get_duration(&self) -> i16 {
        match self {
            ValidDuration::Minute => 60,
            ValidDuration::QuarterHour => 900,
            ValidDuration::HalfHour => 1800,
        }
    }
}

impl Serialize for ValidDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i16(self.get_duration())
    }
}
impl<'de> Deserialize<'de> for ValidDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let d = i16::deserialize(deserializer)?;
        match d {
            60 => Ok(Self::Minute),
            900 => Ok(Self::QuarterHour),
            1800 => Ok(Self::HalfHour),
            _ => Err(serde::de::Error::custom("Invalid duration value")),
        }
    }
}

impl utoipa::ToSchema for ValidDuration {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ValidDuration")
    }
}
impl utoipa::PartialSchema for ValidDuration {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::Integer)
            .enum_values::<[i16; 3usize], i16>(Some([60, 900, 1800]))
            .description(Some("Supported valid durations"))
            .into()
    }
}
