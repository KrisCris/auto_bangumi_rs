use auto_bangumi_rs::parser::Parser;
use regex::Regex;
use rss::Channel;

#[test]
fn test_parser() {
    for title in get_titles() {
        let parsed = Parser::new(title.0.to_owned()).and_then(|parser| parser.to_bangumi(None));
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        println!("{}", parsed);
        assert_eq!(title.2, parsed.season);
        assert_eq!(title.3, parsed.episode);
        assert_eq!(title.4, parsed.group);
        assert_eq!(title.1, parsed.title.get_default_title());
    }
}

#[tokio::test]
async fn test_mikan() {
    for url in get_rss_links() {
        test_url(&url).await;
    }
}

#[tokio::test]
async fn test_group_ani() {
    test_url("https://share.dmhy.org/topics/rss/team_id/816/rss.xml").await;
    test_url("https://bangumi.moe/rss/tags/6039092432f14c00074809b9").await;
    test_url("https://mikanani.me/RSS/Search?searchstr=ANi&subgroupid=583").await;
}

async fn test_url(url: &str) {
    let res = reqwest::get(url).await.unwrap();
    let status = res.status();
    if status.is_success() {
        let bytes = &res.bytes().await.unwrap()[..];
        let channel = Channel::read_from(bytes).unwrap();
        // let link = channel.link.to_owned();
        let filter_re =
            Regex::new(r"\d{1,2}-\d{1,2}|.*Dasu - None.*|铃芽之旅|Suzume|2\d{3}|Anne Frank")
                .unwrap();
        for item in channel.items {
            if let Some(raw_title) = item.title {
                if filter_re.is_match(&raw_title) {
                    println!("[Info] Ignore feed: {}", raw_title);
                    continue;
                }
                println!("- {}", raw_title);
                let result = Parser::new(raw_title).and_then(|p| p.title());
                println!("{}", result.as_ref().unwrap());
                assert!(result.is_some());
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

#[test]
fn test_parser_season() {
    for title in get_titles() {
        let parser = Parser::new(title.0.to_owned());
        println!("- {}", title.0);
        assert_eq!(true, parser.is_some_and(|p| p.season() == title.2));
    }
}

#[test]
fn test_parser_episode() {
    for title in get_titles() {
        let parser = Parser::new(title.0.to_owned()).unwrap();
        println!("- {}", title.0);
        assert!(parser.episode().is_some());
        assert_eq!(title.3, parser.episode().unwrap());
    }
}

#[test]
fn test_parser_title() {
    for title in get_titles() {
        let parser = Parser::new(title.0.to_owned()).unwrap();
        println!("- {}", title.0);
        assert!(parser.title().is_some());
        let titles = parser.title().unwrap();
        println!("{}", titles);
        assert_eq!(title.1, titles.get_default_title());
    }
}

#[test]
fn test_parser_group() {
    for title in get_titles() {
        let parser = Parser::new(title.0.to_owned()).unwrap();
        println!("- {}", title.0);
        let group = parser.group();
        assert!(group.is_some());
        println!("- {}", group.unwrap());
        assert_eq!(title.4, group.unwrap());
    }
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
