use std::io::Write;

/// Writes `id` to `output` as lowercase hex
pub(super) fn write_id(output: &mut dyn Write, id: [u8; 16]) -> std::io::Result<()> {
    for byte in id {
        write!(output, "{:02x}", byte)?;
    }

    Ok(())
}
