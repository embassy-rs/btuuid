#![no_std]

//! Bluetooth UUID values, including 16-, 32- and 128-bit UUIDs.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use core::array::TryFromSliceError;
use core::fmt::{Debug, Display};

pub mod ad_types;
pub mod appearance;
pub mod browse_group_identifiers;
pub mod characteristic;
pub mod declarations;
pub mod descriptors;
pub mod mesh_profile;
pub mod object_types;
pub mod protocol_identifiers;
pub mod service;
pub mod service_class;
pub mod units;

#[deprecated(since = "0.1.0", note = "use BluetoothUuid128::base() instead.")]
#[doc(hidden)]
/// Included for bt-hci semver compatibility
pub const BLUETOOTH_BASE_UUID: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0x80, 0x5F, 0x9B, 0x34, 0xFB,
];

/// A Bluetooth UUID of any valid length (16, 32, or 128 bits).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BluetoothUuid {
    /// A 16-bit Bluetooth UUID.
    Uuid16(BluetoothUuid16),
    /// A 32-bit Bluetooth UUID.
    Uuid32(BluetoothUuid32),
    /// A 128-bit Bluetooth UUID.
    Uuid128(BluetoothUuid128),
}

/// An error returned when a byte slice has an invalid length for a Bluetooth UUID.
///
/// A valid slice must be 2, 4, or 16 bytes long.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InvalidLengthError;

impl Display for InvalidLengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Invalid byte length for Bluetooth UUID")
    }
}

impl core::error::Error for InvalidLengthError {}

impl From<TryFromSliceError> for InvalidLengthError {
    fn from(_: TryFromSliceError) -> Self {
        InvalidLengthError
    }
}

impl BluetoothUuid {
    /// Creates a nil (all-zero) UUID.
    pub const fn nil() -> Self {
        Self::Uuid128(BluetoothUuid128::nil())
    }

    /// Creates a `BluetoothUuid` from a 16-bit unsigned integer.
    pub const fn from_u16(v: u16) -> Self {
        Self::Uuid16(BluetoothUuid16::new(v))
    }

    /// Creates a `BluetoothUuid` from a 32-bit unsigned integer.
    ///
    /// This will create a `Uuid16` if the value fits, otherwise it will create a `Uuid32`.
    pub const fn from_u32(v: u32) -> Self {
        if v <= u16::MAX as u32 {
            Self::from_u16(v as u16)
        } else {
            Self::Uuid32(BluetoothUuid32::new(v))
        }
    }

    /// Creates a `BluetoothUuid` from a 128-bit unsigned integer.
    ///
    /// If the 128-bit value corresponds to a standard Bluetooth Base UUID,
    /// it will be shortened to a `Uuid16` or `Uuid32` representation.
    pub const fn from_u128(v: u128) -> Self {
        let uuid = BluetoothUuid128::new(v);
        let (_, base_d2, base_d3, base_d4) = BluetoothUuid128::base().to_fields();
        let (d1, d2, d3, d4) = uuid.to_fields();
        if d2 == base_d2 && d3 == base_d3 && d4 == base_d4 {
            Self::from_u32(d1)
        } else {
            Self::Uuid128(uuid)
        }
    }

    /// Attempts to create a `BluetoothUuid` from a little-endian byte slice.
    ///
    /// The slice must have a length of 2, 4, or 16 bytes.
    pub fn from_le_slice(b: &[u8]) -> Result<Self, InvalidLengthError> {
        match b.len() {
            2 => Ok(BluetoothUuid16::from_le_slice(b)?.into()),
            4 => Ok(BluetoothUuid32::from_le_slice(b)?.into()),
            16 => Ok(BluetoothUuid128::from_le_slice(b)?.into()),
            _ => Err(InvalidLengthError),
        }
    }

    /// Attempts to create a `BluetoothUuid` from a big-endian byte slice.
    ///
    /// The slice must have a length of 2, 4, or 16 bytes.
    pub fn from_be_slice(b: &[u8]) -> Result<Self, InvalidLengthError> {
        match b.len() {
            2 => Ok(BluetoothUuid16::from_be_slice(b)?.into()),
            4 => Ok(BluetoothUuid32::from_be_slice(b)?.into()),
            16 => Ok(BluetoothUuid128::from_be_slice(b)?.into()),
            _ => Err(InvalidLengthError),
        }
    }

    /// Converts any `BluetoothUuid` variant to its full 128-bit representation.
    pub const fn to_u128(&self) -> u128 {
        match self {
            Self::Uuid16(uuid) => BluetoothUuid128::base().set_initial_group(uuid.to_u16() as u32),
            Self::Uuid32(uuid) => BluetoothUuid128::base().set_initial_group(uuid.to_u32()),
            Self::Uuid128(uuid) => *uuid,
        }
        .to_u128()
    }

    /// Returns a little-endian byte slice representation of the UUID.
    ///
    /// The length of the slice will be 2, 4, or 16 bytes depending on the UUID variant.
    pub const fn as_le_slice(&self) -> &[u8] {
        match self {
            Self::Uuid16(uuid) => uuid.as_le_slice(),
            Self::Uuid32(uuid) => uuid.as_le_slice(),
            Self::Uuid128(uuid) => uuid.as_le_slice(),
        }
    }
}

impl PartialEq<BluetoothUuid16> for BluetoothUuid {
    fn eq(&self, other: &BluetoothUuid16) -> bool {
        matches!(self, Self::Uuid16(v) if v == other)
    }
}

impl PartialEq<BluetoothUuid> for BluetoothUuid16 {
    fn eq(&self, other: &BluetoothUuid) -> bool {
        matches!(other, BluetoothUuid::Uuid16(v) if v == self)
    }
}

impl PartialEq<BluetoothUuid32> for BluetoothUuid {
    fn eq(&self, other: &BluetoothUuid32) -> bool {
        matches!(self, Self::Uuid32(v) if v == other)
    }
}

impl PartialEq<BluetoothUuid> for BluetoothUuid32 {
    fn eq(&self, other: &BluetoothUuid) -> bool {
        matches!(other, BluetoothUuid::Uuid32(v) if v == self)
    }
}

impl PartialEq<BluetoothUuid128> for BluetoothUuid {
    fn eq(&self, other: &BluetoothUuid128) -> bool {
        matches!(self, Self::Uuid128(v) if v == other)
    }
}

impl PartialEq<BluetoothUuid> for BluetoothUuid128 {
    fn eq(&self, other: &BluetoothUuid) -> bool {
        matches!(other, BluetoothUuid::Uuid128(v) if v == self)
    }
}

impl Debug for BluetoothUuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BluetoothUuid({})", self)
    }
}

impl Display for BluetoothUuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BluetoothUuid::Uuid16(uuid) => Display::fmt(uuid, f),
            BluetoothUuid::Uuid32(uuid) => Display::fmt(uuid, f),
            BluetoothUuid::Uuid128(uuid) => Display::fmt(uuid, f),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for BluetoothUuid {
    fn format(&self, fmt: defmt::Formatter) {
        match self {
            BluetoothUuid::Uuid16(uuid) => defmt::write!(fmt, "{}", uuid),
            BluetoothUuid::Uuid32(uuid) => defmt::write!(fmt, "{}", uuid),
            BluetoothUuid::Uuid128(uuid) => defmt::write!(fmt, "{}", uuid),
        }
    }
}

impl Default for BluetoothUuid {
    fn default() -> Self {
        Self::nil()
    }
}

impl AsRef<[u8]> for BluetoothUuid {
    fn as_ref(&self) -> &[u8] {
        self.as_le_slice()
    }
}

impl From<BluetoothUuid16> for BluetoothUuid {
    fn from(value: BluetoothUuid16) -> Self {
        Self::Uuid16(value)
    }
}

impl From<u16> for BluetoothUuid {
    fn from(value: u16) -> Self {
        Self::from_u16(value)
    }
}

impl From<[u8; 2]> for BluetoothUuid {
    fn from(value: [u8; 2]) -> Self {
        // Assumes value is in little-endian order
        Self::Uuid16(BluetoothUuid16(value))
    }
}

impl From<BluetoothUuid32> for BluetoothUuid {
    fn from(value: BluetoothUuid32) -> Self {
        Self::from_u32(value.to_u32())
    }
}

impl From<u32> for BluetoothUuid {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<[u8; 4]> for BluetoothUuid {
    fn from(value: [u8; 4]) -> Self {
        // Assumes value is in little-endian order
        Self::Uuid32(BluetoothUuid32(value))
    }
}

impl From<BluetoothUuid128> for BluetoothUuid {
    fn from(value: BluetoothUuid128) -> Self {
        Self::from_u128(value.to_u128())
    }
}

impl From<u128> for BluetoothUuid {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<[u8; 16]> for BluetoothUuid {
    fn from(value: [u8; 16]) -> Self {
        // Assumes value is in little-endian order
        Self::Uuid128(BluetoothUuid128(value))
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for BluetoothUuid {
    fn from(value: uuid::Uuid) -> Self {
        BluetoothUuid128::new(value.as_u128()).into()
    }
}

#[cfg(feature = "uuid")]
impl From<BluetoothUuid> for uuid::Uuid {
    fn from(value: BluetoothUuid) -> Self {
        uuid::Uuid::from_u128(value.to_u128())
    }
}

#[cfg(feature = "alloc")]
impl From<BluetoothUuid> for alloc::vec::Vec<u8> {
    fn from(value: BluetoothUuid) -> Self {
        value.as_le_slice().to_vec()
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<alloc::vec::Vec<u8>> for BluetoothUuid {
    type Error = InvalidLengthError;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_le_slice(&value)
    }
}

macro_rules! impl_uuid {
    {
        $(#[$attrs:meta])*
        struct $name:ident($num:ty, $bytes:expr, $to:ident);
    } => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        $(#[$attrs])*
        pub struct $name([u8; $bytes]);

        impl $name {
            /// Creates a new UUID.
            pub const fn new(uuid: $num) -> Self {
                Self(uuid.to_le_bytes())
            }

            /// Creates a UUID from a little-endian byte array.
            pub const fn from_le_bytes(bytes: [u8; $bytes]) -> Self {
                Self(bytes)
            }

            /// Creates a UUID from a big-endian byte array.
            pub fn from_be_bytes(mut bytes: [u8; $bytes]) -> Self {
                bytes.reverse();
                Self(bytes)
            }

            /// Attempts to create a UUID from a little-endian byte slice.
            pub fn from_le_slice(bytes: &[u8]) -> Result<Self, InvalidLengthError> {
                Ok(Self::from_le_bytes(bytes.try_into()?))
            }

            /// Attempts to create a UUID from a big-endian byte slice.
            pub fn from_be_slice(bytes: &[u8]) -> Result<Self, InvalidLengthError> {
                let mut bytes: [u8; $bytes] = bytes.try_into()?;
                bytes.reverse();
                Ok(Self::from_le_bytes(bytes))
            }

            /// Returns the UUID as a little-endian byte array.
            pub const fn as_le_bytes(&self) -> &[u8; $bytes] {
                &self.0
            }

            /// Consumes the UUID and returns its little-endian byte array representation.
            pub const fn to_le_bytes(self) -> [u8; $bytes] {
                self.0
            }

            /// Consumes the UUID and returns its big-endian byte array representation.
            pub fn to_be_bytes(self) -> [u8; $bytes] {
                let mut bytes = self.0;
                bytes.reverse();
                bytes
            }

            /// Returns the UUID as a little-endian byte slice.
            pub const fn as_le_slice(&self) -> &[u8] {
                &self.0
            }

            /// Returns the integer representation of this UUID.
            pub const fn $to(self) -> $num {
                <$num>::from_le_bytes(self.0)
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.as_le_bytes()
            }
        }

        impl From<$num> for $name {
            fn from(value: $num) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for $num {
            fn from(value: $name) -> Self {
                value.$to()
            }
        }

        impl From<[u8; $bytes]> for $name {
            fn from(value: [u8; $bytes]) -> Self {
                // Assumes value is in little-endian order
                Self(value)
            }
        }

        impl From<$name> for [u8; $bytes] {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        #[cfg(feature = "uuid")]
        impl From<$name> for uuid::Uuid {
            fn from(value: $name) -> Self {
                BluetoothUuid::from(value).into()
            }
        }
    };

    {
        $(#[$attrs:meta])*
        struct $name:ident($num:ty, $bytes:expr, $to:ident, $fmt:literal, $defmt:literal);
    } => {
        impl_uuid! { $(#[$attrs])* struct $name($num, $bytes, $to); }

        impl Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, concat!(stringify!($name), "(0x{})"), self)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $fmt, self.$to())
            }
        }

        #[cfg(feature = "defmt")]
        impl defmt::Format for $name {
            fn format(&self, f: defmt::Formatter) {
                defmt::write!(f, $defmt, self.0)
            }
        }
    };
}

impl_uuid! {
    /// A 16-bit Bluetooth UUID
    struct BluetoothUuid16(u16, 2, to_u16, "0x{:04X}", "BluetoothUuid16({:04X})");
}
impl_uuid! {
    /// A 32-bit Bluetooth UUID
    struct BluetoothUuid32(u32, 4, to_u32, "0x{:08X}", "BluetoothUuid32({:08X})");
}
impl_uuid! {
    /// A 128-bit Bluetooth UUID
    struct BluetoothUuid128(u128, 16, to_u128);
}

impl BluetoothUuid128 {
    /// The standard Bluetooth Base UUID.
    ///
    /// The value is `00000000-0000-1000-8000-00805F9B34FB`.
    ///
    /// The full 128-bit value of a 16-bit or 32-bit UUID may be computed by a simple arithmetic operation.
    ///
    /// `128_bit_value = 16_bit_value * 2^96 + Bluetooth_Base_UUID`
    ///
    /// See BLUETOOTH CORE SPECIFICATION Version 6.0 | Vol 3, Part B | Page 1250
    /// [(link)](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-54/out/en/host/service-discovery-protocol--sdp--specification.html#UUID-ef710684-4c7e-6793-4350-4a190ea9a7a4)
    pub const fn base() -> Self {
        BluetoothUuid128::new(0x00000000_0000_1000_8000_00805F9B34FB)
    }

    /// Creates a new nil (all-zero) 128-bit UUID.
    pub const fn nil() -> Self {
        Self([0; 16])
    }

    /// Returns the raw components of the 128-bit UUID.
    ///
    /// The fields are returned in big-endian order: `(time_low, time_mid, time_high_and_version, clock_seq_and_node)`.
    pub const fn to_fields(&self) -> (u32, u16, u16, u64) {
        let b = &self.0;
        let d4 = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        let d3 = u16::from_le_bytes([b[8], b[9]]);
        let d2 = u16::from_le_bytes([b[10], b[11]]);
        let d1 = u32::from_le_bytes([b[12], b[13], b[14], b[15]]);
        (d1, d2, d3, d4)
    }

    /// Sets the value of the initial 32-bits of the UUID (in big-endian format) to `val`.
    ///
    /// This can be used to construct a 128-bit UUID value from a 16- or 32-bit UUID
    /// value using [`Self::base()`] as the base 128-bit UUID value.
    pub const fn set_initial_group(mut self, val: u32) -> Self {
        let bytes = val.to_le_bytes();
        self.0[12] = bytes[0];
        self.0[13] = bytes[1];
        self.0[14] = bytes[2];
        self.0[15] = bytes[3];
        self
    }
}

impl Default for BluetoothUuid128 {
    fn default() -> Self {
        Self::nil()
    }
}

impl Debug for BluetoothUuid128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, concat!(stringify!($name), "(0x{})"), self)
    }
}

impl Display for BluetoothUuid128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let (d1, d2, d3, d4) = self.to_fields();
        write!(
            f,
            "{:08X}-{:04X}-{:04X}-{:04X}-{:012X}",
            d1,
            d2,
            d3,
            d4 >> 48,
            d4 & ((1 << 48) - 1)
        )
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for BluetoothUuid128 {
    fn format(&self, f: defmt::Formatter) {
        let (d1, d2, d3, d4) = self.to_fields();
        defmt::write!(
            f,
            "BluetoothUuid128({=u32:08X}-{=u16:04X}-{=u16:04X}-{=u16:04X}-{=u64:012X})",
            d1,
            d2,
            d3,
            (d4 >> 48) as u16,
            d4 & ((1 << 48) - 1)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ble_uuid() {
        const BLE_UUID: BluetoothUuid16 = BluetoothUuid16::new(0x1234);
        assert_eq!(u16::from(BLE_UUID), 0x1234);
        let uuid: u16 = BLE_UUID.into();
        assert_eq!(uuid, 0x1234);
        const UUID: [u8; 2] = BLE_UUID.to_le_bytes();
        assert_eq!(UUID, [0x34, 0x12]);
    }

    #[cfg(feature = "uuid")]
    #[test]
    fn test_uuid_conversion() {
        let result = uuid::Uuid::from(BluetoothUuid16::new(0x1234));
        let expected = "00001234-0000-1000-8000-00805f9b34fb".parse::<uuid::Uuid>().unwrap();

        // defmt::Format not implemented on Uuid
        assert_eq!(result.into_bytes(), expected.into_bytes());
    }

    #[test]
    fn test_enum_partial_eq_16() {
        let value = BluetoothUuid16::new(42);
        let variant = BluetoothUuid::Uuid16(value);
        assert_eq!(variant, value);
        assert_eq!(value, variant);
        assert_ne!(variant, BluetoothUuid16::new(13));
    }

    #[test]
    fn test_enum_partial_eq_32() {
        let value = BluetoothUuid32::new(42);
        let variant = BluetoothUuid::Uuid32(value);
        assert_eq!(variant, value);
        assert_eq!(value, variant);
        assert_ne!(variant, BluetoothUuid32::new(13));
    }

    #[test]
    fn test_enum_partial_eq_128() {
        let value = BluetoothUuid128::new(42);
        let variant = BluetoothUuid::Uuid128(value);
        assert_eq!(variant, value);
        assert_eq!(value, variant);
        assert_ne!(variant, BluetoothUuid128::new(13));
    }
}
