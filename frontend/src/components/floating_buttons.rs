use yew::prelude::*;

#[function_component(FloatingButtons)]
pub fn floating_buttons() -> Html {
    html! {
        <>
            <div class="reader_mark1"><a href="#" title="加入书架">{ "书架" }</a></div>
            <div class="reader_mark0"><a href="#" title="投推荐票">{ "投票" }</a></div>
        </>
    }
}
