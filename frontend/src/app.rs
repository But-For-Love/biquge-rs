use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::nav::Nav;
use crate::components::top_bar::TopBar;
use crate::pages::all_novels::AllNovelsPage;
use crate::pages::category::CategoryPage;
use crate::pages::home::HomePage;
use crate::pages::novel::NovelPage;
use crate::pages::rankings::RankingsPage;
use crate::pages::reader::ChapterReaderPage;
use crate::pages::search::SearchPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/category/:slug")]
    Category { slug: String },
    #[at("/novel/:id")]
    NovelDetail { id: i64 },
    #[at("/novel/:novel_id/:chapter_id")]
    ChapterReader { novel_id: i64, chapter_id: i64 },
    #[at("/search/:q")]
    Search { q: String },
    #[at("/rankings")]
    Rankings,
    #[at("/all")]
    AllNovels,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    let route_clone = route.clone();
    let current_nav = match &route_clone {
        Route::Home => "home",
        Route::Category { slug } => slug.as_str(),
        Route::NovelDetail { .. } => "",
        Route::ChapterReader { .. } => "",
        Route::Search { .. } => "",
        Route::Rankings => "rankings",
        Route::AllNovels => "all",
        Route::NotFound => "",
    };

    let page = match route {
        Route::Home => html! { <HomePage /> },
        Route::Category { slug } => html! { <CategoryPage slug={slug} /> },
        Route::NovelDetail { id } => html! { <NovelPage id={id} /> },
        Route::ChapterReader { novel_id, chapter_id } => html! {
            <ChapterReaderPage novel_id={novel_id} chapter_id={chapter_id} />
        },
        Route::Search { q } => html! { <SearchPage q={q} /> },
        Route::Rankings => html! { <RankingsPage /> },
        Route::AllNovels => html! { <AllNovelsPage /> },
        Route::NotFound => html! {
            <div id="main" style="text-align:center;padding:100px;">
                <h1 style="font-size:72px;color:#88C6E5;">{ "404" }</h1>
                <p style="font-size:18px;color:#999;">{ "页面不存在" }</p>
                <Link<Route> to={Route::Home}>{ "返回首页" }</Link<Route>>
            </div>
        },
    };

    html! {
        <>
            <TopBar />
            <div id="wrapper">
                <Header />
                <Nav current={current_nav.to_string()} />
                { page }
                <Footer />
            </div>
        </>
    }
}
