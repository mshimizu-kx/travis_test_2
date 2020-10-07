//! **rustkdb** is provided as one of [Fusion interfaces](https://code.kx.com/q/interfaces/) of kdb+/q. 
//! This crate provides methods to connect to q/kdb+ process, send a (text) query to q process,
//!  construct objects representing q objects and convert the objects into Rust type objects.
//! 
//! # About kdb+
//! kdb+/q is a columnar time series database/analytic programming language licensed by [Kx Systems](https://kx.com/).
//!  kdb+ is featured by its small size of executable binary file, high performance of table manipulation and
//!  its flexibility to allow aggregation inside the database and stream data by IPC. These features are realized 
//!  by an aspect that kdb+ is a database expressed in q, an analytic language.
//! 
//! q is an interpreter-type language and users can comfortably interact with its console to check immediately the
//!  result of the code they write. Also q has only around 100 built-in named functions and its code is expressed
//!  by a lot of highly overloaded abstract symbols (these symbols are functions, too). As kdb+ is a time series
//!  database, q is a vector orientated language while scalar types exist in normal intuitive manner. Loop structure
//!  by an index no longer appears in the code but instead users will apply abstract symbols like `\` or `/`
//!  to a vector.
//! 
//! For scalability, some frequently used aggregation functions are implemented with built-in multi-threading operation.
//!  Helper functions to support multi-processing/multi-threading execution of custom functions are also provided.
//! 
//! If you want to know more about kdb+/q, [see official website](https://code.kx.com/q/).
//! 
//! # Interface Detail
//! **rustkdb** is using [Tokio](https://tokio.rs/) crate to provide asynchronous execution.
//! 
//! ## Connection
//! Connection to kdb+ is either of TCP or TLS.
//! 
//! ## Query
//! Query to kdb+ is supported by two ways, sending a text query or a functional query which is represented by a
//!  compound list of kdb+ ([See detail of IPC](https://code.kx.com/q4m3/11_IO/#116-interprocess-communication)).
//!  Compression/decompression of messages is done following [kdb+ implementation](https://code.kx.com/q/basics/ipc/#compression).
//! 
//! ## Conversion between Rust Types
//! As for construction of q object, some types have multiple possible interpretation and so multiple constructors are
//!  provided accordingly (whose method names start from `QGEN::new_*`). For example, timestamp can be interpreted as nanoseconds from an epoch (Rust epoch or kdb+ epoch, depending on a context)
//!  as well as datetime. rustkdb provides a variety of constructors for each q type for flexibility. The same effort
//!  was made on conversion from q object into Rust types (whocse method names start from `.into_*`).
//! 
//! Though interpretation can have multiple ways, each q type has a fixed Rust compatible type. **rustkdb** defines some utility types for fully expressing q type:
//! - `QTime`: Intermediate object for q minute, q second and q time to hold either of time value or null or inifinity value.
//! - `QTable`: Struct holding header and column values of table
//! - `QDictionary`: Struct holding key and value of dictionary
//! - `QKeyedTable`: Struct holding key table and value table of keyed table
//! - `QList`: Struct holding an attribute and list values of q list object
//! 
//! Using the utility types above, type mapping between q types and Rust types are followings:
//! 
//! | q                | Rust                                |
//! |------------------|-------------------------------------|
//! | `bool`           | `bool`                              |
//! | `GUID`           | `[u8; 16]`                          |
//! | `byte`           | `u8`                                |
//! | `short`          | `i16`                               |
//! | `int`            | `i32`                               |
//! | `long`           | `i64`                               |
//! | `real`           | `f32`                               |
//! | `float`          | `f64`                               |
//! | `char`           | `char`                              |
//! | `symbol`         | `String`                            |
//! | `timestamp`      | `chrono::DateTime<Utc>`             |
//! | `month`          | `chrono::Date<Utc>`                 |
//! | `date`           | `chrono::Date<Utc>`                 |
//! | `datetime`       | `chrono::DateTime<Utc>`             |
//! | `timespan`       | `chrono::Duration`                  |
//! | `minute`         | `chrono::NaiveTieme` or `QTime`     |
//! | `second`         | `chrono::NaiveTieme` or `QTime`     |
//! | `time`           | `chrono::NaiveTieme` or `QTime`     |
//! | `table`          | `QTable`                            |
//! | `dictionary`     | `QDictionary`                       |
//! | `keyed table`    | `QKeyedTable`                       |
//! | `bool list`      | `QList<Vec<bool>>`                  |
//! | `GUID list`      | `QList<Vec<[u8; 16]>>`              |
//! | `byte list`      | `QList<Vec<u8>>`                    |
//! | `short list`     | `QList<Vec<i16>>`                   |
//! | `int list`       | `QList<Vec<i32>>`                   |
//! | `long list`      | `QList<Vec<i64>>`                   |
//! | `real list`      | `QList<Vec<f32>>`                   |
//! | `float list`     | `QList<Vec<f64>>`                   |
//! | `string`         | `QList<String>`                     |
//! | `symbol list`    | `QList<Vec<String>>`                |
//! | `timestamp list` | `QList<Vec<chrono::DateTime<Utc>>>` |
//! | `month list`     | `QList<Vec<chrono::Date<Utc>>>`     |
//! | `date list`      | `QList<Vec<chrono::Date<Utc>>>`     |
//! | `datetime list`  | `QList<Vec<chrono::DateTime<Utc>>>` |
//! | `timespan list`  | `QList<Vec<chrono::Duration>>`      |
//! | `minute list`    | `QList<Vec<chrono::NaiveTime>>`     |
//! | `second list`    | `QList<Vec<chrono::NaiveTime>>`     |
//! | `time list`      | `QList<Vec<chrono::NaiveTime>>`     |
//! | `general null`   | `QGeneralNull`                      |
//! 
//! # Example
//! Assume that q process is running on `localhost:5000` with TLS only mode (`-E 2`). Set access restriction from
//!  only `kdbuser:pass`.
//! ```text
//! $ q -p 5000 -E 2
//! q)// Set access restriction
//! q).z.pw:{(x ~ `kdbuser) and y ~ "pass"}
//! q)
//! q)// Initialize a price table of sushi
//! q)enter:{[] `price_table set `tuna`salmon`mackerel`seaurchin`octopus`natto`squid!150 130 110 120 100 100 110}
//! q)
//! q)// Function returns total due of orders.
//! q)// If any order is not listed in the price table, return error with the unlisted item. 
//! q)post_order:{[order] if[count unlisted:where not order in key price_table; '"What!? ", (", " sv string order unlisted), "!?"]; sum price_table order}
//! q)
//! q)// Function to update price table
//! q)upd:upsert
//! ```
//! Then Rust connects to the q process and send queries.
//! ```
//! use rustkdb::qtype::*
//! use rustkdb::connection::*;
//! 
//! // Set timeout 1 second (1000 millisecond) and retry to connect every 200 millisecond
//! let mut handle=connect_tls("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
//! 
//! // Send a text query asynchronously in Little Endian encode (Enter a sushi restaurant)
//! // h "enter[]"
//! send_string_query_async_le(&mut handle, "enter[]").await?;
//! 
//! // Send a text query synchronously in Big Endian encode (Ask for a price table)
//! // h "price_table"
//! let price=send_string_query_be(&mut handle, "price_table").await?;
//! // `tuna`salmon`mackerel`seaurchin`octopus`natto`squid!150 130 110 120 100 100 110j
//! println!("{}", price);
//! 
//! // Send a functional query synchronously in Little Endian encode (Post erroneous orders)
//! // h (`post_order; `napolitan`ra_men)
//! if let Err(err)=send_query_le(&mut handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("post_order"), QGEN::new_symbol_list(Attribute::None, vec!["napolitan", "ra_men"])])).await{
//!   // q Error - [ Execution of query failed: What!? napolitan, ra-men!? ]
//!   eprintln!("{}", err);
//! }
//! 
//! // Build Q object (prepare an additional price list to suggest)
//! let additional_menu=QGEN::new_dictionary(
//!   QGEN::new_symbol_list(Attribute::None, vec!["eel", "gunkan", "salmon"]),
//!   QGEN::new_long_list(Attribute::None, vec![120_i64, 110, 100])
//! );
//! 
//! // Send a functional query asynchronously in Little Endian encode (Update price table)
//! // h (`upd; `price_table; additional_menu)
//! send_query_async_le(&mut handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("upd"), QGEN::new_symbol("price_table"), additional_menu])).await?;
//! 
//! // Send a functional query synchronously in Big Endian encode (Post orders)
//! // h (`post_order; `salmon`eel`gunkan`tuna)
//! let payment=send_query(&mut handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("post_order"), QGEN::new_symbol_list(Attribute::None, vec!["salmon", "eel", "gunkan", "tuna"])])).await?;
//! // Order total: 480j yen
//! println!("Order total: {} yen", payment);
//! ```

#[macro_use]
extern crate lazy_static;
extern crate async_recursion;

pub mod qtype;
mod serialization;
mod deserialization;
pub mod connection;
mod compression;
pub mod error;
