use std::collections::HashMap;

use leptos::{html::Input, *};

use wasm_bindgen::prelude::wasm_bindgen;

use reqwest::{multipart, Client, Url};

use crate::model::parser::MatchRecord;

#[wasm_bindgen]
pub async fn file_to_u8(file: web_sys::File) -> Result<js_sys::Uint8Array, wasm_bindgen::JsValue> {
    let buffer = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
        .await
        .unwrap();
    let u8_array = js_sys::Uint8Array::new(&buffer);
    Ok(u8_array)
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let log_file_ref = create_node_ref::<Input>(cx);

    pub fn submit_callback(file_ref: NodeRef<Input>) -> impl Fn(web_sys::Event) {
        move |_event: web_sys::Event| {
            let file_input = file_ref.get().expect("could not capture file input");

            let file = file_input.files().unwrap().get(0).unwrap();

            let buffer = file_to_u8(file);

            spawn_local(async move {
                let buffer = buffer.await.unwrap();
                let mut body = vec![0; buffer.length() as usize];
                buffer.copy_to(&mut body[..]);

                let file = multipart::Part::bytes::<Vec<u8>>(body).file_name("server-log");
                let form = reqwest::multipart::Form::new().part("log", file);

                let client = Client::new();

                let url = Url::parse("http://127.0.0.1:3000/c/parse_log_file").unwrap();

                let res = client.post(url).multipart(form).send().await;
                let res_body = res
                    .expect("failed to get response")
                    .json::<HashMap<String, MatchRecord>>()
                    .await
                    .expect("failed to get payload");

                log!("response: {:?}", res_body);
            })
        }
    }

    let submit_handler = submit_callback(log_file_ref);

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
            <div>
                <p>"An file input will have preference over a pasted log"</p>
                <label>
                    <span>"Upload a server log file:"</span>
                    <input
                        type="file"
                        id="file-input"
                        _ref=log_file_ref
                        name="file-input"
                        accept="application/text"
                        on:change=submit_handler
                    />
                </label>
            </div>
        </div>
    }
}
