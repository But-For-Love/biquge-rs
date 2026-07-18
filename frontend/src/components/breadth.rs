use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct BreadcrumbProps {
    pub segments: Vec<(String, Route)>,
}

#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
    html! {
        <div class="box_con">
            <div class="con_top">
                { props.segments.iter().enumerate().map(|(i, (label, route))| {
                    let is_last = i == props.segments.len() - 1;
                    if is_last {
                        html! { <strong>{ label }</strong> }
                    } else {
                        html! {
                            <>
                                <Link<Route> to={route.clone()}>{ label }</Link<Route>>
                                { " > " }
                            </>
                        }
                    }
                }).collect::<Html>() }
            </div>
        </div>
    }
}
