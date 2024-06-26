// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Ephemeris data.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub enum EphemerisData {
    /// <p>Ephemeris data in Orbit Ephemeris Message (OEM) format.</p>
    Oem(crate::types::OemEphemeris),
    /// <p>Two-line element set (TLE) ephemeris.</p>
    Tle(crate::types::TleEphemeris),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl EphemerisData {
    /// Tries to convert the enum instance into [`Oem`](crate::types::EphemerisData::Oem), extracting the inner [`OemEphemeris`](crate::types::OemEphemeris).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_oem(&self) -> std::result::Result<&crate::types::OemEphemeris, &Self> {
        if let EphemerisData::Oem(val) = &self {
            Ok(val)
        } else {
            Err(self)
        }
    }
    /// Returns true if this is a [`Oem`](crate::types::EphemerisData::Oem).
    pub fn is_oem(&self) -> bool {
        self.as_oem().is_ok()
    }
    /// Tries to convert the enum instance into [`Tle`](crate::types::EphemerisData::Tle), extracting the inner [`TleEphemeris`](crate::types::TleEphemeris).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_tle(&self) -> std::result::Result<&crate::types::TleEphemeris, &Self> {
        if let EphemerisData::Tle(val) = &self {
            Ok(val)
        } else {
            Err(self)
        }
    }
    /// Returns true if this is a [`Tle`](crate::types::EphemerisData::Tle).
    pub fn is_tle(&self) -> bool {
        self.as_tle().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
