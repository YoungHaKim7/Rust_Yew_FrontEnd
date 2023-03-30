use gloo_net::http::Request;
use yew::function_component; // replace with your own path
use yew::prelude::*;

use crate::app::structvideo::Video;
use crate::app::videosdetails::VideoDetails;
mod videosdetails;
use crate::app::videoslist::VideosList;
mod structvideo;
mod videoslist;

#[function_component(App)]
pub fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
            <>
                <h1>{ "RustConf Explorer" }</h1>
                <div>
                    <h3>{"Videos to watch"}</h3>
                    <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
                </div>
                { for details }
                <div>
                    <h3>{ "John Doe: Building and breaking things" }</h3>
                    <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
                </div>
            </>
    }
}
