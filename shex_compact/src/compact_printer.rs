use colored::*;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use pretty::{Arena, DocAllocator, DocBuilder};
use shex_ast::{object_value::ObjectValue, BNode, ShapeExprLabel};
use srdf::literal::Literal;
use std::borrow::Cow;

pub(crate) fn pp_object_value<'a, A>(
    v: &ObjectValue,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match v {
        ObjectValue::IriRef(i) => pp_iri_ref(i, doc, prefixmap),
        ObjectValue::Literal(Literal::BooleanLiteral(_value)) => {
            todo!()
        }
        ObjectValue::Literal(Literal::NumericLiteral(_)) => todo!(),
        ObjectValue::Literal(Literal::DatatypeLiteral { .. }) => todo!(),
        ObjectValue::Literal(Literal::StringLiteral { .. }) => todo!(),
    }
}

pub(crate) fn pp_label<'a, A>(
    label: &ShapeExprLabel,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
    keyword_color: Option<Color>,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match label {
        ShapeExprLabel::BNode { value } => pp_bnode(value, doc),
        ShapeExprLabel::IriRef { value } => pp_iri_ref(value, doc, prefixmap),
        ShapeExprLabel::Start => keyword("START", doc, keyword_color),
    }
}

pub(crate) fn pp_bnode<'a, A>(
    value: &BNode,
    doc: &'a Arena<'a, A>,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(format!("{value}"))
}

fn pp_iri_ref<'a, A>(
    value: &IriRef,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    match value {
        IriRef::Iri(iri) => pp_iri(iri, doc, prefixmap),
        IriRef::Prefixed { prefix, local } => doc
            .text(prefix.clone())
            .append(doc.text(":"))
            .append(doc.text(local.clone())),
    }
}

pub(crate) fn keyword<'a, U, A>(
    s: U,
    doc: &'a Arena<'a, A>,
    color: Option<Color>,
) -> DocBuilder<'a, Arena<'a, A>, A>
where
    U: Into<Cow<'a, str>>,
{
    if let Some(color) = color {
        // use std::borrow::Borrow;
        let data: Cow<str> = s.into();
        let s: String = match data {
            Cow::Owned(t) => t,
            Cow::Borrowed(t) => t.into(),
        };
        doc.text(s.as_str().color(color).to_string())
    } else {
        doc.text(s)
    }
}

fn pp_iri<'a, A>(
    iri: &IriS,
    doc: &'a Arena<'a, A>,
    prefixmap: &PrefixMap,
) -> DocBuilder<'a, Arena<'a, A>, A> {
    doc.text(prefixmap.qualify(iri))
}
