// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![feature(type_ascription)]
#![feature(rustc_private)]
#![feature(specialization)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate quick_error;
extern crate byteorder;
extern crate thrift;
extern crate ordered_float;
extern crate try_from;
extern crate arena;
extern crate snap;
extern crate brotli;
extern crate flate2;
extern crate rand;

#[macro_use]
mod errors;
mod basic;
mod parquet_thrift;
#[macro_use]
mod util;
mod column;
mod compression;
mod encoding;

pub mod schema;
pub mod file;
