use gpt;

use gpt::disk::LogicalBlockSize;

#[test]
fn test_logical_block_size_into_u64() {
    assert_eq!(<LogicalBlockSize as Into<u64>>::into(LogicalBlockSize::Lb512), 512);
    assert_eq!(<LogicalBlockSize as Into<u64>>::into(LogicalBlockSize::Lb4096), 4096);
}

#[test]
fn test_logical_block_size_into_usize() {
    assert_eq!(<LogicalBlockSize as Into<usize>>::into(LogicalBlockSize::Lb512), 512);
    assert_eq!(<LogicalBlockSize as Into<usize>>::into(LogicalBlockSize::Lb4096), 4096);
}

#[test]
fn test_logical_block_size_display() {
    assert_eq!(LogicalBlockSize::Lb512.to_string(), "512");
    assert_eq!(LogicalBlockSize::Lb4096.to_string(), "4096");
}
