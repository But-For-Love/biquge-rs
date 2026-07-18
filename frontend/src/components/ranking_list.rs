use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;
use crate::types::Novel;

#[derive(Properties, PartialEq, Clone)]
pub struct RankingListProps {
    pub title: String,
    pub items: Vec<Novel>,
    pub show_date: bool,
}

#[function_component(RankingList)]
pub fn ranking_list(props: &RankingListProps) -> Html {
    let items: Vec<Html> = props.items.iter().enumerate().map(|(i, n)| {
        html! {
            <li>
                <span>{ i + 1 }</span>
                { " " }
                <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                <span style="float:right">
                    if props.show_date {
                        { &n.last_update }
                    } else {
                        { &n.author }
                    }
                </span>
            </li>
        }
    }).collect();

    html! {
        <>
            <h2>{ &props.title }</h2>
            <ul>
                { items }
            </ul>
        </>
    }
}
