use crate::{
    basic_components::{
        Button, ButtonGroup, ButtonType, Select, Slider, Switch, TextField, UpDown,
    },
    translation::use_trans,
};
use alvr_common::{data::SessionDesc, logging::Event};
use std::{cell::RefCell, rc::Rc};
use yew::{html, Callback, Properties};
use yew_functional::{function_component, use_state};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub events_callback_ref: Rc<RefCell<Callback<Event>>>,
    pub session: SessionDesc,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &Props) -> Html {
    let (label, set_label) = use_state(|| "Hello".to_owned());

    *props.events_callback_ref.borrow_mut() = Callback::from(|event| ());

    let on_click = {
        let label = Rc::clone(&label);
        Callback::from(move |_| set_label(format!("{} world", label)))
    };

    let default_string = use_trans("default");

    let switch_on_click = Callback::from(move |_| ());

    let slider_on_change = Callback::from(move |_| ());

    let on_select = Callback::from(move |_| ());

    let text_field_on_focus_lost = Callback::from(move |_| ());

    let up_down_on_step = Callback::from(move |_| ());

    html! {
        <>
            <div class="flex flex-col space-y-2 items-start">
                <Button on_click=on_click.clone() button_type=ButtonType::None>
                    {label.clone()}
                </Button>
                <Button on_click=on_click.clone() button_type=ButtonType::Primary>
                    {label.clone()}
                </Button>
                <Button on_click=on_click.clone() button_type=ButtonType::Secondary>
                    {label.clone()}
                </Button>
                <Button on_click=on_click button_type=ButtonType::Danger>
                    {label}
                </Button>
            </div>
            <Switch on_click=switch_on_click checked=true/>
            <Slider value="0" default="30" min="-1" max="40" step="0.5" on_change=slider_on_change/>
            <ButtonGroup
                options=vec!["hello1".into(), "hello2".into()]
                selected="hello1"
                on_select=on_select.clone()
            />
            <Select
                options=vec!["hello1".into(), "hello2".into()]
                selected="hello1"
                on_select=on_select
            />
            <div class="space-y-2">
                <TextField
                    value=default_string.clone()
                    on_focus_lost=text_field_on_focus_lost.clone()
                />
                <TextField
                    label="Hi there"
                    value=default_string
                    on_focus_lost=text_field_on_focus_lost.clone()
                />
            </div>
            <UpDown
                value="123"
                on_focus_lost=text_field_on_focus_lost
                on_step_down=up_down_on_step.clone()
                on_step_up=up_down_on_step
            />
        </>
    }
}
