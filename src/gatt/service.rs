use uuid::*;

pub enum Service {
    AlertNotification,
    Battery,
    BloodPressure,
    CurrentTime,
    CyclingPower,
    CyclingSpeedAndCadence,
    DeviceInformation,
    GenericAccess,
    GenericAttribute,
    Glucose,
    HealthThermometer,
    HeartRate,
    HumanInterfaceDevice,
    ImmediateAlert,
    LinkLoss,
    LocationAndNavigation,
    NextDstChange,
    PhoneAlertStatus,
    ReferenceTimeUpdate,
    RunningSpeedAndCadence,
    ScanParameters,
    TXPower,
    AutomationIO,
    BatteryService,
    NetworkAvailablity,
    Unknown(UUID)
}

impl Service {
    pub fn to_uuid(&self) -> UUID {
        UUID::Custom(match *self {
            Service::AlertNotification => 0x1811,
            Service::Battery => 0x180f,
            Service::BloodPressure => 0x1810,
            Service::CurrentTime => 0x1805,
            Service::CyclingPower => 0x1818,
            Service::CyclingSpeedAndCadence => 0x1816,
            Service::DeviceInformation => 0x180a,
            Service::GenericAccess => 0x1800,
            Service::GenericAttribute => 0x1801,
            Service::Glucose => 0x1808,
            Service::HealthThermometer => 0x1809,
            Service::HeartRate => 0x180d,
            Service::HumanInterfaceDevice => 0x1812,
            Service::ImmediateAlert => 0x1802,
            Service::LinkLoss => 0x1803,
            Service::LocationAndNavigation => 0x1819,
            Service::NextDstChange => 0x1807,
            Service::PhoneAlertStatus => 0x180e,
            Service::ReferenceTimeUpdate => 0x1806,
            Service::RunningSpeedAndCadence => 0x1814,
            Service::ScanParameters => 0x1813,
            Service::TXPower => 0x1804,
            Service::AutomationIO => 0x1815,
            Service::BatteryService => 0x180f,
            Service::NetworkAvailablity => 0x180b,
            Service::Unknown(ref uuid) => *uuid.to_hex(),
        })
    }

    pub fn from_uuid(uuid: UUID) -> Service {
        match uuid.to_hex() {
            0x1811 => Service::AlertNotification,
            0x180f => Service::Battery,
            0x1810 => Service::BloodPressure,
            0x1805 => Service::CurrentTime,
            0x1818 => Service::CyclingPower,
            0x1816 => Service::CyclingSpeedAndCadence,
            0x180a => Service::DeviceInformation,
            0x1800 => Service::GenericAccess,
            0x1801 => Service::GenericAttribute,
            0x1808 => Service::Glucose,
            0x1809 => Service::HealthThermometer,
            0x180d => Service::HeartRate,
            0x1812 => Service::HumanInterfaceDevice,
            0x1802 => Service::ImmediateAlert,
            0x1803 => Service::LinkLoss,
            0x1819 => Service::LocationAndNavigation,
            0x1807 => Service::NextDstChange,
            0x180e => Service::PhoneAlertStatus,
            0x1806 => Service::ReferenceTimeUpdate,
            0x1814 => Service::RunningSpeedAndCadence,
            0x1813 => Service::ScanParameters,
            0x1804 => Service::TXPower,
            0x1815 => Service::AutomationIO,
            0x180b => Service::NetworkAvailablity,
            _ => Service::Undefined(uuid),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Service::AlertNotification => "Alert Notification Service",
            Service::Battery => "Battery Service",
            Service::BloodPressure => "Blood Pressure",
            Service::CurrentTime => "Current Time Service",
            Service::CyclingPower => "Device Information",
            Service::CyclingSpeedAndCadence => "Cycling Speed and Cadence",
            Service::DeviceInformation => "Device Information",
            Service::GenericAccess => "Generic Access",
            Service::GenericAttribute => "Generic Attribute",
            Service::Glucose => "Glucose Service",
            Service::HealthThermometer => "Health Thermometer Service",
            Service::HeartRate => "Heart Rate Service",
            Service::HumanInterfaceDevice => "Human Interface Device",
            Service::ImmediateAlert => "Immediate Alert Service",
            Service::LinkLoss => "Link Loss Service",
            Service::LocationAndNavigation => "Location and Navigation Service",
            Service::NextDstChange => "Next DST Change Service",
            Service::PhoneAlertStatus => "Phone Alert Status Service",
            Service::ReferenceTimeUpdate => "Reference Time Update Service",
            Service::RunningSpeedAndCadence => "Running Speed and Cadence Service",
            Service::ScanParameters => "Scan Parameters",
            Service::TXPower => "TX Power",
            Service::AutomationIO => "Automation IO",
            Service::NetworkAvailablity => "Network Availability Service",
            Service::Undefined => "Undefined",
        }
    }
}