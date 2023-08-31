use leptos::{
    ev::SubmitEvent,
    html::{Input, Textarea},
    *,
};

use crate::api::parse::parse_log;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let log_file_ref = create_node_ref::<Input>(cx);
    let log_text_ref = create_node_ref::<Textarea>(cx);

    // let submit_button_handler = move |_event: web_sys::Event| {
    //     let target = log_file_ref.get().unwrap();
    //     if let Some(files) = target.files() {
    //         if files.length().gt(&0) {
    //             log!("{}", "Here");
    //         }
    //     }
    // };

    let parse_log = create_action(cx, move |_ev: &SubmitEvent| {
        let file_path = log_file_ref.get().expect("fileee").value();

        log!("{}", file_path);

        let log = match log_text_ref.get() {
            Some(log) => {
                if log.value().is_empty() {
                    "empty".to_string()
                } else {
                    log.value()
                }
            }
            None => "failed".to_string(),
        };
        log!("{}", format!("log text ref: {:?}", log));

        parse_log(log)
    });

    view! { cx,
        <h1>"Quake 3 Arena server log parser"</h1>
        <div>
            <p>
                "This applications focus on parse and represent data from a Quake 3 Arena server log in a
                more human readable version the file or text expected to be parsed has the following format:"
            </p>
            <pre>
                {r#"
                    0:00 ------------------------------------------------------------
                    0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
                    0:25 ClientConnect: 2
                    0:25 ClientUserinfoChanged: 2 n\Dono da Bola\t\0\model\sarge/krusade\hmodel\sarge/krusade\g_redteam\\g_blueteam\\c1\5\c2\5\hc\95\w\0\l\0\tt\0\tl\0
                    0:27 ClientUserinfoChanged: 2 n\Mocinha\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0
                    0:27 ClientBegin: 2
                    0:29 Item: 2 weapon_rocketlauncher
                    0:35 Item: 2 item_armor_shard
                    0:35 Item: 2 item_armor_shard
                    0:35 Item: 2 item_armor_shard
                    0:35 Item: 2 item_armor_combat
                    0:38 Item: 2 item_armor_shard
                    0:38 Item: 2 item_armor_shard
                    0:38 Item: 2 item_armor_shard
                    0:55 Item: 2 item_health_large
                    0:56 Item: 2 weapon_rocketlauncher
                    0:57 Item: 2 ammo_rockets
                    0:59 ClientConnect: 3
                    0:59 ClientUserinfoChanged: 3 n\Isgalamido\t\0\model\xian/default\hmodel\xian/default\g_redteam\\g_blueteam\\c1\4\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                    1:01 ClientUserinfoChanged: 3 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                    1:01 ClientBegin: 3
                    1:02 Item: 3 weapon_rocketlauncher
                    1:04 Item: 2 item_armor_shard
                    1:04 Item: 2 item_armor_shard
                    1:04 Item: 2 item_armor_shard
                    1:06 ClientConnect: 4
                    1:06 ClientUserinfoChanged: 4 n\Zeh\t\0\model\sarge/default\hmodel\sarge/default\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                    1:08 Kill: 3 2 6: Isgalamido killed Mocinha by MOD_ROCKET
                    1:08 ClientUserinfoChanged: 4 n\Zeh\t\0\model\sarge/default\hmodel\sarge/default\g_redteam\\g_blueteam\\c1\1\c2\5\hc\100\w\0\l\0\tt\0\tl\0
                    1:08 ClientBegin: 4
                    1:10 Item: 3 item_armor_shard
                    1:10 Item: 3 item_armor_shard
                    1:10 Item: 3 item_armor_shard
                    1:10 Item: 3 item_armor_combat
                    1:11 Item: 4 weapon_shotgun
                    1:11 Item: 4 ammo_shells
                    1:16 Item: 4 item_health_large
                    1:18 Item: 4 weapon_rocketlauncher
                    1:18 Item: 4 ammo_rockets
                    1:26 Kill: 1022 4 22: <world> killed Zeh by MOD_TRIGGER_HURT
                    1:26 ClientUserinfoChanged: 2 n\Dono da Bola\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0
                    1:26 Item: 3 weapon_railgun
                    1:29 Item: 2 weapon_rocketlauncher
                    1:29 Item: 3 weapon_railgun
                    1:32 Item: 3 weapon_railgun
                    1:32 Kill: 1022 4 22: <world> killed Zeh by MOD_TRIGGER_HURT
                    1:35 Item: 2 item_armor_shard
                    1:35 Item: 2 item_armor_shard
                    1:35 Item: 2 item_armor_shard
                    1:35 Item: 3 weapon_railgun
                    1:38 Item: 2 item_health_large
                    1:38 Item: 3 weapon_railgun
                    1:41 Kill: 1022 2 19: <world> killed Dono da Bola by MOD_FALLING
                    1:41 Item: 3 weapon_railgun
                    1:43 Item: 2 ammo_rockets
                    1:44 Item: 2 weapon_rocketlauncher
                    1:46 Item: 2 item_armor_shard
                    1:47 Item: 2 item_armor_shard
                    1:47 Item: 2 item_armor_shard
                    1:47 ShutdownGame:
                    1:47 ------------------------------------------------------------
                "#}
            </pre>
        </div>
        <div>
            <form on:submit=move |ev| {
                ev.prevent_default();

                parse_log.dispatch(ev)
            }>
                <p>"An file input will have preference over a pasted log"</p>
                <label>
                    <span>"Upload a server log file"</span>
                    <input
                        type="file"
                        node_ref=log_file_ref
                        id="file-input"
                        _ref=log_file_ref
                        class="upload"
                        name="file-input"
                        accept="application/log"
                        // on:change=submit_button_handler
                    />
                </label>
                <p>"Or"</p>
                <label>
                    <span>"Paste the log text here"</span>
                    <textarea
                        type="text"
                        placeholder="Paste the log text here"
                        node_ref=log_text_ref
                    />
                </label>

                <button type="submit">"Parse"</button>
            </form>
        </div>
    }
}
