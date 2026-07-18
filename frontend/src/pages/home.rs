use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::app::Route;
use crate::types::HomeResponse;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let data = use_state(|| None::<HomeResponse>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_home().await {
                    Ok(home) => {
                        data.set(Some(home));
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
            } else if let Some(ref home) = *data {
                { render_home_content(home) }
            }
        </>
    }
}

fn render_home_content(home: &HomeResponse) -> Html {
    // Pre-compute data before entering html! macro
    let first3: Vec<_> = home.category_blocks.iter().take(3).collect();
    let last3: Vec<_> = home.category_blocks.iter().skip(3).take(3).collect();

    html! {
        <div id="main">
            <div id="hotcontent">
                <div class="l">
                    { home.featured.iter().map(|n| {
                        let cover = cover_url(&n.cover_url, &n.title);
                        html! {
                            <div class="item">
                                <div class="image">
                                    <Link<Route> to={Route::NovelDetail { id: n.id }}>
                                        <img src={cover} width="120" height="150" alt={n.title.clone()} />
                                    </Link<Route>>
                                </div>
                                <dl>
                                    <dt>
                                        <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                        <span>{ &n.author }</span>
                                    </dt>
                                    <dd>{ &n.intro }</dd>
                                </dl>
                            </div>
                        }
                    }).collect::<Html>() }
                </div>
                <div class="r">
                    <h2>{ "周点击榜" }</h2>
                    <ul>
                        { home.weekly_ranking.iter().enumerate().map(|(i, n)| html! {
                            <li>
                                <span>{ i + 1 }</span>
                                { " " }
                                <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                <span style="float:right">{ &n.author }</span>
                            </li>
                        }).collect::<Html>() }
                    </ul>
                    <h2>{ "强力推荐" }</h2>
                    <ul>
                        { home.recommend.iter().take(5).enumerate().map(|(i, n)| html! {
                            <li>
                                <span>{ i + 1 }</span>
                                { " " }
                                <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                <span style="float:right">{ &n.author }</span>
                            </li>
                        }).collect::<Html>() }
                    </ul>
                </div>
            </div>

            <div class="ad-placeholder">{ "— 广告位 —" }</div>

            <div class="novelslist">
                { first3.iter().map(|block| {
                    let cat = &block.category;
                    let featured = &block.featured_novel;
                    let cover = cover_small(&featured.cover_url, &featured.title);
                    html! {
                        <div class="content">
                            <h2>
                                <Link<Route> to={Route::Category { slug: cat.slug.clone() }}>
                                    { &cat.name }
                                </Link<Route>>
                            </h2>
                            <div class="cover-row">
                                <div class="image">
                                    <Link<Route> to={Route::NovelDetail { id: featured.id }}>
                                        <img src={cover} width="67" height="82" alt={featured.title.clone()} />
                                    </Link<Route>>
                                </div>
                                <dl>
                                    <dt>
                                        <Link<Route> to={Route::NovelDetail { id: featured.id }}>
                                            { &featured.title }
                                        </Link<Route>>
                                    </dt>
                                    <dd>{ &featured.intro }</dd>
                                </dl>
                            </div>
                            <ul class="link-list">
                                { block.novels.iter().take(6).map(|n| html! {
                                    <li>
                                        <Link<Route> to={Route::NovelDetail { id: n.id }}>
                                            { &n.title }
                                        </Link<Route>>
                                    </li>
                                }).collect::<Html>() }
                            </ul>
                        </div>
                    }
                }).collect::<Html>() }
            </div>

            <div class="novelslist">
                { last3.iter().map(|block| {
                    let cat = &block.category;
                    let tag = cat.name.chars().take(2).collect::<String>();
                    html! {
                        <div class="content list-block">
                            <h2>
                                <Link<Route> to={Route::Category { slug: cat.slug.clone() }}>
                                    { &cat.name }
                                </Link<Route>>
                            </h2>
                            <ul>
                                { block.novels.iter().take(6).map(|n| html! {
                                    <li>
                                        { "【" }{ &tag }{ "】" }
                                        <Link<Route> to={Route::NovelDetail { id: n.id }}>
                                            { &n.title }
                                        </Link<Route>>
                                        <span style="float:right;color:#B3B3B3">{ &n.author }</span>
                                    </li>
                                }).collect::<Html>() }
                            </ul>
                        </div>
                    }
                }).collect::<Html>() }
            </div>

            <div id="newscontent">
                <div class="l">
                    <h2>{ "最近更新小说列表" }</h2>
                    <ul>
                        { home.latest_updates.iter().take(10).map(|n| {
                            let cat_name = get_cat_name(n.category_id);
                            let ch_title = n.latest_chapter_title.clone().unwrap_or_default();
                            html! {
                                <li>
                                    { "【" }{ cat_name }{ "】 " }
                                    <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                    { " — " }{ ch_title }
                                    <span style="float:right;color:#B3B3B3">{ &n.last_update }</span>
                                </li>
                            }
                        }).collect::<Html>() }
                    </ul>
                </div>
                <div class="r">
                    <h2>{ "最新入库小说" }</h2>
                    <ul>
                        { home.new_arrivals.iter().enumerate().map(|(i, n)| html! {
                            <li>
                                <span>{ i + 1 }</span>
                                { " " }
                                <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                <span style="float:right;color:#B3B3B3">{ &n.last_update }</span>
                            </li>
                        }).collect::<Html>() }
                    </ul>
                </div>
            </div>
        </div>
    }
}

fn cover_url(url: &str, title: &str) -> String {
    if url.is_empty() {
        format!("https://placehold.co/120x150/E1ECED/88C6E5?text={}", &title.chars().take(4).collect::<String>())
    } else {
        url.to_string()
    }
}

fn cover_small(url: &str, title: &str) -> String {
    if url.is_empty() {
        format!("https://placehold.co/67x82/E1ECED/88C6E5?text={}", &title.chars().take(2).collect::<String>())
    } else {
        url.to_string()
    }
}

fn get_cat_name(cat_id: i64) -> &'static str {
    match cat_id {
        1 => "玄幻", 2 => "仙侠", 3 => "都市", 4 => "历史",
        5 => "游戏", 6 => "科幻", 7 => "修真", 8 => "穿越",
        _ => "小说",
    }
}
