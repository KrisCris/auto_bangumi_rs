use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::Range, path::PathBuf};

use crate::bangumi::{Bangumi, BangumiTitle};

lazy_static! {
    static ref RE_GROUP: Regex = Regex::new(r"\[([^\]]+)\]").unwrap();
    static ref RE_MAIN_SPLIT: Regex = Regex::new(r"(?:\[([^\]]+)\])?(?P<season>.*|\[.*])(?P<episode>\(\d{1,3}\)| -? \d+|\[\d+]|\[\d+.?[vV]\d]|第\d+[话話集]|\[第?\d+[话話集]]|\[\d+.?END]|[Ee][Pp]?\d+|\[?特[別别]篇\]?|\[?[總总]集篇\]?| \d+ |\d{1,4}-\d{1,4}|合集)(?P<others>.*)").unwrap();
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
    static ref RE_EXT: Regex = Regex::new(r"(?P<ext>\.\w+)$").unwrap();
    static ref RE_FORMATTED: Regex = Regex::new(r"(.*) - (S\d{2}E\d{2}) - (.*?)(\.\w+)?$").unwrap();
}
pub struct Parser {
    raw: String,
    raw_season: Option<Range<usize>>,
    raw_episode: Option<Range<usize>>,
    raw_others: Option<Range<usize>>,
}

impl Parser {
    pub fn new(raw_title: String) -> Option<Self> {
        match RE_FORMATTED.captures(&raw_title) {
            Some(_) => {
                return None
            },
            None => {}
        }

        // println!("- Raw Title: {}", raw_title);
        // this looks bad but idk if there is a better way...
        let processed = RE_LEFT_BRACKETS.replace_all(&raw_title.trim(), " [");
        let processed = RE_RIGHT_BRACKETS.replace_all(&processed, "] ");
        let processed = processed.trim();

        let binding = RE_SPECIAL.replace_all(&processed, "/");
        let mut token_group: Vec<&str> = binding.split('/').collect();
        token_group.retain(|s| !s.is_empty());
        if token_group.len() == 1 {
            token_group = token_group[0].split_whitespace().collect();
        }

        let mut raw = processed.to_owned();
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

        Some(Parser {
            raw,
            raw_season,
            raw_episode,
            raw_others,
        })
    }

    pub fn from_path(path: &PathBuf) -> Option<Self> {
        if !path.is_file() {
            return None;
        }
        if let Some(filename) = path.file_name() {
            let name = filename.to_string_lossy().as_ref().to_owned();
            return Self::new(name);
        }
        None
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
                        _ if RE_DASH.is_match(tokens[0]) => split_and_trim(&RE_DASH, tokens[0]),
                        _ => tokens,
                    };
                }

                let mut list_jp = Vec::new();
                let mut list_cn = Vec::new();
                let mut list_en = Vec::new();

                for token in tokens {
                    match token {
                        // Do not change the order!
                        _ if RE_JP.is_match(token) => &mut list_jp,
                        _ if RE_CN.is_match(token) => &mut list_cn,
                        _ if RE_EN.is_match(token) => &mut list_en,
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
                            match cn_to_digit(cn_num) {
                                Some(num) => return num,
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

    pub fn extension(&self) -> Option<String> {
        match &self.raw_others {
            Some(range) => {
                if let Some(m) = RE_EXT
                    .captures(&self.raw[range.to_owned()])
                    .and_then(|cap| cap.name("ext"))
                {
                    Some(m.as_str().to_owned())
                } else {
                    None
                }
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

    pub fn to_bangumi(self, season: Option<u32>) -> Option<Bangumi> {
        match self.can_parse() {
            true => {
                let group = self.group().unwrap_or("Unknown").to_owned();
                let Some(title) = self.title() else {
                    return None;
                };
                let season = match season {
                    Some(s) => s,
                    None => self.season()
                };
                // let season = self.season();
                let episode = self.episode().unwrap_or(0);
                Some(Bangumi::new(
                    title,
                    season,
                    episode,
                    group,
                    self.extension().to_owned(),
                ))
            }
            false => None,
        }
    }
}

fn cn_to_digit(cn_digit: &str) -> Option<u32> {
    match cn_digit {
        "一" => Some(1),
        "二" => Some(2),
        "三" => Some(3),
        "四" => Some(4),
        "五" => Some(5),
        "六" => Some(6),
        "七" => Some(7),
        "八" => Some(8),
        "九" => Some(9),
        "十" => Some(10),
        _ => None,
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

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn test_name() {
        let p = Parser::new("【喵萌奶茶屋】★04月新番★[百合是我的工作！/我的百合乃工作是也！/私の百合はお仕事です！/Watashi no Yuri wa Oshigoto desu!][03][1080p][简日双语][招募翻译校对]".to_owned());
        let b = p.unwrap().to_bangumi(None);
        println!("{}", b.unwrap());
    }

    #[test]
    fn test_formatted_name() {
        let p = Parser::new("无职转生，到了异世-界就拿出真本事 第2季 - S02E00 - Skymoon-Raws.mkv".to_owned());
        match p {
            Some(_) => assert!(false),
            None => {}
        }
    }
}
