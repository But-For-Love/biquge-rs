use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub current: String,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let items = [
        ("首页", Route::Home, "home"),
        ("我的书架", Route::Home, "bookshelf"),
        ("玄幻小说", Route::Category { slug: "xuanhuan".into() }, "xuanhuan"),
        ("修真小说", Route::Category { slug: "xiuzhen".into() }, "xiuzhen"),
        ("都市小说", Route::Category { slug: "dushi".into() }, "dushi"),
        ("穿越小说", Route::Category { slug: "chuanyue".into() }, "chuanyue"),
        ("网游小说", Route::Category { slug: "youxi".into() }, "youxi"),
        ("科幻小说", Route::Category { slug: "kehuan".into() }, "kehuan"),
        ("排行榜", Route::Rankings, "rankings"),
        ("全部小说", Route::AllNovels, "all"),
    ];

    html! {
        <div class="nav">
            <ul>
                { items.iter().map(|(label, route, key)| {
                    let cls = if props.current == *key { "current" } else { "" };
                    html! {
                        <li class={cls}>
                            <Link<Route> to={route.clone()}>{ *label }</Link<Route>>
                        </li>
                    }
                }).collect::<Html>() }
            </ul>
        </div>
    }
}
