//! Community and project links (Help menu, About dialog, top-bar Discord button).

/// The app's id in Adobado Kit URLs.
pub const APP: &str = "lightcraft";
/// The Adobado Kit community Discord.
pub const DISCORD: &str = "https://github.com/smouj/Adobado_Kit";
/// The Adobado Kit website.
pub const WEBSITE: &str = "https://github.com/smouj/Adobado_Kit";
/// This app's page on the Adobado Kit website.
pub const APP_PAGE: &str = "https://github.com/smouj/LightAdobado";
/// This app's source repository.
pub const GITHUB: &str = "https://github.com/smouj/LightAdobado";

/// (UI command id, menu label, URL) for each link, in Help-menu order.
/// The user documentation (docs/ in the repository).
pub const HELP: &str = "https://github.com/smouj/LightAdobado/tree/main/docs";

pub const LINKS: &[(&str, &str, &str)] = &[
    ("app.help", "LightAdobado Help", HELP),
    ("app.discord", "Join the Adobado Kit Discord…", DISCORD),
    ("app.website", "LightAdobado Website", APP_PAGE),
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
        assert!(APP_PAGE.ends_with(&format!("/apps/{APP}")));
        assert!(GITHUB.ends_with(&format!("/storytold/{APP}")));
    }
}
