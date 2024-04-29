use std::{
    io::Write,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Writes `system_time` to `output` as an ISO 8601 date-time
pub(super) fn write_time(output: &mut dyn Write, system_time: SystemTime) -> std::io::Result<()> {
    let total_millis = system_time
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis();

    let milli = (total_millis % 1_000) as u16;
    let total_seconds = (total_millis / 1_000) as u64;

    let second = (total_seconds % 60) as u8;
    let total_minutes = total_seconds / 60;

    let minute = (total_minutes % 60) as u8;
    let total_hours = total_minutes / 60;

    let hour = (total_hours % 24) as u8;
    let total_days = total_hours / 24;

    let days_since_0 = total_days + 719468;
    let era = days_since_0 / 146097;
    let doe = days_since_0 - (era * 146097);
    let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + (yoe / 4) - (yoe / 400));
    let mp = ((5 * doy) + 2) / 153;
    let day = doy - ((153 * mp) + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };

    write!(
        output,
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, month, day, hour, minute, second, milli
    )
}

/// Writes `id` to `output` as lowercase hex
pub(super) fn write_id(output: &mut dyn Write, id: [u8; 16]) -> std::io::Result<()> {
    for byte in id {
        write!(output, "{:02x}", byte)?;
    }

    Ok(())
}
