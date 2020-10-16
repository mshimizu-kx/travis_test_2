// tls_connect.rs

/*
* This file provides example to connect to q process over TLS
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

  // Connect to q process over TLS with 1 second timeout and 200 milliseconds retry interval
  let mut handle=connect_tls("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");

  // Build keyed table
  let keyed_table=q_keyed_table![
    vec!["id", "month"];
    vec![
      q_long_list!['s'; vec![0_i64, 1, 2]],
      q_month_list!["date"; '*'; vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)]]
    ];
    vec!["stats", "sym"];
    vec![
      q_float_list!['*'; vec![113.42_f64, 354.923, 2749.4]],
      q_symbol_list!['*'; vec!["Newry", "Belfast", "London"]]
    ]
  ].unwrap();

  // Set keyed table to remote variable "keytab" b calling built-in 'set' function with arguments
  // Send ("set"; `keyedtab; keyed_tab)
  send_query_async_le(&mut handle, q_mixed_list![q_string!['*'; "set"], q_symbol!["keyedtab"], keyed_table]).await?;

  // Check original table
  let res_keyed_table=send_string_query_le(&mut handle, "keyedtab").await?;
  println!("Original table:\n{}", res_keyed_table);

  // Set 'upd' function remotely
  // query is sent in Big Endian ("_be")
  send_string_query_async_be(&mut handle, "upd:upsert").await?;

  // Update the table
  // Send (`upd; `keyedtab; (1; 2000.02m; 1000f; `GoldenCity))
  send_query_async_le(&mut handle, q_mixed_list![
    q_symbol!["upd"],
    q_symbol!["keyedtab"],
    q_mixed_list![q_long![1_i64], q_month![2000, 2], q_float![1000_f64], q_symbol!["GoldenCity"]]
  ]).await?;

  // Get updated table
  let res_keyed_table=send_string_query_le(&mut handle, "keyedtab").await?;
  println!("Updated table:\n{}", res_keyed_table);

  // Decompose keyed table into (header of key table, body of key table, header of value table, body of value table)
  let (kheader, kvalue, vheader, vvalue) = res_keyed_table.into_keyedtable_components()?;

  assert_eq!(kheader, vec!["id", "month"]);
  assert_eq!(kvalue, vec![
      q_long_list!['s'; vec![0_i64, 1, 2]],
      q_month_list!["date"; '*'; vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)]]
    ]);
  assert_eq!(vheader, vec!["stats", "sym"]);
  assert_eq!(vvalue, vec![
      q_float_list!['*'; vec![113.42_f64, 1000.0, 2749.4]],
      q_symbol_list!['*'; vec!["Newry", "GoldenCity", "London"]]
    ]);

  // Close the handle
  close_tls(&mut handle).await?;

  Ok(())
}
