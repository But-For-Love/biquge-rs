use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <div class="footer_link"></div>
            <div class="footer_cont">
                <p>{ "本站所有小说为转载作品，所有章节均由网友上传，转载至本站只是为了宣传本书让更多读者欣赏。" }</p>
                <p>{ "Copyright © 2024 笔趣阁 All Rights Reserved." }</p>
                <p>{ "备案号：XX-XXXXXXXX号" }</p>
            </div>
        </div>
    }
}
