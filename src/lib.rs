#![warn(clippy::pedantic)]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::non_ascii_literal)]

#[cfg(test)]
mod tests {
    #[test]
    fn ascii_to_unicode() {
        assert_eq!(
            Ok("Hello"),
            std::str::from_utf8(b"Hello")
        );
    }

    #[test]
    fn encode_ascii() {
        assert_eq!(
            b"Hello",
            "Hello".as_bytes()
        );
    }

    #[test]
    fn symbols() {
        assert_eq!(
            b"\x41\xe2\x89\xa2\xce\x91\x2e",
            "A≢Α.".as_bytes()
        );
        assert_eq!(
            b"\xe2\x82\xac",
            "€".as_bytes()
        );
    }

    #[test]
    fn encode_japanese() {
        assert_eq!(
            b"\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e",
            "日本語".as_bytes()
        );
    }

    #[test]
    fn stump_of_tree_encoding() {
        assert_eq!(
            b"\xf0\xa3\x8e\xb4",
            "𣎴".as_bytes()
        );
    }

    #[test]
    fn code_point_beyond_end_of_last_valid_range() {
        assert_eq!(
            None,
            std::char::from_u32(0x20_0000_u32)
        );
        assert_eq!(
            None,
            std::char::from_u32(0x11_0000_u32)
        );
    }

    #[test]
    fn high_and_low_surrogate_halves_are_invalid() {
        let mut buffer = [0; 3];
        assert_eq!(
            3,
            std::char::from_u32(0xd7ff).unwrap().encode_utf8(&mut buffer).len()
        );
        assert_eq!(
            b"\xed\x9f\xbf",
            &buffer
        );
        assert_eq!(
            None,
            std::char::from_u32(0xd800)
        );
        assert_eq!(
            None,
            std::char::from_u32(0xd801)
        );
        assert_eq!(
            None,
            std::char::from_u32(0xd803)
        );
        assert_eq!(
            None,
            std::char::from_u32(0xdfef)
        );
        assert_eq!(
            None,
            std::char::from_u32(0xdffe)
        );
        assert_eq!(
            None,
            std::char::from_u32(0xdfff)
        );
        assert_eq!(
            3,
            std::char::from_u32(0xe000).unwrap().encode_utf8(&mut buffer).len()
        );
        assert_eq!(
            b"\xee\x80\x80",
            &buffer
        );
    }

    #[test]
    fn decode_valid_sequences() {
        struct TestVector {
            encoding: &'static str,
            expected_decoding: &'static [u32]
        };
        let test_vectors = [
            TestVector{ encoding: "𣎴", expected_decoding: &[ 0x233B4 ] },
            TestVector{ encoding: "日本語", expected_decoding: &[ 0x65E5, 0x672C, 0x8A9E ] },
            TestVector{ encoding: "A≢Α.", expected_decoding: &[ 0x0041, 0x2262, 0x0391, 0x002E ] },
            TestVector{ encoding: "€", expected_decoding: &[ 0x20AC ] },
            TestVector{ encoding: "Hello", expected_decoding: &[ 0x48, 0x65, 0x6C, 0x6C, 0x6F ] },
        ];
        for test_vector in &test_vectors {
            assert_eq!(
                test_vector.expected_decoding,
                test_vector.encoding.chars()
                    .map(|ch| ch as u32)
                    .collect::<Vec<u32>>()
            );
        }
    }

    #[test]
    fn decode_from_input_vector() {
        let expected_decoding: &'static [u32] = &[ 0x65E5, 0x672C, 0x8A9E ];
        assert_eq!(
            expected_decoding,
            String::from_utf8_lossy(b"\xE6\x97\xA5\xE6\x9C\xAC\xE8\xAA\x9E")
                .chars()
                .map(|ch| ch as u32)
                .collect::<Vec<u32>>()
        ); // 日本語"
    }

    #[test]
    fn unexpected_continuation_bytes() {
        let expected_decoding: &'static [u32] = &[ 0x41, 0x2262, 0xfffd, 0x2e ];
        assert_eq!(
            expected_decoding,
            String::from_utf8_lossy(b"\x41\xe2\x89\xa2\x91\x2e")
                .chars()
                .map(|ch| ch as u32)
                .collect::<Vec<u32>>()
        ); // A≢�.
    }

    #[test]
    fn decode_break_in_sequence() {
        let expected_decoding: &'static [u32] = &[ 0x41, 0x2262, 0xfffd, 0x2e ];
        assert_eq!(
            expected_decoding,
            String::from_utf8_lossy(b"\x41\xe2\x89\xa2\xce\x2e")
                .chars()
                .map(|ch| ch as u32)
                .collect::<Vec<u32>>()
        ); // A≢�.
    }

    #[test]
    fn reject_overlong_sequences() {
        struct TestVector {
            encoding: &'static [u8],
            expected_decoding: &'static str
        };
        let test_vectors = [
            // All U+2F ('/') -- should only need 1 byte
            TestVector{ encoding: b"\xc0\xaf", expected_decoding: "��" },
            TestVector{ encoding: b"\xe0\x80\xaf", expected_decoding: "���" },
            TestVector{ encoding: b"\xf0\x80\x80\xaf", expected_decoding: "����" },

            // One less than the minimum code point value
            // that should require this many encoded bytes
            TestVector{ encoding: b"\xc1\xbf", expected_decoding: "��" },
            TestVector{ encoding: b"\xe0\x9f\xbf", expected_decoding: "���" },
            TestVector{ encoding: b"\xf0\x8f\xbf\xbf", expected_decoding: "����" },
        ];
        for test_vector in &test_vectors {
            assert_eq!(
                test_vector.expected_decoding,
                String::from_utf8_lossy(test_vector.encoding)
            );
        }
    }

    #[test]
    fn stump_of_tree_decoded_in_two_parts() {
        assert_eq!(
            (None, 2),
            bstr::decode_utf8(b"\xf0\xa3")
        );
        assert_eq!(
            (Some('𣎴'), 4),
            bstr::decode_utf8(b"\xf0\xa3\x8e\xb4")
        );
    }

    #[test]
    fn is_valid_encoding() {
        assert!(std::str::from_utf8("abc".as_bytes()).ok().is_some());
        assert!(std::str::from_utf8("𣎴".as_bytes()).ok().is_some());
        assert!(std::str::from_utf8("A≢�".as_bytes()).ok().is_some());
        assert!(std::str::from_utf8(b"\x41\xE2\x89\xA2\xCE\x2E").ok().is_none());
        assert!(std::str::from_utf8(b"\xE6\x97\xA5\xE6\x9C\xAC\xE8\xAA").ok().is_none());
        assert!(std::str::from_utf8(b"\xaa").ok().is_none());
        assert!(std::str::from_utf8("A≢".as_bytes()).ok().is_some());
        assert!(std::str::from_utf8("�".as_bytes()).ok().is_some());
    }
}
