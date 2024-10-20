use std::marker::PhantomData;

use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties)]
pub struct SelectProperty<Ident: Clone + PartialEq + 'static> {
    pub children: ChildrenWithProps<SelectItem<Ident>>,

    #[prop_or_default]
    pub default: Option<Ident>,

    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub class: Option<String>,

    #[prop_or_default]
    pub onselect: Option<Callback<Ident>>,

    #[prop_or_default]
    pub disabled: bool,
}

impl<Ident: Clone + PartialEq> PartialEq for SelectProperty<Ident> {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.disabled == other.disabled
            && self.default == other.default
            && self.name == other.name
            && self.class == other.class
    }
}

pub enum SelectMessage {
    Update,
    Ignore,

    OnSelectIndex(usize),
}

pub struct SelectModule<Ident> {
    node_ref: NodeRef,

    selected_index: usize,

    _ident: PhantomData<Ident>,
}

impl<Ident: Clone + PartialEq + 'static> Component for SelectModule<Ident> {
    type Message = SelectMessage;
    type Properties = SelectProperty<Ident>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            selected_index: 0,

            _ident: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SelectMessage::Update => (),
            SelectMessage::Ignore => return false,

            SelectMessage::OnSelectIndex(index) => {
                self.selected_index = index;

                if let Some(item) = ctx.props().children.iter().nth(self.selected_index) {
                    if let Some(cb) = ctx.props().onselect.as_ref() {
                        cb.emit(item.props.value.clone());
                    }
                }
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SelectProperty {
            children,
            class,
            name,
            disabled,
            ..
        } = ctx.props();

        html! {
            <select
                ref={ self.node_ref.clone() }
                class={ class.clone() }
                name={ name.clone() }
                disabled={ *disabled }
                onchange={ ctx.link().callback(|v: Event| SelectMessage::OnSelectIndex(v.target_unchecked_into::<HtmlSelectElement>().selected_index() as usize)) }
            >
                { for children.iter() }

                // for ctx.props().children.iter()
                //     .enumerate()
                //     .map(|(i, mut child)| {
                //         let mut props = Rc::make_mut(&mut child.props);
                //         props.selected = i == self.selected_index;

                //         child
                //     })
            </select>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.set_default_prop_index(ctx);
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().default != old_props.default {
            self.set_default_prop_index(ctx);
        }

        true
    }
}

impl<Ident: Clone + PartialEq + 'static> SelectModule<Ident> {
    pub fn set_default_prop_index(&mut self, ctx: &Context<Self>) {
        if let Some(def) = ctx.props().default.clone() {
            if let Some(index) = ctx
                .props()
                .children
                .iter()
                .position(|v| v.props.value == def)
            {
                self.selected_index = index;
                self.node_ref
                    .cast::<HtmlSelectElement>()
                    .unwrap()
                    .set_selected_index(self.selected_index as i32);
            }
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SelectItemProps<Ident: Clone + PartialEq + 'static> {
    pub value: Ident,
    pub name: String,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    pub disabled: bool,
}

pub struct SelectItem<Ident: PartialEq> {
    _ident: PhantomData<Ident>,
}

impl<Ident: Clone + PartialEq + 'static> Component for SelectItem<Ident> {
    type Message = ();
    type Properties = SelectItemProps<Ident>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _ident: PhantomData,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <option selected={ ctx.props().selected } disabled={ ctx.props().disabled }>
                { &ctx.props().name }
            </option>
        }
    }
}
