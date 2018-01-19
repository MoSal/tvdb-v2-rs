/*
    This file is a part of tvdb-v2-rs.

    Copyright (C) 2017 Mohammad AlSaleh <CE.Mohammad.AlSaleh at gmail.com>
    https://github.com/MoSal/tvdb-v2-rs

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at <http://mozilla.org/MPL/2.0/>.
*/

use serde_json;

use std::collections::BTreeMap;

use tvdb_errors::*;
use tvdb_from::{TvdbFrom, TvdbFromMulti};
use BASE_URL;

#[derive(Deserialize, Debug, Clone)]
pub struct SeriesSearchInfo {
    id: usize,
    #[serde(rename = "seriesName")]
    name: String,
    status: String,
    #[serde(rename = "firstAired")]
    first_aired: String,
}

impl SeriesSearchInfo {
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &*self.name
    }
    pub fn get_status(&self) -> &str {
        &*self.status
    }
    pub fn get_first_aired(&self) -> &str {
        &*self.first_aired
    }
}

pub struct SeriesSearchParams {
    name: Option<String>,
    // TODO
}

impl SeriesSearchParams {
    pub fn new() -> Self {
        Self {
            name: None,
        }
    }

    pub fn name<S: ToString>(mut self, name: S) -> Self {
        self.name = Some(name.to_string());
        self
    }

    fn get_str(&self) -> Result<String> {
        let mut ret = String::with_capacity(4096);

        match *self {
            Self {name: None} => Err("At least one search parameter should be set")?,
            Self {name: Some(ref name)} => {
                ret  = ret + "name=" + name;
                Ok(ret)
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SeriesSearch {
    data: Vec<SeriesSearchInfo>,
}

impl TvdbFrom for SeriesSearch {
    // id here is all search params
    fn url_from_id(id: &str) -> String {
        String::from(BASE_URL) + "/search/series?" + id
    }
}

impl SeriesSearch {
    pub fn from_params(params: &SeriesSearchParams, auth_token: &str) -> Result<Self> {
        Self::from_id(params.get_str()?, auth_token)
    }

    pub fn get_series(&self) -> &[SeriesSearchInfo] {
        &*self.data
    }

    pub fn get_series_newest_first(&self) -> Vec<SeriesSearchInfo> {
        let mut search_list = self.data.clone();
        search_list.sort_by(|curr, next| next.get_first_aired().cmp(curr.get_first_aired()));
        search_list
    }

    fn _print_series(series: &[SeriesSearchInfo]) {
        println!("{:03}  |  {: ^48}  |  {: ^11}  |  {}",
                 "Num", "Name", "First-Aired", "Status");

        println!("{}", "-".repeat(88));

        for (i, s) in series.iter().enumerate() {
            println!("{:03}  |  {: ^48}  |  {: ^11}  |  {}",
                     i+1, s.get_name(), s.get_first_aired(), s.get_status());
            println!("{}", "-".repeat(88));
        }
    }

    pub fn print_series(&self) {
        Self::_print_series(self.get_series());
    }

    pub fn print_series_newest_first(&self) {
        Self::_print_series(&*self.get_series_newest_first());
    }
}

// ==============

#[derive(Deserialize, Debug, Clone)]
pub struct SeriesDetailedInfo {
    id: usize,
    #[serde(rename = "seriesName")]
    name: String,
    status: String,
    aliases: Vec<String>,
    #[serde(rename = "firstAired")]
    first_aired: String,
    runtime: String,
    network: String,
    genre: Vec<String>,
    #[serde(rename = "rating")]
    parental_rating: String,
    #[serde(rename = "siteRating")]
    rating: f64,
    #[serde(rename = "siteRatingCount")]
    rating_count: usize,
    overview : Option<String>,
}

impl TvdbFrom for SeriesDetailedInfo {
    fn url_from_id(id: &str) -> String {
        String::from(BASE_URL) + "/series/" + id
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Define container
        #[derive(Deserialize)]
        struct SeriesDetailedInfoContainer {
            data: SeriesDetailedInfo,
        }

        // Deserialize container
        let info : SeriesDetailedInfoContainer = serde_json::from_slice(bytes)?;

        // Return contained data directly
        Ok(info.data)
    }
}

impl SeriesDetailedInfo {
    pub fn from_id<S: ToString>(id: S, auth_token: &str) -> Result<Self> {
        <Self as TvdbFrom>::from_id(id, auth_token)
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn get_name(&self) -> &str {
        &*self.name
    }
    pub fn get_status(&self) -> &str {
        &*self.status
    }
    pub fn get_aliases(&self) -> &[String] {
        &*self.aliases
    }
    pub fn get_first_aired(&self) -> &str {
        &*self.first_aired
    }
    pub fn get_runtime(&self) -> &str {
        &*self.runtime
    }
    pub fn get_network(&self) -> &str {
        &*self.network
    }
    pub fn get_genre(&self) -> &[String] {
        &*self.genre
    }
    pub fn get_parental_rating(&self) -> &str {
        &*self.parental_rating
    }
    pub fn get_rating(&self) -> f64 {
        self.rating
    }
    pub fn get_rating_count(&self) -> usize {
        self.rating_count
    }
    pub fn get_overview(&self) -> &str {
        match self.overview {
            Some(ref s) => &*s,
            None => "N/A",
        }
    }

    fn _print_info_main(&self) {
        println!("{} [{}]:", self.name, self.id);
        println!(" {: <12} {}", "First-Aired:", self.first_aired);
        println!(" {: <12} {}", "Network:", self.network);
        println!(" {: <12} {}", "PG:", self.parental_rating);
        println!(" {: <12} {}", "Status:", self.status);
        println!(" {: <12} {}", "Runtime:", self.runtime);
        println!(" {: <12} {} ({})", "Rating:", self.rating, self.rating_count);
        println!(" {: <12} {}", "Genre:", self.genre.join(" | "));
    }

    fn _print_info_overview(&self) {
        let overview = self.get_overview()
            .to_string()
            .replace('\n', " ")
            .replace("  ", " ");

        println!("[{}]", overview);
    }

    fn _print_info_seasons(&self, auth_token: &str) {
        if let Ok(ep_list) = EpisodeList::from_id(self.id, auth_token) {
            println!(" {: <12}", "Seasons:");
            for s in ep_list.list_by_season().keys() {
                if let Some(s_info) = SeasonInfo::from_episode_list(&ep_list, *s) {
                    println!("  {:02}  |  {:02} episodes  |  {: ^11} - {: ^11}",
                             s_info.get_id(), s_info.get_episode_count(),
                             s_info.get_first_aired(), s_info.get_last_aired());
                }
            }
        }
    }

    pub fn print_info(&self) {
        self._print_info_main();
        self._print_info_overview();
    }

    pub fn print_info_with_seasons(&self, auth_token: &str) {
        self._print_info_main();
        self._print_info_seasons(auth_token);
        self._print_info_overview();
    }
}

// ==============

#[derive(Deserialize, Debug, Clone)]
pub struct EpisodeInfo {
    #[serde(rename = "airedSeason")]
    season: usize,
    #[serde(rename = "airedEpisodeNumber")]
    number: usize,
    #[serde(rename = "episodeName")]
    name: Option<String>,
    #[serde(rename = "firstAired")]
    first_aired: String,
}

impl EpisodeInfo {
    pub fn get_season(&self) -> usize {
        self.season
    }
    pub fn get_number(&self) -> usize {
        self.number
    }
    pub fn get_name(&self) -> &str {
        match self.name {
            Some(ref s) => &*s,
            None => "N/A",
        }
    }
    pub fn get_first_aired(&self) -> &str {
        &*self.first_aired
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PageLinks {
    first: usize,
    last: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EpisodeList {
    // Optionally set series name (useful for e.g. printing info)
    #[serde(default)]
    series_name: Option<String>,

    // Episode lists are paginated (100 episodes/page)
    links: PageLinks,

    // The full episode list (deserialized per page)
    data: Vec<EpisodeInfo>,
}

impl TvdbFrom for EpisodeList {
    fn url_from_id(id: &str) -> String {
        String::from(BASE_URL) + "/series/" + id + "/episodes"
    }
}

impl TvdbFromMulti for EpisodeList {
    type Element = EpisodeInfo;

    fn num_pages(&self) -> usize {
        self.links.last
    }

    fn get_data(&self) -> &[Self::Element] {
        &*self.data
    }

    fn get_data_mut(&mut self) -> &mut Vec<Self::Element> {
        &mut self.data
    }
}

impl EpisodeList {
    pub fn from_id<S: ToString>(id: S, auth_token: &str) -> Result<Self> {
        Self::from_id_multi(id, auth_token)
    }

    pub fn from_id_with_series_name<I: ToString, N: ToString>(id: I, series_name: N, auth_token: &str) -> Result<Self> {
        let mut ret = Self::from_id(id, auth_token)?;
        ret.series_name = Some(series_name.to_string());
        Ok(ret)
    }

    pub fn get_episode_list(&self) -> &[EpisodeInfo] {
        &*self.data
    }

    fn list_by_season(&self) -> BTreeMap<usize, Vec<&EpisodeInfo>> {
        let mut b_map : BTreeMap<usize, Vec<&EpisodeInfo>> = BTreeMap::new();

        for e in self.get_episode_list() {
            let season = e.get_season();

            // We don't use get_mut() directly due to lack of non-lexical lifetimes
            if b_map.contains_key(&season) {
                b_map.get_mut(&season).expect("impossible").push(e);
            }
            else {
                b_map.insert(season, vec![e]);
            }

        }

        for season in b_map.values_mut() {
            season.sort_by(|curr, next| curr.get_number().cmp(&next.get_number()));
        }

        b_map
    }

    pub fn print_list(&self) {
        let empty_string = String::new();
        let series_name = self.series_name.as_ref().unwrap_or(&empty_string);

        let list_by_season = self.list_by_season();

        for season in list_by_season.keys() {
            println!("Season {:2}:", season);
            if let Some(episodes) = list_by_season.get(&season) {
                for episode in episodes {
                    println!("  {} S{:02}E{:02}  | {: ^11} |  {}",
                             series_name, episode.get_season(), episode.get_number(),
                             episode.get_first_aired() , episode.get_name());
                }
            }
        }
    }
}

struct SeasonInfo {
    id: usize,
    episode_count: usize,
    first_aired: String,
    last_aired: String,
}

impl SeasonInfo {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_episode_count(&self) -> usize {
        self.episode_count
    }

    fn get_first_aired(&self) -> &str {
        &self.first_aired
    }

    fn get_last_aired(&self) -> &str {
        &self.last_aired
    }

    fn from_episode_list(list: &EpisodeList, id: usize) -> Option<Self> {
        if let Some(season_episodes) = list.list_by_season().get(&id) {
            let episode_count = season_episodes.len();
            let first_aired = season_episodes.iter()
                .nth(0)
                .map(|e| String::from(e.get_first_aired()))
                .unwrap_or(String::new());
            let last_aired = season_episodes.iter()
                .last()
                .map(|e| String::from(e.get_first_aired()))
                .unwrap_or(String::new());

            Some(Self {
                id,
                episode_count,
                first_aired,
                last_aired,
            })

        } else {
            None
        }
    }
}
