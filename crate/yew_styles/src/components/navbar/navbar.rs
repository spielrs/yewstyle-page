use super::navbar_container::NavbarContainer;
use super::navbar_item::NavbarItem;
use crate::container::{JustifyContent, Mode};
use crate::styles::{get_pallete, get_style, Palette, Style};
use crate::utils::create_style;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Fixed {
    None,
    Top,
    Bottom,
}

pub enum Msg {
    TroggleMenu,
}

pub struct Navbar {
    pub link: ComponentLink<Self>,
    pub props: NavbarProps,
    pub display_menu: bool,
}

struct NavbarModel;

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or(Palette::Standard)]
    pub navbar_type: Palette,
    #[prop_or(Style::Regular)]
    pub navbar_style: Style,
    #[prop_or_default]
    pub class_name: String,
    #[prop_or(Fixed::Top)]
    pub fixed: Fixed,
    pub children: Children,
}

#[derive(Clone)]
pub struct NavbarProps {
    pub navbar_type: String,
    pub navbar_style: String,
    pub class_name: String,
    pub fixed: Fixed,
    pub children: Children,
}

impl From<Props> for NavbarProps {
    fn from(props: Props) -> Self {
        NavbarProps {
            navbar_type: get_pallete(props.navbar_type),
            navbar_style: get_style(props.navbar_style),
            class_name: props.class_name,
            fixed: props.fixed,
            children: props.children,
        }
    }
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Navbar {
            link,
            props: NavbarProps::from(props),
            display_menu: false,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        NavbarModel.init(self.props.clone());

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TroggleMenu => {
                self.display_menu = !self.display_menu;
            }
        };

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        NavbarModel.init(self.props.clone());
        self.props = NavbarProps::from(props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div
                    class=format!("navbar-mobile {} {} {}", self.props.navbar_style, self.props.navbar_type, self.props.class_name)
                >
                    <NavbarContainer justify_content=JustifyContent::End(Mode::NoMode)>
                        <NavbarItem
                            onsignal=self.link.callback(move |_| Msg::TroggleMenu)
                        >
                            <img src="./assets/menu.svg"/>
                        </NavbarItem>
                    </NavbarContainer>
                    {if self.display_menu {
                        self.props.children.render()
                    } else {
                        html!{}
                    }}
                </div>

                <div
                    class=format!("navbar {} {} {}", self.props.navbar_style, self.props.navbar_type, self.props.class_name)
                >
                    {self.props.children.render()}
                </div>
            </>
        }
    }
}

impl NavbarModel {
    fn init(self, props: NavbarProps) {
        self.set_fixed(props.fixed);
    }

    fn set_fixed(self, fixed: Fixed) {
        create_style(
            String::from("position"),
            if fixed == Fixed::None {
                String::from("inherit")
            } else {
                String::from("fixed")
            },
            String::from("navbar"),
        );

        if fixed != Fixed::None {
            create_style(
                if fixed == Fixed::Top {
                    String::from("top")
                } else {
                    String::from("bottom")
                },
                String::from("0"),
                String::from("navbar"),
            );
        }
    }
}