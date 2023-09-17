struct BangumiTitle {
    cn: String,
    en: String,
    jp: String
}

struct BangumiRss {
    title: BangumiTitle,
    link: String,
}

impl BangumiRss {
    fn parse_title_name(raw_title: &str) -> String {
        raw_title.to_string()
    }
}
