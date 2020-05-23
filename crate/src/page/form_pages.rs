use wasm_bindgen::JsCast;
use web_sys::HtmlOptionElement;
use yew::prelude::*;
use yew_styles::forms::{
    form_group::{FormGroup, Orientation},
    form_input::{FormInput, InputType},
    form_label::FormLabel,
    form_select::FormSelect,
};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::styles::{Palette, Size};

pub struct FormPage {
    pub link: ComponentLink<Self>,
    pub value: Vec<String>,
    pub multiple_values: Vec<String>,
}

pub enum Msg {
    Input(String, usize),
    Select(String, usize),
    MultipleSelect(Vec<String>),
}

impl Component for FormPage {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        FormPage {
            link,
            value: vec!["".to_string(); 50],
            multiple_values: vec![],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value, index) => {
                self.value[index] = value;
            }
            Msg::Select(value, index) => {
                self.value[index] = value;
            }
            Msg::MultipleSelect(values) => {
                self.multiple_values = values;
            }
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h1>{"Form Component"}</h1>
                </Item>

                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Form input types"}</h2>
                </Item>

                {get_form_inputs(self)}
                {get_select_form(self)}
            </>
        }
    }
}

fn get_form_inputs(form_page: &FormPage) -> Html {
    html! {
        <Container wrap=Wrap::Wrap direction=Direction::Row>
            <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Horizontal>
                    <FormLabel
                        text="standard input"
                    />
                    <FormInput
                        input_type=InputType::Text
                        value=form_page.value[0].clone()
                        input_style=Palette::Standard
                        input_size=Size::Medium
                        id="form-input-test"
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 0))
                        placeholder="test"
                        underline=false
                    />
                    <div>{format!("Value: {}", form_page.value[0].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Horizontal>
                    <FormLabel
                        text="underline input"
                    />
                    <FormInput
                        input_type=InputType::Text
                        value=form_page.value[1].clone()
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 1))
                        input_style=Palette::Standard
                        id="form-input-test"
                        placeholder="test"
                        underline=true
                    />
                    <div>{format!("Value: {}", form_page.value[1].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItL(4), ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup orientation=Orientation::Horizontal>
                    <FormLabel
                        text="Success input type"
                    />
                    <FormInput
                        input_type=InputType::Text
                        value=form_page.value[2].clone()
                        input_style=Palette::Success
                        oninput_signal = form_page.link.callback(|e: InputData| Msg::Input(e.value, 2))
                        input_size=Size::Medium
                        id="form-input-test"
                        placeholder="test"
                        underline=false
                    />
                    <div>{format!("Value: {}", form_page.value[2].clone())}</div>
                </FormGroup>
            </Item>
        </Container>
    }
}

fn get_select_form(form_page: &FormPage) -> Html {
    html! {
        <Container wrap = Wrap::Wrap direction = Direction::Row>
            <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup>
                    <FormSelect
                        select_size=Size::Medium
                        on_change_signal = form_page.link.callback(|e: ChangeData|
                            match e {
                                ChangeData::Select(element) => {
                                    let value = element.value();
                                    Msg::Select(value, 3)
                                },
                                _ => unreachable!(),
                            }
                        )
                        options=html!{
                            <>
                                <option value="" disabled=true>{"Select library"}</option>
                                <option value="yew">{"Yew"}</option>
                                <option value="yew_styles">{"Yew Styles"}</option>
                                <option value="yew_prism">{"Yew prism"}</option>
                            </>
                        }
                    />
                    <div>{format!("Value: {}", form_page.value[3].clone())}</div>
                </FormGroup>
            </Item>
            <Item layouts=vec!(ItemLayout::ItM(6), ItemLayout::ItXs(12))>
                <FormGroup>
                    <FormSelect
                        select_size=Size::Medium
                        multiple=true
                        on_change_signal = form_page.link.callback(|e: ChangeData| {
                            match e {
                                ChangeData::Select(element) => {
                                    let mut values = vec![];
                                    let options = element.options();

                                    for i in 0..options.length() {
                                        let option = options
                                            .get_with_index(i)
                                            .unwrap()
                                            .dyn_into::<HtmlOptionElement>()
                                            .unwrap();
                                        if option.selected() {
                                            values.push(option.value());
                                        }
                                    }
                                    Msg::MultipleSelect(values)
                                },
                                _ => unreachable!(),
                            }
                        })
                        options=html!{
                            <>
                                <option value="" disabled=true>{"Select multiple library"}</option>
                                <option value="yew">{"Yew"}</option>
                                <option value="yew_styles">{"Yew Styles"}</option>
                                <option value="yew_prism">{"Yew prism"}</option>
                            </>
                        }
                    />
                    <div>{format!("Value: {:#?}", form_page.multiple_values.clone())}</div>
                </FormGroup>
            </Item>

        </Container>
    }
}