use std::collections::HashMap;

use serde::Serialize;
use yew::prelude::*;

use super::{Popup, PopupType};

type FieldName = &'static str;

#[derive(Properties, PartialEq)]
pub struct Property {
    #[prop_or_default]
    pub classes: Classes,

    pub on_close: Callback<()>,
    // TODO: Return the struct instead of HashMap.
    pub on_submit: Callback<HashMap<FieldName, serde_json::Value>>,

    pub compare: CompareContainer,

    #[prop_or_default]
    pub show_equal_rows: bool,
}

pub enum Msg {
    Ignore,

    OnClose,
    OnSubmit,

    SetNewSelected(FieldName, SelectedSide),
}

pub struct PopupComparison {
    selected: HashMap<FieldName, SelectedSide>,
}

impl Component for PopupComparison {
    type Message = Msg;
    type Properties = Property;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            selected: ctx
                .props()
                .compare
                .get_field_names()
                .into_iter()
                .map(|v| (v, SelectedSide::Left))
                .collect(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.selected = ctx
            .props()
            .compare
            .get_field_names()
            .into_iter()
            .map(|v| (v, SelectedSide::Left))
            .collect();

        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Ignore => return false,

            Msg::OnClose => ctx.props().on_close.emit(()),
            Msg::OnSubmit => {
                let compare = &ctx.props().compare;

                let mut map = HashMap::new();

                for (&field_name, &side) in &self.selected {
                    if let Some(value) = compare.get_field_side_value(field_name, side) {
                        map.insert(field_name, value);
                    }
                }

                ctx.props().on_submit.emit(map);
            }

            Msg::SetNewSelected(field, side) => {
                self.selected.insert(field, side);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Popup
                type_of={ PopupType::FullOverlay }
                on_close={ ctx.props().on_close.clone() }
                classes={ classes!("popup-comparison-edit") }
            >
                <div class="header">
                    <h2>{ "Book Comparison" }</h2>
                </div>

                { self.render_body(ctx) }

                <div class="footer">
                    <button class="button" onclick={ ctx.link().callback(|_| Msg::OnClose) }>{ "Cancel" }</button>
                    <button class="button" onclick={ ctx.link().callback(|_| Msg::OnSubmit) }>{ "Save" }</button>
                </div>
            </Popup>
        }
    }
}

impl PopupComparison {
    fn render_body(&self, ctx: &Context<Self>) -> Html {
        let compare = &ctx.props().compare;

        html! {
            <div class="body">
                {
                    for compare.fields.iter()
                        .map(|(&field, row)| {
                            let side_selected = self.selected.get(field).copied().unwrap_or(SelectedSide::Left);

                            match row.display {
                                CompareDisplay::Text => Self::display_value_row(
                                    field,
                                    row.title,
                                    side_selected,
                                    row.left.as_ref().map(|v| &v.display_value),
                                    row.right.as_ref().map(|v| &v.display_value),
                                    ctx
                                ),

                                CompareDisplay::Image => Self::display_image_row(
                                    field,
                                    row.title,
                                    side_selected,
                                    row.left.as_ref().map(|v| &v.display_value),
                                    row.right.as_ref().map(|v| &v.display_value),
                                    ctx
                                ),
                            }
                        })
                }
            </div>
        }
    }

    fn display_value_row(
        updating_field: FieldName,
        title: &'static str,
        side_selected: SelectedSide,
        left_value: Option<&CompareValue>,
        right_value: Option<&CompareValue>,
        ctx: &Context<Self>,
    ) -> Html {
        let old_selected = side_selected.is_left().then(|| "selected");
        let new_selected = side_selected.is_right().then(|| "selected");

        match (left_value, right_value) {
            (Some(left_value), Some(right_value))
                if left_value != right_value || ctx.props().show_equal_rows =>
            {
                html! {
                    <div class="comparison-row">
                        <div class="row-title"><span>{ title }</span></div>
                        <div class={ classes!("row-grow", old_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Left)) }>
                        {
                            for left_value.as_vec().iter().map(|v| html! {
                                <div class="label">{ v }</div>
                            })
                        }
                        </div>
                        <div class={ classes!("row-grow", new_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Right)) }>
                        {
                            for right_value.as_vec().iter().map(|v| html! {
                                <div class="label">{ v }</div>
                            })
                        }
                        </div>
                    </div>
                }
            }

            (None, Some(right_value)) => {
                html! {
                    <div class="comparison-row">
                        <div class="row-title"><span>{ title }</span></div>
                        <div class={ classes!("row-grow", old_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Left)) }>
                            <div class="label">{ "(Empty)" }</div>
                        </div>
                        <div class={ classes!("row-grow", new_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Right)) }>
                        {
                            for right_value.as_vec().iter().map(|v| html! {
                                <div class="label">{ v }</div>
                            })
                        }
                        </div>
                    </div>
                }
            }

            _ => html! {},
        }
    }

    fn display_image_row(
        updating_field: FieldName,
        title: &'static str,
        side_selected: SelectedSide,
        left_value: Option<&CompareValue>,
        right_value: Option<&CompareValue>,
        ctx: &Context<Self>,
    ) -> Html {
        let old_selected = side_selected.is_left().then(|| "selected");
        let new_selected = side_selected.is_right().then(|| "selected");

        match (left_value, right_value) {
            (Some(left_value), Some(right_value))
                if left_value != right_value || ctx.props().show_equal_rows =>
            {
                html! {
                    <div class="comparison-row">
                        <div class="row-title"><span>{ title }</span></div>
                        <div class={ classes!("row-grow", old_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Left)) }>
                            {
                                for left_value.as_vec().iter().map(|v| {
                                    html! {
                                        <div class="label">
                                            <div class="poster">
                                                <img src={ v.to_string() } />
                                            </div>
                                        </div>
                                    }
                                })
                            }
                        </div>
                        <div class={ classes!("row-grow", new_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Right)) }>
                        {
                            for right_value.as_vec().iter().map(|v| {
                                html! {
                                    <div class="label">
                                        <div class="poster">
                                            <img src={ v.to_string() } />
                                        </div>
                                    </div>
                                }
                            })
                        }
                        </div>
                    </div>
                }
            }

            (None, Some(new_images)) => {
                html! {
                    <div class="comparison-row">
                        <div class="row-title"><span>{ title }</span></div>
                        <div class={ classes!("row-grow", old_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Left)) }>
                            <div class="label">{ "(Empty)" }</div>
                        </div>
                        <div class={ classes!("row-grow", new_selected) } onclick={ ctx.link().callback(move |_| Msg::SetNewSelected(updating_field, SelectedSide::Right)) }>
                        {
                            for new_images.as_vec().iter().map(|v| {
                                html! {
                                    <div class="label">
                                        <div class="poster">
                                            <img src={ v.to_string() } />
                                        </div>
                                    </div>
                                }
                            })
                        }
                        </div>
                    </div>
                }
            }

            _ => html! {},
        }
    }
}

pub type MapContainer = HashMap<FieldName, MapValue>;

#[derive(Debug, PartialEq)]
pub struct CompareContainer {
    fields: HashMap<FieldName, CompareRow>,
}

impl CompareContainer {
    pub fn create(
        displays: Vec<(FieldName, &'static str, CompareDisplay)>,
        mut left: MapContainer,
        mut right: MapContainer,
    ) -> Self {
        let mut compiled = HashMap::new();

        for (field_name, title, display) in displays {
            compiled.insert(
                field_name,
                CompareRow {
                    display,

                    title,

                    left: left.remove(field_name),
                    right: right.remove(field_name),
                },
            );
        }

        Self { fields: compiled }
    }

    pub fn get_side_value_map(&self, side: SelectedSide) -> HashMap<FieldName, serde_json::Value> {
        let mut map = HashMap::new();

        for (&field_name, row) in &self.fields {
            if let Some(value) = row.get_side_value(side) {
                map.insert(field_name, value);
            }
        }

        map
    }

    #[inline]
    pub fn get_field_side_value(
        &self,
        field: FieldName,
        side: SelectedSide,
    ) -> Option<serde_json::Value> {
        self.fields.get(field).and_then(|v| v.get_side_value(side))
    }

    pub fn get_field_names(&self) -> Vec<FieldName> {
        let mut names = Vec::new();

        for row in self.fields.keys() {
            names.push(*row);
        }

        names
    }
}

pub fn morph_map_value<V: Serialize + Into<CompareValue> + Clone>(
    value: V,
) -> serde_json::Result<MapValue> {
    Ok(MapValue {
        original_value: serde_json::to_value(value.clone())?,
        display_value: value.into(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedSide {
    Left,
    Right,
}

impl SelectedSide {
    pub fn is_left(self) -> bool {
        matches!(self, Self::Left)
    }

    pub fn is_right(self) -> bool {
        !self.is_left()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompareDisplay {
    Text,
    Image,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CompareRow {
    pub display: CompareDisplay,

    pub title: &'static str,

    pub left: Option<MapValue>,
    pub right: Option<MapValue>,
}

impl CompareRow {
    #[inline]
    pub fn get_side_value(&self, side: SelectedSide) -> Option<serde_json::Value> {
        match side {
            SelectedSide::Left => self.left.as_ref().map(|v| v.original_value.clone()),
            SelectedSide::Right => self.right.as_ref().map(|v| v.original_value.clone()),
        }
    }
}

pub trait Comparable {
    fn create_comparison_with(&self, other: &Self) -> serde_json::Result<CompareContainer>;
    fn create_from_comparison(
        map: HashMap<FieldName, serde_json::Value>,
    ) -> serde_json::Result<Self>
    where
        Self: Sized;

    fn create_map(&self) -> serde_json::Result<MapContainer>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct MapValue {
    pub display_value: CompareValue,
    pub original_value: serde_json::Value,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompareValue {
    Single(String),
    Array(Vec<String>),
}

impl CompareValue {
    pub fn as_vec(&self) -> Vec<&str> {
        match self {
            CompareValue::Single(v) => vec![v.as_str()],
            CompareValue::Array(v) => v.iter().map(|v| v.as_str()).collect(),
        }
    }

    pub fn as_single(&self) -> &str {
        match self {
            CompareValue::Single(v) => v.as_str(),
            CompareValue::Array(v) => v.first().map(|v| v.as_str()).unwrap_or("[none]"),
        }
    }
}

impl From<String> for CompareValue {
    fn from(value: String) -> Self {
        Self::Single(value)
    }
}

impl From<u8> for CompareValue {
    fn from(value: u8) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<u16> for CompareValue {
    fn from(value: u16) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<u32> for CompareValue {
    fn from(value: u32) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<u64> for CompareValue {
    fn from(value: u64) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<f32> for CompareValue {
    fn from(value: f32) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<f64> for CompareValue {
    fn from(value: f64) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<bool> for CompareValue {
    fn from(value: bool) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<Vec<String>> for CompareValue {
    fn from(value: Vec<String>) -> Self {
        Self::Array(value)
    }
}
