# rustkdb
Rust interface for kdb+
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/kxsystems/gokdb?include_prereleases)](https://github.com/kxsystems/gokdb/releases)
[![Travis (.org) branch](https://img.shields.io/travis/kxsystems/gokdb/master?label=travis%20build)](https://travis-ci.org/kxsystems/gokdb/branches)
[![Go Report Card](https://goreportcard.com/badge/github.com/kxsystems/gokdb)](https://goreportcard.com/report/github.com/kxsystems/gokdb)
[![GoDoc](https://img.shields.io/badge/GoDoc-reference-007d9c)](https://pkg.go.dev/github.com/kxsystems/gokdb)

## Introduction

KDB IPC interface for the Rust open source programming language.

## New to kdb+?

Kdb+ is the world's fastest time-series database, optimized for ingesting, analyzing and storing massive amounts of structured data. To get started with kdb+, please visit https://code.kx.com/q/learn/ for downloads and developer information. For general information, visit https://kx.com/

## New to Rust?

Rust binary releases can be downloaded following the instruction on https://www.rust-lang.org/tools/install

An online introductory tour of the Rust programming language is available [here](https://doc.rust-lang.org/stable/book/).


## Overview

**rustkdb** is provided as one of Fusion interfaces of kdb+/q. This crate provides methods to connect to q/kdb+ process, send a (text) query to q process, construct objects representing q objects and convert the objects into Rust type objects. **rustkdb** is using [Tokio](https://tokio.rs/) crate to provide asynchronous execution.

### Connection
Connection to kdb+ is either of TCP or TLS.

### Query
Query to kdb+ is supported by two ways, sending a text query or a functional query which is represented by a compound list of kdb+ ([See detail of IPC](https://code.kx.com/q4m3/11_IO/#116-interprocess-communication)). Compression/decompression of messages is done following [kdb+ implementation](https://code.kx.com/q/basics/ipc/#compression).

## Utility Types

**rustkdb** defines some utility types to express distinct q types:
- `QTime`: Intermediate object for q minute, q second and q time to hold either of time value or possible null or inifinity value.
- `QTable`: Struct holding header and column values of table
- `QDictionary`: Struct holding key and value of dictionary
- `QKeyedTable`: Struct holding key table and value table of keyed table
- `QList`: Struct holding an attribute and list values of q list object

*Note: These types are not accessed by a user.*

## Type Mapping

Using the utility types above, type mapping between q types and Rust types are following:

| q                | Rust                                |
|------------------|-------------------------------------|
| `bool`           | `bool`                              |
| `GUID`           | `[u8; 16]`                          |
| `byte`           | `u8`                                |
| `short`          | `i16`                               |
| `int`            | `i32`                               |
| `long`           | `i64`                               |
| `real`           | `f32`                               |
| `float`          | `f64`                               |
| `char`           | `char`                              |
| `symbol`         | `String`                            |
| `timestamp`      | `chrono::DateTime<Utc>`             |
| `month`          | `chrono::Date<Utc>`                 |
| `date`           | `chrono::Date<Utc>`                 |
| `datetime`       | `chrono::DateTime<Utc>`             |
| `timespan`       | `chrono::Duration`                  |
| `minute`         | `chrono::NaiveTieme` or `QTime`     |
| `second`         | `chrono::NaiveTieme` or `QTime`     |
| `time`           | `chrono::NaiveTieme` or `QTime`     |
| `table`          | `QTable`                            |
| `dictionary`     | `QDictionary`                       |
| `keyed table`    | `QKeyedTable`                       |
| `bool list`      | `QList<Vec<bool>>`                  |
| `GUID list`      | `QList<Vec<[u8; 16]>>`              |
| `byte list`      | `QList<Vec<u8>>`                    |
| `short list`     | `QList<Vec<i16>>`                   |
| `int list`       | `QList<Vec<i32>>`                   |
| `long list`      | `QList<Vec<i64>>`                   |
| `real list`      | `QList<Vec<f32>>`                   |
| `float list`     | `QList<Vec<f64>>`                   |
| `string`         | `QList<String>`                     |
| `symbol list`    | `QList<Vec<String>>`                |
| `timestamp list` | `QList<Vec<chrono::DateTime<Utc>>>` |
| `month list`     | `QList<Vec<chrono::Date<Utc>>>`     |
| `date list`      | `QList<Vec<chrono::Date<Utc>>>`     |
| `datetime list`  | `QList<Vec<chrono::DateTime<Utc>>>` |
| `timespan list`  | `QList<Vec<chrono::Duration>>`      |
| `minute list`    | `QList<Vec<chrono::NaiveTime>>`     |
| `second list`    | `QList<Vec<chrono::NaiveTime>>`     |
| `time list`      | `QList<Vec<chrono::NaiveTime>>`     |
| `general null`   | `QGeneralNull`                      |

## Installation

As this interface is a crate itself, using this crate follows ordinary crate import manner, i.e. adding `rustkdb` to `Cargo.toml` in your Rust project.

## Examples

Run q process on localhost:5000 enabling TLS connect though connection to q process is done only by TCP in this example.

```bash

$ q -p 5000 -E 1

```

Define an example function wchich takes one argument of numeric type(short, int or long).

```q

q)fibonacci:{[n] $[n in 1 2; n#1; (n-2) {[seq] seq, sum -2#seq}/ 1 1]}

```

As **rustkdb** is using Tokio crate, it is assumed in the example below that Tokio runtime is imorted in main function.

```Rust

use rustkdb::qtype::*;
use rustkdb::connection::*;

// Connect to q process
let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");

// Send a functional query synchronously with Little Endian encode
// This query is equivalent to (`fibonacci; 10i)
let res_long_list=send_query_le(&mut handle, QGEN::new_mixed_list(vec![QGEN::new_symbol("fibonacci"), QGEN::new_int(10)])).await?;
// 1 1 2 3 5 8 13 21 34 55j
println!("{}", res_long_list);

// Send a text query asynchronously in Big Endian encode
send_string_query_async_be(&mut handle, "a:1+2").await?;

// Send a text query synchroously in Little Endian encode
let res_short_list=send_string_query_le(handle, "type each a + til 3").await?;

// Build q list object
let handmade_short_list=QGEN::new_short_list(Attribute::None, vec![-7_i16, -7, -7]):
assert_eq!(res_short, handmade_short_list);

// Convert it into a tuple of attribute and underlying vector
let (attribute, rust_short_vec)=res_short_list.into_i16_vec()?;
assert_eq!(attribute, Attribute::None);
assert_eq!(rust_short_vec, vec![-7_i16, -7, -7];

```

More examples can be found in crate documentation and [example folder](./examples/) in this repository.