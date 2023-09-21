pub mod bangumi;
pub mod parser;

use colored::Colorize;
use core::fmt;
use regex::Regex;
use std::collections::HashMap;

pub enum LANG {
    EN,
    JP,
    CN,
}
pub struct BangumiTitle {
    cn: Option<String>,
    en: Option<String>,
    jp: Option<String>,
}

impl fmt::Display for BangumiTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.get_title(LANG::CN).green(),
            self.get_title(LANG::EN).blue(),
            self.get_title(LANG::JP).yellow(),
        )
    }
}

impl BangumiTitle {
    pub fn new(cn: Option<String>, en: Option<String>, jp: Option<String>) -> Self {
        BangumiTitle { cn, en, jp }
    }

    pub fn get_title(&self, lang: LANG) -> &str {
        let opt_title = match lang {
            LANG::CN => &self.cn,
            LANG::EN => &self.en,
            LANG::JP => &self.jp,
        };
        if let Some(title) = opt_title {
            return &title[..];
        }
        ""
    }

    pub fn get_default_title(&self) -> &str {
        self.cn
            .as_deref()
            .or(self.en.as_deref())
            .or(self.jp.as_deref())
            .unwrap_or("")
    }
}

pub struct BangumiInfo {
    pub title: BangumiTitle,
    pub season: u32,
    pub episode: u32,
    pub group: Option<String>,
}

impl fmt::Display for BangumiInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nS{:02}E{:02} - [{}]",
            &self.title,
            &self.season,
            &self.episode,
            if let Some(g) = &self.group { g } else { "" }
        )
    }
}

impl BangumiInfo {
    pub fn new(raw_titles: Vec<&str>, link: String) {
        let part_re = Regex::new(r"(?P<season>.*|\[.*])(?P<episode> -? \d+|\[\d+]|\[\d+.?[vV]\d]|第\d+[话話集]|\[第?\d+[话話集]]|\[\d+.?END]|[Ee][Pp]?\d+)(?P<others>.*)").unwrap();
        // let prefix_re = Regex::new(r"[^\w\s\u4e00-\u9fff\u3040-\u309f\u30a0-\u30ff-]").unwrap();

        // let mut votes_group = HashMap::new();
        println!("{link}");
        for raw_title in raw_titles {
            println!("Raw title: {}", raw_title.red());
            let mut raw_title = Self::pre_process(&raw_title);

            let group = Self::pop_group(&mut raw_title);
            // println!("- Group: {}", group.blue());

            if let Some(caps) = part_re.captures(&raw_title) {
                let raw_season = caps.name("season").map_or("", |m| m.as_str()).trim();
                let raw_episode = caps.name("episode").map_or("", |m| m.as_str()).trim();
                // let raw_others = caps.name("others").map_or("", |m| m.as_str()).trim();
                // println!("- raw_season: {}", raw_season.yellow());
                // println!("- raw_episode: {}", raw_episode.yellow());
                // println!("- raw_others: {}", raw_others.yellow());

                let mut raw_bangumi_name = raw_season.to_owned();
                let season = Self::pop_season(&mut raw_bangumi_name);

                // println!("- Season: {}", season.to_string().blue());
                // println!("- raw_bangumi_name: {}", raw_bangumi_name.yellow().green());

                let bangumi_name = Self::get_name(&raw_bangumi_name);
                let ep = Self::get_episode(raw_episode);

                // println!("{:?}", Self::get_name(&raw_bangumi_name));
                let bangumi_info = BangumiInfo {
                    title: bangumi_name,
                    season,
                    episode: ep,
                    group,
                };
                println!("{}", bangumi_info);
            } else {
                eprintln!("No match found!");
            }

            // *votes_group.entry(group).or_insert(0) += 1;
            println!();
        }

        // println!("Group: {}", Self::get_top_voted_item(&votes_group).unwrap());
    }

    pub fn gen_filename(&self, ext: &str) -> String {
        let group = match &self.group {
            Some(g) => Some(format!("- {}", g)),
            None => None,
        };

        format!(
            "{} - S{:02}E{:02} {}.{}",
            self.title.get_default_title(),
            self.season,
            self.episode,
            group.unwrap_or_default(),
            ext
        )
    }

    pub fn parse(raw_title: &str) -> Option<BangumiInfo> {
        // let part_re = Regex::new(r"(?P<season>.*|\[.*])(?P<episode> -? \d+|\[\d+]|\[\d+.?[vV]\d]|第\d+[话話集]|\[第?\d+[话話集]]|\[\d+.?END]|[Ee][Pp]?\d+)(?P<others>.*)").unwrap();
        let part_re = Regex::new(
                r"(?P<season>.*|\[.*])(?P<episode> -? \d+|\[\d+]|\[\d+.?[vV]\d]|第\d+[话話集]|\[第?\d+[话話集]]|\[\d+.?END]|[Ee][Pp]?\d+|\[?特[別别]篇\]?|\[?[總总]集篇\]?| \d+ )(?P<others>.*)"
            ).unwrap();

        println!("Raw title: {}", raw_title.red());
        let mut raw_title = Self::pre_process(&raw_title);

        let group = Self::pop_group(&mut raw_title);

        if let Some(caps) = part_re.captures(&raw_title) {
            let raw_season = caps.name("season").map_or("", |m| m.as_str()).trim();
            let raw_episode = caps.name("episode").map_or("", |m| m.as_str()).trim();
            // let raw_others = caps.name("others").map_or("", |m| m.as_str()).trim();

            let mut raw_bangumi_name = raw_season.to_owned();
            let season = Self::pop_season(&mut raw_bangumi_name);

            let bangumi_name = Self::get_name(&raw_bangumi_name);
            let ep = Self::get_episode(raw_episode);

            let bangumi_info = BangumiInfo {
                title: bangumi_name,
                season,
                episode: ep,
                group,
            };
            println!("{bangumi_info}\n");
            Some(bangumi_info)
        } else {
            None
        }
    }

    fn get_name(raw_name: &str) -> BangumiTitle {
        let main_split_re = Regex::new(r"\/|\s{2}|-\s{2}|\]\[").unwrap();
        // let main_split_re = Regex::new(r"\/|\s{2}|-\s{2}|[\[\]]").unwrap();
        // let mut tokens: Vec<&str> = Self::split_and_trim(&main_split_re, raw_name);
        let remove_surrounding_brackets_re = Regex::new(r"^\[|\]$").unwrap();
        let raw_name = remove_surrounding_brackets_re.replace_all(raw_name, "");
        let mut tokens: Vec<&str> = main_split_re.split(&raw_name).map(|s| s.trim()).collect();

        if tokens.len() == 1 {
            let underscore_re = Regex::new(r"_{1}").unwrap();
            let dash_re = Regex::new(r" - {1}").unwrap();
            tokens = match raw_name {
                _ if underscore_re.is_match(tokens[0]) => {
                    Self::split_and_trim(&underscore_re, tokens[0])
                }
                _ if dash_re.is_match(tokens[0]) => Self::split_and_trim(&dash_re, tokens[0]),
                _ => tokens,
            };
        }

        // let mut is_hard_split = false;
        // if tokens.len() == 1 {
        //     // try hard split
        //     tokens = raw_name
        //         .split(" ")
        //         .filter_map(|s| {
        //             let trimmed = s.trim();
        //             if trimmed.is_empty() {
        //                 None
        //             } else {
        //                 Some(trimmed)
        //             }
        //         })
        //         .collect();
        //     is_hard_split = true;
        // }

        let jp_re = Regex::new(r"[\u0800-\u4e00]{2,}").unwrap();
        let cn_re = Regex::new(r"[\u4e00-\u9fa5]{2,}").unwrap();
        let en_re = Regex::new(r"[a-zA-Z]{3,}").unwrap();

        let mut list_jp = Vec::new();
        let mut list_cn = Vec::new();
        let mut list_en = Vec::new();

        // let mut last_lang = LANG::CN;

        for token in tokens {
            let last_lang = match token {
                _ if jp_re.is_match(token) => LANG::JP,
                _ if cn_re.is_match(token) => LANG::CN,
                _ if en_re.is_match(token) => LANG::EN,
                _ => {
                    continue;
                    // if !is_hard_split {
                    //     continue;
                    // }
                    // last_lang
                }
            };

            match last_lang {
                LANG::CN => &mut list_cn,
                LANG::EN => &mut list_en,
                LANG::JP => &mut list_jp,
            }
            .push(token);
        }

        BangumiTitle {
            cn: Self::join_and_clean(list_cn),
            en: Self::join_and_clean(list_en),
            jp: Self::join_and_clean(list_jp),
        }
    }

    fn get_episode(raw_ep: &str) -> u32 {
        let ep_re = Regex::new(r"(\d+)").unwrap();
        if let Some(cap) = ep_re.captures(raw_ep) {
            if let Ok(ep) = cap[1].parse::<u32>() {
                return ep;
            }
        }
        0
    }

    fn pop_season(raw_bangumi_name: &mut String) -> u32 {
        let cn_num_map = [
            ("一", 1),
            ("二", 2),
            ("三", 3),
            ("四", 4),
            ("五", 5),
            ("六", 6),
            ("七", 7),
            ("八", 8),
            ("九", 9),
            ("十", 10),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();

        let mut name_season = raw_bangumi_name.to_owned();

        let season_re = Regex::new(
            r"(\d{1,2}(?:st|nd|rd|th) Season)|(Season \d{1,2})|(S\d{1,2})|[第].[季期]|\d[季期]",
        )
        .unwrap();
        name_season = name_season.replace("[", " ").replace("]", " ");

        let season_tokens: Vec<&str> = season_re
            .find_iter(&name_season)
            .map(|m| m.as_str())
            .collect();

        if season_tokens.is_empty() {
            return 1;
        }

        raw_bangumi_name.clear();
        raw_bangumi_name.push_str(&season_re.replace_all(&name_season, "").trim());

        let season_en_re = Regex::new(r"Season|S").unwrap();
        let season_en_replace_re = Regex::new(r"st|nd|rd|th|Season|S").unwrap();
        let season_cn_re = Regex::new(r"[第 ].*[季期(部分)]|部分").unwrap();
        let season_cn_replace_re = Regex::new(r"[第季期部分 ]").unwrap();

        let mut season_number = 1;

        for token in season_tokens {
            if season_en_re.is_match(token) {
                let num_str = season_en_replace_re
                    .replace_all(token, "")
                    .trim()
                    .to_owned();
                season_number = num_str.parse().unwrap_or(1);
                break;
            } else if season_cn_re.is_match(token) {
                let num_str = season_cn_replace_re
                    .replace_all(token, "")
                    .trim()
                    .to_owned();
                season_number = num_str
                    .parse()
                    .unwrap_or_else(|_| *cn_num_map.get(&num_str[..]).unwrap_or(&1));
                break;
            }
        }

        season_number
    }

    fn pop_group(raw_title: &mut String) -> Option<String> {
        let group_re = Regex::new(r"\[([^\]]+)\]").unwrap();

        if let Some(caps) = group_re.captures(raw_title) {
            let group = caps[1].to_owned();

            // remove group from the raw title
            let updated_title = Regex::new(&format!(".{}.", group))
                .unwrap()
                .replace_all(&raw_title, "")
                .as_ref()
                .to_owned();
            raw_title.clear();
            raw_title.push_str(&updated_title);

            Some(group)
        } else {
            None
        }
    }

    pub fn pre_process(raw_title: &str) -> String {
        let left_re = Regex::new(r"\s*[【（「{]\s*").unwrap();
        let right_re = Regex::new(r"\s*[】）」}]\s*").unwrap();
        let processed = left_re.replace_all(&raw_title.trim(), " [");
        let processed = right_re.replace_all(&processed, "] ");

        let special_re = Regex::new(r"[^\w\s\u4e00-\u9fff\u3040-\u309f\u30a0-\u30ff-]").unwrap();
        let binding = special_re.replace_all(&processed, "/");
        let mut token_group: Vec<&str> = binding.split('/').collect();
        token_group.retain(|s| !s.is_empty());
        if token_group.len() == 1 {
            token_group = token_group[0].split_whitespace().collect();
        }

        let mut processed_title = processed.as_ref().to_owned();
        for token in token_group {
            if Regex::new(r"新番|月?番").unwrap().is_match(token) && token.chars().count() <= 5
            {
                let sub_re = Regex::new(&format!("([^\\]]?){}([^\\[]?)", token)).unwrap();
                processed_title = sub_re.replace_all(&processed_title, "").trim().to_owned();
            } else if token.contains("港澳台") {
                let sub_re = Regex::new(&format!(".{}.", token)).unwrap();
                processed_title = sub_re.replace_all(&processed_title, "").trim().to_owned();
            }
        }
        processed_title
    }

    fn _get_top_voted_item(map: &HashMap<String, i32>) -> Option<String> {
        if let Some((key_ref, _)) = map.iter().max_by(|&(_, a), &(_, b)| a.cmp(b)) {
            Some(key_ref.to_owned())
        } else {
            None
        }
    }

    fn join_and_clean(list: Vec<&str>) -> Option<String> {
        if list.is_empty() {
            None
        } else {
            let mut name = list.join(" ").trim().to_owned();
            // ugly workaround...
            if name.ends_with(" -") {
                name = name[..name.len() - 2].to_owned();
            }
            Some(name)
        }
    }

    fn split_and_trim<'a>(re: &Regex, s: &'a str) -> Vec<&'a str> {
        re.split(s)
            .filter_map(|token| {
                let trimmed = token.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .collect()
    }
}
