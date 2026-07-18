use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{window, KeyboardEvent};
use gloo_timers::callback::Interval;
use gloo_events::EventListener;
use gloo_utils::document;
use wasm_bindgen::JsCast;

use crate::api;
use crate::app::Route;
use crate::components::floating_buttons::FloatingButtons;
use crate::components::reader_settings::{ReaderPrefs, ReaderSettings};
use crate::types::ChapterResponse;

#[derive(Properties, PartialEq, Clone)]
pub struct ChapterReaderProps {
    pub novel_id: i64,
    pub chapter_id: i64,
}

#[function_component(ChapterReaderPage)]
pub fn chapter_reader_page(props: &ChapterReaderProps) -> Html {
    let data = use_state(|| None::<ChapterResponse>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let prefs = use_state(|| ReaderPrefs::load());
    let navigator = use_navigator().unwrap();

    // Fetch chapter
    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let novel_id = props.novel_id;
        let chapter_id = props.chapter_id;
        use_effect_with((novel_id, chapter_id), move |(nid, cid)| {
            let nid = *nid;
            let cid = *cid;
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_chapter(nid, cid).await {
                    Ok(Some(d)) => {
                        data.set(Some(d));
                        loading.set(false);
                        if let Some(win) = window() {
                            let _ = win.scroll_to_with_x_and_y(0.0, 0.0);
                        }
                    }
                    Ok(None) => {
                        error.set(Some("章节不存在".into()));
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    // Keyboard navigation
    {
        let data = data.clone();
        let navigator = navigator.clone();
        use_effect_with((), move |_| {
            let listener = EventListener::new(&document(), "keydown", move |event: &web_sys::Event| {
                if let Some(ke) = event.dyn_ref::<KeyboardEvent>() {
                    if let Some(ref d) = *data {
                        match ke.key().as_str() {
                            "ArrowLeft" => {
                                if let Some(prev_id) = d.prev_chapter_id {
                                    navigator.push(&Route::ChapterReader {
                                        novel_id: d.novel.id,
                                        chapter_id: prev_id,
                                    });
                                }
                            }
                            "ArrowRight" => {
                                if let Some(next_id) = d.next_chapter_id {
                                    navigator.push(&Route::ChapterReader {
                                        novel_id: d.novel.id,
                                        chapter_id: next_id,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });
            move || drop(listener)
        });
    }

    // Auto-scroll
    {
        let auto_scroll = prefs.auto_scroll;
        use_effect_with(auto_scroll, move |&auto| {
            let interval: Option<Interval> = if auto {
                Some(Interval::new(50, move || {
                    if let Some(win) = window() {
                        let scroll_y = win.scroll_y().unwrap_or(0.0);
                        let inner_h = win.inner_height().unwrap().as_f64().unwrap_or(800.0);
                        let body_h = document().body().map(|b| b.offset_height() as f64).unwrap_or(10000.0);
                        if scroll_y + inner_h < body_h - 10.0 {
                            let _ = win.scroll_by_with_x_and_y(0.0, 1.0);
                        }
                    }
                }))
            } else {
                None
            };
            move || drop(interval)
        });
    }

    let on_prefs_change = {
        let prefs = prefs.clone();
        Callback::from(move |new_prefs: ReaderPrefs| {
            prefs.set(new_prefs);
        })
    };

    let wrapper_classes = if prefs.night_mode {
        Classes::from("night-mode")
    } else {
        Classes::new()
    };

    html! {
        <>
            if *loading {
                <div id="main" style="text-align:center;padding:50px;color:#88C6E5;font-size:16px;">
                    { "加载中..." }
                </div>
            } else if let Some(ref err) = *error {
                <div id="main" style="text-align:center;padding:50px;color:red;">
                    { format!("加载失败: {}", err) }
                </div>
            } else if let Some(ref d) = *data {
                <div id="wrapper" class={wrapper_classes}>
                    <div class="content_read">
                        <div class="box_con">
                            <div class="con_top">
                                <Link<Route> to={Route::Home}>{ "笔趣阁" }</Link<Route>>
                                { " > " }
                                <Link<Route> to={Route::Category { slug: get_slug(d.novel.category_id).into() }}>
                                    { &d.category_name }
                                </Link<Route>>
                                { " > " }
                                <Link<Route> to={Route::NovelDetail { id: d.novel.id }}>
                                    { &d.novel.title }
                                </Link<Route>>
                                { " > " }
                                { &d.chapter.title }

                                <ReaderSettings prefs={(*prefs).clone()} on_change={on_prefs_change} />
                            </div>

                            <div class="bookname">
                                <h1>{ &d.chapter.title }</h1>
                                <div class="bottem1">
                                    <a href="#">{ "投推荐票" }</a>
                                    if let Some(prev_id) = d.prev_chapter_id {
                                        <Link<Route> to={Route::ChapterReader { novel_id: d.novel.id, chapter_id: prev_id }}>
                                            { "← 上一章" }
                                        </Link<Route>>
                                    }
                                    <Link<Route> to={Route::NovelDetail { id: d.novel.id }}>
                                        { "章节目录" }
                                    </Link<Route>>
                                    if let Some(next_id) = d.next_chapter_id {
                                        <Link<Route> to={Route::ChapterReader { novel_id: d.novel.id, chapter_id: next_id }}>
                                            { "下一章 →" }
                                        </Link<Route>>
                                    }
                                    <a href="#">{ "加入书签" }</a>
                                </div>
                            </div>

                            <div id="content" style={format!(
                                "font-family:{};color:{};font-size:{};width:{};",
                                if prefs.font_family.is_empty() { "\"Microsoft YaHei\", \"PingFang SC\", SimSun, sans-serif".to_string() } else { prefs.font_family.clone() },
                                prefs.text_color,
                                prefs.font_size,
                                prefs.content_width
                            )}>
                                { Html::from_html_unchecked(d.chapter.content.clone().into()) }
                            </div>

                            <div class="bottem2">
                                <a href="#">{ "投推荐票" }</a>
                                if let Some(prev_id) = d.prev_chapter_id {
                                    <Link<Route> to={Route::ChapterReader { novel_id: d.novel.id, chapter_id: prev_id }}>
                                        { "← 上一章" }
                                    </Link<Route>>
                                }
                                <Link<Route> to={Route::NovelDetail { id: d.novel.id }}>
                                    { "章节目录" }
                                </Link<Route>>
                                if let Some(next_id) = d.next_chapter_id {
                                    <Link<Route> to={Route::ChapterReader { novel_id: d.novel.id, chapter_id: next_id }}>
                                        { "下一章 →" }
                                    </Link<Route>>
                                }
                                <a href="#">{ "加入书签" }</a>
                            </div>
                        </div>
                    </div>

                    <FloatingButtons />

                    <div class="footer">
                        <div class="footer_link">{ "最新推荐：" }
                            <a href="#">{ "完美世界" }</a>{ " | " }
                            <a href="#">{ "大主宰" }</a>{ " | " }
                            <a href="#">{ "武动乾坤" }</a>
                        </div>
                        <div class="footer_cont">
                            <p>{ format!("《{}》最新章节由网友提供，情节跌宕起伏、扣人心弦，是一本情节与文笔俱佳的玄幻小说，笔趣阁转载收集{}最新章节。", d.novel.title, d.novel.title) }</p>
                            <p>{ "Copyright © 2024 笔趣阁 All Rights Reserved." }</p>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}

fn get_slug(cat_id: i64) -> &'static str {
    match cat_id {
        1 => "xuanhuan", 2 => "wuxia", 3 => "dushi", 4 => "lishi",
        5 => "youxi", 6 => "kehuan", 7 => "xiuzhen", 8 => "chuanyue",
        _ => "xuanhuan",
    }
}
