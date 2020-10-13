// macro.rs

/*
* This file demostrates how to use macros to build q objects.
*/

#[macro_use]
extern crate rustkdb;

use rustkdb::qtype::*;
use rustkdb::connection::*;
use std::io;
use chrono::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()>{

  // Connect to q process with 1 second timeout and 200 milliseconds retry interval
  let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
  
  // Get q bool list
  let q_bool_list=send_string_query_le(&mut handle, "`p#110b").await?;
  // Build q bool list with parted attribute
  let q_bool_list2=q_bool_list!['p'; vec![true, true, false]];
  assert_eq!(q_bool_list, q_bool_list2);

  // Get q timestamp
  let q_timestamp=send_string_query_le(&mut handle, "2011.12.19D19:40:12.000001384").await?;
  // Build q timestamp from DateTime<Utc>
  let q_timestamp2=q_timestamp!["datetime"; Utc.ymd(2011, 12, 19).and_hms_nano(19, 40, 12, 1384)];
  // Build q timestamp from nanosecond from epoch (i64)
  let q_timestamp3=q_timestamp!["nanos"; 377638812000001384_i64 + KDB_TIMESTAMP_OFFSET];
  // Build q timestamp from components of timestamp
  let q_timestamp4=q_timestamp!["ymd_hms_nanos"; 2011, 12, 19, 19, 40, 12, 1384];
  assert_eq!(q_timestamp, q_timestamp2);
  assert_eq!(q_timestamp, q_timestamp3);
  assert_eq!(q_timestamp, q_timestamp4);

  // Get q mixed list
  let q_mixed_list=send_string_query_le(&mut handle, "(2011.12.19D12:04:40.000003023 2008.02.28D02:29:36.945650816 2010.09.28D13:18:03.853207424; `q`Rust`kdbplus; `s#1200 3000 144000)").await?;
  // Build q mixed list
  let q_mixed_list2=q_mixed_list![
    q_timestamp_list!["ymd_hms_nanos"; '*'; vec![(2011, 12, 19, 12, 4, 40, 3023), (2008, 2, 28, 2, 29, 36, 945650816), (2010, 9, 28, 13, 18, 3, 853207424)]],
    q_symbol_list!['*'; vec!["q", "Rust", "kdbplus"]],
    q_long_list!['s'; vec![1200_i64, 3000, 144000]]
  ];
  assert_eq!(q_mixed_list, q_mixed_list2);

  // Get q table
  let q_table=send_string_query_le(&mut handle, "([] time:2020.06.01D07:02:13.238912781 2020.06.01D07:02:14.230892785 2020.06.01D07:03:01.137860387; sym:`g#`Kx`FD`Kx; price:103.68 107.42 103.3; size:1000 2000 3000; ex:\"NLN\")").await?;
  // Build q table
  let q_table2=q_table![
    vec!["time", "sym", "price", "size", "ex"];
    vec![
      q_timestamp_list!["datetime"; '*'; vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]],
      q_symbol_list!['g'; vec!["Kx", "FD", "Kx"]],
      q_float_list!['*'; vec![103.68_f64, 107.42, 103.3]],
      q_long_list!['*'; vec![1000_i64, 2000, 3000]],
      q_string!['*'; "NLN"]
    ]
  ].expect("Failed to build table");
  assert_eq!(q_table, q_table2);

  Ok(())
}