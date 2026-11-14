// Generated automatically by iced_lucide at build time.
// Do not edit manually.
// e1aca11be80568c0fd84d5e9e4fb7b0db9b7b718a6687bc27a9924d7d954bcc0
use iced::widget::text::{self, Text};

pub const FONT: &[u8] = include_bytes!("../fonts/lucide.ttf");

/// All icons as `(name, codepoint_str)` pairs.
/// Use this to populate an icon-picker widget.
#[allow(dead_code)]
pub const ALL_ICONS: &[(&str, &str)] = &[
    ("app_window", "\u{E426}"),
    ("archive", "\u{E041}"),
    ("arrow_down", "\u{E042}"),
    ("arrow_left", "\u{E048}"),
    ("arrow_right", "\u{E049}"),
    ("arrow_up", "\u{E04A}"),
    ("arrow_up_down", "\u{E37D}"),
    ("arrow_up_right", "\u{E04D}"),
    ("audio_lines", "\u{E55A}"),
    ("audio_waveform", "\u{E55B}"),
    ("badge_check", "\u{E241}"),
    ("bell", "\u{E059}"),
    ("bluetooth", "\u{E05C}"),
    ("bold", "\u{E05D}"),
    ("book_open", "\u{E05F}"),
    ("book_open_check", "\u{E381}"),
    ("bookmark", "\u{E060}"),
    ("bot", "\u{E1BB}"),
    ("brain", "\u{E3C6}"),
    ("building_2", "\u{E290}"),
    ("calculator", "\u{E1BC}"),
    ("calendar", "\u{E063}"),
    ("calendar_plus", "\u{E2BC}"),
    ("chart_pie", "\u{E06B}"),
    ("check", "\u{E06C}"),
    ("chevron_down", "\u{E06D}"),
    ("chevron_left", "\u{E06E}"),
    ("chevron_right", "\u{E06F}"),
    ("chevron_up", "\u{E070}"),
    ("chevrons_up_down", "\u{E211}"),
    ("circle", "\u{E076}"),
    ("circle_alert", "\u{E077}"),
    ("circle_check", "\u{E226}"),
    ("circle_check_big", "\u{E07C}"),
    ("circle_fading_plus", "\u{E5BC}"),
    ("circle_question_mark", "\u{E082}"),
    ("clipboard_paste", "\u{E3E8}"),
    ("clock", "\u{E087}"),
    ("cloud", "\u{E088}"),
    ("code", "\u{E093}"),
    ("columns_3", "\u{E099}"),
    ("command", "\u{E09A}"),
    ("copy", "\u{E09E}"),
    ("corner_down_left", "\u{E0A1}"),
    ("credit_card", "\u{E0AA}"),
    ("download", "\u{E0B2}"),
    ("ellipsis", "\u{E0B6}"),
    ("ellipsis_vertical", "\u{E0B7}"),
    ("external_link", "\u{E0B9}"),
    ("eye", "\u{E0BA}"),
    ("eye_off", "\u{E0BB}"),
    ("file", "\u{E0C0}"),
    ("file_code", "\u{E0C3}"),
    ("file_pen", "\u{E31F}"),
    ("file_plus", "\u{E0C9}"),
    ("file_search", "\u{E0CB}"),
    ("file_text", "\u{E0CC}"),
    ("file_warning", "\u{E0CE}"),
    ("folder", "\u{E0D7}"),
    ("folder_code", "\u{E5FB}"),
    ("folder_open", "\u{E247}"),
    ("folder_plus", "\u{E0D9}"),
    ("folder_search", "\u{E339}"),
    ("forward", "\u{E229}"),
    ("frame", "\u{E291}"),
    ("gallery_vertical_end", "\u{E4D2}"),
    ("git_branch", "\u{E0E2}"),
    ("git_commit_horizontal", "\u{E0E3}"),
    ("globe", "\u{E0E8}"),
    ("house", "\u{E0F5}"),
    ("image", "\u{E0F6}"),
    ("inbox", "\u{E0F7}"),
    ("info", "\u{E0F9}"),
    ("italic", "\u{E0FB}"),
    ("keyboard", "\u{E284}"),
    ("languages", "\u{E0FE}"),
    ("layout_grid", "\u{E0FF}"),
    ("list", "\u{E106}"),
    ("list_filter", "\u{E460}"),
    ("log_out", "\u{E10E}"),
    ("mail", "\u{E10F}"),
    ("mail_check", "\u{E361}"),
    ("map", "\u{E110}"),
    ("maximize", "\u{E112}"),
    ("message_circle_dashed", "\u{E563}"),
    ("message_square", "\u{E117}"),
    ("minimize", "\u{E11A}"),
    ("minus", "\u{E11C}"),
    ("monitor", "\u{E11D}"),
    ("moon", "\u{E11E}"),
    ("octagon_x", "\u{E128}"),
    ("palette", "\u{E1DD}"),
    ("panels_top_left", "\u{E12C}"),
    ("paperclip", "\u{E12D}"),
    ("pencil", "\u{E1F9}"),
    ("plus", "\u{E13D}"),
    ("refresh_ccw", "\u{E144}"),
    ("rotate_cw", "\u{E149}"),
    ("save", "\u{E14D}"),
    ("scissors", "\u{E14E}"),
    ("search", "\u{E151}"),
    ("settings", "\u{E154}"),
    ("settings_2", "\u{E245}"),
    ("share", "\u{E155}"),
    ("shield", "\u{E158}"),
    ("shield_alert", "\u{E1FE}"),
    ("smile", "\u{E164}"),
    ("sparkles", "\u{E412}"),
    ("square_terminal", "\u{E20A}"),
    ("star", "\u{E176}"),
    ("sun", "\u{E178}"),
    ("table", "\u{E17D}"),
    ("tag", "\u{E17F}"),
    ("telescope", "\u{E5C5}"),
    ("terminal", "\u{E181}"),
    ("thumbs_down", "\u{E189}"),
    ("thumbs_up", "\u{E18A}"),
    ("trash", "\u{E18D}"),
    ("trash_2", "\u{E18E}"),
    ("triangle_alert", "\u{E193}"),
    ("underline", "\u{E19A}"),
    ("user", "\u{E19F}"),
    ("user_round_x", "\u{E46D}"),
    ("volume_off", "\u{E626}"),
    ("wallet", "\u{E204}"),
    ("wrap_text", "\u{E248}"),
    ("x", "\u{E1B2}"),
    ("zoom_in", "\u{E1B6}"),
    ("zoom_out", "\u{E1B7}"),
];

pub fn app_window<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E426}")
}

pub fn archive<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E041}")
}

pub fn arrow_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E042}")
}

pub fn arrow_left<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E048}")
}

pub fn arrow_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E049}")
}

pub fn arrow_up<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E04A}")
}

pub fn arrow_up_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E37D}")
}

pub fn arrow_up_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E04D}")
}

pub fn audio_lines<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E55A}")
}

pub fn audio_waveform<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E55B}")
}

pub fn badge_check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E241}")
}

pub fn bell<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E059}")
}

pub fn bluetooth<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E05C}")
}

pub fn bold<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E05D}")
}

pub fn book_open<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E05F}")
}

pub fn book_open_check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E381}")
}

pub fn bookmark<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E060}")
}

pub fn bot<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1BB}")
}

pub fn brain<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E3C6}")
}

pub fn building_2<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E290}")
}

pub fn calculator<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1BC}")
}

pub fn calendar<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E063}")
}

pub fn calendar_plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E2BC}")
}

pub fn chart_pie<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06B}")
}

pub fn check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06C}")
}

pub fn chevron_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06D}")
}

pub fn chevron_left<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06E}")
}

pub fn chevron_right<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E06F}")
}

pub fn chevron_up<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E070}")
}

pub fn chevrons_up_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E211}")
}

pub fn circle<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E076}")
}

pub fn circle_alert<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E077}")
}

pub fn circle_check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E226}")
}

pub fn circle_check_big<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E07C}")
}

pub fn circle_fading_plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E5BC}")
}

pub fn circle_question_mark<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E082}")
}

pub fn clipboard_paste<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E3E8}")
}

pub fn clock<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E087}")
}

pub fn cloud<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E088}")
}

pub fn code<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E093}")
}

pub fn columns_3<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E099}")
}

pub fn command<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E09A}")
}

pub fn copy<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E09E}")
}

pub fn corner_down_left<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0A1}")
}

pub fn credit_card<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0AA}")
}

pub fn download<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0B2}")
}

pub fn ellipsis<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0B6}")
}

pub fn ellipsis_vertical<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0B7}")
}

pub fn external_link<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0B9}")
}

pub fn eye<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0BA}")
}

pub fn eye_off<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0BB}")
}

pub fn file<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0C0}")
}

pub fn file_code<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0C3}")
}

pub fn file_pen<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E31F}")
}

pub fn file_plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0C9}")
}

pub fn file_search<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0CB}")
}

pub fn file_text<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0CC}")
}

pub fn file_warning<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0CE}")
}

pub fn folder<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0D7}")
}

pub fn folder_code<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E5FB}")
}

pub fn folder_open<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E247}")
}

pub fn folder_plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0D9}")
}

pub fn folder_search<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E339}")
}

pub fn forward<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E229}")
}

pub fn frame<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E291}")
}

pub fn gallery_vertical_end<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E4D2}")
}

pub fn git_branch<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0E2}")
}

pub fn git_commit_horizontal<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0E3}")
}

pub fn globe<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0E8}")
}

pub fn house<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0F5}")
}

pub fn image<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0F6}")
}

pub fn inbox<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0F7}")
}

pub fn info<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0F9}")
}

pub fn italic<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0FB}")
}

pub fn keyboard<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E284}")
}

pub fn languages<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0FE}")
}

pub fn layout_grid<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E0FF}")
}

pub fn list<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E106}")
}

pub fn list_filter<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E460}")
}

pub fn log_out<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E10E}")
}

pub fn mail<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E10F}")
}

pub fn mail_check<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E361}")
}

pub fn map<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E110}")
}

pub fn maximize<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E112}")
}

pub fn message_circle_dashed<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E563}")
}

pub fn message_square<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E117}")
}

pub fn minimize<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11A}")
}

pub fn minus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11C}")
}

pub fn monitor<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11D}")
}

pub fn moon<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E11E}")
}

pub fn octagon_x<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E128}")
}

pub fn palette<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1DD}")
}

pub fn panels_top_left<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E12C}")
}

pub fn paperclip<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E12D}")
}

pub fn pencil<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1F9}")
}

pub fn plus<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E13D}")
}

pub fn refresh_ccw<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E144}")
}

pub fn rotate_cw<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E149}")
}

pub fn save<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E14D}")
}

pub fn scissors<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E14E}")
}

pub fn search<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E151}")
}

pub fn settings<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E154}")
}

pub fn settings_2<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E245}")
}

pub fn share<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E155}")
}

pub fn shield<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E158}")
}

pub fn shield_alert<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1FE}")
}

pub fn smile<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E164}")
}

pub fn sparkles<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E412}")
}

pub fn square_terminal<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E20A}")
}

pub fn star<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E176}")
}

pub fn sun<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E178}")
}

pub fn table<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E17D}")
}

pub fn tag<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E17F}")
}

pub fn telescope<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E5C5}")
}

pub fn terminal<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E181}")
}

pub fn thumbs_down<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E189}")
}

pub fn thumbs_up<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E18A}")
}

pub fn trash<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E18D}")
}

pub fn trash_2<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E18E}")
}

pub fn triangle_alert<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E193}")
}

pub fn underline<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E19A}")
}

pub fn user<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E19F}")
}

pub fn user_round_x<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E46D}")
}

pub fn volume_off<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E626}")
}

pub fn wallet<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E204}")
}

pub fn wrap_text<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E248}")
}

pub fn x<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1B2}")
}

pub fn zoom_in<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1B6}")
}

pub fn zoom_out<'a, Theme>() -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    icon("\u{E1B7}")
}

/// Render any Lucide icon by its codepoint string.
/// Use this together with [`ALL_ICONS`] to display icons dynamically:
/// ```ignore
/// for (name, cp) in ALL_ICONS {
///     button(render(cp)).on_press(Msg::Pick(name.to_string()))
/// }
/// ```
pub fn render<'a, Theme>(codepoint: &'a str) -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    Text::new(codepoint).font("lucide")
}

fn icon<'a, Theme>(codepoint: &'a str) -> Text<'a, Theme>
where
    Theme: text::Catalog + 'a,
{
    render(codepoint)
}
