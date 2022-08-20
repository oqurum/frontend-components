use std::{marker::PhantomData, rc::Rc};

use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*, virtual_dom::VChild};

#[derive(Properties)]
pub struct MultiSelectProperty<Ident: Clone + PartialEq + 'static> {
    pub children: ChildrenWithProps<MultiSelectItem<Ident>>,

    pub on_event: Option<Callback<MultiSelectEvent<Ident>>>,

    #[prop_or_default]
    pub editing: bool,

    #[prop_or(true)]
    pub create_new: bool,
}

impl<Ident: Clone + PartialEq> PartialEq for MultiSelectProperty<Ident> {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.editing == other.editing
            && self.create_new == other.create_new
    }
}

pub enum MultiSelectMessage<Ident: PartialEq> {
    Update,
    Ignore,

    OnUnfocus,
    OnFocus,
    SetFocus,

    OnSelectItem(Ident),
    OnUnselectItem(Ident),
    OnCreate,

    OnHover(Option<Ident>),

    OnKeyDown(KeyboardEvent),
    OnKeyUp(KeyboardEvent),
    OnPressEnter,
    OnInputChange(KeyboardEvent),
}

pub struct MultiSelectModule<Ident> {
    input_ref: NodeRef,
    // On focus
    is_focused: bool,
    // Dropdown Opened
    is_opened: bool,

    selected_index: usize,

    _ident: PhantomData<Ident>,
}

impl<Ident: Clone + PartialEq + 'static> Component for MultiSelectModule<Ident> {
    type Message = MultiSelectMessage<Ident>;
    type Properties = MultiSelectProperty<Ident>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input_ref: NodeRef::default(),
            is_focused: false,
            is_opened: false,
            selected_index: 0,

            _ident: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MultiSelectMessage::Update => (),
            MultiSelectMessage::Ignore => return false,

            MultiSelectMessage::OnHover(id_or_none) => {
                if let Some(id) = id_or_none {
                    if let Some(index) = self.get_child_index_by_id(id, ctx) {
                        self.selected_index = index;
                    }
                } else {
                    self.selected_index = self.viewable_children_count(ctx);
                }
            }

            MultiSelectMessage::OnInputChange(event) => {
                let key = event.key();

                if key != "ArrowUp" && key != "ArrowDown" {
                    self.selected_index = 0;
                }

                // TODO: Check if we didn't press any special characters
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    let text = input.value().trim().to_string();

                    if let Some(cb) = ctx.props().on_event.as_ref() {
                        cb.emit(MultiSelectEvent::Input { text });
                    }
                }
            }

            MultiSelectMessage::OnPressEnter => {
                let child_count = self.viewable_children_count(ctx);

                if self.selected_index < child_count {
                    let value = self
                        .get_selected_child_id(ctx)
                        .expect_throw("Couldn't get child value");

                    return self.update(ctx, MultiSelectMessage::OnSelectItem(value));
                } else {
                    return self.update(ctx, MultiSelectMessage::OnCreate);
                }
            }

            MultiSelectMessage::OnKeyDown(event) => {
                match event.key().as_str() {
                    "ArrowUp" => {
                        if self.selected_index != 0 {
                            self.selected_index -= 1;
                        }
                    }

                    "ArrowDown" => {
                        let child_count = self.viewable_children_count(ctx);

                        // Correct child count for if statement. If we're not displaying create value, minus one from child count.
                        let child_count = self
                            .create_button_value(ctx)
                            .map(|_| child_count)
                            .unwrap_or_else(|| child_count.saturating_sub(1));

                        if child_count > self.selected_index {
                            self.selected_index += 1;
                        } else {
                            self.selected_index = child_count;
                        }
                    }

                    _ => (),
                }
            }

            MultiSelectMessage::OnKeyUp(event) => {
                if event.key() == "Enter" {
                    return self.update(ctx, MultiSelectMessage::OnPressEnter);
                } else {
                    return self.update(ctx, MultiSelectMessage::OnInputChange(event));
                }
            }

            MultiSelectMessage::OnSelectItem(id) => {
                if let Some(mut item) = ctx.props().children.iter().find(|v| v.props.id == id) {
                    let mut props = Rc::make_mut(&mut item.props);
                    props.selected = true;

                    if let Some(cb) = ctx.props().on_event.as_ref() {
                        cb.emit(MultiSelectEvent::Toggle {
                            toggle: true,
                            id: props.id.clone(),
                        });
                    }
                }
            }

            MultiSelectMessage::OnUnselectItem(id) => {
                // Event is still active when not editing
                if !ctx.props().editing {
                    return false;
                }

                if let Some(mut item) = ctx.props().children.iter().find(|v| v.props.id == id) {
                    let mut props = Rc::make_mut(&mut item.props);
                    props.selected = false;

                    if let Some(cb) = ctx.props().on_event.as_ref() {
                        cb.emit(MultiSelectEvent::Toggle {
                            toggle: false,
                            id: props.id.clone(),
                        });
                    }
                }
            }

            MultiSelectMessage::OnCreate => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    let name = input.value().trim().to_string();

                    if !name.is_empty() {
                        if let Some(cb) = ctx.props().on_event.as_ref() {
                            input.set_value("");

                            cb.emit(MultiSelectEvent::Create(MultiSelectNewItem {
                                name,
                                register: ctx.link().callback(MultiSelectMessage::OnSelectItem),
                            }));
                        }
                    }
                }
            }

            // Focus
            MultiSelectMessage::SetFocus => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    let _ = input.focus();
                }
            }

            MultiSelectMessage::OnFocus => {
                self.is_focused = true;
            }

            MultiSelectMessage::OnUnfocus => {
                self.is_focused = false;
                self.selected_index = 0;
            }
        }

        self.is_opened = self.is_focused
            && (!ctx.props().children.is_empty()
                || self
                    .input_ref
                    .cast::<HtmlInputElement>()
                    .map(|v| !v.value().trim().is_empty())
                    .unwrap_or_default());

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().editing {
            self.display_editing(ctx)
        } else {
            self.display_viewing(ctx)
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        if self.is_focused {
            if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        }

        true
    }
}

impl<Ident: Clone + PartialEq + 'static> MultiSelectModule<Ident> {
    fn display_viewing(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="multi-selection">
                <div class="input">
                    <div class="chosen-list">
                        { for ctx.props().children.iter().filter(|v| v.props.selected).map(|child| Self::create_selected_pill(ctx, &child.props)) }
                    </div>
                </div>
            </div>
        }
    }

    fn display_editing(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={ classes!("multi-selection", Some("focused").filter(|_| self.is_focused), Some("opened").filter(|_| self.is_opened)) }>
                <div class="input" onclick={ ctx.link().callback(|_| MultiSelectMessage::SetFocus) }>
                    <div class="chosen-list">
                        { for ctx.props().children.iter().filter(|v| v.props.selected).map(|child| Self::create_selected_pill(ctx, &child.props)) }
                    </div>
                    <input
                        ref={ self.input_ref.clone() }
                        onfocusin={ ctx.link().callback(|_| MultiSelectMessage::OnFocus) }
                        onfocusout={ ctx.link().callback_future(|_| async {
                            // TODO: Fix. Used since we unfocus when we click the dropdown. This provides enough time for the onmousedown event to fire.
                            TimeoutFuture::new(100).await;
                            MultiSelectMessage::OnUnfocus
                        }) }
                        onkeyup={ ctx.link().callback(MultiSelectMessage::OnKeyUp) }
                        onkeydown={ ctx.link().callback(MultiSelectMessage::OnKeyDown) }
                        type="text"
                        placeholder="Enter Something"
                    />
                </div>
                <div class="dropdown">
                    <div class="dropdown-list">
                        { for ctx.props().children.iter()
                            .filter(|v| self.filter_viewable_child(v))
                            .enumerate()
                            .map(|(index, mut item)| {
                                let mut props = Rc::make_mut(&mut item.props);

                                props.hovering = index == self.selected_index;

                                if props.callback.is_none() {
                                    props.callback = Some(ctx.link().clone());
                                }

                                item
                            })
                        }

                        {
                            if let Some(value) = self.create_button_value(ctx) {
                                let children_count = self.viewable_children_count(ctx);

                                html! {
                                    <div
                                        class={ classes!("list-item", Some("hovering").filter(|_| children_count == self.selected_index)) }
                                        onclick={ ctx.link().callback(|_| MultiSelectMessage::OnCreate) }
                                        onmouseover={ ctx.link().callback(|_| MultiSelectMessage::OnHover(None)) }
                                    >
                                        { format!(r#"Create "{value}""#) }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }

    fn create_button_value(&self, ctx: &Context<Self>) -> Option<String> {
        if ctx.props().create_new {
            self.input_ref
                .cast::<HtmlInputElement>()
                .map(|v| v.value().trim().to_string())
                .filter(|v| !v.is_empty())
        } else {
            None
        }
    }

    fn create_selected_pill(ctx: &Context<Self>, props: &Rc<MultiSelectItemProps<Ident>>) -> Html {
        let item_id = props.id.clone();

        html! {
            <div class="chosen-item" onmousedown={ctx.link().callback(move |_| MultiSelectMessage::OnUnselectItem(item_id.clone()))}>
                { &props.name }
            </div>
        }
    }

    fn filter_viewable_child(&self, item: &VChild<MultiSelectItem<Ident>>) -> bool {
        let input_val_lc = self
            .input_ref
            .cast::<HtmlInputElement>()
            .map(|v| v.value().to_lowercase());

        if let Some(v) = input_val_lc.as_deref() {
            if !item.props.name.to_lowercase().contains(v) {
                return false;
            }
        }

        !item.props.selected
    }

    fn viewable_children_count(&self, ctx: &Context<Self>) -> usize {
        ctx.props()
            .children
            .iter()
            .filter(|v| self.filter_viewable_child(v))
            .count()
    }

    fn get_selected_child_id(&self, ctx: &Context<Self>) -> Option<Ident> {
        ctx.props()
            .children
            .iter()
            .filter(|v| self.filter_viewable_child(v))
            .enumerate()
            .find_map(|(index, item)| {
                if index == self.selected_index {
                    Some(item.props.id.clone())
                } else {
                    None
                }
            })
    }

    fn get_child_index_by_id(&self, id: Ident, ctx: &Context<Self>) -> Option<usize> {
        ctx.props()
            .children
            .iter()
            .filter(|v| self.filter_viewable_child(v))
            .enumerate()
            .find_map(|(index, item)| {
                if id == item.props.id {
                    Some(index)
                } else {
                    None
                }
            })
    }
}

#[derive(Clone)]
pub struct MultiSelectNewItem<Ident: Clone> {
    pub name: String,
    #[must_use = "Register the Value with MultiSelect"]
    /// Registers the new item in the Multi Select Component
    pub register: Callback<Ident>,
}

#[derive(Clone, Properties)]
pub struct MultiSelectItemProps<Ident: Clone + PartialEq + 'static> {
    pub id: Ident,
    pub name: String,

    pub callback: Option<Scope<MultiSelectModule<Ident>>>,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    hovering: bool, // TODO: Better name
}

impl<Ident: Clone + PartialEq> PartialEq for MultiSelectItemProps<Ident> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.selected == other.selected
            && self.hovering == other.hovering
    }
}

#[derive(Clone, PartialEq)]
pub enum MultiSelectItemMessage {
    Selected,
}

pub struct MultiSelectItem<Ident: PartialEq> {
    _ident: PhantomData<Ident>,
}

impl<Ident: Clone + PartialEq + 'static> Component for MultiSelectItem<Ident> {
    type Message = MultiSelectItemMessage;
    type Properties = MultiSelectItemProps<Ident>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _ident: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MultiSelectItemMessage::Selected => {
                let props = ctx.props();
                props
                    .callback
                    .as_ref()
                    .unwrap_throw()
                    .send_message(MultiSelectMessage::OnSelectItem(props.id.clone()));
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.props().callback.clone().unwrap_throw();
        let id = ctx.props().id.clone();

        html! {
            <div
                class={ classes!("list-item", Some("hovering").filter(|_| ctx.props().hovering)) }
                onclick={ ctx.link().callback(|_| MultiSelectItemMessage::Selected) }
                onmouseover={ move |_| cb.send_message(MultiSelectMessage::OnHover(Some(id.clone()))) }
            >
                { &ctx.props().name }
            </div>
        }
    }
}

pub enum MultiSelectEvent<Ident: Clone> {
    Toggle { toggle: bool, id: Ident },

    Create(MultiSelectNewItem<Ident>),

    Input { text: String },
}
