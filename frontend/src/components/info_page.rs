use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    #[prop_or("sql".into())]
    pub language: AttrValue,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CodeHighlight)]
pub fn code_highlight(
    Props {
        id,
        language,
        children,
        class,
    }: &Props,
) -> Html {
    html!(
        <div class={classes!(class.clone())} id={id.clone()}>
            <script>{format!("
            var div = document.querySelector('#{id} .code-try');
            var x = hljs.highlight(div.innerHTML, {{language: '{language}'}});
            div.innerHTML = x.value;
            ")}</script>
            <pre><code class={classes!("code-try")}>
                {children.clone()}
            </code></pre>
        </div>
    )
}

#[function_component(InfoPage)]
pub fn info_page() -> Html {
    use super::Page;
    let create_users = "CREATE TABLE users(
    id integer primary key auto_increment,
    username varchar(390) not null unique,
    password_hash varchar(500) not null,
    is_admin boolean not null default false,
    last_token_generation datetime
)";
    let create_pets = "CREATE TABLE pets(
    id integer primary key auto_increment,
    name varchar(390) not null,
    pettype varchar(100) not null,
    owner_id integer not null references users,
    registration_time datetime,
    CONSTRAINT unique_pet_per_owner
            UNIQUE(name, pettype, owner_id)
)";
    let login_user = "SELECT username FROM users WHERE username='{username}' AND password_hash='{hashed_password}'";
    let get_pets =
        "SELECT name, pettype FROM pets WHERE owner_id={owner_id} AND name LIKE '{name_pattern}'";
    html!(
            <Page title="Info" info_icon=false home_icon=true key_icon=false>
                <h1>{"How to use this page"}</h1>
                <div class={classes!("introduction")}>
                <p>
    {"This page's purpose is to demonstrate how sql injections work and how dangerous they can be. You can complete different exercises for learning about sql injections."}
                </p>
                <ol>
                    <li>{"Login as a regular user"}<br/><i style={"font-size: 0.39em"}>{"If you fail: 'Gooc' is the name of one user encoded with Caesar Cipher"}</i></li>
                    <li>{"Get all user's names"}</li>
                    <li>{"Find out who's the administrator"}
                        <ol type="a">
                            <li>{"Find out what's the administrator's password hash"}</li>
                            <li>{"Use the cracking page (somewhere will be a key button) to find out the password of the admin"}</li>
                        </ol>
                    </li>
                    <li>{"Login as administrator"}</li>
                </ol>
                <p><b>{"All passwords are completly random and too long too guess them. So don't waste your time trying!"}</b></p>
                <p>{"The database's structure is given to you as a hint:"}</p>
                </div>
                <div class={classes!("code-create-tables")}>
                    <CodeHighlight id={"code-create-user-table"}>{create_users}</CodeHighlight>
                    <CodeHighlight id={"code-create-pet-table"}>{create_pets}</CodeHighlight>
                </div>
                <div class={classes!("introduction")}>
                <p>{"Some of the used statements are listed below too:"}</p>
                <div class={classes!("code-select-tables")}>
                    <CodeHighlight id={"code-select-user-login"}>{login_user}</CodeHighlight>
                    <CodeHighlight id={"code-select-pets"}>{get_pets}</CodeHighlight>
                </div>
                </div>
            </Page>
        )
}
