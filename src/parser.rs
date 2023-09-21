use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

use crate::bangumi::{BangumiTitle, Bangumi};

lazy_static! {
    static ref CN_NUM: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("一", 1);
        m.insert("二", 2);
        m.insert("三", 3);
        m.insert("四", 4);
        m.insert("五", 5);
        m.insert("六", 6);
        m.insert("七", 7);
        m.insert("八", 8);
        m.insert("九", 9);
        m.insert("十", 10);
        m
    };

    static ref RE_GROUP: Regex = Regex::new(r"\[([^\]]+)\]").unwrap();
    static ref RE_MAIN_SPLIT: Regex = Regex::new(r"\[([^\]]+)\](?P<season>.*|\[.*])(?P<episode>\(\d{1,3}\)| -? \d+|\[\d+]|\[\d+.?[vV]\d]|第\d+[话話集]|\[第?\d+[话話集]]|\[\d+.?END]|[Ee][Pp]?\d+|\[?特[別别]篇\]?|\[?[總总]集篇\]?| \d+ )(?P<others>.*)").unwrap();
    static ref RE_EPISODE: Regex = Regex::new(r"(\d+)").unwrap();
    static ref RE_SEASON: Regex = Regex::new(r"(\d{1,2}(?:st|nd|rd|th) Season)|(Season \d{1,2})|(S\d{1,2})|[第].[季期]|\d[季期]").unwrap();
    static ref RE_SEASON_EN: Regex = Regex::new(r"Season|S").unwrap();
    static ref RE_SEASON_DIGIT: Regex = Regex::new(r"(\d{1,})").unwrap();
    static ref RE_SEASON_CN: Regex = Regex::new(r"[第 ].*[季期(部分)]|部分").unwrap();
    static ref RE_SEASON_CN_DIGIT: Regex = Regex::new(r"([一二三四五六七八九十]{1})").unwrap();
    static ref RE_LEFT_BRACKETS: Regex = Regex::new(r"\s*[【（「{]\s*").unwrap();
    static ref RE_RIGHT_BRACKETS: Regex = Regex::new(r"\s*[】）」}]\s*").unwrap();
    static ref RE_SPECIAL: Regex = Regex::new(r"[^\w\s\u4e00-\u9fff\u3040-\u309f\u30a0-\u30ff-]").unwrap();
    static ref RE_BANGUMI_CHARS: Regex = Regex::new(r"新番|月?番").unwrap();
    static ref RE_HKTW: Regex = Regex::new(r"港澳台|(?:仅[限僅]?)?(?:台[湾灣])|港澳").unwrap();
    static ref RE_TITLE_SPLIT: Regex = Regex::new(r"\/|\s{2}|-\s{2}|\]\[").unwrap();
    static ref RE_SIDE_EMPTY_BRACKETS: Regex = Regex::new(r"^\[|\]$|\[\]").unwrap();
    static ref RE_UNDERSCORE: Regex = Regex::new(r"_{1}").unwrap();
    static ref RE_DASH: Regex = Regex::new(r" - {1}").unwrap();
    static ref RE_JP: Regex = Regex::new(r"[\u0800-\u4e00]{2,}").unwrap();
    static ref RE_CN: Regex = Regex::new(r"[\u4e00-\u9fa5]{2,}").unwrap();
    static ref RE_EN: Regex = Regex::new(r"[a-zA-Z]{3,}").unwrap();
}
pub struct Parser {
    raw: String,
    raw_season: Option<Range<usize>>,
    raw_episode: Option<Range<usize>>,
    _raw_others: Option<Range<usize>>,
}

impl Parser {
    pub fn new(raw_title: String) -> Self {
        // this looks bad but idk if there is a better way...
        let processed = RE_LEFT_BRACKETS.replace_all(&raw_title.trim(), " [");
        let processed = RE_RIGHT_BRACKETS.replace_all(&processed, "] ");

        let binding = RE_SPECIAL.replace_all(&processed.trim(), "/");
        let mut token_group: Vec<&str> = binding.split('/').collect();
        token_group.retain(|s| !s.is_empty());
        if token_group.len() == 1 {
            token_group = token_group[0].split_whitespace().collect();
        }

        let mut raw = processed.as_ref().to_owned();
        for token in token_group {
            if RE_BANGUMI_CHARS.is_match(token) && token.chars().count() <= 5 {
                let sub_re = Regex::new(&format!("([^\\]]?){}([^\\[]?)", token)).unwrap();
                raw = sub_re.replace_all(&raw, "").trim().to_owned();
            } else if RE_HKTW.is_match(token) {
                let sub_re = Regex::new(&format!(".{}.", token)).unwrap();
                raw = sub_re.replace_all(&raw, "").trim().to_owned();
            }
        }

        let mut raw_season = None;
        let mut raw_episode = None;
        let mut raw_others = None;
        if let Some(caps) = RE_MAIN_SPLIT.captures(&raw) {
            if let Some(raw) = caps.name("season") {
                raw_season = Some(raw.range());
            }
            if let Some(raw) = caps.name("episode") {
                raw_episode = Some(raw.range());
            }
            if let Some(raw) = caps.name("others") {
                raw_others = Some(raw.range());
            }
        } else {
        }

        Parser {
            raw,
            raw_season,
            raw_episode,
            _raw_others: raw_others,
        }
    }

    pub fn group(&self) -> Option<&str> {
        if let Some(caps) = RE_GROUP.captures(&self.raw) {
            if let Some(m) = caps.get(1) {
                return Some(m.as_str());
            }
        }
        None
    }

    pub fn title(&self) -> Option<BangumiTitle> {
        match &self.raw_season {
            Some(range) => {
                let raw_title = RE_SEASON.replace_all(&self.raw[range.to_owned()], "");
                let raw_title = RE_SIDE_EMPTY_BRACKETS.replace_all(raw_title.trim(), "");
                let raw_title = raw_title.trim();

                let mut tokens: Vec<&str> =
                    RE_TITLE_SPLIT.split(&raw_title).map(|s| s.trim()).collect();

                if tokens.len() == 1 {
                    tokens = match raw_title {
                        _ if RE_UNDERSCORE.is_match(tokens[0]) => {
                            split_and_trim(&RE_UNDERSCORE, tokens[0])
                        }
                        _ if RE_DASH.is_match(tokens[0]) => {
                            split_and_trim(&RE_DASH, tokens[0])
                        }
                        _ => tokens,
                    };
                }

                let mut list_jp = Vec::new();
                let mut list_cn = Vec::new();
                let mut list_en = Vec::new();

                for token in tokens {
                    match token {
                        _ if RE_CN.is_match(token) => &mut list_cn,
                        _ if RE_EN.is_match(token) => &mut list_en,
                        _ if RE_JP.is_match(token) => &mut list_jp,
                        _ => {
                            continue;
                        }
                    }
                    .push(token);
                }

                return Some(BangumiTitle::new(
                    join_and_clean(list_cn),
                    join_and_clean(list_en),
                    join_and_clean(list_jp),
                ));
            }
            None => None,
        }
    }

    pub fn season(&self) -> u32 {
        match &self.raw_season {
            Some(range) => {
                let season_tokens: Vec<&str> = RE_SEASON
                    .find_iter(&self.raw[range.to_owned()])
                    .map(|m| m.as_str())
                    .collect();

                if season_tokens.is_empty() {
                    return 1;
                }

                for token in season_tokens {
                    if let Some(m) = RE_SEASON_DIGIT.captures(token).and_then(|c| c.get(1)) {
                        let str_num = m.as_str();
                        match str_num.parse() {
                            Ok(num) => return num,
                            Err(_) => continue,
                        }
                    } else {
                        if let Some(m) = RE_SEASON_CN_DIGIT.captures(token).and_then(|c| c.get(1)) {
                            let cn_num = m.as_str();
                            match CN_NUM.get(cn_num) {
                                Some(&num) => return num,
                                None => continue,
                            }
                        }
                    }
                }
                1
            }
            None => 1,
        }
    }

    pub fn episode(&self) -> Option<u32> {
        match &self.raw_episode {
            Some(range) => {
                if let Some(cap) = RE_EPISODE.captures(&self.raw[range.to_owned()]) {
                    if let Ok(ep) = cap[1].parse::<u32>() {
                        return Some(ep);
                    }
                }
                return Some(0);
            }
            None => None,
        }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn can_parse(&self) -> bool {
        match self.raw_episode {
            Some(_) => true,
            None => false,
        }
    }

    pub fn to_bangumi(self) -> Option<Bangumi> {
        match self.can_parse() {
            true => {
                let group = self.group().unwrap_or("").to_owned();
                let Some(title) = self.title() else {
                    return None;
                };
                let season = self.season();
                let episode = self.episode().unwrap_or(0);
                Some(Bangumi { title, season, episode, group })
            },
            false => None
        }
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
