//! Bauausführung-specific baseline linking helpers.

use std::collections::{BTreeMap, BTreeSet};

use crate::error::ValidationFinding;
use crate::model::{BoqNode, BoqNodeKind, GaebDocument};

/// Clones an X83 baseline and overlays matching X84 offer price fields by ordinal.
///
/// The baseline text and hierarchy remain authoritative. Unit price and total
/// price are copied from matching offer items. Missing, duplicate, and extra
/// offer ordinals are emitted as deterministic findings on the returned document.
#[must_use]
pub fn merge_x84_offer_into_x83_baseline(
    baseline: &GaebDocument,
    offer: &GaebDocument,
) -> GaebDocument {
    let mut merged = baseline.clone();
    let offer_items = collect_item_paths(&offer.boq.nodes);
    let mut matched = BTreeSet::new();

    overlay_nodes(&mut merged.boq.nodes, &offer_items, &mut matched);

    for (ordinal, items) in &offer_items {
        if items.len() > 1 {
            merged.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_bau_x84_duplicate_ordinal",
                    "X84 offer contained duplicate ordinal; first deterministic match was used",
                )
                .at(ordinal.clone()),
            );
        }
    }

    for ordinal in baseline_ordinals(&baseline.boq.nodes) {
        if !offer_items.contains_key(&ordinal) {
            merged.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_bau_x84_missing_ordinal",
                    "X83 baseline ordinal had no matching X84 offer item",
                )
                .at(ordinal),
            );
        }
    }

    for ordinal in offer_items.keys() {
        if !matched.contains(ordinal) {
            merged.findings.push(
                ValidationFinding::warning(
                    "gaeb_xml_bau_x84_extra_ordinal",
                    "X84 offer item had no matching X83 baseline ordinal",
                )
                .at(ordinal.clone()),
            );
        }
    }

    merged
}

fn overlay_nodes(
    nodes: &mut [BoqNode],
    offer_items: &BTreeMap<String, Vec<BoqNode>>,
    matched: &mut BTreeSet<String>,
) {
    for node in nodes {
        if node.kind == BoqNodeKind::Item {
            if let (Some(target), Some(source_nodes)) =
                (&mut node.item, offer_items.get(&node.ordinal))
            {
                if let Some(source_item) =
                    source_nodes.first().and_then(|source| source.item.as_ref())
                {
                    target.unit_price = source_item.unit_price;
                    target.total_price = source_item.total_price;
                    matched.insert(node.ordinal.clone());
                }
            }
        }
        overlay_nodes(&mut node.children, offer_items, matched);
    }
}

fn collect_item_paths(nodes: &[BoqNode]) -> BTreeMap<String, Vec<BoqNode>> {
    let mut items = BTreeMap::new();
    collect_items(nodes, &mut items);
    items
}

fn collect_items(nodes: &[BoqNode], items: &mut BTreeMap<String, Vec<BoqNode>>) {
    for node in nodes {
        if node.kind == BoqNodeKind::Item {
            items
                .entry(node.ordinal.clone())
                .or_default()
                .push(node.clone());
        }
        collect_items(&node.children, items);
    }
}

fn baseline_ordinals(nodes: &[BoqNode]) -> BTreeSet<String> {
    let mut ordinals = BTreeSet::new();
    collect_ordinals(nodes, &mut ordinals);
    ordinals
}

fn collect_ordinals(nodes: &[BoqNode], ordinals: &mut BTreeSet<String>) {
    for node in nodes {
        if node.kind == BoqNodeKind::Item {
            ordinals.insert(node.ordinal.clone());
        }
        collect_ordinals(&node.children, ordinals);
    }
}
