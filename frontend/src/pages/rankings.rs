use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::app::Route;
use crate::types::Novel;

#[function_component(RankingsPage)]
pub fn rankings_page() -> Html {
    let click_rank = use_state(|| Vec::<Novel>::new());
    let recommend_rank = use_state(|| Vec::<Novel>::new());
    let new_rank = use_state(|| Vec::<Novel>::new());
    let loading = use_state(|| true);

    {
        let click_rank = click_rank.clone();
        let recommend_rank = recommend_rank.clone();
        let new_rank = new_rank.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let c = api::fetch_rankings("click").await.unwrap_or_default();
                let r = api::fetch_rankings("recommend").await.unwrap_or_default();
                let n = api::fetch_rankings("new").await.unwrap_or_default();
                click_rank.set(c);
                recommend_rank.set(r);
                new_rank.set(n);
                loading.set(false);
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
                    <strong>{ "排行榜" }</strong>
                </div>
            </div>

            if *loading {
                <div style="text-align:center;padding:50px;color:#88C6E5;font-size:16px;">
                    { "加载中..." }
                </div>
            } else {
                <div id="newscontent">
                    <div class="l">
                        <h2>{ "点击榜" }</h2>
                        <ul>
                            { click_rank.iter().enumerate().map(|(i, n)| html! {
                                <li>
                                    <span>{ i + 1 }</span>{ " " }
                                    <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                    { " — " }<span style="color:#B3B3B3">{ &n.author }</span>
                                    <span style="float:right;color:#B3B3B3">{ &n.last_update }</span>
                                </li>
                            }).collect::<Html>() }
                        </ul>
                    </div>
                    <div class="r">
                        <h2>{ "推荐榜" }</h2>
                        <ul>
                            { recommend_rank.iter().take(10).enumerate().map(|(i, n)| html! {
                                <li>
                                    <span>{ i + 1 }</span>{ " " }
                                    <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                </li>
                            }).collect::<Html>() }
                        </ul>
                    </div>
                </div>
            }
        </div>
    }
}
