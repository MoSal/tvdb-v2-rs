/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

extern crate tvdb_v2;

use std::io::{self, Write};

use tvdb_v2::tvdb_api::{Updated, SeriesDetailedInfo};

macro_rules! exit_if_err {
    ($msg:expr, $result:expr) => {
        match $result {
            Err(_) => {
                println!("{}.", $msg);
                std::process::exit(1);
            },
            Ok(ret) => ret,
        }
    }
}

fn get_num_from_stdin<N: Eq + std::fmt::Display + std::str::FromStr + PartialOrd>(msg: &str, start: N, end: N) -> N {

    let full_msg = if start == end {
        format!("{} ({}, or 'q' to quit): ", msg, start)
    } else {
        format!("{} ([{}-{}], or 'q' to quit): ", msg, start, end)
    };

    print!("\n{}", full_msg);
    exit_if_err!("Flushing stdout failed", io::stdout().flush());

    loop {
        let mut buffer = String::new();
        exit_if_err!("Reading line from stdin failed", io::stdin().read_line(&mut buffer));

        // remove \n at the end
        buffer.pop();

        if buffer == "q" {
            std::process::exit(0);
        }

        if let Ok(num) = buffer.parse() {
            if num >= start && num <= end {
                println!("");
                break num;
            }
        }

        print!("\nInvalid input.\n{}", full_msg);
        exit_if_err!("Flushing stdout failed", io::stdout().flush());
    }
}

async fn async_main() {
    //let auth_token = exit_if_err!("Failed to get AUTH token", tvdb_auth::auth_token("0629B785CE550C8D"));
    let offset = get_num_from_stdin("How many weeks backwards we should start searching from", 0, 100);
    let num = get_num_from_stdin("How many weeks we should search", 1, 100);
    let updated = exit_if_err!("Failed to get updated list ids", Updated::get_from_weeks(offset, num).await);
    let ids = updated.get_ids();
    if !ids.is_empty() {
        let id_count = ids.len();
        let mut filtered = Vec::with_capacity(id_count);
        let filters: [Box<dyn Fn(&SeriesDetailedInfo) -> bool>; 5] = [
            Box::new(|x| x.get_rating_count() >= 10),
            Box::new(|x| x.get_rating() >= 8.0),
            //Box::new(|x| x.get_status() == "Continuing"),
            Box::new(|x| x.get_genre().iter().find(|g| &**g == "Drama").is_some()),
            Box::new(|x| x.get_genre().iter().find(|g| &**g == "Crime").is_none()),
            Box::new(|x| x.get_genre().iter().find(|g| &**g == "Action").is_none()),
        ];
        println!("{} ids found, first={}, last={}", id_count, ids[0], ids.last().unwrap());
        for (idx, id) in ids.iter().enumerate() {
            println!("({}/{}) Getting info from id {}...", idx+1, id_count, id);
            let di = exit_if_err!("Failed to get detailed series info", SeriesDetailedInfo::from_id(id).await);

            let mut pass = true;
            for filter in &filters[..] {
                if !filter(&di) {
                    pass = false;
                    break;
                }
            }

            if pass {
                println!("({}/{}) Adding {} to filtered list", filtered.len()+1, idx+1, di.get_name());
                filtered.push(di);
            }
        }

        if !filtered.is_empty() {
            println!("Filtered list:");
            for (idx, di) in filtered.iter().enumerate() {
                println!("{}. rating: {:>4.2}, votes: {:>4}, first-aired: {:>12} {}",
                         idx+1, di.get_rating(), di.get_rating_count(), di.get_first_aired(), di.get_name());
            }
        }
    }
}

fn main() {
    async_global_executor::block_on(async_main())
}
