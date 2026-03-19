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

/// Overcurrent sensing mode.
#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum CurrentSensing {
    Ganged = 0b00,
    Individual = 0b01,
    NotSupported = 0b10,
}

/// Port power switching mode.
#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 1]
pub enum PowerSwitching {
    Ganged = 0,
    Individual = 1,
}

/// Overcurrent detection timer period.
#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum OcTimer {
    Ms0_1 = 0b00,
    Ms4 = 0b01,
    Ms8 = 0b10,
    Ms16 = 0b11,
}

/// USB signal boost level for upstream or downstream ports.
#[derive(Specifier, Debug, Clone, Copy, PartialEq, Eq)]
#[bits = 2]
pub enum BoostLevel {
    None = 0b00,
    Low = 0b01,
    Medium = 0b10,
    High = 0b11,
}

/// Logical port number for physical-to-logical port remapping.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_product_ids() {
        assert_eq!(Variant::Usb2512b.product_id(), 0x2512);
        assert_eq!(Variant::Usb2513b.product_id(), 0x2513);
        assert_eq!(Variant::Usb2514b.product_id(), 0x2514);
    }

    #[test]
    fn config_byte1_default_roundtrip() {
        let byte = ConfigByte1::from_bytes([0x9B]);
        assert_eq!(byte.self_bus_power(), true);
        assert_eq!(byte.mtt_enable(), true);
        assert_eq!(byte.port_power(), PowerSwitching::Individual);
        assert_eq!(byte.current_sensing(), CurrentSensing::Individual);
        assert_eq!(byte.hs_disable(), false);
        assert_eq!(byte.eop_disable(), true);
        assert_eq!(byte.into_bytes(), [0x9B]);
    }

    #[test]
    fn config_byte2_default_roundtrip() {
        // 0x20 = 0b0010_0000: bits 5:4 = 0b10 = Ms8
        let byte = ConfigByte2::from_bytes([0x20]);
        assert_eq!(byte.oc_timer(), OcTimer::Ms8);
        assert_eq!(byte.compound(), false);
        assert_eq!(byte.dynamic_power(), false);
        assert_eq!(byte.into_bytes(), [0x20]);
    }

    #[test]
    fn config_byte1_builder() {
        let byte = ConfigByte1::new()
            .with_self_bus_power(true)
            .with_mtt_enable(true)
            .with_current_sensing(CurrentSensing::Ganged)
            .with_port_power(PowerSwitching::Ganged);
        assert_eq!(byte.into_bytes(), [0x90]);
    }

    #[test]
    fn port_bitfield_individual_ports() {
        let pf = PortBitfield::new().with_port1(true).with_port3(true);
        assert_eq!(pf.into_bytes(), [0b0000_1010]);
        assert_eq!(pf.port1(), true);
        assert_eq!(pf.port2(), false);
        assert_eq!(pf.port3(), true);
        assert_eq!(pf.port4(), false);
    }

    #[test]
    fn boost_downstream_all_ports() {
        let bd = BoostDownstream::new()
            .with_port1(BoostLevel::Low)
            .with_port2(BoostLevel::Medium)
            .with_port3(BoostLevel::High)
            .with_port4(BoostLevel::None);
        assert_eq!(bd.into_bytes(), [0b00_11_10_01]);
        assert_eq!(bd.port1(), BoostLevel::Low);
        assert_eq!(bd.port2(), BoostLevel::Medium);
        assert_eq!(bd.port3(), BoostLevel::High);
        assert_eq!(bd.port4(), BoostLevel::None);
    }

    #[test]
    fn port_map_roundtrip() {
        let pm = PortMap12::new()
            .with_port1(LogicalPort::Port3)
            .with_port2(LogicalPort::Port1);
        assert_eq!(pm.port1(), LogicalPort::Port3);
        assert_eq!(pm.port2(), LogicalPort::Port1);
    }

    #[test]
    fn status_command_attach() {
        let cmd = StatusCommand::new().with_usb_attach(true);
        assert_eq!(cmd.into_bytes(), [0x01]);
    }

    #[test]
    fn status_command_reset() {
        let cmd = StatusCommand::new().with_reset(true);
        assert_eq!(cmd.into_bytes(), [0x02]);
    }
}
