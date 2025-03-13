use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub enum ValidDuration {
    Minute,
    QuarterHour,
    HalfHour,
}

impl ValidDuration {
    /// Returns duration in seconds
    pub fn get_duration(&self) -> i32 {
        match self {
            ValidDuration::Minute => 60,
            ValidDuration::QuarterHour => 900,
            ValidDuration::HalfHour => 1800,
        }
    }

    /// Returns human readable string
    pub fn to_string(&self) -> String {
        match self {
            ValidDuration::Minute => "1 minute",
            ValidDuration::QuarterHour => "15 minutes",
            ValidDuration::HalfHour => "30 minutes",
        }
        .into()
    }
}

impl Serialize for ValidDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.get_duration())
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

impl From<i32> for ValidDuration {
    fn from(value: i32) -> Self {
        match value {
            900 => Self::QuarterHour,
            1800 => Self::HalfHour,
            _ => Self::Minute,
        }
    }
}
