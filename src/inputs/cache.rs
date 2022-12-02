use std::io::{self, ErrorKind, Seek, Write};
use std::fs::File;
use std::path::{PathBuf, Path};
use reqwest::blocking::{Client, Response};
use reqwest::cookie::Jar;
use reqwest::Url;
use directories::ProjectDirs;

pub struct Cache {
    downloader : Client,
    base_dir: PathBuf
}

impl Cache {
    pub fn new<P : Into<PathBuf>>(downloader: Client, base_dir: P) -> Self {
        Self { downloader, base_dir : base_dir.into() }
    }

    pub fn default() -> Option<Self> {
        let proj_dirs = ProjectDirs::from("net", "programmer-monk", "aoc2022")?;
        let cookie_file = proj_dirs.config_dir().join("session-cookie.txt");
        let jar = Jar::default();
        if let Ok(cookie) = std::fs::read_to_string(cookie_file) {
            jar.add_cookie_str(cookie.trim(), &Url::parse("https://adventofcode.com").unwrap())
        }
        let client = Client::builder().cookie_provider(std::sync::Arc::new(jar)).build().unwrap();
        Some(Self::new(client, proj_dirs.cache_dir()))
    }

    pub fn input_path(&self, day: u32) -> PathBuf {
        self.base_dir.join(format!("input{day:02}.txt"))
    }

    /// Get the input for day as a file. If the input has not already been cached it is downloaded
    /// into the cache
    pub fn get_input(&self, day: u32) -> io::Result<File> {
        match File::open(self.input_path(day)) {
            Ok(file) => Ok(file),
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    self.download_input(day)
                } else {
                    Err(err)
                }
            }
        }
    }

    /// Download and cache the input for the specified day. Returns a File positioned at the
    /// beginning of the input
    pub fn download_input(&self, day: u32) -> io::Result<File> {
        let url = format!("https://adventofcode.com/2022/day/{day}/input");
        let path = self.input_path(day);
        for tmpnum in 1..8 {
            let tmppath = path.with_extension(format!("tmp{tmpnum}"));
            if let Some(parent) = tmppath.parent() {
                std::fs::create_dir_all(parent)?
            }
            match File::options().create_new(true).read(true).write(true).open(&tmppath) {
                Ok(mut tmpfile) => {
                    if let Err(e) =
                        self.downloader.get(url).send()
                            .and_then(Response::error_for_status)
                            .and_then(|mut r| r.copy_to(&mut tmpfile))
                            .map_err(|e| io::Error::new(ErrorKind::Other, e))
                            .and_then(|_| tmpfile.rewind())
                            .and_then(|_| std::fs::rename(&tmppath, path))
                    {
                        std::fs::remove_file(tmppath).unwrap_or(());
                        return Err(e)
                    }
                    return Ok(tmpfile)
                },
                Err(e) => {
                    if e.kind() != ErrorKind::AlreadyExists {
                        return Err(e)
                    }
                }
            }
        };
        Err(io::Error::new(ErrorKind::Other, "could not create temp file"))
    }
}
