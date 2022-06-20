use core::result::Result;

#[derive(Debug)]
struct Port {
    value: u16,
}

impl Port {
    pub fn min() -> Port {
        Port { value: u16::MIN }
    }
    pub fn max() -> Port {
        Port { value: u16::MAX }
    }
}

impl TryFrom<u16> for Port {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(String::from("Port must not be 0"))
        } else {
            Ok(Port { value })
        }
    }
}
