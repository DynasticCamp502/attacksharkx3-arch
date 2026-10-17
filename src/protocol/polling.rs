//! Report 0x06 (polling rate) and Report 0x0C (commit lead burst).

use crate::protocol::constants::*;

/// Build 9-byte Report 0x06 packet for polling rate
pub fn build_polling_report(rate: PollingRate) -> [u8; LEN_REPORT_POLLING] {
    let mut buf = [0u8; LEN_REPORT_POLLING];
    buf[0] = REPORT_POLLING;
    buf[1] = LEN_REPORT_POLLING as u8;
    buf[2] = PROFILE_INDEX;
    let code = rate.wire_code();
    buf[3] = code;
    buf[4] = 0xFF - code;
    buf
}

/// Build 10-byte Report 0x0C packet (leads every apply burst)
pub fn build_commit_report() -> [u8; LEN_REPORT_COMMIT] {
    let mut buf = [0u8; LEN_REPORT_COMMIT];
    buf[0] = REPORT_COMMIT;
    buf[1] = LEN_REPORT_COMMIT as u8;
    buf[2] = 0x01;
    buf[3] = 0xFE;
    buf[4] = 0x01;
    buf[5] = 0xFE;
    buf
}
