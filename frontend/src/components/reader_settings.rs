use std::rc::Rc;
use std::cell::RefCell;

use yew::prelude::*;
use web_sys::{HtmlSelectElement, HtmlInputElement};
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, PartialEq)]
pub struct ReaderPrefs {
    pub font_family: String,
    pub text_color: String,
    pub font_size: String,
    pub bg_color: String,
    pub content_width: String,
    pub night_mode: bool,
    pub auto_scroll: bool,
}

impl Default for ReaderPrefs {
    fn default() -> Self {
        Self {
            font_family: String::new(),
            text_color: "#000".into(),
            font_size: "19pt".into(),
            bg_color: "#E9FAFF".into(),
            content_width: "85%".into(),
            night_mode: false,
            auto_scroll: false,
        }
    }
}

impl ReaderPrefs {
    pub fn load() -> Self {
        Self {
            font_family: LocalStorage::get("font").unwrap_or_default(),
            text_color: LocalStorage::get("color").unwrap_or_else(|_| "#000".into()),
            font_size: LocalStorage::get("size").unwrap_or_else(|_| "19pt".into()),
            bg_color: LocalStorage::get("bgcolor").unwrap_or_else(|_| "#E9FAFF".into()),
            content_width: LocalStorage::get("width").unwrap_or_else(|_| "85%".into()),
            night_mode: LocalStorage::get("night").unwrap_or_else(|_| false),
            auto_scroll: LocalStorage::get("autopage").unwrap_or_else(|_| false),
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ReaderSettingsProps {
    pub prefs: ReaderPrefs,
    pub on_change: Callback<ReaderPrefs>,
}

#[function_component(ReaderSettings)]
pub fn reader_settings(props: &ReaderSettingsProps) -> Html {
    let prefs_rc = Rc::new(RefCell::new(props.prefs.clone()));

    let make_select_handler = |storage_key: &'static str, field: &'static str| -> Callback<yew::events::Event> {
        let prefs_rc = prefs_rc.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: yew::events::Event| {
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                let value = select.value();
                {
                    let mut p = prefs_rc.borrow_mut();
                    match field {
                        "font" => p.font_family = value.clone(),
                        "color" => p.text_color = value.clone(),
                        "size" => p.font_size = value.clone(),
                        "bg" => p.bg_color = value.clone(),
                        "width" => p.content_width = value.clone(),
                        _ => {}
                    }
                }
                let _ = LocalStorage::set(storage_key, &value);
                on_change.emit(prefs_rc.borrow().clone());
            }
        })
    };

    let on_font = make_select_handler("font", "font");
    let on_color = make_select_handler("color", "color");
    let on_size = make_select_handler("size", "size");
    let on_bg = make_select_handler("bgcolor", "bg");
    let on_width = make_select_handler("width", "width");

    let on_night = {
        let prefs_rc = prefs_rc.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: yew::events::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let checked = input.checked();
                {
                    let mut p = prefs_rc.borrow_mut();
                    p.night_mode = checked;
                }
                let _ = LocalStorage::set("night", &checked);
                on_change.emit(prefs_rc.borrow().clone());
            }
        })
    };

    let on_autopage = {
        let prefs_rc = prefs_rc.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |e: yew::events::Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let checked = input.checked();
                {
                    let mut p = prefs_rc.borrow_mut();
                    p.auto_scroll = checked;
                }
                let _ = LocalStorage::set("autopage", &checked);
                on_change.emit(prefs_rc.borrow().clone());
            }
        })
    };

    html! {
        <div id="page_set">
            <select id="selFont" onchange={on_font}>
                <option value="" selected={props.prefs.font_family.is_empty()}>{ "字体" }</option>
                <option value="" selected={props.prefs.font_family.is_empty()}>{ "默认" }</option>
                <option value="SimSun, serif" selected={props.prefs.font_family == "SimSun, serif"}>{ "宋体" }</option>
                <option value="SimHei, sans-serif" selected={props.prefs.font_family == "SimHei, sans-serif"}>{ "黑体" }</option>
                <option value="Microsoft YaHei, sans-serif" selected={props.prefs.font_family == "Microsoft YaHei, sans-serif"}>{ "微软雅黑" }</option>
                <option value="KaiTi, serif" selected={props.prefs.font_family == "KaiTi, serif"}>{ "楷体" }</option>
            </select>

            <select id="selColor" onchange={on_color}>
                <option value="#000">{ "颜色" }</option>
                <option value="#000" selected={props.prefs.text_color == "#000"}>{ "默认" }</option>
                <option value="#9370DB" selected={props.prefs.text_color == "#9370DB"}>{ "暗紫" }</option>
                <option value="#2E8B57" selected={props.prefs.text_color == "#2E8B57"}>{ "暗绿" }</option>
                <option value="#2F4F4F" selected={props.prefs.text_color == "#2F4F4F"}>{ "暗灰" }</option>
                <option value="#778899" selected={props.prefs.text_color == "#778899"}>{ "浅灰" }</option>
                <option value="#800000" selected={props.prefs.text_color == "#800000"}>{ "暗红" }</option>
                <option value="#6A5ACD" selected={props.prefs.text_color == "#6A5ACD"}>{ "岩蓝" }</option>
                <option value="#BC8F8F" selected={props.prefs.text_color == "#BC8F8F"}>{ "玫瑰" }</option>
                <option value="#F4A460" selected={props.prefs.text_color == "#F4A460"}>{ "黄褐" }</option>
            </select>

            <select id="selSize" onchange={on_size}>
                <option value="19pt">{ "字号" }</option>
                <option value="10pt" selected={props.prefs.font_size == "10pt"}>{ "10pt" }</option>
                <option value="12pt" selected={props.prefs.font_size == "12pt"}>{ "12pt" }</option>
                <option value="14pt" selected={props.prefs.font_size == "14pt"}>{ "14pt" }</option>
                <option value="16pt" selected={props.prefs.font_size == "16pt"}>{ "16pt" }</option>
                <option value="19pt" selected={props.prefs.font_size == "19pt"}>{ "默认" }</option>
                <option value="22pt" selected={props.prefs.font_size == "22pt"}>{ "22pt" }</option>
                <option value="25pt" selected={props.prefs.font_size == "25pt"}>{ "25pt" }</option>
                <option value="30pt" selected={props.prefs.font_size == "30pt"}>{ "30pt" }</option>
            </select>

            <select id="selBg" onchange={on_bg}>
                <option value="#E9FAFF">{ "背景" }</option>
                <option value="#E9FAFF" selected={props.prefs.bg_color == "#E9FAFF"}>{ "默认" }</option>
                <option value="#FFFFFF" selected={props.prefs.bg_color == "#FFFFFF"}>{ "纯白" }</option>
                <option value="#000000" selected={props.prefs.bg_color == "#000000"}>{ "漆黑" }</option>
                <option value="#FFFFED" selected={props.prefs.bg_color == "#FFFFED"}>{ "明黄" }</option>
                <option value="#EEFAEE" selected={props.prefs.bg_color == "#EEFAEE"}>{ "护眼" }</option>
                <option value="#CCE8CF" selected={props.prefs.bg_color == "#CCE8CF"}>{ "草绿" }</option>
                <option value="#FCEFFF" selected={props.prefs.bg_color == "#FCEFFF"}>{ "淡紫" }</option>
                <option value="#EFEFEF" selected={props.prefs.bg_color == "#EFEFEF"}>{ "灰色" }</option>
                <option value="#F5F5DC" selected={props.prefs.bg_color == "#F5F5DC"}>{ "米色" }</option>
            </select>

            <select id="selWidth" onchange={on_width}>
                <option value="85%">{ "宽度" }</option>
                <option value="85%" selected={props.prefs.content_width == "85%"}>{ "默认" }</option>
                <option value="95%" selected={props.prefs.content_width == "95%"}>{ "95%" }</option>
                <option value="75%" selected={props.prefs.content_width == "75%"}>{ "75%" }</option>
                <option value="65%" selected={props.prefs.content_width == "65%"}>{ "65%" }</option>
                <option value="50%" selected={props.prefs.content_width == "50%"}>{ "50%" }</option>
            </select>

            <label>
                <input type="checkbox" id="chkAutopage" checked={props.prefs.auto_scroll} onchange={on_autopage} />
                { " 翻页" }
            </label>
            <label>
                <input type="checkbox" id="chkNight" checked={props.prefs.night_mode} onchange={on_night} />
                { " 夜间" }
            </label>
        </div>
    }
}
