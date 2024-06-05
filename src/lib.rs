use std::convert::TryInto;

#[derive(Debug)]
pub enum NtpError {
    InvalidLength,
}

#[derive(Debug)]
pub struct NtpPacket {
    pub leap_indicator: u8,
    pub version: u8,
    pub mode: u8,
    pub stratum: u8,
    pub poll: u8,
    pub precision: i8,
    pub root_delay: u32,
    pub root_dispersion: u32,
    pub reference_id: u32,
    pub reference_timestamp: u64,
    pub originate_timestamp: u64,
    pub receive_timestamp: u64,
    pub transmit_timestamp: u64,
}

impl NtpPacket {
    pub fn parse(payload: &[u8]) -> Result<NtpPacket, NtpError> {
        if payload.len() != 48 {
            return Err(NtpError::InvalidLength);
        }

        Ok(NtpPacket {
            leap_indicator: (payload[0] >> 6) & 0x03,
            version: (payload[0] >> 3) & 0x07,
            mode: payload[0] & 0x07,
            stratum: payload[1],
            poll: payload[2],
            precision: payload[3] as i8,
            root_delay: u32::from_be_bytes(payload[4..8].try_into().unwrap()),
            root_dispersion: u32::from_be_bytes(payload[8..12].try_into().unwrap()),
            reference_id: u32::from_be_bytes(payload[12..16].try_into().unwrap()),
            reference_timestamp: u64::from_be_bytes(payload[16..24].try_into().unwrap()),
            originate_timestamp: u64::from_be_bytes(payload[24..32].try_into().unwrap()),
            receive_timestamp: u64::from_be_bytes(payload[32..40].try_into().unwrap()),
            transmit_timestamp: u64::from_be_bytes(payload[40..48].try_into().unwrap()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ntp_packet() {
        let payload: Vec<u8> = vec![
            0x1b, 0x00, 0x00, 0xe9, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x01, 0x02, 0xc0, 0xa8,
            0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe3, 0x83,
            0x67, 0x45, 0x00, 0x00, 0x00, 0x00,
        ];
        let result = NtpPacket::parse(&payload).unwrap();
        assert_eq!(result.leap_indicator, 0);
        assert_eq!(result.version, 3);
        assert_eq!(result.mode, 3);
        assert_eq!(result.stratum, 0);
        assert_eq!(result.poll, 0);
        assert_eq!(result.precision, -23);
        assert_eq!(result.root_delay, 0x00000102);
        assert_eq!(result.root_dispersion, 0x00000102);
        assert_eq!(result.reference_id, 0xc0a80101);
        assert_eq!(result.reference_timestamp, 0);
        assert_eq!(result.originate_timestamp, 0);
        assert_eq!(result.receive_timestamp, 0);
        assert_eq!(result.transmit_timestamp, 0xe3836745);
    }
}
