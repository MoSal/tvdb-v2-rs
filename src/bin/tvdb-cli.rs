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

use tvdb_v2::tvdb_api::{SeriesSearch, SeriesDetailedInfo, EpisodeList};
use tvdb_v2::tvdb_from::{TvdbFrom, TvdbAuthToken};

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

fn get_num_from_stdin(msg: &str, start: usize, end: usize) -> usize {
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

fn get_search_str() -> String {
    let args_vec_str : Vec<_> = std::env::args()
        .skip(1)
        .map(|a| a.to_string())
        .collect();

    args_vec_str.join("+")
}

fn main() {
    let search_str = get_search_str();

    if search_str.is_empty() {
        println!("Usage: {} <search>", std::env::args().nth(0).unwrap_or("tvdb-cli".into()));
        std::process::exit(0);
    }

    let auth = exit_if_err!("Failed to get AUTH info", TvdbAuthToken::from_key("0629B785CE550C8D"));
    let auth_token = auth.get_auth_token();

    let search = exit_if_err!("Failed to get search results", SeriesSearch::from_id(&search_str, auth_token));
    let series = search.get_series_newest_first();
    search.print_series_newest_first();

    let idx = get_num_from_stdin("Enter result number to get details", 1, series.len()) - 1;
    let choice = get_num_from_stdin("Choose:\n \
                                    1- Series Details\n \
                                    2- Series Details (with seasons overview)\n \
                                    3- Episode List\n",
                                    1, 3);

    match choice {
        1 =>  {
            let series_details = exit_if_err!("Failed to get series details", SeriesDetailedInfo::from_id(series[idx].get_id(), auth_token));
            series_details.print_info();
        },
        2 =>  {
            let series_details = exit_if_err!("failed to get series details", SeriesDetailedInfo::from_id(series[idx].get_id(), auth_token));
            series_details.print_info_with_seasons(auth_token);
        },
        3 =>  {
            let episode_list = exit_if_err!("failed to get episode list", EpisodeList::from_id(series[idx].get_id(), auth_token));
            episode_list.print_list(Some(series[idx].get_name()));
        },
        _ => unreachable!(),
    }

}
