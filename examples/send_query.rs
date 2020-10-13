// send_query.rs

/*
* This file demostrates how to use the function to send a functional query
* and how to retrieve an underlying value of q object.
*/

use rustkdb::connection::*;
use rustkdb::qtype::*;
use std::io;
use chrono::Utc;
use rand::prelude::*;
use rand::seq::SliceRandom;

#[tokio::main]
async fn main() -> io::Result<()>{

  // Connect to q process with 1 second timeout and 200 milliseconds retry interval
  let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");

  // Set remote dyadic function 'pow' by an asynchronous message
  // "_le" means Little Endian
  send_string_query_async_le(&mut handle, "pow:{[base; ex] base xexp ex}").await?;

  // Random generator
  let mut rng=rand::thread_rng();

  // Send (`pow; b; e) synchronously
  let b=rng.gen_range(2, 5) as i64;
  let e=rng.gen_range(1, 5);
  let res_long=send_query_le(&mut handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("pow"), QGEN::new_long(b), QGEN::new_int(e)])).await?;
  println!("pow[{}; {}] = {:.4}", b, e, res_long);
  
  // Set remote table 'trade' by an asynchronous message
  send_string_query_async_le(&mut handle, "trade:flip `time`sym`price`size`country!\"psfjs\"$\\:()").await?;
  send_string_query_async_le(&mut handle, "upd:upsert").await?;

  let syms = ["Apple", "Banana", "Coconut"];  
  let countries=["Equ", "Phi", "Cal"];

  // Send data asynchronously
  for _ in 0_u8 .. 10{
    send_query_async_le(&mut handle, QGEN::new_mixed_list(vec![
      QGEN::new_symbol("upd"),
      QGEN::new_symbol("trade"),
      QGEN::new_mixed_list(vec![
        QGEN::new_timestamp(Utc::now()),
        QGEN::new_symbol(syms.choose(&mut rng).unwrap()),
        QGEN::new_float(rng.gen_range(102.0_f64, 103.0_f64)),
        QGEN::new_long(rng.gen_range(1_i64, 4_i64) * 10000_i64),
        QGEN::new_symbol(countries.choose(&mut rng).unwrap())
      ])
    ])).await?;
  }

  // Get the value of 'trade' (synchronous call)
  let trade=send_string_query_le(&mut handle, "trade").await?;
  println!("{}", trade);

  // Set remote dictionary 'dict'
  send_string_query_async_le(&mut handle, "dict:enlist[`]!enlist (::)").await?;

  // Update 'dict'
  send_query_async_le(&mut handle, QGEN::new_mixed_list(
    vec![
      QGEN::new_symbol("upd"),
      QGEN::new_symbol("dict"),
      QGEN::new_dictionary(
        QGEN::new_symbol_list(Attribute::Sorted, vec!["a", "b", "c"]),
        QGEN::new_mixed_list(vec![QGEN::new_int_list(Attribute::None, vec![10, 20]), QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2010, 10, 30, 20, 1, 35, 256)]), QGEN::new_float(Q_0w)])
      )]
    )
  ).await?;

  // Get the value of 'dict'
  let res_dict=send_string_query_le(&mut handle, "dict").await?;

  // Decompose q dictionary object into key and value
  let (key, value) = res_dict.into_key_value()?;

  // Convert key (q symbol list) into (atribute, vector)
  let (attribute, rust_key) = key.into_string_vec()?;

  // None because enlist[`]!enlist (::) is there
  assert_eq!(attribute, Attribute::None);
  // ``a`b`c
  assert_eq!(rust_key, vec!["".to_string(), "a".to_string(), "b".to_string(), "c".to_string()]);

  // Convert value (q compound list) into vector of q object
  let rust_value = value.into_q_vec()?;

  // (::)
  assert_eq!(rust_value[0], QGEN::new_general_null());
  // 10 20i
  assert_eq!(rust_value[1], QGEN::new_int_list(Attribute::None, vec![10, 20]));
  // enlist 2010.10.30T20:01:35.256
  assert_eq!(rust_value[2], QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2010, 10, 30, 20, 1, 35, 256)]));
  // 0w
  assert!(rust_value[3].clone().into_f64()?.is_infinite());

  // Close the handle
  close(&mut handle).await?;

  Ok(())
}

