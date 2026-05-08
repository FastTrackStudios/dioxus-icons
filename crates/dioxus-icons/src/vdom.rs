use dioxus::core::{
    Attribute, AttributeValue, DynamicNode, Element, Template, TemplateAttribute, TemplateNode,
    VNode,
};

use crate::IconProps;

type AttributeDescription = (&'static str, Option<&'static str>, bool);
const SVG_NAMESPACE: Option<&'static str> = Some("http://www.w3.org/2000/svg");
const EMPTY_CHILDREN: &[TemplateNode] = &[];
const ICON_ATTR_PATHS: &[&[u8]] = &[&[0u8], &[0u8], &[0u8], &[0u8], &[0u8], &[0u8], &[0u8]];
static DEFAULT_SVG_ATTRS: [TemplateAttribute; 10] = svg_attrs("0 0 24 24");

const WIDTH: AttributeDescription = ("width", None, false);
const HEIGHT: AttributeDescription = ("height", None, false);
const STROKE: AttributeDescription = ("stroke", None, false);
const STROKE_WIDTH: AttributeDescription = ("stroke-width", None, false);
const STROKE_LINECAP: AttributeDescription = ("stroke-linecap", None, false);
const STROKE_LINEJOIN: AttributeDescription = ("stroke-linejoin", None, false);
const CLASS: AttributeDescription = ("class", None, false);

#[inline]
pub(crate) const fn icon_template(roots: &'static [TemplateNode]) -> Template {
    Template {
        roots,
        node_paths: &[],
        attr_paths: ICON_ATTR_PATHS,
    }
}

#[inline]
pub(crate) const fn svg_attrs(view_box: &'static str) -> [TemplateAttribute; 10] {
    [
        attr("xmlns", "http://www.w3.org/2000/svg"),
        dynamic_attr(0),
        dynamic_attr(1),
        attr("viewBox", view_box),
        attr("fill", "none"),
        dynamic_attr(2),
        dynamic_attr(3),
        dynamic_attr(4),
        dynamic_attr(5),
        dynamic_attr(6),
    ]
}

#[inline]
pub(crate) const fn svg(children: &'static [TemplateNode]) -> TemplateNode {
    svg_with_attrs(&DEFAULT_SVG_ATTRS, children)
}

#[inline]
pub(crate) const fn svg_with_attrs(
    attrs: &'static [TemplateAttribute],
    children: &'static [TemplateNode],
) -> TemplateNode {
    TemplateNode::Element {
        tag: "svg",
        namespace: SVG_NAMESPACE,
        attrs,
        children,
    }
}

#[inline]
pub(crate) const fn path(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("path", attrs)
}

#[inline]
pub(crate) const fn circle(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("circle", attrs)
}

#[inline]
pub(crate) const fn rect(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("rect", attrs)
}

#[inline]
pub(crate) const fn line(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("line", attrs)
}

#[inline]
pub(crate) const fn polyline(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("polyline", attrs)
}

#[inline]
pub(crate) const fn polygon(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("polygon", attrs)
}

#[inline]
pub(crate) const fn ellipse(attrs: &'static [TemplateAttribute]) -> TemplateNode {
    child("ellipse", attrs)
}

#[inline]
const fn child(tag: &'static str, attrs: &'static [TemplateAttribute]) -> TemplateNode {
    TemplateNode::Element {
        tag,
        namespace: SVG_NAMESPACE,
        attrs,
        children: EMPTY_CHILDREN,
    }
}

#[inline]
pub(crate) const fn attr(name: &'static str, value: &'static str) -> TemplateAttribute {
    TemplateAttribute::Static {
        name,
        value,
        namespace: None,
    }
}

#[inline]
const fn dynamic_attr(id: usize) -> TemplateAttribute {
    TemplateAttribute::Dynamic { id }
}

#[inline]
fn icon_attr(
    (name, namespace, volatile): AttributeDescription,
    value: AttributeValue,
) -> Attribute {
    Attribute {
        name,
        value,
        namespace,
        volatile,
    }
}

#[inline]
pub(crate) fn icon_element(template: Template, props: IconProps) -> Element {
    let IconProps {
        size,
        color,
        stroke_width,
        stroke_linecap,
        stroke_linejoin,
        class,
    } = props;

    let class_value = if class.is_empty() {
        AttributeValue::None
    } else {
        AttributeValue::Text(class.into_owned())
    };
    let dynamic_attributes: [Box<[Attribute]>; 7] = [
        Box::new([icon_attr(WIDTH, AttributeValue::Text(size.to_string()))]),
        Box::new([icon_attr(HEIGHT, AttributeValue::Text(size.to_string()))]),
        Box::new([icon_attr(STROKE, AttributeValue::Text(color.into_owned()))]),
        Box::new([icon_attr(
            STROKE_WIDTH,
            AttributeValue::Text(stroke_width.to_string()),
        )]),
        Box::new([icon_attr(
            STROKE_LINECAP,
            AttributeValue::Text(stroke_linecap.into_owned()),
        )]),
        Box::new([icon_attr(
            STROKE_LINEJOIN,
            AttributeValue::Text(stroke_linejoin.into_owned()),
        )]),
        Box::new([icon_attr(CLASS, class_value)]),
    ];
    let dynamic_nodes: Box<[DynamicNode]> = Box::new([]);
    Ok(VNode::new(
        None,
        template,
        dynamic_nodes,
        Box::new(dynamic_attributes),
    ))
}
