use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::app::Route;
use crate::types::Novel;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchPageProps {
    pub q: String,
}

#[function_component(SearchPage)]
pub fn search_page(props: &SearchPageProps) -> Html {
    let results = use_state(|| Vec::<Novel>::new());
    let loading = use_state(|| true);

    {
        let results = results.clone();
        let loading = loading.clone();
        let q = props.q.clone();
        use_effect_with(q.clone(), move |q| {
            let results = results.clone();
            let loading = loading.clone();
            let query = q.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_search(&query).await {
                    Ok(r) => {
                        results.set(r);
                        loading.set(false);
                    }
                    Err(_) => {
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div id="main">
            <div class="box_con">
                <div class="con_top">
                    <Link<Route> to={Route::Home}>{ "首页" }</Link<Route>>
                    { " > " }
                    <strong>{ format!("搜索: {}", props.q) }</strong>
                </div>
            </div>

            if *loading {
                <div style="text-align:center;padding:50px;color:#88C6E5;font-size:16px;">
                    { "搜索中..." }
                </div>
            } else if results.is_empty() {
                <div class="novelslist">
                    <div class="content list-block" style="border-right:0;flex:1">
                        <h2>{ format!("搜索 \"{}\" 的结果", props.q) }</h2>
                        <ul>
                            <li>{ "未找到相关小说，请尝试其他关键词" }</li>
                        </ul>
                    </div>
                </div>
            } else {
                <div class="novelslist">
                    <div class="content list-block" style="border-right:0;flex:1">
                        <h2>{ format!("搜索 \"{}\" 的结果（共 {} 本）", props.q, results.len()) }</h2>
                        <ul>
                            { results.iter().map(|n| html! {
                                <li>
                                    { "【" }{ get_cat_short(n.category_id) }{ "】 " }
                                    <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                    <span style="float:right;color:#B3B3B3">
                                        { format!("{} · {}", &n.author, &n.last_update) }
                                    </span>
                                </li>
                            }).collect::<Html>() }
                        </ul>
                    </div>
                </div>
            }
        </div>
    }
}

fn get_cat_short(cat_id: i64) -> &'static str {
    match cat_id {
        1 => "玄幻", 2 => "仙侠", 3 => "都市", 4 => "历史",
        5 => "游戏", 6 => "科幻", 7 => "修真", 8 => "穿越",
        _ => "小说",
    }
}
