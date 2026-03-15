use modular_bitfield::prelude::*;

// I2C slave address (7-bit, unshifted)
pub const DEVICE_ADDR: u8 = 0x2C;

// Register addresses
pub const REG_VENDOR_ID_LSB: u8 = 0x00;
pub const REG_VENDOR_ID_MSB: u8 = 0x01;
pub const REG_PRODUCT_ID_LSB: u8 = 0x02;
pub const REG_PRODUCT_ID_MSB: u8 = 0x03;
pub const REG_DEVICE_ID_LSB: u8 = 0x04;
pub const REG_DEVICE_ID_MSB: u8 = 0x05;
pub const REG_CONFIG_BYTE_1: u8 = 0x06;
pub const REG_CONFIG_BYTE_2: u8 = 0x07;
pub const REG_CONFIG_BYTE_3: u8 = 0x08;
pub const REG_NON_REMOVABLE: u8 = 0x09;
pub const REG_PORT_DISABLE_SELF: u8 = 0x0A;
pub const REG_PORT_DISABLE_BUS: u8 = 0x0B;
pub const REG_MAX_POWER_SELF: u8 = 0x0C;
pub const REG_MAX_POWER_BUS: u8 = 0x0D;
pub const REG_HUB_CURRENT_SELF: u8 = 0x0E;
pub const REG_HUB_CURRENT_BUS: u8 = 0x0F;
pub const REG_POWER_ON_TIME: u8 = 0x10;
pub const REG_LANGUAGE_ID_HIGH: u8 = 0x11;
pub const REG_LANGUAGE_ID_LOW: u8 = 0x12;
pub const REG_MANUFACTURER_STRING_LEN: u8 = 0x13;
pub const REG_PRODUCT_STRING_LEN: u8 = 0x14;
pub const REG_SERIAL_STRING_LEN: u8 = 0x15;
pub const REG_MANUFACTURER_STRING: u8 = 0x16;
pub const REG_PRODUCT_STRING: u8 = 0x54;
pub const REG_SERIAL_STRING: u8 = 0x92;
pub const REG_BATTERY_CHARGING: u8 = 0xD0;
pub const REG_BOOST_UPSTREAM: u8 = 0xF6;
pub const REG_BOOST_DOWNSTREAM: u8 = 0xF8;
pub const REG_PORT_SWAP: u8 = 0xFA;
pub const REG_PORT_MAP_12: u8 = 0xFB;
pub const REG_PORT_MAP_34: u8 = 0xFC;
pub const REG_STATUS_COMMAND: u8 = 0xFF;

/// USB251xB variant (2, 3, or 4 downstream ports).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Usb2512b,
    Usb2513b,
    Usb2514b,
}

impl Variant {
    pub const fn product_id(self) -> u16 {
        match self {
            Self::Usb2512b => 0x2512,
            Self::Usb2513b => 0x2513,
            Self::Usb2514b => 0x2514,
        }
    }
}

#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum CurrentSensing {
    Ganged = 0b00,
    Individual = 0b01,
    NotSupported = 0b10,
}

#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 1]
pub enum PowerSwitching {
    Ganged = 0,
    Individual = 1,
}

#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum OcTimer {
    Ms0_1 = 0b00,
    Ms4 = 0b01,
    Ms8 = 0b10,
    Ms16 = 0b11,
}

#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum BoostLevel {
    None = 0b00,
    Low = 0b01,
    Medium = 0b10,
    High = 0b11,
}

#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 4]
pub enum LogicalPort {
    Disabled = 0,
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ConfigByte1 {
    pub port_power: PowerSwitching,
    pub current_sensing: CurrentSensing,
    pub eop_disable: bool,
    pub mtt_enable: bool,
    pub hs_disable: bool,
    #[skip]
    __reserved: B1,
    pub self_bus_power: bool,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ConfigByte2 {
    #[skip]
    __reserved_low: B3,
    pub compound: bool,
    pub oc_timer: OcTimer,
    #[skip]
    __reserved_high: B1,
    pub dynamic_power: bool,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct ConfigByte3 {
    pub string_enable: bool,
    #[skip]
    __reserved_low: B2,
    pub port_map_enable: bool,
    #[skip]
    __reserved_high: B4,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct PortBitfield {
    pub upstream: bool,
    pub port1: bool,
    pub port2: bool,
    pub port3: bool,
    pub port4: bool,
    #[skip]
    __reserved: B3,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct BoostUpstream {
    pub level: BoostLevel,
    #[skip]
    __reserved: B6,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct BoostDownstream {
    pub port1: BoostLevel,
    pub port2: BoostLevel,
    pub port3: BoostLevel,
    pub port4: BoostLevel,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct PortMap12 {
    pub port1: LogicalPort,
    pub port2: LogicalPort,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct PortMap34 {
    pub port3: LogicalPort,
    pub port4: LogicalPort,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct StatusCommand {
    pub usb_attach: bool,
    pub reset: bool,
    pub intf_power_down: bool,
    #[skip]
    __reserved: B5,
}
