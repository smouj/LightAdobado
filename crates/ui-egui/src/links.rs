//! Support and project links (Help menu, About dialog and top-bar links).

/// This app's public repository name.
pub const APP: &str = "LightAdobado";
/// The app's issue tracker for help and bug reports.
pub const SUPPORT: &str = "https://github.com/smouj/LightAdobado/issues";
/// Compatibility alias for the former community-link constant.
pub const DISCORD: &str = SUPPORT;
/// The Adobado Kit website.
pub const WEBSITE: &str = "https://smouj.github.io/Adobado_Kit/";
/// This app's project page.
pub const APP_PAGE: &str = "https://github.com/smouj/LightAdobado";
/// This app's source repository.
pub const GITHUB: &str = "https://github.com/smouj/LightAdobado";

/// (UI command id, menu label, URL) for each link, in Help-menu order.
/// The user documentation (docs/ in the repository).
pub const HELP: &str = "https://github.com/smouj/LightAdobado/tree/main/docs";

pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "LightAdobado Help", HELP),
    ("app.discord", "Get help and report an issue…", SUPPORT),
    ("app.website", "LightAdobado Project Page", APP_PAGE),
    ("app.github", "LightAdobado on GitHub", GITHUB),
    ("app.artcraft", "Adobado Kit Website", WEBSITE),
    ("app.feedback", "Send Feedback…", FEEDBACK),
];

/// Where feedback and bug reports go.
pub const FEEDBACK: &str = "https://github.com/smouj/LightAdobado/issues/new";

/// The URL behind a link command id.
pub fn url_of(cmd: &str) -> Option<&'static str> {
    LINKS.iter().find(|(id, _, _)| *id == cmd).map(|(_, _, u)| *u)
}

/// Open `url` in the user's browser (through the host's `open_url` service).
pub fn open(app: &mut crate::LightcraftApp, url: &str) -> Result<serde_json::Value, String> {
    let open = app.services.open_url.as_mut().ok_or("can't open links here")?;
    open(url)?;
    Ok(serde_json::json!({ "url": url }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_https_and_name_this_app() {
        for (id, label, url) in LINKS {
            assert!(url.starts_with("https://"), "{id}");
            assert!(!label.is_empty());
            assert_eq!(url_of(id), Some(*url));
        }
        assert_eq!(APP_PAGE, GITHUB);
        assert!(GITHUB.ends_with(&format!("/smouj/{APP}")));
    }
}
