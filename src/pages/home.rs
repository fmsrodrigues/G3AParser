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
    let (logs, set_logs) = create_signal::<Vec<(String, MatchRecord)>>(cx, Vec::new());
    let log_file_ref = create_node_ref::<Input>(cx);

    pub fn submit_callback(
        file_ref: NodeRef<Input>,
        set_logs: WriteSignal<Vec<(String, MatchRecord)>>,
    ) -> impl Fn(web_sys::Event) {
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

                let url = Url::parse("http://127.0.0.1:3000/api/parse_log_file").unwrap();

                let res = client.post(url).multipart(form).send().await;
                let res_body = res
                    .expect("failed to get response")
                    .json::<HashMap<String, MatchRecord>>()
                    .await
                    .expect("failed to get payload");

                let mut ordered_logs = Vec::new();
                for (match_number, record) in res_body.iter() {
                    ordered_logs.push((match_number.to_owned(), record.to_owned()));
                }
                ordered_logs.sort_by(|a, b| {
                    let a =
                        a.0.chars()
                            .filter(|c| c.is_numeric())
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();
                    let b =
                        b.0.chars()
                            .filter(|c| c.is_numeric())
                            .collect::<String>()
                            .parse::<i32>()
                            .unwrap();

                    a.cmp(&b)
                });

                set_logs(ordered_logs);
            });
        }
    }

    let submit_handler = submit_callback(log_file_ref, set_logs);

    view! { cx,
                <div class="w-full flex flex-col gap-8">
                    <h1 class="text-3xl font-bold text-center">"Quake 3 Arena server log parser"</h1>

                    <div class="w-full flex flex-col items-center gap-4">
                        <p class="text-xl">
                            "This application focus on parse and represent data from a Quake 3 Arena server log in a
                            more human readable version the file. The expected file to be parsed has a similar format to the following:"
                        </p>
                        <div class="flex p-4 w-1/2 h-[480px] overflow-auto bg-[#3c4450] rounded-lg text-left">
                            <pre class="text-sm">
    {r#"
0:00 ------------------------------------------------------------
0:00 InitGame:
0:25 ClientConnect: 2
0:25 ClientUserinfoChanged: 2 n\Dono da Bola
0:27 ClientUserinfoChanged: 2 n\Mocinha
0:27 ClientBegin: 2
0:29 Item: 2 weapon_rocketlauncher
0:59 ClientConnect: 3
0:59 ClientUserinfoChanged: 3 n\Isgalamido
1:01 ClientBegin: 3
1:02 Item: 3 weapon_rocketlauncher
1:06 ClientConnect: 4
1:08 Kill: 3 2 6: Isgalamido killed Mocinha by MOD_ROCKET
1:08 ClientUserinfoChanged: 4 n\Zeh\
1:08 ClientBegin: 4
1:10 Item: 3 item_armor_combat
1:18 Item: 4 ammo_rockets
1:41 Kill: 1022 2 19: <world> killed Dono da Bola by MOD_FALLING
1:41 Item: 3 weapon_railgun
1:47 Item: 2 item_armor_shard
1:47 ShutdownGame:
1:47 ------------------------------------------------------------
"#}
                            </pre>
                        </div>
                    </div>

                    <div>
                        <div>
                            <label class="text-md">
                                <span class="block mb-2 font-medium">"Upload a server log file:"</span>
                                <input
                                    type="file"
                                    accept=".log,.txt"
                                    id="file-input"
                                    _ref=log_file_ref
                                    name="file-input"
                                    on:change=submit_handler
                                    class="block w-full border rounded-lg cursor-pointer text-gray-400 focus:outline-none bg-gray-700 border-gray-600 placeholder-gray-400"
                                />
                                <p class="mt-1 text-sm text-gray-300">"TXT or LOG."</p>
                            </label>
                        </div>

                        {move || {
                            if  logs.get().is_empty() {
                                view! { cx,
                                    <div  class="flex flex-col my-12 gap-12 justify-center">
                                        <p class="text-xl text-center">"No logs parsed yet"</p>
                                    </div> }
                            } else {
                                view! { cx,
                                <div class="flex flex-col my-12 gap-12">
                                    <For
                                        each={move || logs.get()}
                                        key={|(match_number, _)| match_number.to_owned()}
                                        view=move |cx, (match_number, record)| {
                                            let match_number = match_number.chars()
                                                .filter(|c| c.is_numeric())
                                                .collect::<String>()
                                                .parse::<i32>()
                                                .unwrap();

                                            view! {
                                                cx,
                                                <div class="flex flex-col gap-4 w-full">
                                                    <h2 class="text-2xl font-bold text-center">{format!("Match {match_number}")}</h2>
                                                    <div class="flex flex-row w-full justify-around">
                                                        <div>
                                                            <h3 class="text-xl font-medium text-center mb-2">"Players ranking"</h3>
                                                            <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                                                                <table class="w-full text-sm text-left text-gray-400">
                                                                    <thead class="text-xs uppercase bg-gray-700 text-gray-400">
                                                                        <tr>
                                                                            <th scope="col" class="px-6 py-3">"Ranking"</th>
                                                                            <th scope="col" class="px-6 py-3">"Player"</th>
                                                                            <th scope="col" class="px-6 py-3">"Kill Score"</th>
                                                                        </tr>
                                                                    </thead>
                                                                    <tbody>
                                                                        <For
                                                                            each={move || record.ranking.clone()}
                                                                            key={|ranking_position| ranking_position.player.to_owned()}
                                                                            view=move |cx, ranking_position| {
                                                                                view! {
                                                                                    cx,
                                                                                    <tr class="border-b bg-gray-900 border-gray-700">
                                                                                        <td class="px-6 py-4">{format!("{}", ranking_position.position)}</td>
                                                                                        <td class="px-6 py-4">{format!("{}", ranking_position.player)}</td>
                                                                                        <td class="px-6 py-4">{format!("{}", ranking_position.kills)}</td>
                                                                                    </tr>
                                                                                }
                                                                            }
                                                                        />
                                                                    </tbody>
                                                                </table>
                                                            </div>
                                                        </div>

                                                        <div>
                                                            <h3 class="text-xl font-medium text-center mb-2">"Means of kills"</h3>
                                                            {
                                                                if record.kills_by_means.is_empty() {
                                                                    view! { cx,
                                                                        <div>
                                                                            <p class="text-center">"No valid means of kills registered"</p>
                                                                        </div>
                                                                    }
                                                                } else {
                                                                    view! { cx,
                                                                        <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                                                                            <table class="w-full text-sm text-left text-gray-400">
                                                                                <thead class="text-xs uppercase bg-gray-700 text-gray-400">
                                                                                    <tr>
                                                                                        <th scope="col" class="px-6 py-3">"Damage source"</th>
                                                                                        <th scope="col" class="px-6 py-3">"Kills"</th>
                                                                                    </tr>
                                                                                </thead>
                                                                                <tbody>
                                                                                    <For
                                                                                        each={move || record.kills_by_means.clone()}
                                                                                        key={|(means, _)| means.to_owned()}
                                                                                        view=move |cx, (means, kills)| {
                                                                                            view! {
                                                                                                cx,
                                                                                                <tr class="border-b bg-gray-900 border-gray-700">
                                                                                                    <td class="px-6 py-4">{format!("{means}")}</td>
                                                                                                    <td class="px-6 py-4">{format!("{kills}")}</td>
                                                                                                </tr>
                                                                                            }
                                                                                        }
                                                                                    />
                                                                                </tbody>
                                                                            </table>
                                                                        </div>
                                                                    }
                                                                }
                                                            }
                                                        </div>
                                                    </div>
                                                    <span class="text-sm">{format!("Total match kills: {}", record.total_kills)}</span>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                                }
                                }
                            }
                        }
                    </div>
                </div>
            }
}
