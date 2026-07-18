use yew::prelude::*;

#[function_component(TopBar)]
pub fn top_bar() -> Html {
    html! {
        <div class="ywtop">
            <div class="ywtop_con">
                <div class="ywtop_sethome"><a href="#">{ "将笔趣阁设为首页" }</a></div>
                <div class="ywtop_addfavorite"><a href="#">{ "收藏笔趣阁" }</a></div>
                <div class="nri">
                    <div class="cc"><span class="txt">{ "账号：" }</span><input type="text" name="username" placeholder="用户名" /></div>
                    <div class="cc"><span class="txt">{ "密码：" }</span><input type="password" name="password" placeholder="密码" /></div>
                    <input type="submit" value="登录" />
                    <div class="ccc"><a href="#">{ "忘记密码" }</a><a href="#">{ "用户注册" }</a></div>
                </div>
            </div>
        </div>
    }
}
