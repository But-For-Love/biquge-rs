use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps {
    pub page: i64,
    pub total_pages: i64,
    pub on_page: Callback<i64>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    if props.total_pages <= 1 {
        return html! {};
    }

    let mut pages: Vec<i64> = Vec::new();
    pages.push(1);
    let start = (props.page - 2).max(2);
    let end = (props.page + 2).min(props.total_pages - 1);
    if start > 2 {
        pages.push(-1);
    }
    for p in start..=end {
        pages.push(p);
    }
    if end < props.total_pages - 1 {
        pages.push(-2);
    }
    if props.total_pages > 1 {
        pages.push(props.total_pages);
    }

    let page_items: Vec<Html> = pages.iter().map(|&p| {
        let on_page = props.on_page.clone();
        if p < 0 {
            html! { <span>{ "…" }</span> }
        } else if p == props.page {
            html! { <span class="current">{ p }</span> }
        } else {
            html! {
                <a href="#" onclick={Callback::from(move |e: yew::events::MouseEvent| {
                    e.prevent_default();
                    on_page.emit(p);
                })}>{ p }</a>
            }
        }
    }).collect();

    let next_btn = if props.page < props.total_pages {
        let on_page = props.on_page.clone();
        let next = props.page + 1;
        html! {
            <a href="#" onclick={Callback::from(move |e: yew::events::MouseEvent| {
                e.prevent_default();
                on_page.emit(next);
            })}>{ "下一页 →" }</a>
        }
    } else {
        html! {}
    };

    html! {
        <div class="paginate">
            { page_items }
            { next_btn }
        </div>
    }
}
