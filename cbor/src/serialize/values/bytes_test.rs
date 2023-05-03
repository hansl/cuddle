#![cfg(feature = "std")]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn bytes_small() {
    let bytes = [1, 2, 3, 4, 5, 6];

    assert_value(values::bytes(&bytes), "46010203040506");
}

#[test]
fn bytes_60() {
    // 260 elements (enough for 2 bytes).
    let bytes = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
    ];

    assert_value(
        values::bytes(&bytes),
        concat!(
            "583c",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
        ),
    );
}

#[test]
fn bytes_260() {
    // 260 elements (enough for 2 bytes).
    let bytes = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0,
    ];

    assert_value(
        values::bytes(&bytes),
        concat!(
            "590104",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900",
            "0102030405060708090001020304050607080900"
        ),
    );
}

#[test]
fn indefinite_bytes_spec() {
    let chunk1: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let chunk2: [u8; 3] = [0xEE, 0xFF, 0x99];
    let chunks: [&[u8]; 2] = [&chunk1, &chunk2];

    assert_value(values::indefinite_bytes(&chunks), "5f44aabbccdd43eeff99ff");
}
