use serde::{Deserialize, Serialize};


pub use search::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthFormLink {
    pub server_owner_name: Option<String>,
    pub server_name: Option<String>,
    pub server_id: Option<String>,

    pub redirect_uri: String,

    pub state: String,
    pub scope: Scope,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthQueryHandshake {
    /// Used for verifying
    pub state: Option<String>,

    /// Private Server ID
    pub server_id: String,
    /// Public Server ID
    pub public_id: String,

    pub scope: Scope,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    ServerRegister,
}




mod search {
    use chrono::{DateTime, Utc, NaiveDate};
    use serde::{Serialize, Deserialize};
    use crate::{api::{QueryListResponse, WrappingResponse}, util::{serialize_datetime, serialize_datetime_opt, deserialize_datetime_opt, deserialize_datetime, serialize_naivedate_opt, deserialize_naivedate_opt}};


    pub type PublicSearchResponse = WrappingResponse<PublicSearchType>;
    // TODO: Incorporate Authors, Collections, etc..


    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(tag = "type", content = "value")]
    pub enum PublicSearchType {
        BookList(QueryListResponse<PartialBook>),
        BookItem(Option<PublicBook>),

        AuthorList(QueryListResponse<PublicAuthor>),
        AuthorItem(Option<PublicAuthor>),
    }



    // Public Search
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GetSearchQuery {
        pub query: String,

        pub offset: Option<usize>,
        pub limit: Option<usize>,

        #[serde(default)]
        pub view_private: bool,

        pub server_id: String,
    }


    // Author
    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct PublicAuthor {
        pub id: usize,

        pub name: String,
        pub description: Option<String>,
        #[serde(serialize_with = "serialize_naivedate_opt", deserialize_with = "deserialize_naivedate_opt")]
        pub birth_date: Option<NaiveDate>,

        pub thumb_url: String,

        #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
        pub updated_at: DateTime<Utc>,
        #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
        pub created_at: DateTime<Utc>,
    }

    // Book

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct PartialBook {
        pub id: usize,

        pub title: Option<String>,

        pub description: Option<String>,
        pub rating: f64,

        pub thumb_url: String,

        pub isbn_10: Option<String>,
        pub isbn_13: Option<String>,

        pub is_public: bool,

        #[serde(serialize_with = "serialize_naivedate_opt", deserialize_with = "deserialize_naivedate_opt")]
        pub available_at: Option<NaiveDate>,
        pub language: u16,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct PublicBook {
        pub id: usize,

        pub title: Option<String>,
        pub clean_title: Option<String>,

        pub description: Option<String>,
        pub rating: f64,

        pub thumb_url: String,

        pub isbn_10: Option<String>,
        pub isbn_13: Option<String>,

        pub display_author_id: Option<usize>,
        pub publisher: Option<String>,

        pub author_ids: Vec<usize>,

        pub is_public: bool,
        pub edition_count: usize,

        #[serde(serialize_with = "serialize_naivedate_opt", deserialize_with = "deserialize_naivedate_opt")]
        pub available_at: Option<NaiveDate>,
        pub language: u16,

        #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
        pub created_at: DateTime<Utc>,
        #[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
        pub updated_at: DateTime<Utc>,
        #[serde(serialize_with = "serialize_datetime_opt", deserialize_with = "deserialize_datetime_opt")]
        pub deleted_at: Option<DateTime<Utc>>,
    }
}