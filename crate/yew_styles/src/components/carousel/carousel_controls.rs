use crate::styles::{get_palette, get_size, get_style, Palette, Size, Style};
use yew::prelude::*;
use yew_assets::controller_assets::{ControllerAssets, ControllerIcon};

pub struct CarouselControls {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub prev_signal: Callback<MouseEvent>,
    pub next_signal: Callback<MouseEvent>,
    /// controls styles. Default `Style::Regular`
    #[prop_or(Style::Regular)]
    pub controls_style: Style,
    /// Type controls style. Default `Palette::Standard`
    #[prop_or(Palette::Standard)]
    pub controls_palette: Palette,
    /// Three diffent button standard sizes. Default `Size::Medium`
    #[prop_or(Size::Medium)]
    pub controls_size: Size,
    /// General property to get the ref of the component
    #[prop_or_default]
    pub code_ref: NodeRef,
    /// General property to add keys
    #[prop_or_default]
    pub key: String,
    /// General property to add custom class styles
    #[prop_or_default]
    pub class_name: String,
    /// General property to add custom id
    #[prop_or_default]
    pub id: String,
}

pub enum Msg {
    PrevClicked(MouseEvent),
    NextClicked(MouseEvent),
}

impl Component for CarouselControls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PrevClicked(mouse_event) => {
                self.props.prev_signal.emit(mouse_event);
            }
            Msg::NextClicked(mouse_event) => {
                self.props.next_signal.emit(mouse_event);
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            return true;
        }

        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="carousel-control"
                key=self.props.key.clone()
                ref=self.props.code_ref.clone()
            >
                <div
                    class="carousel-control-left-container"

                    onclick=self.link.callback(|e| Msg::PrevClicked(e))>
                    <ControllerAssets
                        size=("50".to_string(),"50".to_string())
                        class_name=format!("carousel-control-left {} {} {} {}",
                            get_size(self.props.controls_size.clone()),
                            get_style(self.props.controls_style.clone()),
                            get_palette(self.props.controls_palette.clone()),
                            self.props.class_name.clone(),
                        )
                        icon=ControllerIcon::ChevronLeft
                    />
                </div>
                <div
                    class="carousel-control-right-container"
                    onclick=self.link.callback(|e| Msg::NextClicked(e))
                >
                    <ControllerAssets
                        size=("50".to_string(),"50".to_string())
                        class_name=format!("carousel-control-right {} {} {} {}",
                        get_size(self.props.controls_size.clone()),
                        get_style(self.props.controls_style.clone()),
                        get_palette(self.props.controls_palette.clone()),
                        self.props.class_name.clone(),
                    )
                        icon=ControllerIcon::ChevronRight/>
                </div>
            </div>
        }
    }
}
