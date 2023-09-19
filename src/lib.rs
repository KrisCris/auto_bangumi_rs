use core::fmt;
use std::collections::HashMap;

use colored::Colorize;
use regex::Regex;

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
    title: BangumiTitle,
    season: u32,
    episode: u32,
    group: Option<String>,
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

#[cfg(test)]
mod test {
    use super::BangumiInfo;
    use regex::Regex;
    use rss::Channel;

    #[test]
    fn test_parser() {
        for title in get_titles() {
            let parsed = BangumiInfo::parse(title.0);
            assert!(parsed.is_some());
            let parsed = parsed.unwrap();
            assert_eq!(title.2, parsed.season);
            assert_eq!(title.3, parsed.episode);
            assert_eq!(title.4, parsed.group.unwrap());
            assert_eq!(title.1, parsed.title.get_default_title());
        }
    }

    #[tokio::test]
    async fn massive_test() {
        for url in get_rss_links() {
            let res = reqwest::get(url).await.unwrap();
            let status = res.status();
            if status.is_success() {
                let bytes = &res.bytes().await.unwrap()[..];
                let channel = Channel::read_from(bytes).unwrap();
                // let link = channel.link.to_owned();
                let filter_re = Regex::new(r"\d{1,2}-\d{1,2}|.*Dasu - None.*").unwrap();
                for item in channel.items() {
                    if let Some(raw_title) = item.title() {
                        if filter_re.is_match(raw_title) {
                            println!("[Info] Ignore feed: {}", raw_title);
                            continue;
                        }
                        let result = BangumiInfo::parse(raw_title);
                        assert!(result.is_some());
                    }
                }
            }
        }
    }

    fn get_rss_links() -> Vec<String> {
        let group_ids = [
            12, 19, 21, 203, 213, 370, 382, 477, 534, 552, 574, 576, 583, 615,
        ];
        let bangumi_ids = [3070, 3099, 3060, 3093, 2930, 3087, 3094, 3089];
        let mut links = Vec::new();
        for bid in bangumi_ids {
            for gid in group_ids {
                links.push(format!(
                    "https://mikanani.me/RSS/Bangumi?bangumiId={}&subgroupid={}",
                    bid, gid
                ))
            }
        }
        links
    }

    fn get_titles() -> Vec<(&'static str, &'static str, u32, u32, &'static str)> {
        vec![
                (
                    "[ANi] 卡片战斗!! 先导者 will+Dress 第三季 - 09 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4].mkv", 
                    "卡片战斗!! 先导者 will+Dress", 3, 9, "ANi"
                ),
                (
                    "[GJ.Y] 卡片战斗!! 先导者 will+Dress 第三季 / Cardfight!! Vanguard: will+Dress Season 3 - 35 (Sentai 1920x1080 AVC AAC MKV).mp4", 
                    "卡片战斗!! 先导者 will+Dress", 3, 35, "GJ.Y"
                ),
                (
                    "[动漫国字幕组&LoliHouse] 打工吧!! 魔王大人 / Hataraku Maou-sama S2 - 19 [WebRip 1080p HEVC-10bit AAC][简繁外挂字幕]",
                    "打工吧!! 魔王大人", 2, 19, "动漫国字幕组&LoliHouse"
                ),
                (
                    "[Skymoon-Raws] 打工吧，魔王大人！第二季 / Hataraku Maou-sama! S02 - 21 [ViuTV][WEB-RIP][1080p][AVC AAC][CHT][SRT][MKV](先行版本) IPFS服务器种",
                    "打工吧，魔王大人！", 2, 21, "Skymoon-Raws",
                ),
                (
                    "[Lilith-Raws] 打工吧，魔王大人！ / Hataraku Maou-sama! S02 - 20 [Baha][WEB-DL][1080p][AVC AAC][CHT][MP4]",
                    "打工吧，魔王大人！", 2, 20, "Lilith-Raws"
                ),
                (
                    "[GJ.Y] 打工吧，魔王大人！第二季 / Hataraku Maou-sama!! - 21 (CR 1920x1080 AVC AAC MKV)",
                    "打工吧，魔王大人！", 2, 21, "GJ.Y"
                ),
                (
                    "[ANi] Andeddo Gaaru Maadaafarusu - 不死少女的谋杀闹剧 - 09 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]",
                    "不死少女的谋杀闹剧", 1, 9, "ANi",
                ),
                (
                    "[Lilith-Raws] 不死少女的谋杀闹剧 / Undead Girl Murder Farce - 09 [Baha][WebDL 1080p AVC AAC][CHT]",
                    "不死少女的谋杀闹剧", 1, 9, "Lilith-Raws",
                ),
                (
                    "[喵萌奶茶屋&LoliHouse] 不死少女・杀人笑剧 / Undead Girl Murder Farce - 07 [WebRip 1080p HEVC-10bit AAC][简繁日内封字幕]",
                    "不死少女・杀人笑剧", 1, 7, "喵萌奶茶屋&LoliHouse",
                ),
                (
                    "[Lilith-Raws] 神剑闯江湖 ―明治剑客浪漫谭― (2023) / Rurouni Kenshin：Meiji Kenkaku Romantan (2023) - 10 [Baha][WebDL 1080p AVC AAC][CHT]",
                    "神剑闯江湖 ―明治剑客浪漫谭― (2023)", 1, 10, "Lilith-Raws",
                ),
                (
                    "[LoliHouse] 浪客剑心 -明治剑客浪漫谭- / Rurouni Kenshin (2023) - 07 [WebRip 1080p HEVC-10bit AAC][简繁日内封字幕]",
                    "浪客剑心 -明治剑客浪漫谭-", 1, 7, "LoliHouse",
                ),
                (
                    "[GJ.Y] 神剑闯江湖 ―明治剑客浪漫谭― / Rurouni Kenshin: Meiji Kenkaku Romantan (2023) - 08 (CR 1920x1080 AVC AAC MKV)",
                    "神剑闯江湖 ―明治剑客浪漫谭―", 1, 8, "GJ.Y",
                ),
                (
                    "[ANi] Rurouni Kenshin - 神剑闯江湖 ―明治剑客浪漫谭― - 07 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]",
                    "神剑闯江湖 ―明治剑客浪漫谭―", 1, 7, "ANi",
                ),
                (
                    "[SweetSub&LoliHouse] 堀与宫村 -piece- / Horimiya - piece - 10 [WebRip 1080p HEVC-10bit AAC][简繁日内封字幕]",
                    "堀与宫村 -piece-", 1, 10, "SweetSub&LoliHouse",
                ),
                (
                    "【动漫国字幕组】★07月新番[堀与宫村 -piece-][11][720P][繁体][MP4]",
                    "堀与宫村 -piece-", 1, 11, "动漫国字幕组",
                ),
                (
                    "[Lilith-Raws] 堀与宫村 -piece- / Horimiya：Piece - 08 [Baha][WebDL 1080p AVC AAC][CHT]",
                    "堀与宫村 -piece-", 1, 8, "Lilith-Raws",
                ),
                (
                    "[ANi] Horimiya The Missing Pieces - 堀与宫村 -piece- - 07 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]",
                    "堀与宫村 -piece-", 1, 7, "ANi"
                ),
                (
                    "[GJ.Y] 堀与宫村 第二季 / Horimiya: Piece - 11 (B-Global 1920x1080 HEVC AAC MKV)",
                    "堀与宫村", 2, 11, "GJ.Y",
                ),
                (
                    "[桜都字幕组] 堀与宫村 -piece- / Horimiya Piece [10][1080p][简繁内封]",
                    "堀与宫村 -piece-", 1, 10, "桜都字幕组",
                ),
                (
                    "[LoliHouse] AYAKA ‐绫岛奇谭‐ - 12 [WebRip 1080p HEVC-10bit AAC][简繁内封字幕][END]",
                    "AYAKA ‐绫岛奇谭‐", 1, 12, "LoliHouse",
                ),
                (
                    "[ANi] 僵尸 100～在成为僵尸前要做的 100 件事～ - 06 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]",
                    "僵尸 100～在成为僵尸前要做的 100 件事～", 1, 6, "ANi",
                ),
                (
                    "[GJ.Y] 僵尸百分百～变成僵尸之前想做的100件事～ / Zom 100 - 07 (B-Global 3840x2160 HEVC AAC MKV)",
                    "僵尸百分百～变成僵尸之前想做的100件事～", 1, 7, "GJ.Y",
                ),
                // (
                //     "[猎户不鸽压制] 僵尸百分百～在成为僵尸前要做的100件事～ Zom 100: Zombie ni Naru made ni Shitai 100 no Koto [05] [1080p] [简日内嵌] [2023年7月番]",
                //     "僵尸百分百～在成为僵尸前要做的100件事～", 1, 5, "猎户不鸽压制",
                // ),
                (
                    "[漫猫字幕社][7月新番][僵尸百分百～变成僵尸之前想做的100件事][Zom 100 - Zombie ni Naru made ni Shitai 100 no Koto][05][1080P][MP4][繁日双语]",
                    "僵尸百分百～变成僵尸之前想做的100件事", 1, 5, "漫猫字幕社",
                ),
                (
                    "[喵萌奶茶屋&LoliHouse] 僵尸100 ~变成僵尸前想要完成的100件事~ / Zom 100: Zombie ni Naru made ni Shitai 100 no Koto - 02 [WebRip 1080p HEVC-10bit AAC][简繁日内封字幕]",
                    "僵尸100 ~变成僵尸前想要完成的100件事~", 1, 2, "喵萌奶茶屋&LoliHouse",
                ),
                (
                    "【喵萌奶茶屋】★07月新番★[僵尸100 ~变成僵尸前想要完成的100件事~ / Zom 100: Zombie ni Naru made ni Shitai 100 no Koto][04][1080p][繁日双语][招募翻译]",
                    "僵尸100 ~变成僵尸前想要完成的100件事~", 1, 4, "喵萌奶茶屋",
                ),
                (
                    "[Lilith-Raws] 僵尸 100～在成为僵尸前要做的 100 件事～ / Zom 100 - 05 [Baha][WebDL 1080p AVC AAC][CHT]",
                    "僵尸 100～在成为僵尸前要做的 100 件事～", 1, 5, "Lilith-Raws",
                ),
                (
                    "[Skymoon-Raws] 无职转生，到了异世界就拿出真本事 第2季 - 11 [ViuTV][WEB-RIP][1080p][AVC AAC][CHT][SRT][MKV](先行版本) IPFS服务器种",
                    "无职转生，到了异世界就拿出真本事", 2, 11, "Skymoon-Raws",
                ),
                (
                    "[Skymoon-Raws] 无职转生，到了异世界就拿出真本事 第2季 / Mushoku Tensei 2nd Season - 00 [ViuTV][WEB-RIP][1080p][AVC AAC][CHT][SRT][MKV](先行版本)",
                    "无职转生，到了异世界就拿出真本事", 2, 0, "Skymoon-Raws",
                ),
                (
                    "[Lilith-Raws] 无职转生～到了异世界就拿出真本事 / Mushoku Tensei S02 - 11 [Baha][WebDL 1080p AVC AAC][CHT]",
                    "无职转生～到了异世界就拿出真本事", 2, 11, "Lilith-Raws",
                ),
                (
                    "[GJ.Y] 无职转生～到了异世界就拿出真本事 第二季 / Mushoku Tensei II: Isekai Ittara Honki Dasu - 10 (CR 1920x1080 AVC AAC MKV)",
                    "无职转生～到了异世界就拿出真本事", 2, 10, "GJ.Y",
                ),
                (
                    "[ANi] 无职转生～到了异世界就拿出真本事 第二季 - 08 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]",
                    "无职转生～到了异世界就拿出真本事", 2, 8, "ANi",
                ),
                (
                    "[ANi] 无职转生～到了异世界就拿出真本事 第二季 - 特别篇 [1080P][Baha][WEB-DL][AAC AVC][CHT][MP4]",
                    "无职转生～到了异世界就拿出真本事", 2, 0, "ANi",
                ),
                (
                    "【喵萌奶茶屋】★07月新番★[无职转生 2期 / Mushoku Tensei S2][00][1080p][简日双语][招募翻译]",
                    "无职转生", 2, 0, "喵萌奶茶屋",
                ),
                (
                    "[喵萌奶茶屋&LoliHouse] SYNDUALITY Noir - 09 [WebRip 1080p HEVC-10bit AAC][简繁日内封字幕]",
                    "SYNDUALITY Noir", 1, 9, "喵萌奶茶屋&LoliHouse",
                ),
                (
                    "[豌豆字幕组&风之圣殿字幕组&LoliHouse] 死神 千年血战篇 / BLEACH Sennen Kessen-hen - 21 [WebRip 1080p HEVC-10bit AAC][简繁外挂字幕]",
                    "死神 千年血战篇", 1, 21, "豌豆字幕组&风之圣殿字幕组&LoliHouse",
                ),
                (
                    "【悠哈璃羽字幕社】[死神千年血战诀别谭_Bleach - Thousand-Year Blood War][22][1080p][CHS]",
                    "死神千年血战诀别谭", 1, 22, "悠哈璃羽字幕社",
                ),
                (
                    "[TEST] 僵尸百分百～变成僵尸之前想做的100件事 S01E02",
                    "僵尸百分百～变成僵尸之前想做的100件事", 1, 2, "TEST",
                ),
            ]
    }
}
