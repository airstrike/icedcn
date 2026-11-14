/// Split off the first nonblank line when it is a level 1–3 ATX heading.
/// Both the build-time navigation index and the loaded article use this rule.
pub fn split(source: &str) -> Option<(&str, &str)> {
    let first = source.trim_start();
    let (line, body) = first.split_once('\n').unwrap_or((first, ""));
    let title = line
        .strip_prefix("### ")
        .or_else(|| line.strip_prefix("## "))
        .or_else(|| line.strip_prefix("# "))?;
    Some((title.trim(), body))
}
