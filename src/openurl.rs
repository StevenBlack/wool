// this is from https://github.com/overdrivenpotato/url_open/tree/master/src

use url::Url;

#[cfg(target_os = "macos")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("open").arg(url.to_string()).output();
}

#[cfg(target_os = "linux")]
pub fn open(url: &Url) {
    let _ = std::process::Command::new("xdg-open").arg(url.to_string()).output();
}
