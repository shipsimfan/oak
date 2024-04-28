use std::{io::Write, time::SystemTime};

/// Writes `system_time` to `output` as an ISO 8601 date-time
pub(super) fn write_time(output: &mut dyn Write, system_time: SystemTime) -> std::io::Result<()> {
    todo!()
}

/// Writes `id` to `output` as lowercase hex
pub(super) fn write_id(output: &mut dyn Write, id: [u8; 16]) -> std::io::Result<()> {
    for byte in id {
        write!(output, "{:02x}", byte)?;
    }

    Ok(())
}
