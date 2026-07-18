use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::app::Route;
use crate::components::breadth::Breadcrumb;
use crate::components::novel_card::NovelCardWide;
use crate::components::pagination::Pagination;
use crate::types::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CategoryPageProps {
    pub slug: String,
}

#[function_component(CategoryPage)]
pub fn category_page(props: &CategoryPageProps) -> Html {
    let novels = use_state(|| Vec::<NovelWithLatest>::new());
    let total = use_state(|| 0i64);
    let total_pages = use_state(|| 0i64);
    let page = use_state(|| 1i64);
    let loading = use_state(|| true);
    let cat_name = use_state(|| String::new());
    let cat_id = use_state(|| None::<i64>);
    let categories = use_state(|| Vec::<Category>::new());

    let slug = props.slug.clone();

    // Reset page when slug changes
    {
        let page = page.clone();
        let slug = slug.clone();
        use_effect_with(slug, move |_| {
            page.set(1);
            || ()
        });
    }

    // Fetch categories to get the matching cat_id
    {
        let categories = categories.clone();
        let cat_id = cat_id.clone();
        let slug2 = slug.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(cats) = api::fetch_categories().await {
                    for c in &cats {
                        if c.slug == slug2 {
                            cat_id.set(Some(c.id));
                            break;
                        }
                    }
                    categories.set(cats);
                }
            });
            || ()
        });
    }

    // Fetch novels when cat_id or page changes
    {
        let novels = novels.clone();
        let loading = loading.clone();
        let total = total.clone();
        let total_pages = total_pages.clone();
        let cat_name = cat_name.clone();
        let cat_id_val = *cat_id;
        let page_val = *page;

        use_effect_with((cat_id_val, page_val), move |(cid, p)| {
            if let Some(cid_val) = *cid {
                let novels = novels.clone();
                let loading = loading.clone();
                let total = total.clone();
                let total_pages = total_pages.clone();
                let cat_name = cat_name.clone();
                let page_val = *p;
                wasm_bindgen_futures::spawn_local(async move {
                    match api::fetch_novels(Some(cid_val), page_val, "click").await {
                        Ok(resp) => {
                            novels.set(resp.novels);
                            total.set(resp.total);
                            total_pages.set((resp.total + resp.page_size - 1) / resp.page_size);
                            if let Some(name) = resp.category_name {
                                cat_name.set(name);
                            }
                            loading.set(false);
                        }
                        Err(_) => {
                            loading.set(false);
                        }
                    }
                });
            }
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
            <Breadcrumb segments={vec![
                ("首页".into(), Route::Home),
                ((*cat_name).clone(), Route::Category { slug: slug.clone() }),
            ]} />

            // Featured novels
            if !(*loading) && !novels.is_empty() {
                <div id="hotcontent">
                    <div class="ll">
                        { novels.iter().take(3).map(|n| html! {
                            <NovelCardWide novel={Novel {
                                id: n.id, title: n.title.clone(), author: n.author.clone(),
                                cover_url: n.cover_url.clone(), intro: n.intro.clone(),
                                category_id: n.category_id, status: n.status.clone(),
                                last_update: n.last_update.clone(), created_at: n.created_at.clone(),
                                click_count: n.click_count, recommend_count: n.recommend_count,
                            }} />
                        }).collect::<Html>() }
                    </div>
                </div>
            }

            <div class="ad-placeholder">{ "— 广告位 —" }</div>

            // Novel list
            if !(*loading) {
                <div class="novelslist">
                    <div class="content list-block" style="border-right:0;flex:1">
                        <h2>{ format!("好看的{}小说列表", cat_name.as_str()) }</h2>
                        <ul>
                            { novels.iter().map(|n| {
                                let cat_tag = get_cat_short(n.category_id);
                                html! {
                                    <li>
                                        { "【" }{ cat_tag }{ "】 " }
                                        <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                        <span style="float:right;color:#B3B3B3">
                                            { format!("{} · {}", &n.author, &n.last_update) }
                                        </span>
                                    </li>
                                }
                            }).collect::<Html>() }
                        </ul>
                    </div>
                </div>
            }

            <Pagination page={*page} total_pages={*total_pages} on_page={on_page} />

            // Latest updates section
            if !(*loading) && !novels.is_empty() {
                <div id="newscontent">
                    <div class="l">
                        <h2>{ format!("{} · 最新更新", cat_name.as_str()) }</h2>
                        <ul>
                            { novels.iter().take(6).map(|n| {
                                let cat_tag = get_cat_short(n.category_id);
                                html! {
                                    <li>
                                        { "【" }{ cat_tag }{ "】 " }
                                        <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                        { " — " }
                                        if let Some(ref ch) = n.latest_chapter_title {
                                            { ch }
                                        }
                                        <span style="float:right;color:#B3B3B3">{ &n.last_update }</span>
                                    </li>
                                }
                            }).collect::<Html>() }
                        </ul>
                    </div>
                    <div class="r">
                        <h2>{ format!("{}热门", cat_name.as_str()) }</h2>
                        <ul>
                            { novels.iter().take(6).enumerate().map(|(i, n)| html! {
                                <li>
                                    <span>{ i + 1 }</span>
                                    { " " }
                                    <Link<Route> to={Route::NovelDetail { id: n.id }}>{ &n.title }</Link<Route>>
                                    <span style="float:right;color:#B3B3B3">{ &n.author }</span>
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
