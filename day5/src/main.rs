use intbits::Bits;

fn main() {
    let mut x: u32 = 0;
    x.set_bit(2, true);
    println!("{}", x);
    let (row, column) = decode_seat("FBFBBFFRLR").unwrap();
    println!("row: {},  column: {}", row, column);

}

fn decode_seat(partition_code: &str) -> Option<(u32, u32)> {
    let mut row: u32 = 0;
    let mut column: u32 = 0;

    let row_bits = 7;
    let column_bits = 3;

    if partition_code.len() != row_bits + column_bits {
        return None;
    }

    // Decode the row
    for (i, ch) in partition_code[..row_bits].chars().enumerate() {
        let bit = match ch {
            'F' => false, // Front of the partition
            'B' => true, // Back of the partition
            _ => return None, // `partition_code` is invalid.
        };
        row.set_bit(row_bits - i - 1, bit);
    }

    // Decode the column
    for (i, ch) in partition_code[row_bits..].chars().enumerate() {
        let bit = match ch {
            'L' => false, // Left of the partition
            'R' => true, // Right of the partition
            _ => return None, // `partition_code` is invalid.
        };
        column.set_bit(column_bits - i - 1, bit);
    }
    return Some((row, column));
}