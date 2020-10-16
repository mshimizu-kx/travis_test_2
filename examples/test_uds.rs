// test_uds.rs

/*
* Tests of Unix Domain Socket are done here because async functionality is used in the interface.
*/

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                       Settings                        //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#[macro_use]
extern crate rustkdb;
#[macro_use]
extern crate float_cmp;

use rustkdb::connection::*;
use rustkdb::qtype::*;
use std::io;
use std::panic;
use chrono::prelude::*;
use chrono::{Duration, Utc, NaiveTime};

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                        Macros                         //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Expand vector reference to compare
macro_rules! assert_vec_eq {
  ($vec1: expr, $vec2: expr) => {
    for (item1, item2) in $vec1.iter().zip($vec2.iter()){
      assert_eq!(item1, item2);
    }
  };
}

// Convert assertion result into pass or fail
macro_rules! assert_to_truefalse {
  ($left: expr, $right: expr, $success: expr, $failure: expr) => {
    match panic::catch_unwind(|| {assert_eq!($left, $right)}){
      Ok(_) => {$success+=1; println!(" ... pass")},
      Err(_) => {$failure+=1; println!(" ... fail")}
    }
  };
}

// Convert custom assertion result into pass or fail
macro_rules! assert_to_truefalse_custom {
  ($func: expr, $success: expr, $failure: expr) => {
    match panic::catch_unwind($func){
      Ok(_) => {$success+=1; println!(" ... pass")},
      Err(_) => {$failure+=1; println!(" ... fail")}
    }
  };
}

// Convert assertion result for float into pass or fail
macro_rules! assert_to_truefalse_real {
  ($left: expr, $right: expr, $ep: expr, $success: expr, $failure: expr) => {
    match panic::catch_unwind(|| assert!(approx_eq!(f32, $left, $right, epsilon=$ep))){
      Ok(_) => {$success+=1; println!(" ... pass")},
      Err(_) => {$failure+=1; println!(" ... fail")}
    }
  };
}

// Convert assertion result for float into pass or fail
macro_rules! assert_to_truefalse_float {
  ($left: expr, $right: expr, $ep: expr, $success: expr, $failure: expr) => {
    match panic::catch_unwind(|| assert!(approx_eq!(f64, $left, $right, epsilon=$ep))){
      Ok(_) => {$success+=1; println!(" ... pass")},
      Err(_) => {$failure+=1; println!(" ... fail")}
    }
  };
}

// Convert assertion result for float list into pass or fail
macro_rules! assert_to_truefalse_float_list {
  ($left: expr, $right: expr, $ep: expr, $success: expr, $failure: expr) => {
    match panic::catch_unwind(|| {
      for (&v1, &v2) in $left.into_f64_vec().expect("Failed to convert into Vec<f64>").1.iter().zip($right.iter()){
        assert!(approx_eq!(f64, v1, v2, epsilon=$ep));
      };
    })
    {
      Ok(_) => {$success+=1; println!(" ... pass")},
      Err(_) => {$failure+=1; println!(" ... fail")}
    }
  };
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Main Function                     //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

#[tokio::main]
async fn main() -> Result<(), io::Error>{

  // Connect to q process
  let mut handle=connect_uds(5000, "kdbuser:pass", 1000).await.expect("Failed to connect");

  // Vector to store execution time of each test
  let mut execution_time=Vec::new();
  let mut success_failure=Vec::new();

  // Deserialize Atom test
  let now=Utc::now();
  success_failure.push(deserialize_atom_test(&mut handle).await?);
  execution_time.push(("deserialize atom", Utc::now()-now));

  // Serialize Atom test
  let now=Utc::now();
  success_failure.push(serialize_atom_test(&mut handle).await?);
  execution_time.push(("serialize atom", Utc::now()-now));

  // Deserialize List test
  let now=Utc::now();
  success_failure.push(deserialize_list_test(&mut handle).await?);
  execution_time.push(("deserialize list", Utc::now()-now));

  // Serialize List test
  let now=Utc::now();
  success_failure.push(serialize_list_test(&mut handle).await?);
  execution_time.push(("serialize list", Utc::now()-now));

  // Deserialize Null and Infinity test
  let now=Utc::now();
  success_failure.push(deserialize_null_infinity_test(&mut handle).await?);
  execution_time.push(("serialize null and infinity", Utc::now()-now));

  // Serialize Null and Infinity test
  let now=Utc::now();
  success_failure.push(serialize_null_infinity_test(&mut handle).await?);
  execution_time.push(("serialize null and infinity", Utc::now()-now));

  // Deserialize Dictionary test
  let now=Utc::now();
  success_failure.push(deserialize_dictionary_test(&mut handle).await?);
  deserialize_dictionary_test(&mut handle).await?;
  execution_time.push(("deserialize dictionary", Utc::now()-now));

  // Serialize Dictionary test
  let now=Utc::now();
  success_failure.push(serialize_dictionary_test(&mut handle).await?);
  execution_time.push(("serialize dictionary", Utc::now()-now));

  // Deserialize Table test
  let now=Utc::now();
  success_failure.push(deserialize_table_test(&mut handle).await?);
  execution_time.push(("deserialize table", Utc::now()-now));

  // Serialize Table test
  let now=Utc::now();
  success_failure.push(serialize_table_test(&mut handle).await?);
  execution_time.push(("serialize table", Utc::now()-now));

  // Deserialize Table test
  let now=Utc::now();
  success_failure.push(deserialize_keyed_table_test(&mut handle).await?);
  execution_time.push(("deserialize keyed table", Utc::now()-now));

  // Serialize Table test
  let now=Utc::now();
  success_failure.push(serialize_keyed_table_test(&mut handle).await?);
  execution_time.push(("serialize keyed table", Utc::now()-now));

  // Atom Constructor test
  let now=Utc::now();
  success_failure.push(atom_constructor_test()?);
  execution_time.push(("atom constructor", Utc::now()-now));

  // List Constructor test
  let now=Utc::now();
  success_failure.push(list_constructor_test()?);
  execution_time.push(("list constructor", Utc::now()-now));

  // Atom Macro test
  let now=Utc::now();
  success_failure.push(atom_macro_test()?);
  execution_time.push(("atom macro", Utc::now()-now));

  // List Macro test
  let now=Utc::now();
  success_failure.push(list_macro_test()?);
  execution_time.push(("list macro", Utc::now()-now));

  // Table Macro test
  let now=Utc::now();
  success_failure.push(table_macro_test()?);
  execution_time.push(("table macro", Utc::now()-now));

  // Dictionary Macro test
  let now=Utc::now();
  success_failure.push(dictionary_macro_test()?);
  execution_time.push(("dictionary macro", Utc::now()-now));

  // Keyed Table Macro test
  let now=Utc::now();
  success_failure.push(keyed_table_macro_test()?);
  execution_time.push(("keyed_table macro", Utc::now()-now));

  // Atom Conversion test
  let now=Utc::now();
  success_failure.push(atom_conversion_test()?);
  execution_time.push(("atom conversion", Utc::now()-now));

  // List Conversion test
  let now=Utc::now();
  success_failure.push(list_conversion_test()?);
  execution_time.push(("list conversion", Utc::now()-now));

  // Dictionary Conversion test
  let now=Utc::now();
  success_failure.push(dictionary_conversion_test()?);
  execution_time.push(("dictionary conversion", Utc::now()-now));

  // Table Conversion test
  let now=Utc::now();
  success_failure.push(table_conversion_test()?);
  execution_time.push(("table conversion", Utc::now()-now));

  // Keyed Table Conversion test
  let now=Utc::now();
  success_failure.push(keyed_table_conversion_test()?);
  execution_time.push(("keyed table conversion", Utc::now()-now));

  // Compression test
  let now=Utc::now();
  success_failure.push(compression_test(&mut handle).await?);
  execution_time.push(("compression", Utc::now()-now));

  // Display Result Table
  println!("\n+{:-^70}+\n", "|| Test Result ||");
  println!("{:^30} | {:^20} | {:^6} | {:^6}", "Item", "Time (nanosecond)", "Pass", "Fail");
  println!("{:-^30} | {:-^20} | {:-^6} | {:-^6}" , "-", "-", "-", "-");
  for ((item, time), (success, failure)) in execution_time.iter().zip(success_failure.iter()){
    println!("{:^30} | {:>20} | {:>6} | {:>6}", item, time.num_nanoseconds().unwrap(), success, failure);
  }
  
  // Display ok or FAILED
  let mut num_success=0_u32;
  let mut num_failure=0_u32;
  for (success, failure) in success_failure{
    num_success+=success;
    num_failure+=failure;
  }
  let result=if num_failure > 0{
    "FAILED"
  }
  else{
    "ok"
  };
  println!("\ntest result: {}. {} passed; {} failed; 0 ignored; 0 measured; 0 filtered out", result, num_success, num_failure);
 
  Ok(())

}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Test Functions                    //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/*
* Test three aspects of interface:
* - deserialize atom q object
* - serialize a text query in both Big Endian and Little Endian
* Note: enode is related to sending the query not receiving;
* therefore testing with a few cases should be enough to verfy correctness.
* - synchronous and asynchronous call of a text query function
*/
async fn deserialize_atom_test(handle: &mut UnixStreamH) -> Result<(u32, u32), io::Error>{
  println!("\n+{:-^70}+\n", "|| Deserialize Atom ||");
  
  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Boolean //-------------------------------------/
  print!("<<{:^58}>>", "bool - query sent in LE");

  let res_bool=send_string_query_le_uds(handle, "`boolean$-42").await?;
  assert_to_truefalse!(res_bool, QGEN::new_bool(true), num_success, num_failure);

  print!("<<{:^58}>>", "async call 1");

  let res_void=send_string_query_async_le_uds(handle, "b:12+23").await?;
  assert_to_truefalse!(res_void, (), num_success, num_failure);

  print!("<<{:^58}>>", "bool - query sent in BE");

  let res_bool=send_string_query_be_uds(handle, "34 = b").await?;
  assert_to_truefalse!(res_bool, QGEN::new_bool(false), num_success, num_failure);
  
  // GUID //---------------------------------------/
  print!("<<{:^58}>>", "GUID - query sent in LE");

  let res_guid=send_string_query_le_uds(handle, "0x0 sv 0x8c6b8b64681560840a3e178401251b68").await?;
  assert_to_truefalse!(res_guid, QGEN::new_GUID([0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68]), num_success, num_failure);
  
  print!("<<{:^58}>>", "GUID - query sent in BE");

  let res_guid=send_string_query_be_uds(handle, "\"G\"$\"8c6b8b64-6815-6084-0a3e-178401251b68\"").await?;
  assert_to_truefalse!(res_guid, QGEN::new_GUID([0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68]), num_success, num_failure);

  // Byte //---------------------------------------/
  print!("<<{:^58}>>", "byte - query sent in LE");

  let res_byte=send_string_query_le_uds(handle, "\"x\"$1+2").await?;
  assert_to_truefalse!(res_byte, QGEN::new_byte(3_u8), num_success, num_failure);

  print!("<<{:^58}>>", "byte - query sent in BE");

  let res_byte=send_string_query_be_uds(handle, "`byte$12").await?;
  assert_to_truefalse!(res_byte, QGEN::new_byte(12_u8), num_success, num_failure);
  
  // Short //--------------------------------------/
  print!("<<{:^58}>>", "async call 2");

  let res_void=send_string_query_async_be_uds(handle, "a:1+2").await?;
  assert_to_truefalse!(res_void, (), num_success, num_failure);

  print!("<<{:^58}>>", "short");
  
  let res_short=send_string_query_be_uds(handle, "`short$-12+a").await?;
  assert_to_truefalse!(res_short, QGEN::new_short(-9_i16), num_success, num_failure);
  
  // Int //----------------------------------------/
  print!("<<{:^58}>>", "int");

  let res_int=send_string_query_le_uds(handle, "prd 1 -3 5i").await?;
  assert_to_truefalse!(res_int, QGEN::new_int(-15_i32), num_success, num_failure);
  
  // Long //---------------------------------------/
  print!("<<{:^58}>>", "long");

  let res_long=send_string_query_le_uds(handle, "3i+b").await?;
  assert_to_truefalse!(res_long, QGEN::new_long(38_i64), num_success, num_failure);
  
  // Real //---------------------------------------/
  print!("<<{:^58}>>", "real");
  
  let res_real=send_string_query_le_uds(handle, "`real$1.5*1.2").await?;
  assert_to_truefalse!(res_real, QGEN::new_real(1.8_f32), num_success, num_failure);
  
  // Float //--------------------------------------/
  print!("<<{:^58}>>", "float");

  let res_float=send_string_query_le_uds(handle, "dev 1 2 3 4f").await?;
  assert_to_truefalse_float!(res_float.into_f64().expect("Failed to convert into f64"), 1.118034_f64, 0.00001, num_success, num_failure);
  
  // Char //---------------------------------------/
  print!("<<{:^58}>>", "char");

  let res_char=send_string_query_le_uds(handle, ".Q.a[3]").await?;
  assert_to_truefalse!(res_char, QGEN::new_char('d'), num_success, num_failure);

  // Symbol //-------------------------------------/
  print!("<<{:^58}>>", "symbol");

  let res_symbol=send_string_query_le_uds(handle, "`$\"Hiya\"").await?;
  assert_to_truefalse!(res_symbol, QGEN::new_symbol("Hiya"), num_success, num_failure);
  
  // Timestamp //----------------------------------/
  print!("<<{:^58}>>", "timestamp");

  let res_timestamp=send_string_query_le_uds(handle, "2015.03.16D08:00:25.000007368").await?;
  assert_to_truefalse!(res_timestamp, QGEN::new_timestamp_ymd_hms_nanos(2015, 3, 16, 8, 0, 25, 7368), num_success, num_failure);

  // Month //--------------------------------------/
  print!("<<{:^58}>>", "month");

  let res_month=send_string_query_le_uds(handle, "2000.01m+70").await?;
  assert_to_truefalse!(res_month, QGEN::new_month_ym(2005, 11), num_success, num_failure);
  
  // Date //---------------------------------------/
  print!("<<{:^58}>>", "date");

  let res_date=send_string_query_le_uds(handle, "2000.01.01+7320").await?;
  assert_to_truefalse!(res_date, QGEN::new_date_ymd(2020, 1, 16), num_success, num_failure);
  
  // Datetime //-----------------------------------/
  print!("<<{:^58}>>", "datetime");

  let res_datetime=send_string_query_le_uds(handle, "`datetime$2020.09.05D15:12:39.569230892").await?;
  assert_to_truefalse!(res_datetime, QGEN::new_datetime_ymd_hms_millis(2020, 9, 5, 15, 12, 39, 569), num_success, num_failure);
  
  // Timespan //---------------------------------------/
  print!("<<{:^58}>>", "timespan");

  let res_timespan=send_string_query_le_uds(handle, "1D+1").await?;
  assert_to_truefalse!(res_timespan, QGEN::new_timespan_nanos(86400000000001_i64), num_success, num_failure);
  
  // Minute //-----------------------------------------/
  print!("<<{:^58}>>", "minute");

  let res_minute=send_string_query_le_uds(handle, "14:29").await?;
  assert_to_truefalse!(res_minute, QGEN::new_minute_hm(14, 29), num_success, num_failure);
  
  // Second //---------------------------------------/
  print!("<<{:^58}>>", "second");

  let res_second=send_string_query_le_uds(handle, "`second$8000").await?;
  assert_to_truefalse!(res_second, QGEN::new_second_hms(2, 13, 20), num_success, num_failure);
  
  // Time //---------------------------------------/
  print!("<<{:^58}>>", "time");

  let res_time=send_string_query_le_uds(handle, "`time$2020.02.18D12:30:45.678333825").await?;
  assert_to_truefalse!(res_time, QGEN::new_time_hms_millis(12, 30, 45, 678), num_success, num_failure);
  
  Ok((num_success, num_failure))
}

/*
* Test one aspect of interface:
* - serialize atom q object in both Big Endian and Little Endian
*/
async fn serialize_atom_test(handle:&mut  UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Serialize Atom ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Boolean //-------------------------------------/
  print!("<<{:^58}>>", "bool");

  let res_bool=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_bool(true)])).await?;
  assert_to_truefalse!(res_bool, QGEN::new_bool(true), num_success, num_failure);

  // GUID //----------------------------------------/
  print!("<<{:^58}>>", "GUID");

  let res_GUID=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_GUID([0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24])])).await?;
  assert_to_truefalse!(res_GUID, QGEN::new_GUID([0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]), num_success, num_failure);

  // Byte //----------------------------------------/
  print!("<<{:^58}>>", "byte");

  let res_byte=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_byte(0x3c)])).await?;
  assert_to_truefalse!(res_byte, QGEN::new_byte(0x3c), num_success, num_failure);

  // Short //---------------------------------------/
  print!("<<{:^58}>>", "short - query sent in LE");

  let res_short=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_short(17)])).await?;
  assert_to_truefalse!(res_short, QGEN::new_short(17_i16), num_success, num_failure);

  print!("<<{:^58}>>", "short - query sent in BE");

  let res_short=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_short(17)])).await?;
  assert_to_truefalse!(res_short, QGEN::new_short(17_i16), num_success, num_failure);

  // Int //-----------------------------------------/
  print!("<<{:^58}>>", "int - query sent in LE");

  let res_int=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_int(-34567789)])).await?;
  assert_to_truefalse!(res_int, QGEN::new_int(-34567789), num_success, num_failure);

  print!("<<{:^58}>>", "int - query sent in BE");

  let res_int=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_int(-34567789)])).await?;
  assert_to_truefalse!(res_int, QGEN::new_int(-34567789), num_success, num_failure);

  // Long //----------------------------------------/
  print!("<<{:^58}>>", "long - query sent in LE");

  let res_long=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_long(86400000000000_i64)])).await?;
  assert_to_truefalse!(res_long, QGEN::new_long(86400000000000_i64), num_success, num_failure);

  print!("<<{:^58}>>", "long - query sent in BE");

  let res_long=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_long(86400000000000_i64)])).await?;
  assert_to_truefalse!(res_long, QGEN::new_long(86400000000000_i64), num_success, num_failure);

  // Real //----------------------------------------/
  print!("<<{:^58}>>", "real - query sent in LE");

  let res_real=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_real(10.25)])).await?;
  assert_to_truefalse!(res_real, QGEN::new_real(10.25), num_success, num_failure);

  print!("<<{:^58}>>", "real - query sent in BE");

  let res_real=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_real(10.25)])).await?;
  assert_to_truefalse!(res_real, QGEN::new_real(10.25), num_success, num_failure);

  // Float //----------------------------------------/
  print!("<<{:^58}>>", "float - query sent in LE");

  let res_float=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_float(103.678_f64)])).await?;
  assert_to_truefalse!(res_float, QGEN::new_float(103.678), num_success, num_failure);
  //assert_to_truefalse_float!(res_float.into_f64().expect("Failed to convert into f64"), 103.678_f64, 0.0001);

  // Float //----------------------------------------/
  print!("<<{:^58}>>", "float - query sent in BE");

  let res_float=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_float(103.678)])).await?;
  assert_to_truefalse!(res_float, QGEN::new_float(103.678), num_success, num_failure);
  //assert_to_truefalse_float!(res_float.into_f64().expect("Failed to convert into f64"), 103.678_f64, 0.0001, num_success, num_failure);

  // Char //-----------------------------------------/
  print!("<<{:^58}>>", "char");

  let res_char=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_char('q')])).await?;
  assert_to_truefalse!(res_char, QGEN::new_char('q'), num_success, num_failure);

  // Symbol //-----------------------------------------/
  print!("<<{:^58}>>", "symbol");

  let res_symbol=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_symbol("kdb+")])).await?;
  assert_to_truefalse!(res_symbol, QGEN::new_symbol("kdb+"), num_success, num_failure);

  // Timestamp //--------------------------------------/
  print!("<<{:^58}>>", "timestamp - query sent in LE");

  let res_timestamp=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timestamp_ymd_hms_nanos(2018, 2, 18, 4, 0, 0, 100)])).await?;
  assert_to_truefalse!(res_timestamp, QGEN::new_timestamp_ymd_hms_nanos(2018, 2, 18, 4, 0, 0, 100), num_success, num_failure);

  print!("<<{:^58}>>", "timestamp - query sent in BE");

  let res_timestamp=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timestamp_ymd_hms_nanos(2018, 2, 18, 4, 0, 0, 100)])).await?;
  assert_to_truefalse!(res_timestamp, QGEN::new_timestamp_ymd_hms_nanos(2018, 2, 18, 4, 0, 0, 100), num_success, num_failure);

  // Month //-----------------------------------------/
  print!("<<{:^58}>>", "month - query sent in LE");
  
  let res_month=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_month_ym(2013, 9)])).await?;
  assert_to_truefalse!(res_month, QGEN::new_month_ym(2013, 9), num_success, num_failure);

  print!("<<{:^58}>>", "month - query sent in BE");
  
  let res_month=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_month_ym(2013, 9)])).await?;
  assert_to_truefalse!(res_month, QGEN::new_month_ym(2013, 9), num_success, num_failure);

  // Date //-----------------------------------------/
  print!("<<{:^58}>>", "date - query sent in LE");

  let res_date=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_date_ymd(2000, 2, 9)])).await?;
  assert_to_truefalse!(res_date, QGEN::new_date_ymd(2000, 2, 9), num_success, num_failure);

  print!("<<{:^58}>>", "date - query sent in BE");

  let res_date=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_date_ymd(2000, 2, 9)])).await?;
  assert_to_truefalse!(res_date, QGEN::new_date_ymd(2000, 2, 9), num_success, num_failure);

  // Datetime //-------------------------------------/
  print!("<<{:^58}>>", "datetime - query sent in LE");

  let res_datetime=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_datetime_ymd_hms_millis(2004, 6, 17, 11, 32, 40, 803)])).await?;
  assert_to_truefalse!(res_datetime, QGEN::new_datetime_ymd_hms_millis(2004, 6, 17, 11, 32, 40, 803), num_success, num_failure);

  print!("<<{:^58}>>", "datetime - query sent in BE");

  let res_datetime=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_datetime_ymd_hms_millis(2004, 6, 17, 11, 32, 40, 803)])).await?;
  assert_to_truefalse!(res_datetime, QGEN::new_datetime_ymd_hms_millis(2004, 6, 17, 11, 32, 40, 803), num_success, num_failure);

  // Timespan //------------------------------------/
  print!("<<{:^58}>>", "timespan - query sent in LE");

  let res_timespan=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timespan_millis(999_i64)])).await?;
  assert_to_truefalse!(res_timespan, QGEN::new_timespan_millis(999_i64), num_success, num_failure);

  print!("<<{:^58}>>", "timespan - query sent in BE");

  let res_timespan=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timespan_millis(999_i64)])).await?;
  assert_to_truefalse!(res_timespan, QGEN::new_timespan_millis(999_i64), num_success, num_failure);

  // Minute //-------------------------------------/
  print!("<<{:^58}>>", "minute - query sent in LE");

  let res_minute=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_minute_min(1231)])).await?;
  assert_to_truefalse!(res_minute, QGEN::new_minute_hm(20, 31), num_success, num_failure);

  print!("<<{:^58}>>", "minute - query sent in BE");

  let res_minute=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_minute_min(1231)])).await?;
  assert_to_truefalse!(res_minute, QGEN::new_minute_hm(20, 31), num_success, num_failure);

  // Second //-------------------------------------/
  print!("<<{:^58}>>", "second - query sent in LE");

  let res_second=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_second_hms(3, 17, 26)])).await?;
  assert_to_truefalse!(res_second, QGEN::new_second_hms(3, 17, 26), num_success, num_failure);

  print!("<<{:^58}>>", "second - query sent in BE");

  let res_second=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_second_hms(3, 17, 26)])).await?;
  assert_to_truefalse!(res_second, QGEN::new_second_hms(3, 17, 26), num_success, num_failure);

  // Time //--------------------------------------/
  print!("<<{:^58}>>", "time - query sent in LE");

  let res_time=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_time_hms_millis(21, 56, 7, 302)])).await?;
  assert_to_truefalse!(res_time, QGEN::new_time_hms_millis(21, 56, 7, 302), num_success, num_failure);

  print!("<<{:^58}>>", "time - query sent in BE");

  let res_time=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_time_hms_millis(21, 56, 7, 302)])).await?;
  assert_to_truefalse!(res_time, QGEN::new_time_hms_millis(21, 56, 7, 302), num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test one aspect of interface
* - deserialize list q object
*/
async fn deserialize_list_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Deserialize List ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;
  
  // Boolean //-----------------------------------/
  print!("<<{:^58}>>", "bool list");

  let res_bool=send_string_query_le_uds(handle, "`p#0000111b").await?;
  assert_to_truefalse!(res_bool, QGEN::new_bool_list(Attribute::Parted, vec![false, false, false, false, true, true, true]), num_success, num_failure);
  
  // GUID //--------------------------------------/
  print!("<<{:^58}>>", "GUID list");

  let res_guid=send_string_query_le_uds(handle, "`u#\"G\"$/:(\"8c6b8b64-6815-6084-0a3e-178401251b68\"; \"5ae7962d-49f2-404d-5aec-f7c8abbae288\")").await?;
  assert_to_truefalse!(res_guid, QGEN::new_GUID_list(Attribute::Unique, vec![[0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68], [0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]]), num_success, num_failure);
  
  // Byte //--------------------------------------/
  print!("<<{:^58}>>", "byte list");

  let res_byte=send_string_query_le_uds(handle, "`byte$3 4 62").await?;
  assert_to_truefalse!(res_byte, QGEN::new_byte_list(Attribute::None, vec![0x03, 0x04, 0x3e]), num_success, num_failure);
  
  // Short //-------------------------------------/
  print!("<<{:^58}>>", "short list");

  let res_short=send_string_query_le_uds(handle, "`short$8 -128 1260").await?;
  assert_to_truefalse!(res_short, QGEN::new_short_list(Attribute::None, vec![8_i16, -128, 1260]), num_success, num_failure);
  
  // Int //---------------------------------------/
  print!("<<{:^58}>>", "int list");

  let res_int=send_string_query_le_uds(handle, "enlist 65537i").await?;
  assert_to_truefalse!(res_int, QGEN::new_int_list(Attribute::None, vec![65537_i32]), num_success, num_failure);
  
  // Int //---------------------------------------/
  print!("<<{:^58}>>", "long list");

  let res_long=send_string_query_le_uds(handle, "200 300 300").await?;
  assert_to_truefalse!(res_long, QGEN::new_long_list(Attribute::None, vec![200_i64, 300, 300]), num_success, num_failure);
  
  // Long //--------------------------------------/
  print!("<<{:^58}>>", "real list");

  let res_real=send_string_query_le_uds(handle, "`s#2.35 102.32 82389.679e").await?;
  assert_to_truefalse!(res_real, QGEN::new_real_list(Attribute::Sorted, vec![2.35_f32, 102.32, 82389.679]), num_success, num_failure);
  
  // Float //--------------------------------------/
  print!("<<{:^58}>>", "float list");

  let res_float=send_string_query_le_uds(handle, "(acos; asin) @\\: 1").await?;
  assert_to_truefalse_float_list!(res_float, vec![0_f64, 1.570796_f64], 0.000001, num_success, num_failure);
  
  // Char //---------------------------------------/
  print!("<<{:^58}>>", "string");

  let res_char=send_string_query_le_uds(handle, ".Q.a[0 1 2 3]").await?;
  assert_to_truefalse!(res_char, QGEN::new_char_list(Attribute::None, "abcd"), num_success, num_failure);
    
  // Symbol //-------------------------------------/
  print!("<<{:^58}>>", "symbol list");

  let res_symbol=send_string_query_le_uds(handle, "`u#`Kx`Systems").await?;
  assert_to_truefalse!(res_symbol, QGEN::new_symbol_list(Attribute::Unique, vec!["Kx", "Systems"]), num_success, num_failure);

  // Timestamp //----------------------------------/
  print!("<<{:^58}>>", "timestamp list");
  
  let res_timestamp=send_string_query_le_uds(handle, "2007.10.12D18:43:20.123456789 + 1D 2D").await?;
  assert_to_truefalse!(res_timestamp, QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2007, 10, 13, 18, 43, 20, 123456789), (2007, 10, 14, 18, 43, 20, 123456789)]), num_success, num_failure);

  // Month //--------------------------------------/
  print!("<<{:^58}>>", "month list");

  let res_month=send_string_query_le_uds(handle, "`month$79 103 221").await?;
  assert_to_truefalse!(res_month, QGEN::new_month_list_ym(Attribute::None, vec![(2006, 8), (2008, 8), (2018, 6)]), num_success, num_failure);

  // Date //---------------------------------------/
  print!("<<{:^58}>>", "date list");

  let res_date=send_string_query_le_uds(handle, "`s#2018.09.20 2019.03.10 2020.07.12").await?;
  assert_to_truefalse!(res_date, QGEN::new_date_list_ymd(Attribute::Sorted, vec![(2018, 9, 20), (2019, 3, 10), (2020, 7, 12)]), num_success, num_failure);
  
  // Datetime //-----------------------------------/
  print!("<<{:^58}>>", "datetime list");

  let res_datetime=send_string_query_le_uds(handle, "2020.09.03T08:50:48.257 + til 3").await?;
  assert_to_truefalse!(res_datetime, QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2020, 9, 3, 8, 50, 48, 257), (2020, 9, 4, 8, 50, 48, 257), (2020, 9, 5, 8, 50, 48, 257)]), num_success, num_failure);

  // Timespan //-----------------------------------/
  print!("<<{:^58}>>", "timespan list");
  
  let res_timespan=send_string_query_le_uds(handle, "1D 2D 3D+1").await?;
  assert_to_truefalse!(res_timespan, QGEN::new_timespan_list_nanos(Attribute::None, vec![86400000000001_i64, 172800000000001, 259200000000001]), num_success, num_failure);

  // Minute //--------------------------------------/
  print!("<<{:^58}>>", "minute list");
  
  let res_minute=send_string_query_le_uds(handle, "enlist 10:32").await?;
  assert_to_truefalse!(res_minute, QGEN::new_minute_list(Attribute::None, vec![QTimeGEN::new_minute(NaiveTime::from_hms(10, 32, 0))]), num_success, num_failure);

  // Second //--------------------------------------/
  print!("<<{:^58}>>", "second list");

  let res_second=send_string_query_le_uds(handle, "18:19:31 19:35:22").await?;
  assert_to_truefalse!(res_second, QGEN::new_second_list_hms(Attribute::None, vec![(18, 19, 31), (19, 35, 22)]), num_success, num_failure);

  // Time //----------------------------------------/
  print!("<<{:^58}>>", "time list");

  let res_time=send_string_query_le_uds(handle, "07:15:00.902 12:30:45.678").await?;
  assert_to_truefalse!(res_time, QGEN::new_time_list_hms_millis(Attribute::None, vec![(7, 15, 0, 902), (12, 30, 45, 678)]), num_success, num_failure);
  
  // General List //--------------------------------/
  print!("<<{:^58}>>", "general list");

  let res_mixed=send_string_query_le_uds(handle, "(4 5i; `s#0 1 2 3; 2020.03.16; 2.5 1023.71e; `s#4 5h; 63979.32113; 12:30:45.123 22:51:59.030; `p#00011b; 2020.01.03 2020.3.16 2020.08.20; 12:30:45 22:51:59; 2013.04m; \"don't ignore me!\"; 2010.02 2020.05 2013.04m; `u#`more`intensive`test; enlist 1D+300; 2012.04.20D21:17:18.229100200 2012.04.21D18:35:49.469050213)").await?;
  assert_to_truefalse!(res_mixed, QGEN::new_mixed_list(vec![
    QGEN::new_int_list(Attribute::None, vec![4_i32, 5]),
    QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2, 3]),
    QGEN::new_date_ymd(2020, 3, 16),
    QGEN::new_real_list(Attribute::None, vec![2.5_f32, 1023.71]),
    QGEN::new_short_list(Attribute::Sorted, vec![4_i16, 5]),
    QGEN::new_float(63979.32113_f64),
    QGEN::new_time_list_hms_millis(Attribute::None, vec![(12, 30, 45, 123), (22, 51, 59, 30)]),
    QGEN::new_bool_list(Attribute::Parted, vec![false, false, false, true, true]),
    QGEN::new_date_list_ymd(Attribute::None, vec![(2020, 1, 3), (2020, 3, 16), (2020, 8, 20)]),
    QGEN::new_second_list_hms(Attribute::None, vec![(12, 30, 45), (22, 51, 59)]),
    QGEN::new_month_ym(2013, 4),
    QGEN::new_char_list(Attribute::None, "don't ignore me!"),
    QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2010, 2, 1), Utc.ymd(2020, 5, 1), Utc.ymd(2013, 4, 1)]),
    QGEN::new_symbol_list(Attribute::Unique, vec![String::from("more"), String::from("intensive"), String::from("test")]),
    QGEN::new_timespan_list_nanos(Attribute::None, vec![86400000000300_i64]),
    QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 4, 20).and_hms_nano(21, 17, 18, 229100200), Utc.ymd(2012, 4, 21).and_hms_nano(18, 35, 49, 469050213)])
  ]), num_success, num_failure);
  
  print!("<<{:^58}>>", "general list 2");

  let res_mixed_dict_table_null=send_string_query_le_uds(handle, "(`a`b`c!1 2 3; `d`e!100.12 113.433; ([] a:1 2; b:2020.03.12D03:15:00.987 2020.05.30D19:14:24.0100304); ::)").await?;
  assert_to_truefalse!(res_mixed_dict_table_null, QGEN::new_mixed_list(
    vec![
      QGEN::new_dictionary(QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]), QGEN::new_long_list(Attribute::None, vec![1_i64, 2, 3])), 
      QGEN::new_dictionary(QGEN::new_symbol_list(Attribute::None, vec!["d", "e"]), QGEN::new_float_list(Attribute::None, vec![100.12_f64, 113.433])),
      QGEN::new_table(
        vec!["a", "b"],
        vec![
          QGEN::new_long_list(Attribute::None, vec![1_i64, 2]),
          QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2020, 3, 12, 3, 15, 0, 987000000), (2020, 5, 30, 19, 14, 24, 10030400)])
        ]
      ).expect("Failed to build table"),
      QGEN::new_general_null()
    ]
  ), num_success, num_failure);
  
  Ok((num_success, num_failure))
}

/*
* Test one aspect of interface:
* - serialize list q object in both Big Endian and Little Endian
*/
async fn serialize_list_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Serialize List ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Boolean //-------------------------------------/
  print!("<<{:^58}>>", "bool list");

  let res_bool=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_bool_list(Attribute::None, vec![true, false])])).await?;
  assert_to_truefalse!(res_bool, QGEN::new_bool_list(Attribute::None, vec![true, false]), num_success, num_failure);

  // GUID //----------------------------------------/
  print!("<<{:^58}>>", "GUID list");

  let res_GUID=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_GUID_list(Attribute::None, vec![[0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]])])).await?;
  assert_to_truefalse!(res_GUID, QGEN::new_GUID_list(Attribute::None, vec![[0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]]), num_success, num_failure);
  
  // Byte //----------------------------------------/
  print!("<<{:^58}>>", "byte list");

  let res_byte=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_byte_list(Attribute::None, vec![0x3c, 0x22, 0x4f])])).await?;
  assert_to_truefalse!(res_byte, QGEN::new_byte_list(Attribute::None, vec![0x3c, 0x22, 0x4f]), num_success, num_failure);

  // Short //---------------------------------------/
  print!("<<{:^58}>>", "short list - query sent in LE");

  let res_short=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_short_list(Attribute::Sorted, vec![70_i16, 128, 1028, 2000])])).await?;
  assert_to_truefalse!(res_short, QGEN::new_short_list(Attribute::Sorted, vec![70_i16, 128, 1028, 2000]), num_success, num_failure);

  print!("<<{:^58}>>", "short list - query sent in BE");

  let res_short=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_short_list(Attribute::Sorted, vec![70_i16, 128, 1028, 2000])])).await?;
  assert_to_truefalse!(res_short, QGEN::new_short_list(Attribute::Sorted, vec![70_i16, 128, 1028, 2000]), num_success, num_failure);

  // Int //-----------------------------------------/
  print!("<<{:^58}>>", "int list - query sent in LE");

  let res_int=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_int_list(Attribute::None, vec![234789_i32, -34567789])])).await?;
  assert_to_truefalse!(res_int, QGEN::new_int_list(Attribute::None, vec![234789_i32, -34567789]), num_success, num_failure);

  print!("<<{:^58}>>", "int list - query sent in BE");

  let res_int=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_int_list(Attribute::None, vec![234789_i32, -34567789])])).await?;
  assert_to_truefalse!(res_int, QGEN::new_int_list(Attribute::None, vec![234789_i32, -34567789]), num_success, num_failure);

  // Long //----------------------------------------/
  print!("<<{:^58}>>", "long list - query sent in LE");

  let res_long=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_long_list(Attribute::None, vec![86400000000000_i64, -86400000000000_i64])])).await?;
  assert_to_truefalse!(res_long, QGEN::new_long_list(Attribute::None, vec![86400000000000_i64, -86400000000000_i64]), num_success, num_failure);

  print!("<<{:^58}>>", "long list - query sent in BE");

  let res_long=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_long_list(Attribute::None, vec![86400000000000_i64, -86400000000000_i64])])).await?;
  assert_to_truefalse!(res_long, QGEN::new_long_list(Attribute::None, vec![86400000000000_i64, -86400000000000_i64]), num_success, num_failure);

  // Real //----------------------------------------/
  print!("<<{:^58}>>", "real list - query sent in LE");

  let res_real=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_real_list(Attribute::Sorted, vec![-1.25_f32, 100.23, 3000.5639])])).await?;
  assert_to_truefalse!(res_real, QGEN::new_real_list(Attribute::Sorted, vec![-1.25_f32, 100.23, 3000.5639]), num_success, num_failure);

  print!("<<{:^58}>>", "real list - query sent in BE");

  let res_real=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_real_list(Attribute::Sorted, vec![-1.25_f32, 100.23, 3000.5639])])).await?;
  assert_to_truefalse!(res_real, QGEN::new_real_list(Attribute::Sorted, vec![-1.25_f32, 100.23, 3000.5639]), num_success, num_failure);

  // Float //---------------------------------------/
  print!("<<{:^58}>>", "real list - query sent in LE");

  let res_float=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_float_list(Attribute::None, vec![103.678_f64, 0.00034])])).await?;
  //assert_to_truefalse_float_list!(res_float, vec![103.678_f64, 0.00034], 0.00001);
  assert_to_truefalse!(res_float, QGEN::new_float_list(Attribute::None, vec![103.678_f64, 0.00034]), num_success, num_failure);

  print!("<<{:^58}>>", "real list - query sent in BE");

  let res_float=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_float_list(Attribute::None, vec![103.678_f64, 0.00034])])).await?;
  assert_to_truefalse_float_list!(res_float, vec![103.678_f64, 0.00034], 0.00001, num_success, num_failure);

  // Char //----------------------------------------/
  print!("<<{:^58}>>", "real list");

  let res_char=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_char_list(Attribute::Parted, "aabbccc")])).await?;
  assert_to_truefalse!(res_char, QGEN::new_char_list(Attribute::Parted, "aabbccc"), num_success, num_failure);

  // Symbol //--------------------------------------/
  print!("<<{:^58}>>", "symbol list");

  let res_symbol=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_symbol_list(Attribute::Unique, vec!["kdb+", "q"])])).await?;
  assert_to_truefalse!(res_symbol, QGEN::new_symbol_list(Attribute::Unique, vec!["kdb+", "q"]), num_success, num_failure);

  // Timespan //------------------------------------/
  print!("<<{:^58}>>", "timestamp list - query sent in LE");

  let res_timestamp=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2018, 2, 18, 4, 0, 0, 100), (2019, 12, 3, 4, 5, 10, 3456)])])).await?;
  assert_to_truefalse!(res_timestamp, QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2018, 2, 18, 4, 0, 0, 100), (2019, 12, 3, 4, 5, 10, 3456)]), num_success, num_failure);
  
  print!("<<{:^58}>>", "timestamp list - query sent in BE");

  let res_timestamp=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2018, 2, 18, 4, 0, 0, 100), (2019, 12, 3, 4, 5, 10, 3456)])])).await?;
  assert_to_truefalse!(res_timestamp, QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2018, 2, 18, 4, 0, 0, 100), (2019, 12, 3, 4, 5, 10, 3456)]), num_success, num_failure);

  // Month //---------------------------------------/
  print!("<<{:^58}>>", "month list - query sent in LE");

  let res_month=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_month_list_ym(Attribute::None, vec![(2013, 9), (2009, 2)])])).await?;
  assert_to_truefalse!(res_month, QGEN::new_month_list_ym(Attribute::None, vec![(2013, 9), (2009, 2)]), num_success, num_failure);

  print!("<<{:^58}>>", "month list - query sent in BE");

  let res_month=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_month_list_ym(Attribute::None, vec![(2013, 9), (2009, 2)])])).await?;
  assert_to_truefalse!(res_month, QGEN::new_month_list_ym(Attribute::None, vec![(2013, 9), (2009, 2)]), num_success, num_failure);

  // Date //----------------------------------------/
  print!("<<{:^58}>>", "date list - query sent in LE");

  let res_date=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2000, 2, 9)])])).await?;
  assert_to_truefalse!(res_date, QGEN::new_date_list_ymd(Attribute::None, vec![(2000, 2, 9)]), num_success, num_failure);

  print!("<<{:^58}>>", "date list - query sent in BE");

  let res_date=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2000, 2, 9)])])).await?;
  assert_to_truefalse!(res_date, QGEN::new_date_list_ymd(Attribute::None, vec![(2000, 2, 9)]), num_success, num_failure);

  // Datetime //----------------------------------------/
  print!("<<{:^58}>>", "datetime list - query sent in LE");

  let res_datetime=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2004, 6, 17, 11, 32, 40, 803)])])).await?;
  assert_to_truefalse!(res_datetime, QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2004, 6, 17, 11, 32, 40, 803)]), num_success, num_failure);

  print!("<<{:^58}>>", "datetime list - query sent in BE");

  let res_datetime=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2004, 6, 17, 11, 32, 40, 803)])])).await?;
  assert_to_truefalse!(res_datetime, QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2004, 6, 17, 11, 32, 40, 803)]), num_success, num_failure);

  // Timespan //----------------------------------------/
  print!("<<{:^58}>>", "timespan list - query sent in LE");

  let res_timespan=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timespan_list_nanos(Attribute::None, vec![999_i64, 10000, 100000000])])).await?;
  assert_to_truefalse!(res_timespan, QGEN::new_timespan_list_nanos(Attribute::None, vec![999_i64, 10000, 100000000]), num_success, num_failure);

  print!("<<{:^58}>>", "timespan list - query sent in BE");

  let res_timespan=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_timespan_list_nanos(Attribute::None, vec![999_i64, 10000, 100000000])])).await?;
  assert_to_truefalse!(res_timespan, QGEN::new_timespan_list_nanos(Attribute::None, vec![999_i64, 10000, 100000000]), num_success, num_failure);

  // Minute //------------------------------------------/
  print!("<<{:^58}>>", "minute list - query sent in LE");
  
  let res_minute=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_minute_list_hm(Attribute::None, vec![(12, 21), (3,2)])])).await?;
  assert_to_truefalse!(res_minute, QGEN::new_minute_list_hm(Attribute::None, vec![(12, 21), (3,2)]), num_success, num_failure);

  print!("<<{:^58}>>", "minute list - query sent in BE");

  let res_minute=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_minute_list_hm(Attribute::None, vec![(12, 21), (3,2)])])).await?;
  assert_to_truefalse!(res_minute, QGEN::new_minute_list_hm(Attribute::None, vec![(12, 21), (3,2)]), num_success, num_failure);

  // Second //------------------------------------------/
  print!("<<{:^58}>>", "second list - query sent in LE");

  let res_second=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_second_list_hms(Attribute::Sorted, vec![(3, 17, 26), (4, 0, 49)])])).await?;
  assert_to_truefalse!(res_second, QGEN::new_second_list_hms(Attribute::Sorted, vec![(3, 17, 26), (4, 0, 49)]), num_success, num_failure);

  print!("<<{:^58}>>", "second list - query sent in BE");

  let res_second=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_second_list_hms(Attribute::Sorted, vec![(3, 17, 26), (4, 0, 49)])])).await?;
  assert_to_truefalse!(res_second, QGEN::new_second_list_hms(Attribute::Sorted, vec![(3, 17, 26), (4, 0, 49)]), num_success, num_failure);

  // Time //--------------------------------------------/
  print!("<<{:^58}>>", "time list - query sent in LE");

  let res_time=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_time_list_hms_millis(Attribute::None, vec![(21, 56, 7, 302), (0, 4, 15, 0)])])).await?;
  assert_to_truefalse!(res_time, QGEN::new_time_list_hms_millis(Attribute::None, vec![(21, 56, 7, 302), (0, 4, 15, 0)]), num_success, num_failure);

  print!("<<{:^58}>>", "time list - query sent in BE");

  let res_time=send_query_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), QGEN::new_time_list_hms_millis(Attribute::None, vec![(21, 56, 7, 302), (0, 4, 15, 0)])])).await?;
  assert_to_truefalse!(res_time, QGEN::new_time_list_hms_millis(Attribute::None, vec![(21, 56, 7, 302), (0, 4, 15, 0)]), num_success, num_failure);

  // General List //------------------------------------/
  print!("<<{:^58}>>", "general list - query sent in LE");

  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_char_list(Attribute::None, "set"),
    QGEN::new_symbol("a"),
    QGEN::new_mixed_list(vec![
      QGEN::new_long(42),
      QGEN::new_real_list(Attribute::Sorted, vec![3.927524_f32, 5.170911]),
      QGEN::new_timestamp_ymd_hms_nanos(2020, 2, 10, 3, 19, 3, 247856731),
      QGEN::new_symbol_list(Attribute::None, vec!["KxSystems", "kdb+"]),
      QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2020, 10, 1, 3, 30, 12, 45), (2008, 2, 18, 21, 39, 10, 567)]),
      QGEN::new_char('k')
    ])
  ])).await?;

  let res_mixed=send_string_query_le_uds(handle, "a").await?;
  assert_to_truefalse!(res_mixed, 
    QGEN::new_mixed_list(vec![
      QGEN::new_long(42),
      QGEN::new_real_list(Attribute::Sorted, vec![3.927524_f32, 5.170911]),
      QGEN::new_timestamp_ymd_hms_nanos(2020, 2, 10, 3, 19, 3, 247856731),
      QGEN::new_symbol_list(Attribute::None, vec!["KxSystems", "kdb+"]),
      QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2020, 10, 1, 3, 30, 12, 45), (2008, 2, 18, 21, 39, 10, 567)]),
      QGEN::new_char('k')
    ]), num_success, num_failure);

  print!("<<{:^58}>>", "general list - query sent in BE");

  send_query_async_be_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_char_list(Attribute::None, "set"),
    QGEN::new_symbol("a"),
    QGEN::new_mixed_list(vec![
      QGEN::new_long(42),
      QGEN::new_real_list(Attribute::Sorted, vec![3.927524_f32, 5.170911]),
      QGEN::new_timestamp_ymd_hms_nanos(2020, 2, 10, 3, 19, 3, 247856731),
      QGEN::new_symbol_list(Attribute::None, vec!["kdb+", "KxSystems"]),
      QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2020, 10, 1, 3, 30, 12, 45), (2008, 2, 18, 21, 39, 10, 567)]),
      QGEN::new_char('k')
    ])
  ])).await?;

  let res_mixed=send_string_query_le_uds(handle, "a").await?;
  assert_to_truefalse!(res_mixed, 
    QGEN::new_mixed_list(vec![
      QGEN::new_long(42),
      QGEN::new_real_list(Attribute::Sorted, vec![3.927524_f32, 5.170911]),
      QGEN::new_timestamp_ymd_hms_nanos(2020, 2, 10, 3, 19, 3, 247856731),
      QGEN::new_symbol_list(Attribute::None, vec!["kdb+", "KxSystems"]),
      QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2020, 10, 1, 3, 30, 12, 45), (2008, 2, 18, 21, 39, 10, 567)]),
      QGEN::new_char('k')
    ]), num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test deserialization of null or infinity q object
*/
async fn deserialize_null_infinity_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Deserialize Null & Infinity ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Non Float Null //----------------------------------/
  print!("<<{:^58}>>", "non-float null");

  let res_null=send_string_query_le_uds(handle, "(0Ng; 0Nh; 0Ni; 0Nj; 0Np; 0Nm; 0Nd; 0Nz; 0Nn; 0Nu; 0Nv; 0Nt)").await?;
  assert_to_truefalse!(res_null, QGEN::new_mixed_list(vec![
    QGEN::new_GUID(Q_0Ng), QGEN::new_short(Q_0Nh), QGEN::new_int(Q_0Ni), QGEN::new_long(Q_0Nj), QGEN::new_timestamp(Q_0Np), QGEN::new_month(Q_0Nm), QGEN::new_date(Q_0Nd), QGEN::new_datetime(Q_0Nz), QGEN::new_timespan(*Q_0Nn), QGEN::new_minute(Q_0Nu), QGEN::new_second(Q_0Nv), QGEN::new_time(Q_0Nt)
  ]), num_success, num_failure);

  print!("<<{:^58}>>", "float null");
  let res_decimal_null=send_string_query_le_uds(handle, "(0Ne; 0n)").await?;
  let rust_q_vec=res_decimal_null.into_q_vec()?;
  assert_to_truefalse_custom!(||{
    assert!(rust_q_vec[0].clone().into_f32().expect("Failed to convert into f32").is_nan());
    assert!(rust_q_vec[1].clone().into_f64().expect("Failed to convert into f64").is_nan());
  }, num_success, num_failure);
  
  print!("<<{:^58}>>", "non-float infinity");

  let res_infinity=send_string_query_le_uds(handle, "(0Wh; -0Wh; 0Wi; -0Wi; 0Wj; -0Wj; 0Wp; 0Wm; 0Wd; 0Wz; 0Wn; -0Wn; 0Wu; 0Wv; 0Wt)").await?;
  assert_to_truefalse!(res_infinity, QGEN::new_mixed_list(vec![
    QGEN::new_short(Q_0Wh), QGEN::new_short(Q_NEG_0Wh), QGEN::new_int(Q_0Wi), QGEN::new_int(Q_NEG_0Wi), QGEN::new_long(Q_0Wj), QGEN::new_long(Q_NEG_0Wj), QGEN::new_timestamp(Q_0Wp), QGEN::new_month(Q_0Wm), QGEN::new_date(Q_0Wd), QGEN::new_datetime(*Q_0Wz), QGEN::new_timespan(*Q_0Wn), QGEN::new_timespan(*Q_NEG_0Wn), QGEN::new_minute(Q_0Wu), QGEN::new_second(Q_0Wv), QGEN::new_time(Q_0Wt)
  ]), num_success, num_failure);

  print!("<<{:^58}>>", "decimal infinity");

  let res_decimal_infinity=send_string_query_le_uds(handle, "(0We; -0We; 0w; -0w)").await?;
  let rust_q_vec = res_decimal_infinity.into_q_vec()?;
  assert_to_truefalse_custom!(||{
    assert!(rust_q_vec[0].clone().into_f32().expect("Failed to convert into f32").is_infinite());
    assert!(rust_q_vec[1].clone().into_f32().expect("Failed to convert into f32").is_sign_negative() && rust_q_vec[1].clone().into_f32().expect("Failed to convert into f32").is_infinite());
    assert!(rust_q_vec[2].clone().into_f64().expect("Failed to convert into f64").is_infinite());
    assert!(rust_q_vec[3].clone().into_f64().expect("Failed to convert into f64").is_sign_negative() && rust_q_vec[3].clone().into_f64().expect("Failed to convert into f64").is_infinite());
  }, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test serialization of null or infinity q object.
* Note: All basic type q objects have been tested in both Little Endian and Big Endian;
* thereofore sending only in Little Endian is enough.
*/
async fn serialize_null_infinity_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Serialize Null & Infinity ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Non Float Null //----------------------------------/
  print!("<<{:^58}>>", "non-float null");

  let res_null=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), 
    QGEN::new_mixed_list(vec![
      QGEN::new_GUID(Q_0Ng), QGEN::new_short(Q_0Nh), QGEN::new_int(Q_0Ni), QGEN::new_long(Q_0Nj), QGEN::new_timestamp(Q_0Np), QGEN::new_month(Q_0Nm), QGEN::new_date(Q_0Nd), QGEN::new_datetime(Q_0Nz), QGEN::new_timespan(*Q_0Nn), QGEN::new_minute(Q_0Nu), QGEN::new_second(Q_0Nv), QGEN::new_time(Q_0Nt)
    ])
  ])).await?;
  assert_to_truefalse!(res_null, QGEN::new_mixed_list(vec![
    QGEN::new_GUID(Q_0Ng), QGEN::new_short(Q_0Nh), QGEN::new_int(Q_0Ni), QGEN::new_long(Q_0Nj), QGEN::new_timestamp(Q_0Np), QGEN::new_month(Q_0Nm), QGEN::new_date(Q_0Nd), QGEN::new_datetime(Q_0Nz), QGEN::new_timespan(*Q_0Nn), QGEN::new_minute(Q_0Nu), QGEN::new_second(Q_0Nv), QGEN::new_time(Q_0Nt)
  ]), num_success, num_failure);

  print!("<<{:^58}>>", "decimal null");

  let res_decimal_null_list=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), 
    QGEN::new_mixed_list(vec![
      QGEN::new_real(Q_0Ne),
      QGEN::new_float(Q_0n)
    ])
  ])).await?;
  let rust_q_vec=res_decimal_null_list.into_q_vec()?;
  assert_to_truefalse_custom!(||{
    assert!(rust_q_vec[0].clone().into_f32().expect("Failed to convert into f32").is_nan());
    assert!(rust_q_vec[1].clone().into_f64().expect("Failed to convert into f64").is_nan());
  }, num_success, num_failure);
  
  print!("<<{:^58}>>", "non-float infinity");

  let res_infinity=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(),
    QGEN::new_mixed_list(vec![
      QGEN::new_short(Q_0Wh), QGEN::new_short(Q_NEG_0Wh), QGEN::new_int(Q_0Wi), QGEN::new_int(Q_NEG_0Wi), QGEN::new_long(Q_0Wj), QGEN::new_long(Q_NEG_0Wj), QGEN::new_timestamp(Q_0Wp), QGEN::new_month(Q_0Wm), QGEN::new_date(Q_0Wd), QGEN::new_datetime(*Q_0Wz), QGEN::new_timespan(*Q_0Wn), QGEN::new_timespan(*Q_NEG_0Wn), QGEN::new_minute(Q_0Wu), QGEN::new_second(Q_0Wv), QGEN::new_time(Q_0Wt)
    ])
  ])).await?;
  assert_to_truefalse!(res_infinity, QGEN::new_mixed_list(vec![
    QGEN::new_short(Q_0Wh), QGEN::new_short(Q_NEG_0Wh), QGEN::new_int(Q_0Wi), QGEN::new_int(Q_NEG_0Wi), QGEN::new_long(Q_0Wj), QGEN::new_long(Q_NEG_0Wj), QGEN::new_timestamp(Q_0Wp), QGEN::new_month(Q_0Wm), QGEN::new_date(Q_0Wd), QGEN::new_datetime(*Q_0Wz), QGEN::new_timespan(*Q_0Wn), QGEN::new_timespan(*Q_NEG_0Wn), QGEN::new_minute(Q_0Wu), QGEN::new_second(Q_0Wv), QGEN::new_time(Q_0Wt)
  ]), num_success, num_failure);

  print!("<<{:^58}>>", "float infinity");

  let res_decimal_infinity=send_query_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_general_null(), 
    QGEN::new_mixed_list(vec![QGEN::new_real(Q_0We), QGEN::new_real(Q_NEG_0We), QGEN::new_float(Q_0w), QGEN::new_float(Q_NEG_0w)])
  ])).await?;
  let rust_q_vec = res_decimal_infinity.into_q_vec()?;
  assert_to_truefalse_custom!(||{
    assert!(rust_q_vec[0].clone().into_f32().expect("Failed to convert into f32").is_infinite());
    assert!(rust_q_vec[1].clone().into_f32().expect("Failed to convert into f32").is_sign_negative() && rust_q_vec[1].clone().into_f32().expect("Failed to convert into f32").is_infinite());
    assert!(rust_q_vec[2].clone().into_f64().expect("Failed to convert into f64").is_infinite());
    assert!(rust_q_vec[3].clone().into_f64().expect("Failed to convert into f64").is_sign_negative() && rust_q_vec[3].clone().into_f64().expect("Failed to convert into f64").is_infinite());
  }, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test deserialization of dictionary q object in both Big Endian and Little Endian
*/
async fn deserialize_dictionary_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Deserialize Dictionary ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Atom Dictionry //----------------------------------/
  print!("<<{:^58}>>", "atom simple dictionary - query sent in LE");

  let res_atom_dict=send_string_query_le_uds(handle, "`a`b`c!2009.01 2001.12 2017.08m").await?;
  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
    ), num_success, num_failure
  );

  print!("<<{:^58}>>", "atom simple dictionary - query sent in BE");

  let res_atom_dict=send_string_query_be_uds(handle, "`a`b`c!2009.01 2001.12 2017.08m").await?;
  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
    ), num_success, num_failure
  );

  print!("<<{:^58}>>", "atom mixed dictionary - query sent in LE");

  let res_atom_dict=send_string_query_le_uds(handle, "`a`b`c`d!(2020.10.01D00:09:28.879392249; `Rust; 0.032809; 09:23:04.540)").await?;
  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c", "d"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_ymd_hms_nanos(2020, 10, 1, 0, 9, 28, 879392249),
      QGEN::new_symbol("Rust"),
      QGEN::new_float(0.032809_f64),
      QGEN::new_time_hms_millis(9, 23, 4, 540)
    ])
  ), num_success, num_failure);

  print!("<<{:^58}>>", "atom mixed dictionary - query sent in BE");

  let res_atom_dict=send_string_query_be_uds(handle, "`a`b`c`d!(2020.10.01D00:09:28.879392249; `Rust; 0.032809; 09:23:04.540)").await?;
  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c", "d"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_ymd_hms_nanos(2020, 10, 1, 0, 9, 28, 879392249),
      QGEN::new_symbol("Rust"),
      QGEN::new_float(0.032809_f64),
      QGEN::new_time_hms_millis(9, 23, 4, 540)
    ])
  ), num_success, num_failure);

  // Sorted Dictionry //--------------------------------/
  print!("<<{:^58}>>", "sorted dictionary - query sent in LE");

  let res_sorted_dict=send_string_query_le_uds(handle, "`s#`john`luke`mark`mattew!(149.582 39.78; 2019.11.01 2012.04.09 2000.02.03; 30 93 0 44; 10001b)").await?;
  assert_to_truefalse!(res_sorted_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::Sorted, vec!["john", "luke", "mark", "mattew"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_float_list(Attribute::None, vec![149.582_f64, 39.78]),
      QGEN::new_date_list_ymd(Attribute::None, vec![(2019, 11, 1), (2012, 4, 9), (2000, 2, 3)]),
      QGEN::new_long_list(Attribute::None, vec![30_i64, 93, 0, 44]),
      QGEN::new_bool_list(Attribute::None, vec![true, false, false, false, true])
    ])
  ), num_success, num_failure);

  print!("<<{:^58}>>", "sorted dictionary - query sent in BE");

  let res_sorted_dict=send_string_query_be_uds(handle, "`s#`john`luke`mark`mattew!(149.582 39.78; 2019.11.01 2012.04.09 2000.02.03; 30 93 0 44; 10001b)").await?;
  assert_to_truefalse!(res_sorted_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::Sorted, vec!["john", "luke", "mark", "mattew"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_float_list(Attribute::None, vec![149.582_f64, 39.78]),
      QGEN::new_date_list_ymd(Attribute::None, vec![(2019, 11, 1), (2012, 4, 9), (2000, 2, 3)]),
      QGEN::new_long_list(Attribute::None, vec![30_i64, 93, 0, 44]),
      QGEN::new_bool_list(Attribute::None, vec![true, false, false, false, true])
    ])
  ), num_success, num_failure);

  // List Dictionry //---------------------------------/
  print!("<<{:^58}>>", "sorted dictionary - query sent in LE");

  let res_list_dict=send_string_query_le_uds(handle, "`integer`times`syms`floats`bools`dates`timestamp`timestamps!(1 2i; 22:45:25.122 21:19:59.091; `p#`Belfast`Belfast`Newry`Newry`Newry`Tokyo`Tokyo; 2011.003 102.34 7.19995; 1101b; `s#2020.02.19 2020.07.19; 2012.09.09D20:10:52.347; 2012.09.09D20:10:52.347 2012.09.09D20:10:52.347000002)").await?;
  assert_to_truefalse!(res_list_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["integer", "times", "syms", "floats", "bools", "dates", "timestamp", "timestamps"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_int_list(Attribute::None, vec![1_i32, 2]),
      QGEN::new_time_list_hms_millis(Attribute::None, vec![(22, 45, 25, 122), (21, 19, 59, 091)]),
      QGEN::new_symbol_list(Attribute::Parted, vec!["Belfast", "Belfast", "Newry", "Newry", "Newry", "Tokyo", "Tokyo"]),
      QGEN::new_float_list(Attribute::None, vec![2011.003_f64, 102.34, 7.19995]),
      QGEN::new_bool_list(Attribute::None, vec![true, true, false, true]),
      QGEN::new_date_list(Attribute::Sorted, vec![Utc.ymd(2020, 2, 19), Utc.ymd(2020, 7, 19)]),
      QGEN::new_timestamp_ymd_hms_nanos(2012, 9, 9, 20, 10, 52, 347000000),
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000000), Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000002)])
    ])
  ), num_success, num_failure);

  print!("<<{:^58}>>", "sorted dictionary - query sent in BE");

  let res_list_dict=send_string_query_be_uds(handle, "`integer`times`syms`floats`bools`dates`timestamp`timestamps!(1 2i; 22:45:25.122 21:19:59.091; `p#`Belfast`Belfast`Newry`Newry`Newry`Tokyo`Tokyo; 2011.003 102.34 7.19995; 1101b; `s#2020.02.19 2020.07.19; 2012.09.09D20:10:52.347; 2012.09.09D20:10:52.347 2012.09.09D20:10:52.347000002)").await?;
  assert_to_truefalse!(res_list_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["integer", "times", "syms", "floats", "bools", "dates", "timestamp", "timestamps"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_int_list(Attribute::None, vec![1_i32, 2]),
      QGEN::new_time_list_hms_millis(Attribute::None, vec![(22, 45, 25, 122), (21, 19, 59, 091)]),
      QGEN::new_symbol_list(Attribute::Parted, vec!["Belfast", "Belfast", "Newry", "Newry", "Newry", "Tokyo", "Tokyo"]),
      QGEN::new_float_list(Attribute::None, vec![2011.003_f64, 102.34, 7.19995]),
      QGEN::new_bool_list(Attribute::None, vec![true, true, false, true]),
      QGEN::new_date_list(Attribute::Sorted, vec![Utc.ymd(2020, 2, 19), Utc.ymd(2020, 7, 19)]),
      QGEN::new_timestamp_ymd_hms_nanos(2012, 9, 9, 20, 10, 52, 347000000),
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000000), Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000002)])
    ])
  ), num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test serialization of dictionary q object in both Little Endian and Big Endian
*/
async fn serialize_dictionary_test(handle:&mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Dictionary Test ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Update Dictionry //--------------------------------/
  print!("<<{:^58}>>", "update dictionary - query sent in LE");

  send_string_query_async_le_uds(handle, "upd:upsert").await?;
  send_string_query_async_le_uds(handle, "dict:enlist[`]!enlist (::)").await?;
  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_symbol("upd"),
    QGEN::new_symbol("dict"),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::Sorted, vec!["a", "b", "c"]),
      QGEN::new_mixed_list(vec![QGEN::new_int_list(Attribute::None, vec![10, 20]), QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2010, 10, 30, 20, 1, 35, 256)]), QGEN::new_float(Q_0w)])
    )]
  )).await?;

  let res_dict=send_string_query_le_uds(handle, "dict").await?;
  assert_to_truefalse!(res_dict, 
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["", "a", "b", "c"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_general_null(),
        QGEN::new_int_list(Attribute::None, vec![10, 20]),
        QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2010, 10, 30, 20, 1, 35, 256)]),
        QGEN::new_float(Q_0w)
      ])
    ), num_success, num_failure);

  print!("<<{:^58}>>", "update dictionary - query sent in BE");

  send_string_query_async_le_uds(handle, "upd:upsert").await?;
  send_string_query_async_le_uds(handle, "dict:enlist[`]!enlist (::)").await?;
  send_query_async_be_uds(handle, QGEN::new_mixed_list(
    vec![
      QGEN::new_symbol("upd"),
      QGEN::new_symbol("dict"),
      QGEN::new_dictionary(
        QGEN::new_symbol_list(Attribute::Sorted, vec!["a", "b", "c"]),
        QGEN::new_mixed_list(vec![
          QGEN::new_int_list(Attribute::None, vec![10, 20]),
          QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2010, 10, 30, 20, 1, 35, 256)]),
          QGEN::new_float(Q_0w)
        ])
      )]
    )
  ).await?;

  let res_dict=send_string_query_le_uds(handle, "dict",).await?;
  assert_to_truefalse!(res_dict, 
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["", "a", "b", "c"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_general_null(),
        QGEN::new_int_list(Attribute::None, vec![10, 20]),
        QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2010, 10, 30, 20, 1, 35, 256)]),
        QGEN::new_float(Q_0w)
      ])
    ), num_success, num_failure);

  // Atom Dictionry //----------------------------------/
  print!("<<{:^58}>>", "atom simple dictionary - query sent in LE");

  let res_atom_dict=send_query_le_uds(handle, QGEN::new_mixed_list(
    vec![
      QGEN::new_general_null(),
      QGEN::new_dictionary(
        QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]),
        QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
      )
    ]
  )).await?;

  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
    ), num_success, num_failure
  );

  print!("<<{:^58}>>", "atom simple dictionary - query sent in BE");

  let res_atom_dict=send_query_be_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
    )
  ])).await?;
  
  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c"]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
    ), num_success, num_failure
  );

  print!("<<{:^58}>>", "atom mixed dictionary - query sent in LE");

  let res_atom_dict=send_query_le_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c", "d"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_timestamp_ymd_hms_nanos(2020, 10, 1, 0, 9, 28, 879392249),
        QGEN::new_symbol("Rust"),
        QGEN::new_float(0.032809_f64),
        QGEN::new_time_hms_millis(9, 23, 4, 540)
      ])
    )
  ])).await?;

  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c", "d"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_ymd_hms_nanos(2020, 10, 1, 0, 9, 28, 879392249),
      QGEN::new_symbol("Rust"),
      QGEN::new_float(0.032809_f64),
      QGEN::new_time_hms_millis(9, 23, 4, 540)
    ])
  ), num_success, num_failure);

  print!("<<{:^58}>>", "atom mixed dictionary - query sent in BE");

  let res_atom_dict=send_query_be_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c", "d"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_timestamp_ymd_hms_nanos(2020, 10, 1, 0, 9, 28, 879392249),
        QGEN::new_symbol("Rust"),
        QGEN::new_float(0.032809_f64),
        QGEN::new_time_hms_millis(9, 23, 4, 540)
      ])
    )
  ])).await?;
  
  assert_to_truefalse!(res_atom_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["a", "b", "c", "d"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_ymd_hms_nanos(2020, 10, 1, 0, 9, 28, 879392249),
      QGEN::new_symbol("Rust"),
      QGEN::new_float(0.032809_f64),
      QGEN::new_time_hms_millis(9, 23, 4, 540)
    ])
  ), num_success, num_failure);

  // Sorted Dictionry //--------------------------------/
  print!("<<{:^58}>>", "sorted dictionary - query sent in LE");

  let res_sorted_dict=send_query_le_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::Sorted, vec!["john", "luke", "mark", "mattew"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_float_list(Attribute::None, vec![149.582_f64, 39.78]),
        QGEN::new_date_list_ymd(Attribute::None, vec![(2019, 11, 1), (2012, 4, 9), (2000, 2, 3)]),
        QGEN::new_long_list(Attribute::None, vec![30_i64, 93, 0, 44]),
        QGEN::new_bool_list(Attribute::None, vec![true, false, false, false, true])
      ])
    )
  ])).await?;
  assert_to_truefalse!(res_sorted_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::Sorted, vec!["john", "luke", "mark", "mattew"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_float_list(Attribute::None, vec![149.582_f64, 39.78]),
      QGEN::new_date_list_ymd(Attribute::None, vec![(2019, 11, 1), (2012, 4, 9), (2000, 2, 3)]),
      QGEN::new_long_list(Attribute::None, vec![30_i64, 93, 0, 44]),
      QGEN::new_bool_list(Attribute::None, vec![true, false, false, false, true])
    ])
  ), num_success, num_failure);

  print!("<<{:^58}>>", "sorted dictionary - query sent in BE");

  let res_sorted_dict=send_query_be_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::Sorted, vec!["john", "luke", "mark", "mattew"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_float_list(Attribute::None, vec![149.582_f64, 39.78]),
        QGEN::new_date_list_ymd(Attribute::None, vec![(2019, 11, 1), (2012, 4, 9), (2000, 2, 3)]),
        QGEN::new_long_list(Attribute::None, vec![30_i64, 93, 0, 44]),
        QGEN::new_bool_list(Attribute::None, vec![true, false, false, false, true])
      ])
    )
  ])).await?;

  assert_to_truefalse!(res_sorted_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::Sorted, vec!["john", "luke", "mark", "mattew"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_float_list(Attribute::None, vec![149.582_f64, 39.78]),
      QGEN::new_date_list_ymd(Attribute::None, vec![(2019, 11, 1), (2012, 4, 9), (2000, 2, 3)]),
      QGEN::new_long_list(Attribute::None, vec![30_i64, 93, 0, 44]),
      QGEN::new_bool_list(Attribute::None, vec![true, false, false, false, true])
    ])
  ), num_success, num_failure);

  // List Dictionry //---------------------------------/
  print!("<<{:^58}>>", "list dictionary - query sent in LE");

  let res_list_dict=send_query_le_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["integer", "times", "syms", "floats", "bools", "dates", "timestamp", "timestamps"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_int_list(Attribute::None, vec![1_i32, 2]),
        QGEN::new_time_list_hms_millis(Attribute::None, vec![(22, 45, 25, 122), (21, 19, 59, 091)]),
        QGEN::new_symbol_list(Attribute::Parted, vec!["Belfast", "Belfast", "Newry", "Newry", "Newry", "Tokyo", "Tokyo"]),
        QGEN::new_float_list(Attribute::None, vec![2011.003_f64, 102.34, 7.19995]),
        QGEN::new_bool_list(Attribute::None, vec![true, true, false, true]),
        QGEN::new_date_list(Attribute::Sorted, vec![Utc.ymd(2020, 2, 19), Utc.ymd(2020, 7, 19)]),
        QGEN::new_timestamp_ymd_hms_nanos(2012, 9, 9, 20, 10, 52, 347000000),
        QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000000), Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000002)])
      ])
    )
  ])).await?;

  assert_to_truefalse!(res_list_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["integer", "times", "syms", "floats", "bools", "dates", "timestamp", "timestamps"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_int_list(Attribute::None, vec![1_i32, 2]),
      QGEN::new_time_list_hms_millis(Attribute::None, vec![(22, 45, 25, 122), (21, 19, 59, 091)]),
      QGEN::new_symbol_list(Attribute::Parted, vec!["Belfast", "Belfast", "Newry", "Newry", "Newry", "Tokyo", "Tokyo"]),
      QGEN::new_float_list(Attribute::None, vec![2011.003_f64, 102.34, 7.19995]),
      QGEN::new_bool_list(Attribute::None, vec![true, true, false, true]),
      QGEN::new_date_list(Attribute::Sorted, vec![Utc.ymd(2020, 2, 19), Utc.ymd(2020, 7, 19)]),
      QGEN::new_timestamp_ymd_hms_nanos(2012, 9, 9, 20, 10, 52, 347000000),
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000000), Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000002)])
    ])
  ), num_success, num_failure);

  print!("<<{:^58}>>", "list dictionary - query sent in BE");

  let res_list_dict=send_query_be_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_general_null(),
    QGEN::new_dictionary(
      QGEN::new_symbol_list(Attribute::None, vec!["integer", "times", "syms", "floats", "bools", "dates", "timestamp", "timestamps"]),
      QGEN::new_mixed_list(vec![
        QGEN::new_int_list(Attribute::None, vec![1_i32, 2]),
        QGEN::new_time_list_hms_millis(Attribute::None, vec![(22, 45, 25, 122), (21, 19, 59, 091)]),
        QGEN::new_symbol_list(Attribute::Parted, vec!["Belfast", "Belfast", "Newry", "Newry", "Newry", "Tokyo", "Tokyo"]),
        QGEN::new_float_list(Attribute::None, vec![2011.003_f64, 102.34, 7.19995]),
        QGEN::new_bool_list(Attribute::None, vec![true, true, false, true]),
        QGEN::new_date_list(Attribute::Sorted, vec![Utc.ymd(2020, 2, 19), Utc.ymd(2020, 7, 19)]),
        QGEN::new_timestamp_ymd_hms_nanos(2012, 9, 9, 20, 10, 52, 347000000),
        QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000000), Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000002)])
      ])
    )
  ])).await?;
  
  assert_to_truefalse!(res_list_dict, QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::None, vec!["integer", "times", "syms", "floats", "bools", "dates", "timestamp", "timestamps"]),
    QGEN::new_mixed_list(vec![
      QGEN::new_int_list(Attribute::None, vec![1_i32, 2]),
      QGEN::new_time_list_hms_millis(Attribute::None, vec![(22, 45, 25, 122), (21, 19, 59, 091)]),
      QGEN::new_symbol_list(Attribute::Parted, vec!["Belfast", "Belfast", "Newry", "Newry", "Newry", "Tokyo", "Tokyo"]),
      QGEN::new_float_list(Attribute::None, vec![2011.003_f64, 102.34, 7.19995]),
      QGEN::new_bool_list(Attribute::None, vec![true, true, false, true]),
      QGEN::new_date_list(Attribute::Sorted, vec![Utc.ymd(2020, 2, 19), Utc.ymd(2020, 7, 19)]),
      QGEN::new_timestamp_ymd_hms_nanos(2012, 9, 9, 20, 10, 52, 347000000),
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000000), Utc.ymd(2012, 9, 9).and_hms_nano(20, 10, 52, 347000002)])
    ])
  ), num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test deserialization of table q object
*/
async fn deserialize_table_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Deserialize Table ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Table //------------------------------------------/
  print!("<<{:^58}>>", "table");

  // define table
  send_string_query_async_le_uds(handle, "trade:flip `time`sym`price`size`ex!\"psfjc\"$\\:()").await?;
  send_string_query_async_le_uds(handle, "`trade insert (2020.06.01D07:02:13.238912781 2020.06.01D07:02:14.230892785 2020.06.01D07:03:01.137860387; `Kx`FD`Kx; 103.68 107.42 103.3; 1000 2000 3000; \"NLN\")").await?;
  send_string_query_async_le_uds(handle, "update sym:`g#sym from `trade").await?;

  let res_table=send_string_query_le_uds(handle, "select from trade").await?;
  assert_to_truefalse!(res_table, QGEN::new_table(
    vec!["time", "sym", "price", "size", "ex"],
    vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]).expect("Failed to build table"),
    num_success, num_failure
  );

  Ok((num_success, num_failure))
}

/*
* Test serialization of table q object in both Little Endian and Big Endian
* Note: As components of table are symbol list and general list, thorough check of
* these components are not conducted. They are covered in list test.
*/
async fn serialize_table_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^60}+\n", "|| Serialize Table ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Update Table //-----------------------------------/
  print!("<<{:^58}>>", "update table - query sent in LE");

  // define table
  send_string_query_async_le_uds(handle, "upd:insert").await?;
  send_string_query_async_le_uds(handle, "trade:flip `time`sym`price`size`ex!\"psfjc\"$\\:()").await?;
  // Update table
  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_symbol("upd"),
    QGEN::new_symbol("trade"),
    QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2020, 6, 1, 7, 2, 13, 238912781), (2020, 6, 1, 7, 2, 14, 230892785), (2020, 6, 1, 7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::None, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ])
  ])).await?;
  send_string_query_async_le_uds(handle, "update sym:`g#sym from `trade").await?;

  let res_table=send_string_query_le_uds(handle, "select from trade").await?;
  assert_to_truefalse!(res_table, QGEN::new_table(
    vec!["time", "sym", "price", "size", "ex"],
    vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]
  ).expect("Failed to build table"), num_success, num_failure);

  print!("<<{:^58}>>", "update table - query sent in BE");

  // define table
  send_string_query_async_le_uds(handle, "upd:insert").await?;
  send_string_query_async_le_uds(handle, "trade:flip `time`sym`price`size`ex!\"psfjc\"$\\:()").await?;
  // Update table
  send_query_async_be_uds(handle, QGEN::new_mixed_list(vec![
    QGEN::new_symbol("upd"),
    QGEN::new_symbol("trade"),
    QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2020, 6, 1, 7, 2, 13, 238912781), (2020, 6, 1, 7, 2, 14, 230892785), (2020, 6, 1, 7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::None, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ])
  ])).await?;
  send_string_query_async_le_uds(handle, "update sym:`g#sym from `trade").await?;

  let res_table=send_string_query_le_uds(handle, "select from trade").await?;
  assert_to_truefalse!(res_table, QGEN::new_table(
    vec!["time", "sym", "price", "size", "ex"],
    vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]
  ).expect("Failed to build table"), num_success, num_failure);


  Ok((num_success, num_failure))
}

/*
* Test deserialization of keyed table q object in both Little Endian and Big Endian
*/
async fn deserialize_keyed_table_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Deserialize Keyed Table ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Keyed Table //------------------------------------/
  print!("<<{:^58}>>", "keyed table");

  let res_keyed_table=send_string_query_le_uds(handle, "([id:`s#til 3; month: 2000.01 2000.02 2000.03m] stats:113.42 354.923 2749.4f; sym:`Newry`Belfast`London)").await?;
  assert_to_truefalse!(res_keyed_table, QGEN::new_keyed_table(
    vec!["id", "month"],
    vec![
      QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
    ],   
    vec!["stats", "sym"],
    vec![
      QGEN::new_float_list(Attribute::None, vec![113.42_f64, 354.923, 2749.4]),
      QGEN::new_symbol_list(Attribute::None, vec!["Newry", "Belfast", "London"])
    ]).expect("Failed to build keyed table"),
    num_success, num_failure
  );

  Ok((num_success, num_failure))
}

/*
* Test serialization of keyed table q object in both Little Endian and Big Endian.
* Note: As the components of keyed table are two tables, thorough test of table value
* itself is not necessary. It is covered in the table test. Therefore the test case is
* restricted to sending a simple keyed table for updating the table. 
*/
async fn serialize_keyed_table_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Serialize Keyed Table ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Keyed Table //------------------------------------/
  print!("<<{:^58}>>", "update keyed table - query sent in LE");

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
  ).expect("Failed to build keyed table");
  
  // Set keyed table
  send_string_query_async_le_uds(handle, "assign:set").await?;
  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("assign"), QGEN::new_symbol("keyedtab"), keyed_table])).await?;

  // Update the table
  send_string_query_async_le_uds(handle, "upd:upsert").await?;
  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("upd"), QGEN::new_symbol("keyedtab"), QGEN::new_mixed_list(vec![
    QGEN::new_long(1), QGEN::new_month_ym(2000, 2), QGEN::new_float(1000_f64), QGEN::new_symbol("GoldenCity")
  ])])).await?;

  let res_keyed_table=send_string_query_le_uds(handle, "keyedtab").await?;
  assert_to_truefalse!(res_keyed_table, QGEN::new_keyed_table(
    vec!["id", "month"],
    vec![
      QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
    ],   
    vec!["stats", "sym"],
    vec![
      QGEN::new_float_list(Attribute::None, vec![113.42_f64, 1000.0, 2749.4]),
      QGEN::new_symbol_list(Attribute::None, vec!["Newry", "GoldenCity", "London"])
    ]).expect("Failed to build keyed table"),
    num_success, num_failure
  );

  print!("<<{:^58}>>", "update keyed table - query sent in BE");

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
  ).expect("Failed to build keyed table");
  
  // Set keyed table
  send_string_query_async_le_uds(handle, "assign:set").await?;
  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("assign"), QGEN::new_symbol("keyedtab"), keyed_table])).await?;

  // Update the table
  send_string_query_async_le_uds(handle, "upd:upsert").await?;
  send_query_async_be_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("upd"), QGEN::new_symbol("keyedtab"), QGEN::new_mixed_list(vec![
    QGEN::new_long(1), QGEN::new_month_ym(2000, 2), QGEN::new_float(1000_f64), QGEN::new_symbol("GoldenCity")
  ])])).await?;

  let res_keyed_table=send_string_query_le_uds(handle, "keyedtab").await?;
  assert_to_truefalse!(res_keyed_table, QGEN::new_keyed_table(
    vec!["id", "month"],
    vec![
      QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
    ],   
    vec!["stats", "sym"],
    vec![
      QGEN::new_float_list(Attribute::None, vec![113.42_f64, 1000.0, 2749.4]),
      QGEN::new_symbol_list(Attribute::None, vec!["Newry", "GoldenCity", "London"])
    ]).expect("Failed to build keyed table"),
    num_success, num_failure
  );

  Ok((num_success, num_failure))
}

/*
* Test various atom constructors of each q type object if they provide the same value as each other.
* Note: Basic constructors have been tested in the tests above. The focus of this test is the other
* constructors provided for some types.
*/
fn atom_constructor_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Atom Constructor ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Timestamp //--------------------------------------/
  print!("<<{:^58}>>", "timestamp from nanosecond");

  // Base
  let q_timestamp=QGEN::new_timestamp_ymd_hms_nanos(2020, 4, 1, 3, 50, 12, 000001234);

  let q_timestamp2=QGEN::new_timestamp_nanos(KDB_TIMESTAMP_OFFSET + 639028212000001234_i64);
  assert_to_truefalse!(q_timestamp, q_timestamp2, num_success, num_failure);

  print!("<<{:^58}>>", "timestamp from DateTime<Utc>");

  let q_timestamp3=QGEN::new_timestamp(Utc.ymd(2020, 4, 1).and_hms_nano(3, 50, 12, 1234));
  assert_to_truefalse!(q_timestamp, q_timestamp3, num_success, num_failure);

  // Month //------------------------------------------/
  print!("<<{:^58}>>", "month from Date<Utc>");

  // Base
  let q_month=QGEN::new_month_ym(2019, 8);

  // Day should be supressed
  let q_month2=QGEN::new_month(Utc.ymd(2019, 8, 15));
  assert_to_truefalse!(q_month, q_month2, num_success, num_failure);

  // Date //------------------------------------------/
  print!("<<{:^58}>>", "date from Date<Utc>");

  // Base
  let q_date=QGEN::new_date_ymd(2005, 5, 8);

  let q_date2=QGEN::new_date(Utc.ymd(2005, 5, 8));
  assert_to_truefalse!(q_date, q_date2, num_success, num_failure);

  // Datetime //--------------------------------------/
  print!("<<{:^58}>>", "datetime from millisecond");

  // Base
  let q_datetime=QGEN::new_datetime_ymd_hms_millis(2008, 2, 1, 2, 31, 25, 828);

  let q_datetime2=QGEN::new_datetime_millis((KDB_DAY_OFFSET * ONE_DAY_MILLIS) + 255148285828);
  assert_to_truefalse!(q_datetime, q_datetime2, num_success, num_failure);

  print!("<<{:^58}>>", "datetime from DateTime<Utc>");

  let q_datetime3=QGEN::new_datetime(Utc.ymd(2008, 2, 1).and_hms_milli(2, 31, 25, 828));
  assert_to_truefalse!(q_datetime, q_datetime3, num_success, num_failure);

  // Timespan //--------------------------------------/
  print!("<<{:^58}>>", "timespan from nanosecond");

  // Base
  let q_timespan=QGEN::new_timespan_millis(172800000);

  let q_timespan2=QGEN::new_timespan_nanos(172800000000000);
  assert_to_truefalse!(q_timespan, q_timespan2, num_success, num_failure);

  print!("<<{:^58}>>", "timespan from Duration");

  let q_timespan3=QGEN::new_timespan(Duration::nanoseconds(172800000000000_i64));
  assert_to_truefalse!(q_timespan, q_timespan3, num_success, num_failure);

  // Minute //----------------------------------------/
  print!("<<{:^58}>>", "minute from min");

  // Base
  let q_minute=QGEN::new_minute_hm(13, 4);

  // 24:00 is supressed as 00:00
  let q_minute2=QGEN::new_minute_min(2224);
  assert_to_truefalse!(q_minute, q_minute2, num_success, num_failure);

  print!("<<{:^58}>>", "minute from NaiveTime");

  // Second is supressed
  let q_minute3=QGEN::new_minute_naive(NaiveTime::from_hms(13, 4, 30));
  assert_to_truefalse!(q_minute, q_minute3, num_success, num_failure);

  print!("<<{:^58}>>", "minute from QTime");

  // Second is supressed
  let q_minute4=QGEN::new_minute(QTimeGEN::new_minute(NaiveTime::from_hms(13, 4, 50)));
  assert_to_truefalse!(q_minute, q_minute4, num_success, num_failure);

  // Second //----------------------------------------/
  print!("<<{:^58}>>", "second from sec");

  // Base
  let q_second=QGEN::new_second_hms(8, 10, 2);

  // 48:00:00 is supressed to 00:00:00
  let q_second2=QGEN::new_second_sec(202202);
  assert_to_truefalse!(q_second, q_second2, num_success, num_failure);

  print!("<<{:^58}>>", "second from NaiveTime");

  // Millisecond is supressed
  let q_second3=QGEN::new_second_naive(NaiveTime::from_hms_milli(8, 10, 2, 325));
  assert_to_truefalse!(q_second, q_second3, num_success, num_failure);

  print!("<<{:^58}>>", "second from QTime");

  // Millisecond is supressed
  let q_second4=QGEN::new_second(QTimeGEN::new_second(NaiveTime::from_hms_milli(8, 10, 2, 325)));
  assert_to_truefalse!(q_second, q_second4, num_success, num_failure);

  // Time //------------------------------------------/
  print!("<<{:^58}>>", "time from millisecond");

  // Base
  let q_time=QGEN::new_time_hms_millis(20, 23, 25, 800);

  // 24:00:00.000 is supressed to 00:00:00
  let q_time2=QGEN::new_time_millis(159805800);
  assert_to_truefalse!(q_time, q_time2, num_success, num_failure);

  print!("<<{:^58}>>", "time from NaiveTime");

  // Precision under millisecond is supressed
  let q_time3=QGEN::new_time_naive(NaiveTime::from_hms_nano(20, 23, 25, 800123456));
  assert_to_truefalse!(q_time, q_time3, num_success, num_failure);

  print!("<<{:^58}>>", "time from QTime");

  // Precision under millisecond is supressed
  let q_time4=QGEN::new_time(QTimeGEN::new_time(NaiveTime::from_hms_nano(20, 23, 25, 800123456)));
  assert_to_truefalse!(q_time, q_time4, num_success, num_failure);

  Ok((num_success, num_failure))
}


/*
* Test various list constructors of each q type object if they provide the same value as each other.
* Note: Basic constructors have been tested in the tests above. The focus of this test is the other
* constructors provided for some types.
*/
fn list_constructor_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| List Constructor ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Timestamp //-------------------------------------/
  print!("<<{:^58}>>", "timestamp from nanosecond");

  // Base
  let q_timestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2020, 4, 1, 3, 50, 12, 000001234)]);

  let q_timestamp_list2=QGEN::new_timestamp_list_nanos(Attribute::None, vec![KDB_TIMESTAMP_OFFSET + 639028212000001234_i64]);
  assert_to_truefalse!(q_timestamp_list, q_timestamp_list2, num_success, num_failure);

  print!("<<{:^58}>>", "timestamp from DateTime<Utc>");

  let q_timestamp_list3=QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 4, 1).and_hms_nano(3, 50, 12, 1234)]);
  assert_to_truefalse!(q_timestamp_list, q_timestamp_list3, num_success, num_failure);

  // Month //------------------------------------------/
  print!("<<{:^58}>>", "month from Date<Utc>");

  // Base
  let q_month_list=QGEN::new_month_list_ym(Attribute::None, vec![(2019, 8)]);

  // Day should be supressed
  let q_month_list2=QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2019, 8, 15)]);
  assert_to_truefalse!(q_month_list, q_month_list2, num_success, num_failure);

  // Date //------------------------------------------/
  print!("<<{:^58}>>", "date from Date<Utc>");

  // Base
  let q_date_list=QGEN::new_date_list_ymd(Attribute::None, vec![(2005, 5, 8)]);

  let q_date_list2=QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2005, 5, 8)]);
  assert_to_truefalse!(q_date_list, q_date_list2, num_success, num_failure);

  // Datetime //--------------------------------------/
  print!("<<{:^58}>>", "datetime from millisecond");

  // Base
  let q_datetime_list=QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2008, 2, 1, 2, 31, 25, 828)]);

  let q_datetime_list2=QGEN::new_datetime_list_millis(Attribute::None, vec![(KDB_DAY_OFFSET * ONE_DAY_MILLIS) + 255148285828]);
  assert_to_truefalse!(q_datetime_list, q_datetime_list2, num_success, num_failure);

  print!("<<{:^58}>>", "datetime from DateTime<Utc>");

  let q_datetime_list3=QGEN::new_datetime_list(Attribute::None, vec![Utc.ymd(2008, 2, 1).and_hms_milli(2, 31, 25, 828)]);
  assert_to_truefalse!(q_datetime_list, q_datetime_list3, num_success, num_failure);

  // Timespan //--------------------------------------/
  print!("<<{:^58}>>", "timespan from millisecond");

  // Base
  let q_timespan_list=QGEN::new_timespan_list_millis(Attribute::None, vec![172800000]);

  let q_timespan_list2=QGEN::new_timespan_list_nanos(Attribute::None, vec![172800000000000]);
  assert_to_truefalse!(q_timespan_list, q_timespan_list2, num_success, num_failure);

  print!("<<{:^58}>>", "timespan from Duration");

  let q_timespan_list3=QGEN::new_timespan_list(Attribute::None, vec![Duration::nanoseconds(172800000000000_i64)]);
  assert_to_truefalse!(q_timespan_list, q_timespan_list3, num_success, num_failure);

  // Minute //----------------------------------------/
  print!("<<{:^58}>>", "minute from min");

  // Base
  let q_minute_list=QGEN::new_minute_list_hm(Attribute::None, vec![(13, 4)]);

  // 24:00 is supressed as 00:00
  let q_minute_list2=QGEN::new_minute_list_min(Attribute::None, vec![2224]);
  assert_to_truefalse!(q_minute_list, q_minute_list2, num_success, num_failure);

  print!("<<{:^58}>>", "minute from NaiveTime");

  // Second is supressed
  let q_minute_list3=QGEN::new_minute_list_naive(Attribute::None, vec![NaiveTime::from_hms(13, 4, 30)]);
  assert_to_truefalse!(q_minute_list, q_minute_list3, num_success, num_failure);

  print!("<<{:^58}>>", "minute from QTime");

  // Second is supressed
  let q_minute_list4=QGEN::new_minute_list(Attribute::None, vec![QTimeGEN::new_minute(NaiveTime::from_hms(13, 4, 50))]);
  assert_to_truefalse!(q_minute_list, q_minute_list4, num_success, num_failure);

  print!("<<{:^58}>>", "minute from null or infinity QTime");

  let q_minute_list5=QGEN::new_minute_list_min(Attribute::None, vec![Q_0Ni, Q_0Wi]);
  let q_minute_list6=QGEN::new_minute_list(Attribute::None, vec![Q_0Nu, Q_0Wu]);
  assert_to_truefalse!(q_minute_list5, q_minute_list6, num_success, num_failure);
  
  // Second //----------------------------------------/
  print!("<<{:^58}>>", "second from sec");

  // Base
  let q_second_list=QGEN::new_second_list_hms(Attribute::None, vec![(8, 10, 2)]);

  // 48:00:00 is supressed to 00:00:00
  let q_second_list2=QGEN::new_second_list_sec(Attribute::None, vec![202202]);
  assert_to_truefalse!(q_second_list, q_second_list2, num_success, num_failure);

  print!("<<{:^58}>>", "second from NaiveTime");

  // Millisecond is supressed
  let q_second_list3=QGEN::new_second_list_naive(Attribute::None, vec![NaiveTime::from_hms_milli(8, 10, 2, 325)]);
  assert_to_truefalse!(q_second_list, q_second_list3, num_success, num_failure);

  print!("<<{:^58}>>", "second from QTime");

  // Millisecond is supressed
  let q_second_list4=QGEN::new_second_list(Attribute::None, vec![QTimeGEN::new_second(NaiveTime::from_hms_milli(8, 10, 2, 325))]);
  assert_to_truefalse!(q_second_list, q_second_list4, num_success, num_failure);

  print!("<<{:^58}>>", "second from null or infinity QTime");

  let q_second_list5=QGEN::new_second_list_sec(Attribute::None, vec![Q_0Ni, Q_0Wi]);
  let q_second_list6=QGEN::new_second_list(Attribute::None, vec![Q_0Nv, Q_0Wv]);
  assert_to_truefalse!(q_second_list5, q_second_list6, num_success, num_failure);

  // Time //------------------------------------------/
  print!("<<{:^58}>>", "time from millisecond");

  // Base
  let q_time_list=QGEN::new_time_list_hms_millis(Attribute::None, vec![(20, 23, 25, 800)]);

  // 24:00:00.000 is supressed to 00:00:00
  let q_time_list2=QGEN::new_time_list_millis(Attribute::None, vec![159805800]);
  assert_to_truefalse!(q_time_list, q_time_list2, num_success, num_failure);

  print!("<<{:^58}>>", "time from NaiveTime");

  // Precision under millisecond is supressed
  let q_time_list3=QGEN::new_time_list_naive(Attribute::None, vec![NaiveTime::from_hms_nano(20, 23, 25, 800123456)]);
  assert_to_truefalse!(q_time_list, q_time_list3, num_success, num_failure);

  print!("<<{:^58}>>", "time from QTime");

  // Precision under millisecond is supressed
  let q_time_list4=QGEN::new_time_list(Attribute::None, vec![QTimeGEN::new_time(NaiveTime::from_hms_nano(20, 23, 25, 800123456))]);
  assert_to_truefalse!(q_time_list, q_time_list4, num_success, num_failure);

  print!("<<{:^58}>>", "time from null or infinity QTime");

  let q_time_list5=QGEN::new_time_list_millis(Attribute::None, vec![Q_0Ni, Q_0Wi]);
  let q_time_list6=QGEN::new_time_list(Attribute::None, vec![Q_0Nt, Q_0Wt]);
  assert_to_truefalse!(q_time_list5, q_time_list6, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test various atom macros of each q type object if they provide the same value as each other.
*/
fn atom_macro_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Atom Macro ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Bool //-------------------------------------------/
  print!("<<{:^58}>>", "bool");

  let qbool=QGEN::new_bool(false);
  let qbool2=q_bool![false];
  assert_to_truefalse!(qbool, qbool2, num_success, num_failure);

  // GUID //-------------------------------------------/
  print!("<<{:^58}>>", "GUID");

  let qGUID=QGEN::new_GUID([0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]);
  let qGUID2=q_GUID![[0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]];
  assert_to_truefalse!(qGUID, qGUID2, num_success, num_failure);

  // Byte //-------------------------------------------/
  print!("<<{:^58}>>", "byte");

  let qbyte=QGEN::new_byte(0xa7);
  let qbyte2=q_byte![0xa7];
  assert_to_truefalse!(qbyte, qbyte2, num_success, num_failure);

  // Short //------------------------------------------/
  print!("<<{:^58}>>", "short");

  let qshort=QGEN::new_short(-1024_i16);
  let qshort2=q_short![-1024_i16];
  assert_to_truefalse!(qshort, qshort2, num_success, num_failure);

  // Int //--------------------------------------------/
  print!("<<{:^58}>>", "int");

  let qint=QGEN::new_int(392452);
  let qint2=q_int![392452];
  assert_to_truefalse!(qint, qint2, num_success, num_failure);

  // Long //-------------------------------------------/
  print!("<<{:^58}>>", "long");

  let qlong=QGEN::new_long(-3622501337297584128_i64);
  let qlong2=q_long![-3622501337297584128_i64];
  assert_to_truefalse!(qlong, qlong2, num_success, num_failure);

  // Real //-------------------------------------------/
  print!("<<{:^58}>>", "real");

  let qreal=QGEN::new_real(1.23_f32);
  let qreal2=q_real![1.23_f32];
  assert_to_truefalse!(qreal, qreal2, num_success, num_failure);

  // Float //------------------------------------------/
  print!("<<{:^58}>>", "float");

  let qfloat=q_float![103.2342];
  assert_to_truefalse_float!(qfloat.into_f64().expect("Failed to convert into f64"), 103.2342, 0.0001, num_success, num_failure);

  // Char //-------------------------------------------/
  print!("<<{:^58}>>", "char");

  let qchar=QGEN::new_char('+');
  let qchar2=q_char!['+'];
  assert_to_truefalse!(qchar, qchar2, num_success, num_failure);

  // Symbol //-----------------------------------------/
  print!("<<{:^58}>>", "symbol");

  let qsymbol=QGEN::new_symbol("🐉");
  let qsymbol2=q_symbol!["🐉"];
  assert_to_truefalse!(qsymbol, qsymbol2, num_success, num_failure);

  // Timestamp //--------------------------------------/
  print!("<<{:^58}>>", "timestamp from DateTime<Utc>");

  // Base
  let qtimestamp=QGEN::new_timestamp(Utc.ymd(2020, 4, 1).and_hms_nano(3, 50, 12, 1234));
  let qtimestamp2=q_timestamp!["datetime"; Utc.ymd(2020, 4, 1).and_hms_nano(3, 50, 12, 1234)];
  assert_to_truefalse!(qtimestamp, qtimestamp2, num_success, num_failure);

  print!("<<{:^58}>>", "timestamp from nanosecond");

  let qtimestamp3=q_timestamp!["nanos"; KDB_TIMESTAMP_OFFSET + 639028212000001234_i64];
  assert_to_truefalse!(qtimestamp, qtimestamp3, num_success, num_failure);

  print!("<<{:^58}>>", "timestamp from ymd_hms_nanos");

  let qtimestamp4=q_timestamp!["ymd_hms_nanos"; 2020, 4, 1, 3, 50, 12, 1234];
  assert_to_truefalse!(qtimestamp, qtimestamp4, num_success, num_failure);

  // Month //------------------------------------------/
  print!("<<{:^58}>>", "month from Date<Utc>");

  // Base
  let qmonth=QGEN::new_month_ym(2019, 8);

  // Day should be supressed
  let qmonth2=q_month![Utc.ymd(2019, 8, 15)];
  assert_to_truefalse!(qmonth, qmonth2, num_success, num_failure);

  print!("<<{:^58}>>", "month from ym");

  let qmonth3=q_month![2019, 8];
  assert_to_truefalse!(qmonth, qmonth3, num_success, num_failure);

  // Date //------------------------------------------/
  print!("<<{:^58}>>", "date from Date<Utc>");

  // Base
  let q_date=QGEN::new_date_ymd(2005, 5, 8);

  let q_date2=QGEN::new_date(Utc.ymd(2005, 5, 8));
  assert_to_truefalse!(q_date, q_date2, num_success, num_failure);

  // Datetime //--------------------------------------/
  print!("<<{:^58}>>", "datetime from millisecond");

  // Base
  let qdatetime=QGEN::new_datetime_ymd_hms_millis(2008, 2, 1, 2, 31, 25, 828);

  let qdatetime2=q_datetime!["millis"; (KDB_DAY_OFFSET * ONE_DAY_MILLIS) + 255148285828_i64];
  assert_to_truefalse!(qdatetime, qdatetime2, num_success, num_failure);

  print!("<<{:^58}>>", "datetime from DateTime<Utc>");

  let qdatetime3=q_datetime!["datetime"; Utc.ymd(2008, 2, 1).and_hms_milli(2, 31, 25, 828)];
  assert_to_truefalse!(qdatetime, qdatetime3, num_success, num_failure);

  print!("<<{:^58}>>", "datetime from ymd_hms_millis");

  let qdatetime4=q_datetime!["ymd_hms_millis"; 2008, 2, 1, 2, 31, 25, 828];
  assert_to_truefalse!(qdatetime, qdatetime4, num_success, num_failure);

  // Timespan //--------------------------------------/
  print!("<<{:^58}>>", "timespan from milliecond");

  // Base
  let qtimespan=QGEN::new_timespan_millis(172800000_i64);

  let qtimespan2=q_timespan!["millis"; 172800000_i64];
  assert_to_truefalse!(qtimespan, qtimespan2, num_success, num_failure);

  print!("<<{:^58}>>", "timespan from nanosecond");

  let qtimespan3=q_timespan!["nanos"; 172800000000000_i64];
  assert_to_truefalse!(qtimespan, qtimespan3, num_success, num_failure);

  print!("<<{:^58}>>", "timespan from Duration");

  let qtimespan4=q_timespan!["duration"; Duration::nanoseconds(172800000000000_i64)];
  assert_to_truefalse!(qtimespan, qtimespan4, num_success, num_failure);

  // Minute //----------------------------------------/
  print!("<<{:^58}>>", "minute from hm");

  // Base
  let qminute=QGEN::new_minute_hm(13, 4);

  // 24:00 is supressed as 00:00
  let qminute2=q_minute!["hm"; 13, 4];
  assert_to_truefalse!(qminute, qminute2, num_success, num_failure);

  print!("<<{:^58}>>", "minute from min");

  // 24:00 is supressed as 00:00
  let qminute3=q_minute!["min"; 2224];
  assert_to_truefalse!(qminute, qminute3, num_success, num_failure);

  print!("<<{:^58}>>", "minute from NaiveTime");

  // Second is supressed
  let qminute4=q_minute!["naivetime"; NaiveTime::from_hms(13, 4, 30)];
  assert_to_truefalse!(qminute, qminute4, num_success, num_failure);

  print!("<<{:^58}>>", "minute from QTime");

  // Second is supressed
  let qminute5=q_minute!["qtime"; QTimeGEN::new_minute(NaiveTime::from_hms(13, 4, 50))];
  assert_to_truefalse!(qminute, qminute5, num_success, num_failure);

  // Second //----------------------------------------/
  print!("<<{:^58}>>", "second from hms");

  // Base
  let qsecond=QGEN::new_second_hms(8, 10, 2);

  // 48:00:00 is supressed to 00:00:00
  let qsecond2=q_second!["hms"; 8, 10, 2];
  assert_to_truefalse!(qsecond, qsecond2, num_success, num_failure);

  print!("<<{:^58}>>", "second from sec");

  // 48:00:00 is supressed to 00:00:00
  let qsecond3=q_second!["sec"; 202202];
  assert_to_truefalse!(qsecond, qsecond3, num_success, num_failure);

  print!("<<{:^58}>>", "second from NaiveTime");

  // Millisecond is supressed
  let qsecond4=q_second!["naivetime"; NaiveTime::from_hms_milli(8, 10, 2, 325)];
  assert_to_truefalse!(qsecond, qsecond4, num_success, num_failure);

  print!("<<{:^58}>>", "second from QTime");

  // Millisecond is supressed
  let qsecond5=q_second!["qtime"; QTimeGEN::new_second(NaiveTime::from_hms_milli(8, 10, 2, 325))];
  assert_to_truefalse!(qsecond, qsecond5, num_success, num_failure);

  // Time //------------------------------------------/
  print!("<<{:^58}>>", "time from hms_millis");

  // Base
  let qtime=QGEN::new_time_hms_millis(20, 23, 25, 800);

  // 24:00:00.000 is supressed to 00:00:00
  let qtime2=q_time!["hms_millis"; 20, 23, 25, 800];
  assert_to_truefalse!(qtime, qtime2, num_success, num_failure);

  print!("<<{:^58}>>", "time from millisecond");

  // 24:00:00.000 is supressed to 00:00:00
  let qtime3=q_time!["millis"; 159805800];
  assert_to_truefalse!(qtime, qtime3, num_success, num_failure);

  print!("<<{:^58}>>", "time from NaiveTime");

  // Precision under millisecond is supressed
  let qtime4=q_time!["naivetime"; NaiveTime::from_hms_nano(20, 23, 25, 800123456)];
  assert_to_truefalse!(qtime, qtime4, num_success, num_failure);

  print!("<<{:^58}>>", "time from QTime");

  // Precision under millisecond is supressed
  let qtime5=q_time!["qtime"; QTimeGEN::new_time(NaiveTime::from_hms_nano(20, 23, 25, 800123456))];
  assert_to_truefalse!(qtime, qtime5, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test various list macros of each q type object if they provide the same value as each other.
*/
fn list_macro_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| List Constructor ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Bool //------------------------------------------/
  print!("<<{:^58}>>", "bool");

  let qbool_list=QGEN::new_bool_list(Attribute::Parted, vec![true, true, false]);
  let qbool_list2=q_bool_list!['p'; vec![true, true, false]];
  assert_to_truefalse!(qbool_list, qbool_list2, num_success, num_failure);

  // GUID //------------------------------------------/
  print!("<<{:^58}>>", "GUID");

  let qGUID_list=QGEN::new_GUID_list(Attribute::None, vec![[0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68], [0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]]);
  let qGUID_list2=q_GUID_list!['*'; vec![[0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68], [0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]]];
  assert_to_truefalse!(qGUID_list, qGUID_list2, num_success, num_failure);

  // Byte //------------------------------------------/
  print!("<<{:^58}>>", "byte");

  let qbyte_list=QGEN::new_byte_list(Attribute::None, vec![0x4b, 0x78, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x73]); 
  let qbyte_list2=q_byte_list!['*'; vec![0x4b, 0x78, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x73]];
  assert_to_truefalse!(qbyte_list, qbyte_list2, num_success, num_failure);

  // Short //-----------------------------------------/
  print!("<<{:^58}>>", "short");

  let qshort_list=QGEN::new_short_list(Attribute::None, vec![10_i16, -30, 20]);
  let qshort_list2=q_short_list!['*'; vec![10_i16, -30, 20]];
  assert_to_truefalse!(qshort_list, qshort_list2, num_success, num_failure);

  // Int //-------------------------------------------/
  print!("<<{:^58}>>", "int");

  let qint_list=QGEN::new_int_list(Attribute::Sorted, vec![-3429000, 120000]);
  let qint_list2=q_int_list!['s'; vec![-3429000, 120000]];
  assert_to_truefalse!(qint_list, qint_list2, num_success, num_failure);

  // Long //------------------------------------------/
  print!("<<{:^58}>>", "long");

  let qlong_list=QGEN::new_long_list(Attribute::None, vec![42_i64]);
  let qlong_list2=q_long_list!['*'; vec![42_i64]];
  assert_to_truefalse!(qlong_list, qlong_list2, num_success, num_failure);

  // Real //------------------------------------------/
  print!("<<{:^58}>>", "real");

  let qreal_list=QGEN::new_real_list(Attribute::None, vec![0.940909_f32, Q_0We, 2039.30499]);
  let qreal_list2=q_real_list!['*'; vec![0.940909_f32, Q_0We, 2039.30499]];
  assert_to_truefalse!(qreal_list, qreal_list2, num_success, num_failure);

  // Float //-----------------------------------------/
  print!("<<{:^58}>>", "float");

  let qfloat_list=q_float_list!['*'; vec![-0.9, Q_NEG_0w, 1.0, -1.1, 1.2, Q_0n]];
  let (_, rust_float_list)=qfloat_list.into_f64_vec()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(rust_float_list[0], -0.9);
    assert!(rust_float_list[1].is_infinite());
    assert_eq!(rust_float_list[2..5], [1.0, -1.1, 1.2]);
    assert!(rust_float_list[5].is_nan());
  }, num_success, num_failure);

  // Char //------------------------------------------/
  print!("<<{:^58}>>", "char");

  let qchar_list=QGEN::new_char_list(Attribute::Unique, "Joshua");
  let qchar_list2=q_string!['u'; String::from("Joshua")];
  assert_to_truefalse!(qchar_list, qchar_list2, num_success, num_failure);

  // Symbol //----------------------------------------/
  print!("<<{:^58}>>", "symbol");

  let qsymbol_list=QGEN::new_symbol_list(Attribute::Unique, vec!["Last", "Derivatives"]);
  let qsymbol_list2=q_symbol_list!['u'; vec![String::from("Last"), String::from("Derivatives")]];
  assert_to_truefalse!(qsymbol_list, qsymbol_list2, num_success, num_failure);

  // Timestamp //-------------------------------------/
  print!("<<{:^58}>>", "timestamp from ymd_hms_nanos");

  // Base
  let qtimestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2020, 4, 1, 3, 50, 12, 000001234)]);
  let qtimestamp_list2=q_timestamp_list!["ymd_hms_nanos"; '*'; vec![(2020, 4, 1, 3, 50, 12, 000001234)]];
  assert_to_truefalse!(qtimestamp_list, qtimestamp_list2, num_success, num_failure);

  print!("<<{:^58}>>", "timestamp from nanosecond");

  let qtimestamp_list3=q_timestamp_list!["nanos"; '*'; vec![KDB_TIMESTAMP_OFFSET + 639028212000001234_i64]];
  assert_to_truefalse!(qtimestamp_list, qtimestamp_list3, num_success, num_failure);

  print!("<<{:^58}>>", "timestamp from DateTime<Utc>");

  let qtimestamp_list4=q_timestamp_list!["datetime"; '*'; vec![Utc.ymd(2020, 4, 1).and_hms_nano(3, 50, 12, 1234)]];
  assert_to_truefalse!(qtimestamp_list, qtimestamp_list4, num_success, num_failure);

  // Month //------------------------------------------/
  print!("<<{:^58}>>", "month from ym");

  // Base
  let qmonth_list=QGEN::new_month_list_ym(Attribute::None, vec![(2019, 8), (2019, 9)]);

  // Day should be supressed
  let qmonth_list2=q_month_list!["ym"; '*'; vec![(2019, 8), (2019, 9)]];
  assert_to_truefalse!(qmonth_list, qmonth_list2, num_success, num_failure);

  print!("<<{:^58}>>", "month from Date<Utc>");

  // Day should be supressed
  let qmonth_list3=q_month_list!["date"; '*'; vec![Utc.ymd(2019, 8, 15), Utc.ymd(2019, 9, 21)]];
  assert_to_truefalse!(qmonth_list, qmonth_list3, num_success, num_failure);

  // Date //------------------------------------------/
  print!("<<{:^58}>>", "date from ymd");

  // Base
  let qdate_list=QGEN::new_date_list_ymd(Attribute::None, vec![(2005, 5, 8)]);

  let qdate_list2=q_date_list!["ymd"; '*'; vec![(2005, 5, 8)]];
  assert_to_truefalse!(qdate_list, qdate_list2, num_success, num_failure);

  print!("<<{:^58}>>", "date from Date<Utc>");

  let qdate_list3=q_date_list!["date"; '*'; vec![Utc.ymd(2005, 5, 8)]];
  assert_to_truefalse!(qdate_list, qdate_list3, num_success, num_failure);

  // Datetime //--------------------------------------/
  print!("<<{:^58}>>", "datetime from ymd_hms_millis");

  // Base
  let qdatetime_list=QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2005, 7, 23, 2, 23, 37, 172), (2008, 2, 1, 2, 31, 25, 828)]);

  let qdatetime_list2=q_datetime_list!["ymd_hms_millis"; '*'; vec![(2005, 7, 23, 2, 23, 37, 172), (2008, 2, 1, 2, 31, 25, 828)]];
  assert_to_truefalse!(qdatetime_list, qdatetime_list2, num_success, num_failure);

  print!("<<{:^58}>>", "datetime from millisecond");

  let qdatetime_list3=q_datetime_list!["millis"; '*'; vec![(KDB_DAY_OFFSET * ONE_DAY_MILLIS) + 175400617172_i64, (KDB_DAY_OFFSET * ONE_DAY_MILLIS) + 255148285828_i64]];
  assert_to_truefalse!(qdatetime_list, qdatetime_list3, num_success, num_failure);

  print!("<<{:^58}>>", "datetime from DateTime<Utc>");

  let qdatetime_list4=q_datetime_list!["datetime"; '*'; vec![Utc.ymd(2005, 7, 23).and_hms_milli(2, 23, 37, 172), Utc.ymd(2008, 2, 1).and_hms_milli(2, 31, 25, 828)]];
  assert_to_truefalse!(qdatetime_list, qdatetime_list4, num_success, num_failure);

  // Timespan //--------------------------------------/
  print!("<<{:^58}>>", "timespan from millisecond");

  // Base
  let qtimespan_list=QGEN::new_timespan_list_millis(Attribute::None, vec![172800000_i64, 215608000]);
  let qtimespan_list2=q_timespan_list!["millis"; '*'; vec![172800000_i64, 215608000]];
  assert_to_truefalse!(qtimespan_list, qtimespan_list2, num_success, num_failure);

  print!("<<{:^58}>>", "timespan from nanosecond");

  let qtimespan_list3=q_timespan_list!["nanos"; '*'; vec![172800000000000_i64, 215608000000000]];
  assert_to_truefalse!(qtimespan_list, qtimespan_list3, num_success, num_failure);

  print!("<<{:^58}>>", "timespan from Duration");

  let qtimespan_list4=q_timespan_list!["duration"; '*'; vec![Duration::nanoseconds(172800000000000_i64), Duration::nanoseconds(215608000000000)]];
  assert_to_truefalse!(qtimespan_list, qtimespan_list4, num_success, num_failure);

  // Minute //----------------------------------------/
  print!("<<{:^58}>>", "minute from hm");

  // Base
  let qminute_list=QGEN::new_minute_list_hm(Attribute::None, vec![(13, 4)]);
  let qminute_list2=q_minute_list!["hm"; '*'; vec![(13, 4)]];
  assert_to_truefalse!(qminute_list, qminute_list2, num_success, num_failure);

  print!("<<{:^58}>>", "minute from min");

  // 24:00 is supressed as 00:00
  let qminute_list3=q_minute_list!["min"; '*'; vec![2224]];
  assert_to_truefalse!(qminute_list, qminute_list3, num_success, num_failure);

  print!("<<{:^58}>>", "minute from NaiveTime");

  // Second is supressed
  let qminute_list4=q_minute_list!["naivetime"; '*'; vec![NaiveTime::from_hms(13, 4, 30)]];
  assert_to_truefalse!(qminute_list, qminute_list4, num_success, num_failure);

  print!("<<{:^58}>>", "minute from QTime");

  // Second is supressed
  let qminute_list5=q_minute_list!["qtime"; '*'; vec![QTimeGEN::new_minute(NaiveTime::from_hms(13, 4, 50))]];
  assert_to_truefalse!(qminute_list, qminute_list5, num_success, num_failure);

  print!("<<{:^58}>>", "minute from null or infinity QTime");

  let qminute_list6=QGEN::new_minute_list(Attribute::None, vec![Q_0Nu, Q_0Wu]);
  let qminute_list7=q_minute_list!["min"; '*'; vec![Q_0Ni, Q_0Wi]];
  assert_to_truefalse!(qminute_list6, qminute_list7, num_success, num_failure);
  
  // Second //----------------------------------------/
  print!("<<{:^58}>>", "second from hms");

  // Base
  let qsecond_list=QGEN::new_second_list_hms(Attribute::None, vec![(8, 10, 2), (13, 5, 39)]);
  // 48:00:00 is supressed to 00:00:00
  let qsecond_list2=q_second_list!["hms"; '*'; vec![(8, 10, 2), (13, 5, 39)]];
  assert_to_truefalse!(qsecond_list, qsecond_list2, num_success, num_failure);

  print!("<<{:^58}>>", "second from sec");

  // 48:00:00 is supressed to 00:00:00
  let qsecond_list3=q_second_list!["sec"; '*'; vec![202202, 47139]];
  assert_to_truefalse!(qsecond_list, qsecond_list3, num_success, num_failure);

  print!("<<{:^58}>>", "second from NaiveTime");

  // Millisecond is supressed
  let qsecond_list4=q_second_list!["naivetime"; '*'; vec![NaiveTime::from_hms_milli(8, 10, 2, 325), NaiveTime::from_hms_milli(13, 5, 39, 127)]];
  assert_to_truefalse!(qsecond_list, qsecond_list4, num_success, num_failure);

  print!("<<{:^58}>>", "second from QTime");

  // Millisecond is supressed
  let qsecond_list5=q_second_list!["qtime"; '*'; vec![QTimeGEN::new_second(NaiveTime::from_hms_milli(8, 10, 2, 325)), QTimeGEN::new_second(NaiveTime::from_hms_milli(13, 5, 39, 127))]];
  assert_to_truefalse!(qsecond_list, qsecond_list5, num_success, num_failure);

  print!("<<{:^58}>>", "second from null or infinity QTime");

  let q_second_list6=QGEN::new_second_list(Attribute::None, vec![Q_0Nv, Q_0Wv]);
  let q_second_list7=q_second_list!["sec"; '*'; vec![Q_0Ni, Q_0Wi]]; 
  assert_to_truefalse!(q_second_list6, q_second_list7, num_success, num_failure);

  // Time //------------------------------------------/
  print!("<<{:^58}>>", "time from hms_millis");

  // Base
  let qtime_list=QGEN::new_time_list_hms_millis(Attribute::None, vec![(20, 23, 25, 800), (1, 31, 20, 527)]);
  // 24:00:00.000 is supressed to 00:00:00
  let qtime_list2=q_time_list!["hms_millis"; '*'; vec![(20, 23, 25, 800), (1, 31, 20, 527)]];
  assert_to_truefalse!(qtime_list, qtime_list2, num_success, num_failure);

  print!("<<{:^58}>>", "time from millisecond");

  // 24:00:00.000 is supressed to 00:00:00
  let qtime_list3=q_time_list!["millis"; '*'; vec![159805800, 178280527]];
  assert_to_truefalse!(qtime_list, qtime_list3, num_success, num_failure);

  print!("<<{:^58}>>", "time from NaiveTime");

  // Precision under millisecond is supressed
  let qtime_list4=q_time_list!["naivetime"; '*'; vec![NaiveTime::from_hms_nano(20, 23, 25, 800123456), NaiveTime::from_hms_nano(1, 31, 20, 527123456)]];
  assert_to_truefalse!(qtime_list, qtime_list4, num_success, num_failure);

  print!("<<{:^58}>>", "time from QTime");

  // Precision under millisecond is supressed
  let qtime_list5=q_time_list!["qtime"; '*'; vec![QTimeGEN::new_time(NaiveTime::from_hms_nano(20, 23, 25, 800123456)), QTimeGEN::new_time(NaiveTime::from_hms_nano(1, 31, 20, 527123456))]];
  assert_to_truefalse!(qtime_list, qtime_list5, num_success, num_failure);

  print!("<<{:^58}>>", "time from null or infinity QTime");

  let qtime_list6=QGEN::new_time_list(Attribute::None, vec![Q_0Nt, Q_0Wt]);
  let qtime_list7=q_time_list!["millis"; '*'; vec![Q_0Ni, Q_0Wi]];
  assert_to_truefalse!(qtime_list6, qtime_list7, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test table macro
*/
fn table_macro_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Table Macro ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Table //-----------------------------------------/
  print!("<<{:^58}>>", "table");

  let qtable=QGEN::new_table(
    vec!["time", "sym", "price", "size", "ex"],
    vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]
  ).expect("Failed to build table");

  let qtable2=q_table![
    vec!["time", "sym", "price", "size", "ex"];
    vec![
      q_timestamp_list!["datetime"; '*'; vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]],
      q_symbol_list!['g'; vec!["Kx", "FD", "Kx"]],
      q_float_list!['*'; vec![103.68_f64, 107.42, 103.3]],
      q_long_list!['*'; vec![1000_i64, 2000, 3000]],
      q_string!['*'; "NLN"]
    ]
  ].expect("Failed to build table");

  assert_to_truefalse!(qtable, qtable2, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test dictionary macro.
* Note: As macros for atoms and lists have been tested above, only the structural
*  conversion from vectors to dictionary is the focus.
*/
fn dictionary_macro_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Dictionary Macro ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Dictionary //------------------------------------/
  print!("<<{:^58}>>", "dictionary");

  let qdict=QGEN::new_dictionary(QGEN::new_int_list(Attribute::Sorted, vec![100, 200, 300]), QGEN::new_mixed_list(vec![QGEN::new_char_list(Attribute::None, "super.firstderivatives.com"), QGEN::new_float(4.0), QGEN::new_month_list_ym(Attribute::None, vec![(2010, 3), (2011, 3)])]));
  let qdict2=q_dictionary![
    q_int_list!['s'; vec![100, 200, 300]];
    q_mixed_list![q_string!['*'; "super.firstderivatives.com"], q_float![4.0], q_month_list!["ym"; '*'; vec![(2010, 3), (2011, 3)]]]
  ];
  assert_to_truefalse!(qdict, qdict2, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test keyed table macro.
*/
fn keyed_table_macro_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Keyed Table Macro ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Keyed Table //-----------------------------------/
  print!("<<{:^58}>>", "keyed table");

  let qkeyed_table=QGEN::new_keyed_table(
    vec!["city"],
    vec![
      QGEN::new_symbol_list(Attribute::None, vec!["Tokyo", "London", "NewYork"])
    ],
    vec!["area_skm", "population"],
    vec![
      QGEN::new_int_list(Attribute::None, vec![13500, 17300, 1740]),
      QGEN::new_long_list(Attribute::None, vec![37400000_i64, 9046000, 18819000])
    ]
  ).unwrap();

  let qkeyed_table2=q_keyed_table![
    vec!["city"];
    vec![
      q_symbol_list!['*'; vec!["Tokyo", "London", "NewYork"]]
    ];
    vec!["area_skm", "population"];
    vec![
      q_int_list!['*'; vec![13500, 17300, 1740]],
      q_long_list!['*'; vec![37400000_i64, 9046000, 18819000]]
    ]
  ]?;

  assert_to_truefalse!(qkeyed_table, qkeyed_table2, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test conversion from atom q object into Rust object.
*/
fn atom_conversion_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Atom Conversion ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Bool //------------------------------------------/
  print!("<<{:^58}>>", "bool from Bool");

  let q_bool=QGEN::new_bool(true);
  let rust_bool=q_bool.into_bool()?;
  assert_to_truefalse!(rust_bool, true, num_success, num_failure);

  // GUID //------------------------------------------/
  print!("<<{:^58}>>", "[u8; 16] from GUID");

  let q_GUID=QGEN::new_GUID([0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]);
  let rust_GUID=q_GUID.into_GUID()?;
  assert_to_truefalse!(rust_GUID, [0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24], num_success, num_failure);

  // Byte //------------------------------------------/
  print!("<<{:^58}>>", "u8 from Byte");

  let q_byte=QGEN::new_byte(0x3c);
  let rust_u8=q_byte.into_u8()?;
  assert_to_truefalse!(rust_u8, 0x3c, num_success, num_failure);

  // Short //-----------------------------------------/
  print!("<<{:^58}>>", "i16 from Short");

  let q_short=QGEN::new_short(17);
  let rust_i16=q_short.into_i16()?;
  assert_to_truefalse!(rust_i16, 17_i16, num_success, num_failure);

  // Int //-------------------------------------------/
  print!("<<{:^58}>>", "i32 from Int");

  let q_int=QGEN::new_int(-34567789);
  let rust_i32=q_int.into_i32()?;
  assert_to_truefalse!(rust_i32, -34567789_i32, num_success, num_failure);

  // Long //------------------------------------------/
  print!("<<{:^58}>>", "i64 from Long");

  let q_long=QGEN::new_long(86400000000000_i64);
  let rust_i64=q_long.into_i64()?;
  assert_to_truefalse!(rust_i64, 86400000000000_i64, num_success, num_failure);

  // Real //------------------------------------------/
  print!("<<{:^58}>>", "f32 from Real");

  let q_real=QGEN::new_real(10.25);
  let rust_f32=q_real.into_f32()?;
  assert_to_truefalse_real!(rust_f32, 10.25, 0.001, num_success, num_failure);

  // Float //-----------------------------------------/
  print!("<<{:^58}>>", "f64 from Float");

  let q_float=QGEN::new_float(103.678_f64);
  let rust_f64=q_float.into_f64()?;
  assert_to_truefalse_float!(rust_f64, 103.678_f64, 0.0001, num_success, num_failure);

  // Char //------------------------------------------/
  print!("<<{:^58}>>", "char from Char");

  let q_char=QGEN::new_char('q');
  let rust_char=q_char.into_char()?;
  assert_to_truefalse!(rust_char, 'q', num_success, num_failure);

  // Symbol //----------------------------------------/
  print!("<<{:^58}>>", "String from Symbol");

  let q_symbol=QGEN::new_symbol("kdb+");
  let rust_string=q_symbol.into_string()?;
  assert_to_truefalse!(rust_string, "kdb+".to_string(), num_success, num_failure);

  // Timestamp //-------------------------------------/
  print!("<<{:^58}>>", "DateTime<Utc> from Timestamp");

  let q_timestamp=QGEN::new_timestamp_ymd_hms_nanos(2018, 2, 18, 4, 0, 0, 100);
  let rust_datetime=q_timestamp.into_datetime()?;
  assert_to_truefalse!(rust_datetime, Utc.ymd(2018, 2, 18).and_hms_nano(4, 0, 0, 100), num_success, num_failure);

  print!("<<{:^58}>>", "i64 from Timestamp");

  let q_timestamp=QGEN::new_timestamp_ymd_hms_nanos(2018, 2, 18, 4, 0, 0, 100);
  let rust_i64=q_timestamp.into_i64()?;
  assert_to_truefalse!(rust_i64, 572241600000000100_i64 + KDB_TIMESTAMP_OFFSET, num_success, num_failure);

  print!("<<{:^58}>>", "0N from Timestamp");

  let q_timestamp=QGEN::new_timestamp(Q_0Np);
  let rust_i64=q_timestamp.into_i64()?;
  assert_to_truefalse!(rust_i64, Q_0Nj, num_success, num_failure);

  print!("<<{:^58}>>", "0W from Timestamp");

  let q_timestamp=QGEN::new_timestamp(Q_0Wp);
  let rust_i64=q_timestamp.into_i64()?;
  assert_to_truefalse!(rust_i64, Q_0Wj, num_success, num_failure);

  // Month //-----------------------------------------/
  print!("<<{:^58}>>", "Date<Utc> from Month");

  let q_month=QGEN::new_month_ym(2013, 9);
  let rust_date=q_month.into_date()?;
  assert_to_truefalse!(rust_date, Utc.ymd(2013, 9, 1), num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Month");

  let q_month= QGEN::new_month_ym(2013, 9);
  let rust_i32=q_month.into_i32()?;
  assert_to_truefalse!(rust_i32, 164 + KDB_MONTH_OFFSET, num_success, num_failure);

  print!("<<{:^58}>>", "0Ni from Month");

  let q_month= QGEN::new_month(Q_0Nm);
  let rust_i32=q_month.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Ni, num_success, num_failure);

  print!("<<{:^58}>>", "0Wi from Month");

  let q_month= QGEN::new_month(Q_0Wm);
  let rust_i32=q_month.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Wi, num_success, num_failure);

  // Date //------------------------------------------/
  print!("<<{:^58}>>", "Date<Utc> from Date");

  let q_date=QGEN::new_date_ymd(2000, 2, 9);
  let rust_date=q_date.into_date()?;
  assert_to_truefalse!(rust_date, Utc.ymd(2000, 2, 9), num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Date");

  let q_date= QGEN::new_date_ymd(2000, 2, 9);
  let rust_i32=q_date.into_i32()?;
  assert_to_truefalse!(rust_i32, 39 + KDB_DAY_OFFSET as i32, num_success, num_failure);

  print!("<<{:^58}>>", "0Ni from Date");

  let q_date= QGEN::new_date(Q_0Nd);
  let rust_i32=q_date.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Ni, num_success, num_failure);

  print!("<<{:^58}>>", "0Wi from Date");

  let q_date= QGEN::new_date(Q_0Wd);
  let rust_i32=q_date.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Wi, num_success, num_failure);

  // Datetime //-------------------------------------/
  print!("<<{:^58}>>", "DateTime<Utc> from Datetime");

  let q_datetime=QGEN::new_datetime_ymd_hms_millis(2004, 6, 17, 11, 32, 40, 803);
  let rust_datetime=q_datetime.into_datetime()?;
  assert_to_truefalse!(rust_datetime, Utc.ymd(2004, 6, 17).and_hms_milli(11, 32, 40, 803), num_success, num_failure);

  print!("<<{:^58}>>", "f64 from Datetime");

  let q_datetime=QGEN::new_datetime_ymd_hms_millis(2004, 6, 17, 11, 32, 40, 803);
  let rust_f64=q_datetime.into_f64()?;
  assert_to_truefalse_float!(rust_f64, 1629.481 + KDB_DAY_OFFSET as f64, 0.001, num_success, num_failure);

  print!("<<{:^58}>>", "0n from Datetime");

  let q_datetime=QGEN::new_datetime(Q_0Nz);
  let rust_f64=q_datetime.into_f64()?;
  assert_to_truefalse_custom!(||{assert!(rust_f64.is_nan())}, num_success, num_failure);

  print!("<<{:^58}>>", "0w from Datetime");

  let q_datetime=QGEN::new_datetime(*Q_0Wz);
  let rust_f64=q_datetime.into_f64()?;
  assert_to_truefalse_custom!(||{assert!(rust_f64.is_infinite())}, num_success, num_failure);

  // Timespan //------------------------------------/
  print!("<<{:^58}>>", "Duration from Timespan");
  
  let q_timespan=QGEN::new_timespan_millis(999);
  let rust_duration=q_timespan.into_duration()?;
  assert_to_truefalse!(rust_duration, Duration::nanoseconds(999000000_i64), num_success, num_failure);

  print!("<<{:^58}>>", "i64 from Timespan");
  
  let q_timespan=QGEN::new_timespan_millis(999);
  let rust_i64=q_timespan.into_i64()?;
  assert_to_truefalse!(rust_i64, 999000000_i64, num_success, num_failure);

  print!("<<{:^58}>>", "0N from Timespan");
  
  let q_timespan=QGEN::new_timespan(*Q_0Nn);
  let rust_i64=q_timespan.into_i64()?;
  assert_to_truefalse!(rust_i64, Q_0Nj, num_success, num_failure);

  print!("<<{:^58}>>", "0W from Timespan");
  
  let q_timespan=QGEN::new_timespan(*Q_0Wn);
  let rust_i64=q_timespan.into_i64()?;
  assert_to_truefalse!(rust_i64, Q_0Wj, num_success, num_failure);

  // Minute //-------------------------------------/
  print!("<<{:^58}>>", "NaiveTime from Minute");

  let q_minute=QGEN::new_minute_min(1231);
  let rust_naivetime=q_minute.into_naivetime()?;
  assert_to_truefalse!(rust_naivetime, NaiveTime::from_hms(20, 31, 0), num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Minute");

  let q_minute=QGEN::new_minute_min(1231);
  let rust_i32=q_minute.into_i32()?;
  assert_to_truefalse!(rust_i32, 1231, num_success, num_failure);

  print!("<<{:^58}>>", "0Ni from Minute");

  let q_minute=QGEN::new_minute(Q_0Nu);
  let rust_i32=q_minute.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Ni, num_success, num_failure);

  print!("<<{:^58}>>", "0Wi from Minute");

  let q_minute=QGEN::new_minute(Q_0Wu);
  let rust_i32=q_minute.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Wi, num_success, num_failure);

  // Second //-------------------------------------/
  print!("<<{:^58}>>", "NaiveTime from Second");

  let q_second=QGEN::new_second_hms(3, 17, 26);
  let rust_naivetime=q_second.into_naivetime()?;
  assert_to_truefalse!(rust_naivetime, NaiveTime::from_hms(3, 17, 26), num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Second");

  let q_second=QGEN::new_second_hms(3, 17, 26);
  let rust_i32=q_second.into_i32()?;
  assert_to_truefalse!(rust_i32, 11846, num_success, num_failure);

  print!("<<{:^58}>>", "0Ni from Second");

  let q_second=QGEN::new_second(Q_0Nv);
  let rust_i32=q_second.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Ni, num_success, num_failure);

  print!("<<{:^58}>>", "0Wi from Second");

  let q_second=QGEN::new_second(Q_0Wv);
  let rust_i32=q_second.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Wi, num_success, num_failure);

  // Time //---------------------------------------/
  print!("<<{:^58}>>", "NaiveTime from Second");

  let q_time=QGEN::new_time_hms_millis(21, 56, 7, 302);
  let rust_naivetime=q_time.into_naivetime()?;
  assert_to_truefalse!(rust_naivetime, NaiveTime::from_hms_milli(21, 56, 7, 302), num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Time");

  let q_time=QGEN::new_time_hms_millis(21, 56, 7, 302);
  let rust_i32=q_time.into_i32()?;
  assert_to_truefalse!(rust_i32, 78967302, num_success, num_failure);

  print!("<<{:^58}>>", "0Ni from Time");

  let q_time=QGEN::new_time(Q_0Nt);
  let rust_i32=q_time.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Ni, num_success, num_failure);

  print!("<<{:^58}>>", "0Wi from Time");

  let q_time=QGEN::new_time(Q_0Wt);
  let rust_i32=q_time.into_i32()?;
  assert_to_truefalse!(rust_i32, Q_0Wi, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test conversion from list q object into Rust object.
* Note: Logics behind into and get are same; therefore 'get' and 'get_mut' are tested
* only once.
*/
fn list_conversion_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| List Conversion ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Bool //---------------------------------------/
  print!("<<{:^58}>>", "bool reference from Bool");

  let mut q_bool_list=QGEN::new_bool_list(Attribute::None, vec![true, false]);
  let (attribute, rust_bool_vec) = q_bool_list.get_bool_vec()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(attribute, Attribute::None);
    assert_vec_eq!(rust_bool_vec, vec![true, false]);
  }, num_success, num_failure);

  print!("<<{:^58}>>", "bool mutable reference from Bool");

  let (attribute, rust_bool_vec) = q_bool_list.get_bool_vec_mut()?;
  rust_bool_vec.swap(0, 1);
  assert_to_truefalse_custom!(||{
    assert_eq!(attribute, Attribute::None);
    assert_vec_eq!(rust_bool_vec, vec![false, true]);
  }, num_success, num_failure);

  print!("<<{:^58}>>", "bool from Bool");

  let (_, rust_bool_vec) = q_bool_list.into_bool_vec()?;
  // Underlying vector has been changed by mutable reference test
  assert_to_truefalse!(rust_bool_vec, vec![false, true], num_success, num_failure);

  // GUID //---------------------------------------/
  print!("<<{:^58}>>", "[u8, 16] from GUID");

  let q_GUID_list=QGEN::new_GUID_list(Attribute::None, vec![[0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]]);
  let (_, rust_GUID_vec) = q_GUID_list.into_GUID_vec()?;
  assert_to_truefalse!(rust_GUID_vec, vec![[0x1e, 0x11, 0x17, 0x0c, 0x42, 0x24, 0x25, 0x2c, 0x1c, 0x14, 0x1e, 0x22, 0x4d, 0x3d, 0x46, 0x24]], num_success, num_failure);

  // Byte //---------------------------------------/
  print!("<<{:^58}>>", "u8 from Byte");

  let q_byte=QGEN::new_byte_list(Attribute::None, vec![0x3c, 0x22, 0x4f]);
  let (_, rust_u8_vec) = q_byte.into_u8_vec()?; 
  assert_to_truefalse!(rust_u8_vec, vec![0x3c, 0x22, 0x4f], num_success, num_failure);

  // Short //--------------------------------------/
  print!("<<{:^58}>>", "i16 from Short");

  let q_short=QGEN::new_short_list(Attribute::Sorted, vec![70_i16, 128, 1028, 2000]);
  let (attribute, rust_i16_vec) = q_short.into_i16_vec()?;
  assert_to_truefalse!((attribute, rust_i16_vec), (Attribute::Sorted, vec![70_i16, 128, 1028, 2000]), num_success, num_failure);

  // Int //---------------------------------------/
  print!("<<{:^58}>>", "i32 from Int");

  let q_int_list=QGEN::new_int_list(Attribute::None, vec![Q_0Ni, -34567789]);
  let (_, rust_i32_vec) = q_int_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![Q_0Ni, -34567789], num_success, num_failure);

  // Long //--------------------------------------/
  print!("<<{:^58}>>", "i64 from Long");

  let q_long_list=QGEN::new_long_list(Attribute::None, vec![86400000000000_i64, -86400000000000_i64]);
  let (_, rust_i64_vec) = q_long_list.into_i64_vec()?;
  assert_to_truefalse!(rust_i64_vec, vec![86400000000000_i64, -86400000000000_i64], num_success, num_failure);

  // Real //--------------------------------------/
  print!("<<{:^58}>>", "f32 from Real");

  let q_real_list=QGEN::new_real_list(Attribute::Sorted, vec![-1.25, 100.23, 3000.5639]);
  let (attribute, rust_f32_vec) = q_real_list.into_f32_vec()?;
  assert_to_truefalse!((attribute, rust_f32_vec), (Attribute::Sorted, vec![-1.25, 100.23, 3000.5639]), num_success, num_failure);

  // Float //-------------------------------------/
  print!("<<{:^58}>>", "f64 from Float");

  let q_float_list=QGEN::new_float_list(Attribute::None, vec![Q_0w, 103.678_f64, Q_0n]);
  let (_, rust_f64_vec) = q_float_list.into_f64_vec()?;
  assert_to_truefalse_custom!(||{
    assert!(rust_f64_vec[0].is_infinite());
    assert!(approx_eq!(f64, rust_f64_vec[1], 103.678_f64, epsilon=0.001));
    assert!(rust_f64_vec[2].is_nan());
  }, num_success, num_failure);

  // Char //--------------------------------------/
  print!("<<{:^58}>>", "char from Char");

  let q_char_list=QGEN::new_char_list(Attribute::Parted, "aabbccc");
  let (attribute, rust_char_vec) = q_char_list.into_char_vec()?;
  assert_to_truefalse!((attribute, rust_char_vec), (Attribute::Parted, String::from("aabbccc")), num_success, num_failure);

  // Symbol //--------------------------------------/
  print!("<<{:^58}>>", "String from Symbol");

  let q_symbol_list=QGEN::new_symbol_list(Attribute::Unique, vec!["kdb+", "db"]);
  let (attribute, rust_string_vec) = q_symbol_list.into_string_vec()?;
  assert_to_truefalse!((attribute, rust_string_vec), (Attribute::Unique, vec!["kdb+".to_string(), "db".to_string()]), num_success, num_failure);

  // Timestamp //--------------------------------------/
  print!("<<{:^58}>>", "DateTime<Utc> from Timestamp");

  let q_timestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2018, 2, 18, 4, 0, 0, 100), (2019, 12, 3, 4, 5, 10, 3456)]);
  let (_, rust_datetime_vec) = q_timestamp_list.into_datetime_vec()?;
  assert_to_truefalse!(rust_datetime_vec, vec![Utc.ymd(2018, 2, 18).and_hms_nano(4, 0, 0, 100), Utc.ymd(2019, 12, 3).and_hms_nano(4, 5, 10, 3456)], num_success, num_failure);

  print!("<<{:^58}>>", "i64 from Timestamp");

  let q_timestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2018, 2, 18, 4, 0, 0, 100), (2019, 12, 3, 4, 5, 10, 3456)]);
  let (_, rust_i64_vec) = q_timestamp_list.into_i64_vec()?;
  assert_to_truefalse!(rust_i64_vec, vec![572241600000000100_i64 + KDB_TIMESTAMP_OFFSET, 628661110000003456 + KDB_TIMESTAMP_OFFSET], num_success, num_failure);

  print!("<<{:^58}>>", "0N and 0W from Timestamp");

  let q_timestamp_list=QGEN::new_timestamp_list(Attribute::None, vec![Q_0Wp, Q_0Np]);
  let (_, rust_i64_vec) = q_timestamp_list.into_i64_vec()?;
  assert_to_truefalse!(rust_i64_vec, vec![Q_0Wj, Q_0Nj], num_success, num_failure);

  // Month //------------------------------------------/
  print!("<<{:^58}>>", "Date<Utc> from Month");

  let q_month_list=QGEN::new_month_list_ym(Attribute::None, vec![(2013, 9), (2009, 2)]);
  let (_, rust_date_vec) = q_month_list.into_date_vec()?;
  assert_to_truefalse!(rust_date_vec, vec![Utc.ymd(2013, 9, 1), Utc.ymd(2009, 2, 1)], num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Month");

  let q_month_list=QGEN::new_month_list_ym(Attribute::None, vec![(2013, 9), (2009, 2)]);
  let (_, rust_i32_vec) = q_month_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![164 + KDB_MONTH_OFFSET, 109 + KDB_MONTH_OFFSET], num_success, num_failure);

  print!("<<{:^58}>>", "0Ni and 0Wi from Month");

  let q_month_list=QGEN::new_month_list(Attribute::None, vec![Q_0Nm, Q_0Wm]);
  let (_, rust_i32_vec) = q_month_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![Q_0Ni, Q_0Wi], num_success, num_failure);

  // Date //-------------------------------------------/
  print!("<<{:^58}>>", "Date<Utc> from Date");

  let q_date_list=QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2000, 2, 9)]);
  let (_, rust_date_vec) = q_date_list.into_date_vec()?;
  assert_to_truefalse!(rust_date_vec, vec![Utc.ymd(2000, 2, 9)], num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Date");

  let q_date_list=QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2000, 2, 9)]);
  let (_, rust_i32_vec) = q_date_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![39 + KDB_DAY_OFFSET as i32], num_success, num_failure);

  print!("<<{:^58}>>", "0Ni and 0Wi from Date");

  let q_date_list=QGEN::new_date_list(Attribute::None, vec![Q_0Wd, Q_0Nd]);
  let (_, rust_i32_vec) = q_date_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![Q_0Wi, Q_0Ni], num_success, num_failure);

  // Datetime //---------------------------------------/
  print!("<<{:^58}>>", "DateTime<Utc> from Datetime");

  let q_datetime_list=QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2004, 6, 17, 11, 32, 40, 803), (2007, 11, 21, 14, 58, 53, 172)]);
  let (_, rust_datetime_vec) = q_datetime_list.into_datetime_vec()?;
  assert_to_truefalse!(rust_datetime_vec, vec![Utc.ymd(2004, 6, 17).and_hms_milli(11, 32, 40, 803), Utc.ymd(2007, 11, 21).and_hms_milli(14, 58, 53, 172)], num_success, num_failure);

  print!("<<{:^58}>>", "f64 from Datetime");

  let q_datetime_list=QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2004, 6, 17, 11, 32, 40, 803),  (2007, 11, 21, 14, 58, 53, 172)]);
  assert_to_truefalse_float_list!(q_datetime_list, vec![1629.481 + KDB_DAY_OFFSET as f64, 2881.624 + KDB_DAY_OFFSET as f64], 0.001, num_success, num_failure);

  print!("<<{:^58}>>", "0n or 0w from Datetime");

  let q_datetime_list=QGEN::new_datetime_list(Attribute::None, vec![Q_0Nz, *Q_0Wz]);
  let (_, rust_f64_vec) = q_datetime_list.into_f64_vec()?;
  assert_to_truefalse_custom!(||{
    assert!(rust_f64_vec[0].is_nan());
    assert!(rust_f64_vec[1].is_infinite());
  }, num_success, num_failure);

  // Timespan //---------------------------------------/
  print!("<<{:^58}>>", "Duration from Timespan");

  let q_timespan_list=QGEN::new_timespan_list_nanos(Attribute::None, vec![999_i64, 10000, 100000000]);
  let (_, rust_duration_vec)=q_timespan_list.into_duration_vec()?;
  assert_to_truefalse!(rust_duration_vec, vec![Duration::nanoseconds(999), Duration::nanoseconds(10000), Duration::nanoseconds(100000000)], num_success, num_failure);

  print!("<<{:^58}>>", "i64 from Timespan");

  let q_timespan_list=QGEN::new_timespan_list_nanos(Attribute::None, vec![999_i64, 10000, 100000000]);
  let (_, rust_i64_vec)=q_timespan_list.into_i64_vec()?;
  assert_to_truefalse!(rust_i64_vec, vec![999, 10000, 100000000], num_success, num_failure);

  print!("<<{:^58}>>", "0Nj and 0Wj from Timespan");

  let q_timespan_list=QGEN::new_timespan_list_nanos(Attribute::None, vec![Q_NEG_0Wj, Q_0Nj, Q_0Wj]);
  let (_, rust_i64_vec)=q_timespan_list.into_i64_vec()?;
  assert_to_truefalse!(rust_i64_vec, vec![Q_NEG_0Wj, Q_0Nj, Q_0Wj], num_success, num_failure);

  // Minute //-----------------------------------------/
  print!("<<{:^58}>>", "QTime from Minute");

  let q_minute_list=QGEN::new_minute_list(Attribute::None, vec![QTimeGEN::new_minute(NaiveTime::from_hms(6, 37, 4)), Q_0Nu, Q_0Wu, QTimeGEN::new_minute(NaiveTime::from_hms(17, 2, 18))]);
  let (_, rust_qtime_vec) = q_minute_list.into_qtime_vec()?;
  // Second should be ignored
  assert_to_truefalse!(rust_qtime_vec, vec![QTimeGEN::new_minute(NaiveTime::from_hms(6, 37, 0)), Q_0Nu, Q_0Wu, QTimeGEN::new_minute(NaiveTime::from_hms(17, 2, 0))], num_success, num_failure);

  print!("<<{:^58}>>", "NaiveTime from Minute");

  let q_minute_list=QGEN::new_minute_list(Attribute::None, vec![QTimeGEN::new_minute(NaiveTime::from_hms(6, 37, 4)), Q_0Nu, Q_0Wu, QTimeGEN::new_minute(NaiveTime::from_hms(17, 2, 18))]);
  let (_, rust_naivetime_vec) = q_minute_list.into_naivetime_vec()?;
  // Second should be ignored
  // Null and infinity are supressed into 00:00:00
  assert_to_truefalse!(rust_naivetime_vec, vec![NaiveTime::from_hms(6, 37, 0), NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(17, 2, 0)], num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Minute");

  let q_minute_list=QGEN::new_minute_list(Attribute::None, vec![QTimeGEN::new_minute(NaiveTime::from_hms(6, 37, 4)), Q_0Nu, Q_0Wu, QTimeGEN::new_minute(NaiveTime::from_hms(17, 2, 18))]);
  let (_, rust_i32_vec) = q_minute_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![397, Q_0Ni, Q_0Wi, 1022], num_success, num_failure);

  // Second //-----------------------------------------/
  print!("<<{:^58}>>", "QTime from Second");

  let q_second_list=QGEN::new_second_list(Attribute::None, vec![QTimeGEN::new_second(NaiveTime::from_hms(6, 37, 4)), Q_0Nv, Q_0Wv, QTimeGEN::new_second(NaiveTime::from_hms(17, 2, 18))]);
  let (_, rust_qtime_vec) = q_second_list.into_qtime_vec()?;
  assert_to_truefalse!(rust_qtime_vec, vec![QTimeGEN::new_second(NaiveTime::from_hms(6, 37, 4)), Q_0Nv, Q_0Wv, QTimeGEN::new_second(NaiveTime::from_hms(17, 2, 18))], num_success, num_failure);

  print!("<<{:^58}>>", "NaiveTime from Second");

  let q_second_list=QGEN::new_second_list(Attribute::None, vec![QTimeGEN::new_second(NaiveTime::from_hms(6, 37, 4)), Q_0Nv, Q_0Wv, QTimeGEN::new_second(NaiveTime::from_hms(17, 2, 18))]);
  let (_, rust_naivetime_vec) = q_second_list.into_naivetime_vec()?;
  // Null and infinity are supressed into 00:00:00
  assert_to_truefalse!(rust_naivetime_vec, vec![NaiveTime::from_hms(6, 37, 4), NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(17, 2, 18)], num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Second");

  let q_second_list=QGEN::new_second_list(Attribute::None, vec![QTimeGEN::new_second(NaiveTime::from_hms(6, 37, 4)), Q_0Nv, Q_0Wv, QTimeGEN::new_second(NaiveTime::from_hms(17, 2, 18))]);
  let (_, rust_i32_vec) = q_second_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![23824, Q_0Ni, Q_0Wi, 61338], num_success, num_failure);

  // Time //-------------------------------------------/
  print!("<<{:^58}>>", "QTime from Time");

  let q_time_list=QGEN::new_time_list(Attribute::None, vec![QTimeGEN::new_time(NaiveTime::from_hms_milli(6, 37, 4, 123)), Q_0Nt, Q_0Wt, QTimeGEN::new_time(NaiveTime::from_hms_milli(17, 2, 18, 456))]);
  let (_, rust_qtime_vec) = q_time_list.into_qtime_vec()?;
  assert_to_truefalse!(rust_qtime_vec, vec![QTimeGEN::new_time(NaiveTime::from_hms_milli(6, 37, 4, 123)), Q_0Nt, Q_0Wt, QTimeGEN::new_time(NaiveTime::from_hms_milli(17, 2, 18, 456))], num_success, num_failure);

  print!("<<{:^58}>>", "NaiveTime from Time");

  let q_time_list=QGEN::new_time_list(Attribute::None, vec![QTimeGEN::new_time(NaiveTime::from_hms_milli(6, 37, 4, 123)), Q_0Nt, Q_0Wt, QTimeGEN::new_time(NaiveTime::from_hms_milli(17, 2, 18, 456))]);
  let (_, rust_naivetime_vec) = q_time_list.into_naivetime_vec()?;
  // Null and infinity are supressed into 00:00:00
  assert_to_truefalse!(rust_naivetime_vec, vec![NaiveTime::from_hms_milli(6, 37, 4, 123), NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms(0, 0, 0), NaiveTime::from_hms_milli(17, 2, 18, 456)], num_success, num_failure);

  print!("<<{:^58}>>", "i32 from Time");

  let q_time_list=QGEN::new_time_list(Attribute::None, vec![QTimeGEN::new_time(NaiveTime::from_hms_milli(6, 37, 4, 123)), Q_0Nt, Q_0Wt, QTimeGEN::new_time(NaiveTime::from_hms_milli(17, 2, 18, 456))]);
  let (_, rust_i32_vec) = q_time_list.into_i32_vec()?;
  assert_to_truefalse!(rust_i32_vec, vec![23824123, Q_0Ni, Q_0Wi, 61338456], num_success, num_failure);

  // Mixed List //------------------------------------/
  print!("<<{:^58}>>", "Vec<Q> from MixedL");

  let q_mixed_list=QGEN::new_mixed_list(vec![QGEN::new_time_hms_millis(21, 4, 9, 85), QGEN::new_real_list(Attribute::Sorted, vec![72.548_f32, 237.89, 1002.236]), QGEN::new_char_list(Attribute::None, "praise")]);
  let rust_q_vec=q_mixed_list.into_q_vec()?;
  assert_to_truefalse!(rust_q_vec, vec![QGEN::new_time_hms_millis(21, 4, 9, 85), QGEN::new_real_list(Attribute::Sorted, vec![72.548_f32, 237.89, 1002.236]), QGEN::new_char_list(Attribute::None, "praise")], num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test conversion from dictionary q object into Rust object.
*/
fn dictionary_conversion_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Dictionary Conversion ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Dictionary //------------------------------------/
  print!("<<{:^58}>>", "(Q, Q) from Dictionary");

  let q_atom_dictionary=QGEN::new_dictionary(
    QGEN::new_symbol_list(Attribute::Sorted, vec!["a", "b", "c"]),
    QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)])
  );
  let (key, value) = q_atom_dictionary.into_key_value()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(key, QGEN::new_symbol_list(Attribute::Sorted, vec!["a", "b", "c"]));
    assert_eq!(value, QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2009, 1, 1), Utc.ymd(2001, 12, 1), Utc.ymd(2017, 8, 1)]));
  }, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test conversion from table q object into Rust object.
*/
fn table_conversion_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Dictionary Conversion ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Table //-----------------------------------------/
  print!("<<{:^58}>>", "(Q, Q) from Table");

  let q_table=QGEN::new_table(
    vec!["time", "sym", "price", "size", "ex"],
    vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]
  )?;
  let (key, value) = q_table.into_key_value()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(key, QGEN::new_symbol_list(Attribute::None, vec!["time", "sym", "price", "size", "ex"]));
    assert_eq!(value, QGEN::new_mixed_list(vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]))
  }, num_success, num_failure);

  print!("<<{:^58}>>", "(Vec<String>, Vec<Q>) from Table");

  let q_table=QGEN::new_table(
    vec!["time", "sym", "price", "size", "ex"],
    vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ]
  )?;
  let (header, body) = q_table.into_header_body()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(header, vec!["time".to_string(), "sym".to_string(), "price".to_string(), "size".to_string(), "ex".to_string()]);
    assert_eq!(body, vec![
      QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 13, 238912781), Utc.ymd(2020, 6, 1).and_hms_nano(7, 2, 14, 230892785), Utc.ymd(2020, 6, 1).and_hms_nano(7, 3, 1, 137860387)]),
      QGEN::new_symbol_list(Attribute::Grouped, vec!["Kx", "FD", "Kx"]),
      QGEN::new_float_list(Attribute::None, vec![103.68_f64, 107.42, 103.3]),
      QGEN::new_long_list(Attribute::None, vec![1000_i64, 2000, 3000]),
      QGEN::new_char_list(Attribute::None, "NLN")
    ])
  }, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test conversion from keyed table q object into Rust object.
*/
fn keyed_table_conversion_test() -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Dictionary Conversion ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  // Keyed Table //-----------------------------------/
  print!("<<{:^58}>>", "(Q, Q) from KeyedTable");

  let q_keyed_table=QGEN::new_keyed_table(
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
  )?;
  let (key, value) = q_keyed_table.into_key_value()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(key, QGEN::new_table(
      vec!["id", "month"],
      vec![
        QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
        QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
      ]).expect("Failed to build table")
    );
    assert_eq!(value, QGEN::new_table(
      vec!["stats", "sym"],
      vec![
        QGEN::new_float_list(Attribute::None, vec![113.42_f64, 354.923, 2749.4]),
        QGEN::new_symbol_list(Attribute::None, vec!["Newry", "Belfast", "London"])
      ]).expect("Failed to build table")
    );
  }, num_success, num_failure);

  print!("<<{:^58}>>", "(Vec<String>, Vec<Q>, Vec<String>, Vec<Q>) from KeyedTable");

  let q_keyed_table=QGEN::new_keyed_table(
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
  )?;

  let (kheader, kvalue, vheader, vvalue) = q_keyed_table.into_keyedtable_components()?;
  assert_to_truefalse_custom!(||{
    assert_eq!(kheader, vec!["id".to_string(), "month".to_string()]);
    assert_eq!(kvalue, vec![
      QGEN::new_long_list(Attribute::Sorted, vec![0_i64, 1, 2]),
      QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2000, 1, 1), Utc.ymd(2000, 2, 1), Utc.ymd(2000, 3, 1)])
    ]);
    assert_eq!(vheader, vec!["stats".to_string(), "sym".to_string()]);
    assert_eq!(vvalue, vec![
      QGEN::new_float_list(Attribute::None, vec![113.42_f64, 354.923, 2749.4]),
      QGEN::new_symbol_list(Attribute::None, vec!["Newry", "Belfast", "London"])
    ])
  }, num_success, num_failure);

  Ok((num_success, num_failure))
}

/*
* Test if compression and decompresion are triggered properly
*/
async fn compression_test(handle: &mut UnixStreamH) -> io::Result<(u32, u32)>{
  println!("\n+{:-^70}+\n", "|| Compression ||");

  let mut num_success: u32=0;
  let mut num_failure: u32=0;

  print!("<<{:^58}>>", "uncompressed message");
  
  // Set test table remotely
  send_string_query_async_le_uds(handle, "tab:([]time:2000.01.01D00:00:00+86400000000000*til 1000; sym:raze 250#/: `AAPL`MSFT`AMZ`GOOGL)").await?;

  // Prepare q table which will NOT be compressed
  let mut time_vec=vec![KDB_TIMESTAMP_OFFSET; 1000];
  for i in 0..1000{
    time_vec[i]+=ONE_DAY_NANOS * i as i64;
  }
  let time_col=QGEN::new_timestamp_list_nanos(Attribute::None, time_vec);
  let sym_col=QGEN::new_symbol_list(Attribute::None, [vec!["AAPL"; 250], vec!["MSFT"; 250], vec!["AMZ"; 250], vec!["GOOGL"; 250]].concat());
  let original=QGEN::new_table(vec!["time", "sym"], vec![time_col, sym_col])?;

  // Set 'set' function remotely
  send_string_query_async_le_uds(handle, "set0: set").await?;

  // Assign sent table as `tab2`
  send_query_async_le_uds(handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("set0"), QGEN::new_symbol("tab2"), original])).await?;
  // Compare with `tab` sent before `tab2`
  let res_compare=send_string_query_le_uds(handle, "tab ~ tab2").await?;
  assert_to_truefalse!(res_compare, QGEN::new_bool(true), num_success, num_failure);

  print!("<<{:^58}>>", "compressed message");

  // Prepare a table which should be compressed
  send_string_query_async_le_uds(handle, "tab:([]time:1000#2000.01.01D00:00:00; sym:raze 250#/: `AAPL`MSFT`AMZ`GOOGL)").await?;
  let res_compressed=send_string_query_le_uds(handle, "tab").await?;

  let time_col=QGEN::new_timestamp_list_nanos(Attribute::None, vec![KDB_TIMESTAMP_OFFSET; 1000]);
  let sym_col=QGEN::new_symbol_list(Attribute::None, [vec!["AAPL"; 250], vec!["MSFT"; 250], vec!["AMZ"; 250], vec!["GOOGL"; 250]].concat());
  let original=QGEN::new_table(vec!["time", "sym"], vec![time_col, sym_col])?;

  assert_to_truefalse!(res_compressed, original, num_success, num_failure);

  Ok((num_success, num_failure))
}