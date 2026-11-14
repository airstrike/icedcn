use iced::widget::svg;
use icedcn::Theme;
pub fn archive<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <rect width="20" height="5" x="2" y="3" rx="1" /> <path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8" /> <path d="M10 12h4" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn arrow_left<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m12 19-7-7 7-7" /> <path d="M19 12H5" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn arrow_right<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="m12 5 7 7-7 7" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn audio_lines<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 10v3" /> <path d="M6 6v11" /> <path d="M10 3v18" /> <path d="M14 8v7" /> <path d="M18 5v13" /> <path d="M22 10v3" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn bot<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 8V4H8" /> <rect width="16" height="12" x="4" y="8" rx="2" /> <path d="M2 14h2" /> <path d="M20 14h2" /> <path d="M15 13v2" /> <path d="M9 13v2" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn calendar_plus<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 2v4" /> <path d="M16 2v4" /> <path d="M21 13V6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h8" /> <path d="M3 10h18" /> <path d="M16 19h6" /> <path d="M19 16v6" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn check<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 6 9 17l-5-5" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn chevron_down<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m6 9 6 6 6-6" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn clock<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="10" /> <polyline points="12 6 12 12 16 14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn copy<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <rect width="14" height="14" x="8" y="8" rx="2" ry="2" /> <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn ellipsis<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="1" /> <circle cx="19" cy="12" r="1" /> <circle cx="5" cy="12" r="1" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn list_filter<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M7 12h10" /> <path d="M10 18h4" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn mail_check<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" /> <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" /> <path d="m16 19 2 2 4-4" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn minus<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn plus<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M12 5v14" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn search<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="11" cy="11" r="8" /> <path d="m21 21-4.3-4.3" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn share<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" /> <polyline points="16 6 12 2 8 6" /> <line x1="12" x2="12" y1="2" y2="15" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn tabler_plus<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"/></svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn tag<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z" /> <circle cx="7.5" cy="7.5" r=".5" fill="currentColor" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn trash<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.destructive.base.into()) })
}
pub fn trash_2<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M3 6h18" /> <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /> <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" /> <line x1="10" x2="10" y1="11" y2="17" /> <line x1="14" x2="14" y1="11" y2="17" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.destructive.base.into()) })
}
pub fn triangle_alert<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" /> <path d="M12 9v4" /> <path d="M12 17h.01" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn user_round_x<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M2 21a8 8 0 0 1 11.873-7" /> <circle cx="10" cy="8" r="5" /> <path d="m17 17 5 5" /> <path d="m22 17-5 5" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
pub fn volume_off<'a>() -> svg::Svg<'a, Theme> {
    svg(svg::Handle::from_memory(r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M16 9a5 5 0 0 1 .95 2.293" /> <path d="M19.364 5.636a9 9 0 0 1 1.889 9.96" /> <path d="m2 2 20 20" /> <path d="m7 7-.587.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298V11" /> <path d="M9.828 4.172A.686.686 0 0 1 11 4.657v.686" /> </svg>"#.as_bytes())).width(16).height(16)
        .style(|theme: &Theme, _| svg::Style { color: Some(theme.neutral.foreground.into()) })
}
