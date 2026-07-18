use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlInputElement;

use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let search_ref = use_node_ref();
    let navigator = use_navigator().unwrap();

    let on_search = {
        let search_ref = search_ref.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            e.prevent_default();
            if let Some(input) = search_ref.cast::<HtmlInputElement>() {
                let query = input.value();
                if !query.is_empty() {
                    navigator.push(&Route::Search { q: query });
                }
            }
        })
    };

    html! {
        <div class="header">
            <div class="header_logo">
                <Link<Route> to={Route::Home}>{ "笔趣阁" }</Link<Route>>
            </div>
            <div class="header_search">
                <form onsubmit={on_search}>
                    <input type="text" name="wd" placeholder="输入书名或作者，也可以搜主角名" ref={search_ref} />
                    <button type="submit">{ "搜 索" }</button>
                </form>
            </div>
            <div class="userpanel">
                <span class="text-danger">{ "联系：" }</span>
                <a href="#">{ "通过邮件" }</a>{ " | " }
                <a href="#">{ "站内短信" }</a><br />
                <a href="#"><strong>{ "积分规则" }</strong></a>
                <span class="sep">{ "|" }</span>
                <a href="#">{ "无法访问备用站" }</a>
            </div>
        </div>
    }
}
