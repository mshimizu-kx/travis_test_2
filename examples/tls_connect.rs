// tls_connect.rs

/*
* This file provides example to connect to q process over TLS
* and how to use the handle returned by connecting function.
*/

use rustkdb::connection::*;
use rustkdb::qtype::*;
use std::io;
use chrono::prelude::*;
use chrono::Utc;

#[tokio::main]
async fn main() -> io::Result<()>{
  let mut handle=connect_tls("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");

  // Build keyed table
  let keyed_table=QGEN::new_keyed_table(
    vec!["id", "month"],
    vec![
      QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
    ],   
    vec!["stats", "sym"],
    vec![
      QGEN::new_float_list(Attribute::None, vec![113.42_f64, 354.923, 2749.4]),
      QGEN::new_symbol_list(Attribute::None, vec!["Newry", "Belfast", "London"])
    ]
  ).unwrap();
    
  // Set remote function 'assign'
  send_string_query_async_le(&mut handle, "assign:set").await?;

  // Set keyed table to remote variable "keytab" b calling 'assign' with arguments
  // Send (`assign; `keyedtab; keyed_tab)
  send_query_async_le(&mut handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("assign"), QGEN::new_symbol("keyedtab"), keyed_table])).await?;

  // Check original table
  let res_keyed_table=send_string_query_le(&mut handle, "keyedtab").await?;
  println!("Original table:\n{}", res_keyed_table);

  // Set 'upd' function remotely
  send_string_query_async_be(&mut handle, "upd:upsert").await?;

  // Update the table
  // Send (`upd; `keyedtab; (1; 2000.02m; 1000f; `GoldenCity))
  send_query_async_le(&mut handle, QGEN::new_mixed_list(vec![
    QGEN::new_symbol("upd"),
    QGEN::new_symbol("keyedtab"),
    QGEN::new_mixed_list(vec![QGEN::new_long(1), QGEN::new_month_ym(2000, 2), QGEN::new_float(1000_f64), QGEN::new_symbol("GoldenCity")])
  ])).await?;

  // Get updated table
  let res_keyed_table=send_string_query_le(&mut handle, "keyedtab").await?;
  println!("Updated table:\n{}", res_keyed_table);

  // Decompose keyed table into (header of key table, body of key table, header of value table, body of value table)
  let (kheader, kvalue, vheader, vvalue) = res_keyed_table.into_keyedtable_components()?;

  assert_eq!(kheader, vec!["id", "month"]);
  assert_eq!(kvalue, vec![
      QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
    ]);
  assert_eq!(vheader, vec!["stats", "sym"]);
  assert_eq!(vvalue, vec![
      QGEN::new_float_list(Attribute::None, vec![113.42_f64, 1000.0, 2749.4]),
      QGEN::new_symbol_list(Attribute::None, vec!["Newry", "GoldenCity", "London"])
    ]);

  Ok(())
}
