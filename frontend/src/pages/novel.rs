use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::app::Route;
use crate::components::breadth::Breadcrumb;
use crate::types::NovelDetail;

#[derive(Properties, PartialEq, Clone)]
pub struct NovelPageProps {
    pub id: i64,
}

#[function_component(NovelPage)]
pub fn novel_page(props: &NovelPageProps) -> Html {
    let detail = use_state(|| None::<NovelDetail>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let detail = detail.clone();
        let loading = loading.clone();
        let error = error.clone();
        let id = props.id;
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match api::fetch_novel_detail(id).await {
                    Ok(Some(d)) => {
                        detail.set(Some(d));
                        loading.set(false);
                    }
                    Ok(None) => {
                        error.set(Some("小说不存在".into()));
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
            } else if let Some(ref d) = *detail {
                <NovelDetailContent detail={d.clone()} />
            }
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct NovelDetailContentProps {
    detail: NovelDetail,
}

#[function_component(NovelDetailContent)]
fn novel_detail_content(props: &NovelDetailContentProps) -> Html {
    let d = &props.detail;
    let novel = &d.novel;
    let status_class = if novel.status == "completed" { "a" } else { "b" };
    let cover = if novel.cover_url.is_empty() {
        format!("https://placehold.co/120x150/E1ECED/88C6E5?text={}", &novel.title.chars().take(4).collect::<String>())
    } else {
        novel.cover_url.clone()
    };

    // Chapter groups
    let chapters = &d.chapters;
    let latest_start = if chapters.len() > 12 { chapters.len() - 12 } else { 0 };
    let latest_chapters: Vec<_> = chapters.iter().skip(latest_start).collect();
    let earlier_chapters: Vec<_> = chapters.iter().take(latest_start).collect();

    html! {
        <div id="main">
            <Breadcrumb segments={vec![
                ("首页".into(), Route::Home),
                (d.category_name.clone(), Route::Category { slug: get_slug(novel.category_id).into() }),
                (novel.title.clone(), Route::NovelDetail { id: novel.id }),
            ]} />

            <div class="box_con">
                <div id="maininfo">
                    <div id="info">
                        <h1>{ &novel.title }</h1>
                        <div class="meta">
                            <p><label>{ "作 者：" }</label>{ &novel.author }</p>
                            <p><label>{ "动 作：" }</label><a href="#">{ "加入书架" }</a>{ ", " }<a href="#">{ "投推荐票" }</a>{ ", " }<a href="#footer">{ "直达底部" }</a></p>
                            <p>{ "最后更新：" }{ &novel.last_update }</p>
                            <p><label>{ "下 载：" }</label><span class="text-muted">{ "( TXT, CHM, UMD, JAR, APK, HTML )" }</span></p>
                        </div>
                    </div>
                    <div id="intro">
                        <p>{ &novel.intro }</p>
                    </div>
                </div>
                <div id="sidebar">
                    <div id="fmimg">
                        <img src={cover} width="120" height="150" alt={novel.title.clone()} />
                        <span class={status_class}></span>
                    </div>
                </div>
                <div id="listtj">
                    { "推荐阅读：" }
                    <a href="#">{ "完美世界" }</a>{ " | " }
                    <a href="#">{ "大主宰" }</a>{ " | " }
                    <a href="#">{ "武动乾坤" }</a>{ " | " }
                    <a href="#">{ "斗罗大陆" }</a>{ " | " }
                    <a href="#">{ "盘龙" }</a>{ " | " }
                    <a href="#">{ "星辰变" }</a>
                </div>
            </div>

            <div class="ad-placeholder">{ "— 广告位 —" }</div>

            <div class="box_con">
                <div id="list">
                    <dl>
                        if !latest_chapters.is_empty() {
                            <dt>{ format!("《{}》最新章节", novel.title) }</dt>
                            { latest_chapters.iter().map(|ch| html! {
                                <dd>
                                    <Link<Route> to={Route::ChapterReader { novel_id: novel.id, chapter_id: ch.id }}>
                                        { &ch.title }
                                    </Link<Route>>
                                </dd>
                            }).collect::<Html>() }
                        }
                        if !earlier_chapters.is_empty() {
                            <dt>{ "全部章节" }</dt>
                            { earlier_chapters.iter().map(|ch| html! {
                                <dd>
                                    <Link<Route> to={Route::ChapterReader { novel_id: novel.id, chapter_id: ch.id }}>
                                        { &ch.title }
                                    </Link<Route>>
                                </dd>
                            }).collect::<Html>() }
                        }
                    </dl>
                </div>
            </div>

            <div class="ad-placeholder">{ "— 广告位 —" }</div>
        </div>
    }
}

fn get_slug(cat_id: i64) -> &'static str {
    match cat_id {
        1 => "xuanhuan", 2 => "wuxia", 3 => "dushi", 4 => "lishi",
        5 => "youxi", 6 => "kehuan", 7 => "xiuzhen", 8 => "chuanyue",
        _ => "xuanhuan",
    }
}
