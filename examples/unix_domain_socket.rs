// tls_connect.rs

/*
* This file provides example to connect to q process with Unix Domain Socket
* and how to use the handle returned by connecting function.
*/

#[macro_use]
extern crate rustkdb;

use rustkdb::connection::*;
use rustkdb::qtype::*;
use std::io;
use chrono::prelude::*;
use chrono::Utc;

#[tokio::main]
async fn main() -> io::Result<()>{

  // Connect to q process over Unix Domain Socket with timeout of 1000 milliseconds
  let mut handle=connect_uds(5000, "kdbuser:pass", 1000).await.expect("Failed to connect");

  let qtable=q_table![
    vec!["time", "sym", "price", "size", "ex"];
    vec![
      q_timestamp_list!["datetime"; '*'; vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]],
      q_symbol_list!['g'; vec!["Kx", "FD", "Kx"]],
      q_float_list!['*'; vec![103.68_f64, 107.42, 103.3]],
      q_long_list!['*'; vec![1000_i64, 2000, 3000]],
      q_string!['*'; "NLN"]
    ]
  ].expect("Failed to build table");

  // Send functional query asynchronously
  send_query_async_le_uds(&mut handle, q_mixed_list![q_string!['*'; "set"], q_symbol!["tab"], qtable]).await?;

  // Send text query synchronously
  let equal=send_string_query_le_uds(&mut handle, "tab ~ ([] time:2020.06.01D07:02:13.238912781 2020.06.01D07:02:14.230892785 2020.06.01D07:03:01.137860387; sym:`g#`Kx`FD`Kx; price:103.68 107.42 103.3; size:1000 2000 3000; ex:\"NLN\")").await?;
  assert!(equal.into_bool()?);

  let qmixed_list=q_mixed_list![
    q_timestamp_list!["ymd_hms_nanos"; '*'; vec![(2011, 12, 19, 12, 4, 40, 3023), (2008, 2, 28, 2, 29, 36, 945650816), (2010, 9, 28, 13, 18, 3, 853207424)]],
    q_symbol_list!['*'; vec!["q", "Rust", "kdbplus"]],
    q_long_list!['s'; vec![1200_i64, 3000, 144000]]
  ];

  // Send functional query synchronously
  // This query is equivalent to h ("{[x] flast first x}"; qmixed_list)
  let qtimestamp=send_query_le_uds(&mut handle, q_mixed_list![q_string!['*'; "{[x] last first x}"], qmixed_list]).await?;
  assert_eq!(qtimestamp, q_timestamp!["nanos"; 338995083853207424_i64 + KDB_TIMESTAMP_OFFSET]);
  
  // Close the handle
  close_uds(&mut handle).await?;

  Ok(())
}
