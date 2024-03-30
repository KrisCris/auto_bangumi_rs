use colored::Colorize;
use core::fmt;
use std::path::PathBuf;

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
            self.get_title(LANG::CN).bright_green(),
            self.get_title(LANG::EN).bright_blue(),
            self.get_title(LANG::JP).bright_yellow(),
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
            .unwrap_or("Unknown")
    }
}

pub struct Bangumi {
    pub title: BangumiTitle,
    pub season: u32,
    pub episode: u32,
    pub group: String,
    pub extension: Option<String>,
}

impl fmt::Display for Bangumi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sp = format!("S{:02}E{:02}", &self.season, &self.episode);
        let group = match self.group.len() > 0 {
            true => format!("{}", self.group),
            false => String::from("Unknown"),
        };
        write!(
            f,
            "{} - {} - {}",
            &self.title,
            sp.bright_red(),
            group.bright_cyan()
        )
    }
}

impl Bangumi {
    pub fn new(
        title: BangumiTitle,
        season: u32,
        episode: u32,
        group: String,
        extension: Option<String>,
    ) -> Self {
        Bangumi {
            title,
            season,
            episode,
            group,
            extension,
        }
    }
    pub fn gen_filename(&self) -> String {
        let group = match self.group.len() > 0 {
            true => format!("{}", self.group),
            false => String::from("Unknown"),
        };

        let ext = match &self.extension {
            Some(p) => p,
            None => ""
        };

        format!(
            "{} - S{:02}E{:02} - {}{}",
            self.title.get_default_title(),
            self.season,
            self.episode,
            group,
            ext
        )
    }

    pub fn gen_fullpath(&self, dest: &PathBuf, group: bool) -> PathBuf {
        if group {
            dest.join(self.title.get_default_title()).join(format!("Season {:02}", self.season)).join(self.gen_filename())
        } else {
            dest.join(self.gen_filename())
        }
    }
}
