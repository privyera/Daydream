use crate::app::components::events::{get_sender_displayname, is_new_user};
use matrix_sdk::{
    events::room::message::{MessageEvent, VideoMessageEventContent},
    Room,
};
use rand::random;
use yew::prelude::*;

pub(crate) struct Video {
    props: Props,
}

#[derive(Clone, Properties, Debug)]
pub struct Props {
    #[prop_or_default]
    pub prev_event: Option<MessageEvent>,
    #[prop_or_default]
    pub event: Option<MessageEvent>,
    #[prop_or_default]
    pub video_event: Option<VideoMessageEventContent>,
    #[prop_or_default]
    pub room: Option<Room>,
}

impl Component for Video {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Video { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        // TODO fix the PartialEq hack
        if format!("{:#?}", self.props) != format!("{:#?}", props) {
            self.props = props;
            true
        } else {
            false
        }
    }

    //noinspection RsTypeCheck
    fn view(&self) -> Html {
        let new_user = is_new_user(
            self.props.prev_event.clone(),
            self.props.event.clone().unwrap(),
        );
        let sender_displayname = if new_user {
            get_sender_displayname(
                self.props.room.clone().unwrap(),
                self.props.event.clone().unwrap(),
            )
        } else {
            "".to_string()
        };

        let _caption = format!(
            "{}: {}",
            sender_displayname,
            self.props.video_event.as_ref().unwrap().body
        );
        if self
            .props
            .video_event
            .as_ref()
            .unwrap()
            .url
            .clone()
            .is_some()
        {
            let video_url = self
                .props
                .video_event
                .as_ref()
                .unwrap()
                .url
                .clone()
                .unwrap();
            let thumbnail = match self
                .props
                .video_event
                .as_ref()
                .unwrap()
                .info
                .clone()
                .unwrap()
                .thumbnail_url
            {
                None => video_url.clone(),
                Some(v) => v,
            };
            let lightbox_id: u8 = random();
            let lightbox_id_full = format!("video_{}", lightbox_id);
            let lightbox_href_full = format!("#video_{}", lightbox_id);
            if new_user {
                html! {
                    <div>
                        <p><displayname>{sender_displayname}{": "}</displayname></p>
                        <a href={lightbox_href_full.clone()}><img src=thumbnail/></a>
                        <div class="lightbox short-animate" id={lightbox_id_full.clone()}>
                            <video class="long-animate" controls=true>
                              <source src=video_url type="video/mp4"/>
                            {"Your browser does not support the video tag."}
                            </video>
                        </div>
                        <div id="lightbox-controls" class="short-animate">
                            <a id="close-lightbox" class="long-animate" href="#!">{"Close Lightbox"}</a>
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div>
                        <a href={lightbox_href_full.clone()}><img src=thumbnail/></a>
                        <div class="lightbox short-animate" id={lightbox_id_full.clone()}>
                            <video class="long-animate" controls=true>
                              <source src=video_url type="video/mp4"/>
                            {"Your browser does not support the video tag."}
                            </video>
                        </div>
                        <div id="lightbox-controls" class="short-animate">
                            <a id="close-lightbox" class="long-animate" href="#!">{"Close Lightbox"}</a>
                        </div>
                    </div>
                }
            }
        } else {
            html! {}
        }
    }
}
