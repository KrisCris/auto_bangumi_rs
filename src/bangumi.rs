use colored::Colorize;
use core::fmt;

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
            "{} {} {}",
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

pub struct Bangumi {
    pub title: BangumiTitle,
    pub season: u32,
    pub episode: u32,
    pub group: Option<String>,
}

impl fmt::Display for Bangumi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sp = format!("S{:02}E{:02}", &self.season, &self.episode);
        let group = match &self.group {
            Some(g) => Some(format!("- {}", g)),
            None => None,
        };
        write!(
            f,
            "{} - {} {}",
            &self.title,
            sp.yellow(),
            group.unwrap_or_default().bold()
        )
    }
}

impl Bangumi {
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
}