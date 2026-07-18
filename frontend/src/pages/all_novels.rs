use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::app::Route;
use crate::components::pagination::Pagination;
use crate::types::NovelWithLatest;

#[function_component(AllNovelsPage)]
pub fn all_novels_page() -> Html {
    let novels = use_state(|| Vec::<NovelWithLatest>::new());
    let total = use_state(|| 0i64);
    let total_pages = use_state(|| 0i64);
    let page = use_state(|| 1i64);
    let loading = use_state(|| true);

    {
        let novels = novels.clone();
        let loading = loading.clone();
        let total = total.clone();
        let total_pages = total_pages.clone();
        let p = *page;
        use_effect_with(p, move |p| {
            let novels = novels.clone();
            let loading = loading.clone();
            let total = total.clone();
            let total_pages = total_pages.clone();
            let p = *p;
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_novels(None, p, "click").await {
                    Ok(resp) => {
                        novels.set(resp.novels);
                        total.set(resp.total);
                        total_pages.set((resp.total + resp.page_size - 1) / resp.page_size);
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

    let on_page = {
        let page = page.clone();
        Callback::from(move |p: i64| {
            page.set(p);
        })
    };

    html! {
        <div id="main">
            <div class="box_con">
                <div class="con_top">
                    <Link<Route> to={Route::Home}>{ "首页" }</Link<Route>>
                    { " > " }
                    <strong>{ "全部小说" }</strong>
                </div>
            </div>

            if *loading {
                <div style="text-align:center;padding:50px;color:#88C6E5;font-size:16px;">
                    { "加载中..." }
                </div>
            } else {
                <div class="novelslist">
                    <div class="content list-block" style="border-right:0;flex:1">
                        <h2>{ "全部小说列表" }</h2>
                        <ul>
                            { novels.iter().map(|n| html! {
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

            <Pagination page={*page} total_pages={*total_pages} on_page={on_page} />
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
