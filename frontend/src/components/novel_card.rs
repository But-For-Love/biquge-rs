use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;
use crate::types::Novel;

#[derive(Properties, PartialEq, Clone)]
pub struct NovelCardHotProps {
    pub novel: Novel,
}

/// Card for #hotcontent .l .item (335px, 2 per row on homepage)
#[function_component(NovelCardHot)]
pub fn novel_card_hot(props: &NovelCardHotProps) -> Html {
    let n = &props.novel;
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
}

/// Wide card for #hotcontent .ll .item (315px, 3 per row on list page)
#[function_component(NovelCardWide)]
pub fn novel_card_wide(props: &NovelCardHotProps) -> Html {
    let n = &props.novel;
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
}

fn cover_url(url: &str, title: &str) -> String {
    if url.is_empty() {
        format!("https://placehold.co/120x150/E1ECED/88C6E5?text={}", &title.chars().take(4).collect::<String>())
    } else {
        url.to_string()
    }
}
