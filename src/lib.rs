// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

//! This library provides access to device libraries exported from Pioneer's Rekordbox DJ software.

#![warn(unsafe_code)]
#![allow(missing_docs)]
#![allow(private_interfaces)]
#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(rust_2018_idioms)]
#![deny(rust_2021_compatibility)]
#![deny(missing_debug_implementations)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::all)]
#![allow(clippy::explicit_deref_methods)]
#![allow(clippy::explicit_into_iter_loop)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::must_use_candidate)]

pub mod anlz;
pub mod pdb;
pub mod setting;
pub mod util;
pub mod xml;
pub(crate) mod xor;

pub use crate::util::RekordcrateError as Error;
pub use crate::util::RekordcrateResult as Result;
