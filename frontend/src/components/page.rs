use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::MainRoute;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub title: Html,
    pub children: Children,
    #[prop_or(false)]
    pub logout: bool,
    #[prop_or(false)]
    pub key_icon: bool,
    #[prop_or_default]
    pub home_icon: Option<bool>,
    #[prop_or(true)]
    pub info_icon: bool,
}

#[function_component(Page)]
pub fn page(
    PageProps {
        title,
        children,
        logout,
        key_icon,
        home_icon,
        info_icon,
    }: &PageProps,
) -> Html {
    use super::MaterialIcon;

    let home_icon = home_icon.unwrap_or(!key_icon);

    let history = use_navigator().unwrap();

    let cloned_history = history.clone();
    let onlogout = Callback::from(move |_| cloned_history.push(&MainRoute::Logout));
    //let onkey = Callback::from(move |_| cloned_history.push(&MainRoute::Cracking));
    let cloned_history = history.clone();
    let onhome = Callback::from(move |_| cloned_history.push(&MainRoute::Home));
    //let oninfo = Callback::from(move |_| history.push(&MainRoute::Info));

    html!(
        <>
        <div class={classes!("left-main-border")}></div>
        <div class={classes!("right-main-border")}></div>
        <div class={classes!("main-middle")}>

        <main class={"content"}>
        <header>
            <span class={classes!("page-title-text")}>{title.clone()}</span>
            <div class={classes!("page-title-icons")}>
            {
                if *info_icon {
                    html!(
                        <Link<MainRoute> to={MainRoute::Info} classes={classes!("page-info-btn", "material-icon-btn")}>
                        //<button class={classes!("page-info-btn", "material-icon-btn")} onclick={oninfo}>
                        <MaterialIcon name={"info"}/>
                        //</button>
                        </Link<MainRoute>>
                    )
                } else {html!()}
            }
            {
                if *key_icon {
                    html!(
                        <Link<MainRoute> to={MainRoute::Cracking} classes={classes!("page-key-btn", "material-icon-btn")}>
                        //<button class={classes!("page-key-btn", "material-icon-btn")} onclick={onkey}>
                        <MaterialIcon name={"key"}/>
                        //</button>
                        </Link<MainRoute>>
                    )
                } else {html!()}
            }
            {
                if home_icon {
                    html!(
                        <button class={classes!("page-home-btn")} onclick={onhome}>
                        <MaterialIcon name={"home"}/>
                        </button>
                    )
                } else {html!()}
            }
            {
                if *logout {
                    html!(
                        <button class={classes!("page-logout-btn", "material-icon-btn")} onclick={onlogout}>
                        <MaterialIcon name={"logout"}/>
                        </button>
                    )
                } else {html!()}
            }
            </div>
        </header>
            {children}
        </main>
        </div>
        </>
    )
}
