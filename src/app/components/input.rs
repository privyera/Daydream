use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct InputProps {
    pub on_submit: Callback<String>,
}

pub struct InputState {
    value: Option<String>,
}

pub struct Input {
    on_input: Callback<InputData>,
    on_submit: Callback<KeyboardEvent>,
    state: InputState,
    props: InputProps,
}

#[allow(clippy::large_enum_variant)]
pub enum Msg {
    ValueChange(InputData),
    ValueSubmit(KeyboardEvent),
}

impl Component for Input {
    type Message = Msg;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = InputState { value: None };
        Self {
            props,
            on_input: link.callback(Msg::ValueChange),
            on_submit: link.callback(Msg::ValueSubmit),
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ValueChange(data) => {
                self.state.value = Some(data.value);
                true
            }
            Msg::ValueSubmit(data) => {
                if data.key() == "Enter" {
                    self.props
                        .on_submit
                        .emit(self.state.value.as_ref().unwrap_or(&"".to_string()).clone());
                    self.state.value = None;
                    return true;
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="uk-inline" style="display: block !important;">
                <span class="material-icons" id="ma-icon">{"create"}</span>
                <input class="uk-input" type="text"
                    value=&self.state.value.as_ref().unwrap_or(&"".to_string())
                    oninput=&self.on_input
                    onkeypress=&self.on_submit
                />
            </div>
        }
    }
}
