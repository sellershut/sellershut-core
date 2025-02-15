tonic::include_proto!("common.pagination");

#[cfg(feature = "rpc-server-categories")]
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use cursor::Index;

/// Pagination cursor
#[derive(Debug, PartialEq, Eq)]
#[cfg(feature = "rpc-server-categories")]
pub struct CursorBuilder {
    id: String,
    dt: String,
}

#[cfg(feature = "rpc-server-categories")]
impl CursorBuilder {
    /// Create cursor
    pub fn new(id: &str, dt: &str) -> Self {
        Self {
            id: id.to_string(),
            dt: dt.to_string(),
        }
    }
    /// decode a cursor
    pub fn decode(
        params: &cursor::cursor_value::CursorType,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cursor = match params {
            cursor::cursor_value::CursorType::After(cursor) => cursor,
            cursor::cursor_value::CursorType::Before(cursor) => cursor,
        };

        let bytes = BASE64_URL_SAFE_NO_PAD.decode(cursor)?;

        let decoded = String::from_utf8(bytes)?;

        let mut tokens = decoded.split('|');
        if let (Some(dt), Some(id)) = (tokens.next(), tokens.next()) {
            Ok(Self {
                id: id.to_string(),
                dt: dt.parse()?,
            })
        } else {
            Err("missing tokens".into())
        }
    }

    /// get id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// get date time
    pub fn dt(&self) -> &str {
        &self.dt
    }

    /// encode a cursor
    pub fn encode(&self) -> String {
        BASE64_URL_SAFE_NO_PAD.encode(format!("{}|{}", self.dt, self.id))
    }

    /// Gets pagination direction
    pub fn is_paginating_from_left(pagination: &Cursor) -> bool {
        let index = pagination.index.expect("index should exist");

        match &pagination.cursor_value {
            Some(value) => match &value.cursor_type {
                Some(cursor::cursor_value::CursorType::After(_)) => {
                    matches!(index, Index::First(_))
                }
                Some(cursor::cursor_value::CursorType::Before(_)) => {
                    matches!(index, Index::First(_))
                }
                None => matches!(index, Index::First(_)),
            },
            None => matches!(index, Index::First(_)),
        }
    }

    /// Checks if cursor is unavailable
    pub fn is_cursor_unavailable(pagination: &Cursor) -> bool {
        if let Some(value) = pagination.cursor_value.as_ref() {
            match value.cursor_type.as_ref() {
                Some(val) => match val {
                    cursor::cursor_value::CursorType::After(_) => false,
                    cursor::cursor_value::CursorType::Before(_) => false,
                },
                None => true,
            }
        } else {
            true
        }
    }
}

/// Gets maximum query results from pagination data
pub fn query_count(max: i32, pagination: &Index) -> i32 {
    let user_param = match pagination {
        Index::First(value) => value,
        Index::Last(value) => value,
    };
    if *user_param > max {
        max
    } else {
        *user_param
    }
}

#[cfg(test)]
#[cfg(feature = "rpc-server-categories")]
mod tests {
    use crate::common::pagination::cursor::{cursor_value, CursorValue};

    use super::*;

    #[test]
    fn test_cursor_builder_new() {
        let id = "123";
        let dt = "2025-02-15T12:00:00";
        let cursor_builder = CursorBuilder::new(id, dt);

        assert_eq!(cursor_builder.id(), id);
        assert_eq!(cursor_builder.dt(), dt);
    }

    #[test]
    fn test_cursor_builder_encode() {
        let id = "123";
        let dt = "2025-02-15T12:00:00";
        let cursor_builder = CursorBuilder::new(id, dt);

        let encoded = cursor_builder.encode();
        let expected = BASE64_URL_SAFE_NO_PAD.encode(format!("{}|{}", dt, id));

        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_cursor_builder_decode_success() {
        let id = "123";
        let dt = "2025-02-15T12:00:00";
        let encoded = BASE64_URL_SAFE_NO_PAD.encode(format!("{}|{}", dt, id));

        let cursor_type = cursor_value::CursorType::After(encoded.clone());
        let cursor = CursorBuilder::decode(&cursor_type);

        assert!(cursor.is_ok());
        let decoded_cursor = cursor.unwrap();

        assert_eq!(decoded_cursor.id(), id);
        assert_eq!(decoded_cursor.dt(), dt);
    }

    #[test]
    fn test_cursor_builder_decode_failure_missing_tokens() {
        let encoded = BASE64_URL_SAFE_NO_PAD.encode("invalid_cursor_data");

        let cursor_type = cursor_value::CursorType::After(encoded);
        let cursor = CursorBuilder::decode(&cursor_type);

        assert!(cursor.is_err());
    }

    #[test]
    fn test_cursor_builder_is_paginating_from_left_first() {
        let cursor_value = CursorValue {
            cursor_type: Some(cursor_value::CursorType::After(
                "encoded_cursor".to_string(),
            )),
            ..Default::default()
        };

        let pagination = Cursor {
            index: Some(Index::First(10)),
            cursor_value: Some(cursor_value),
        };

        assert!(CursorBuilder::is_paginating_from_left(&pagination));
    }

    #[test]
    fn test_cursor_builder_is_paginating_from_left_not_first() {
        let cursor_value = CursorValue {
            cursor_type: Some(cursor_value::CursorType::After(
                "encoded_cursor".to_string(),
            )),
            ..Default::default()
        };

        let pagination = Cursor {
            index: Some(Index::Last(10)),
            cursor_value: Some(cursor_value),
        };

        assert!(!CursorBuilder::is_paginating_from_left(&pagination));
    }

    #[test]
    fn test_cursor_builder_is_cursor_unavailable_with_no_cursor() {
        let pagination = Cursor {
            index: Some(Index::First(10)),
            cursor_value: None,
        };

        assert!(CursorBuilder::is_cursor_unavailable(&pagination));
    }

    #[test]
    fn test_cursor_builder_is_cursor_unavailable_with_invalid_cursor_type() {
        let cursor_value = CursorValue {
            cursor_type: None,
            ..Default::default()
        };

        let pagination = Cursor {
            index: Some(Index::First(10)),
            cursor_value: Some(cursor_value),
        };

        assert!(CursorBuilder::is_cursor_unavailable(&pagination));
    }

    #[test]
    fn test_cursor_builder_is_cursor_unavailable_with_valid_cursor_type() {
        let cursor_value = CursorValue {
            cursor_type: Some(cursor_value::CursorType::After(
                "encoded_cursor".to_string(),
            )),
            ..Default::default()
        };

        let pagination = Cursor {
            index: Some(Index::First(10)),
            cursor_value: Some(cursor_value),
        };

        assert!(!CursorBuilder::is_cursor_unavailable(&pagination));
    }

    #[test]
    fn test_query_count_within_max() {
        let pagination = Index::First(5);
        let max = 10;
        let count = query_count(max, &pagination);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_query_count_exceeds_max() {
        let pagination = Index::First(15);
        let max = 10;
        let count = query_count(max, &pagination);

        assert_eq!(count, 10);
    }

    #[test]
    fn test_query_count_for_last_index() {
        let pagination = Index::Last(5);
        let max = 10;
        let count = query_count(max, &pagination);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_query_count_for_last_index_exceeds_max() {
        let pagination = Index::Last(15);
        let max = 10;
        let count = query_count(max, &pagination);

        assert_eq!(count, 10);
    }
}
