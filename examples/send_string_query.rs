// send_string_query.rs

/*
* This file demostrates examples of how to use a function to send a text query
* and how to retrieve underlying value of q object.
*/

use rustkdb::connection::*;
use rustkdb::qtype::*;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error>{
  let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");

  // Set remote initializer function 'init'
  send_string_query_async_le(&mut handle, "init:{[] i:6; while[i-:1; -1 string[i], \"...\"; system \"sleep 1\"]; \"Done.\"}").await?;

  // Call 'init'
  // Blocked until response is back
  let response=send_string_query_le(&mut handle, "init[]").await?;
  println!("response: {}", response);

  // Send erroneous query
  if let Err(e)=send_string_query_le(&mut handle, "`a+42").await{
    // type
    eprintln!("{}", e)
  }

  // Send erroneous query
  if let Err(e)=send_string_query_le(&mut handle, "(1 2; 3 4; 5;").await{
    // (
    eprintln!("{}", e)
  }  

  // Get compound olist of dictionary and table
  let res_mixed_dict_table_null=send_string_query_be(&mut handle, "(`a`b`c!1 2 3; `d`e!100.12 113.433; ([] a:1 2; b:2020.03.12D03:15:00.987 2020.05.30D19:14:24.0100304); ::)").await?;
  assert_eq!(res_mixed_dict_table_null, QGEN::new_mixed_list(vec![
    QGEN::new_dictionary(QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]), QGEN::new_long_list(Attribute::None, vec![1_i64, 2, 3])), 
    QGEN::new_dictionary(QGEN::new_symbol_list(Attribute::None, vec!["d", "e"]), QGEN::new_float_list(Attribute::None, vec![100.12_f64, 113.433])),
    QGEN::new_table(vec!["a", "b"], vec![
        QGEN::new_long_list(Attribute::None, vec![1_i64, 2]),
        QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2020, 3, 12, 3, 15, 0, 987000000), (2020, 5, 30, 19, 14, 24, 10030400)])
      ]
    ).expect("Failed to build table"),
    QGEN::new_general_null()
  ]));

  // Convert q compiund list into vector of q object
  let rust_q_vec=res_mixed_dict_table_null.into_q_vec()?;

  // Show dictionary
  // `d`e!100.12 113.433f
  println!("{}", rust_q_vec[1]);
  
  // Deconpose into key and value
  let (key, value) = rust_q_vec[1].clone().into_key_value()?;
  assert_eq!(key, QGEN::new_symbol_list(Attribute::None, vec!["d", "e"]));
  assert_eq!(value,  QGEN::new_float_list(Attribute::None, vec![100.12_f64, 113.433]));

   Ok(())
}