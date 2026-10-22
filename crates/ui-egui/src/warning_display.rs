//! Typed presentation of Boolean retry warnings alongside other feature warnings.

use solidcraft_engine::kernel::BooleanDiagnostics;

/// The report proves the canonical suffix belongs to the Boolean renderer. Other
/// warnings are kept as one intact value; no arbitrary names or errors are split.
pub(crate) fn feature(source: &str, report: &BooleanDiagnostics) -> String {
    let canonical = report.warning();
    if let Some(canonical) = canonical {
        if source == canonical {
            return report.warning_with(crate::i18n::diagnostic).unwrap_or_else(|| source.to_string());
        }
        if let Some(other) = source.strip_suffix(&format!("; {canonical}")) {
            let translated = report.warning_with(crate::i18n::diagnostic).unwrap_or(canonical);
            return format!("{}; {translated}", crate::display_error(other));
        }
    }
    if report.unverified().as_deref() == Some(source) {
        return report.unverified_with(crate::i18n::diagnostic).unwrap_or_else(|| source.to_string());
    }
    crate::display_error(source)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use solidcraft_engine::kernel::{BooleanFallback, BooleanStrategy};

    fn report(strategy: BooleanStrategy) -> BooleanDiagnostics {
        BooleanDiagnostics {
            events: vec![BooleanFallback { strategy, translation_mm: 1e-6, rotation_rad: 2e-6, face_shift_mm: 0., tolerance_mm: 3e-6 }],
            truncated: false,
        }
    }

    #[test]
    fn canonical_english_and_mixed_warning_names_remain_intact() {
        let _language = crate::i18n::scope("en");
        let report = report(BooleanStrategy::ToolNudge);
        let warning = report.warning().unwrap();
        assert_eq!(feature(&warning, &report), warning);
        let other = "Vendor: `Steel`; custom {name}: C:\\models\\Boss";
        let mixed = format!("{other}; {warning}");
        assert_eq!(feature(&mixed, &report), mixed);
    }

    #[test]
    fn portuguese_replaces_only_the_proven_boolean_suffix() {
        let _language = crate::i18n::scope("pt-BR");
        let report = report(BooleanStrategy::ToolNudge);
        let warning = report.warning().unwrap();
        let other = "Vendor: `Steel`; custom {name}: C:\\models\\Boss";
        let mixed = format!("{other}; {warning}");
        let shown = feature(&mixed, &report);
        assert!(shown.starts_with(&format!("{other}; ")));
        assert!(!shown.contains("Boolean retry"), "{shown}");
        assert!(!shown.contains("tool displacement"), "{shown}");
        assert!(shown.contains("1.000000e-6 mm"), "{shown}");
        let authored = "the radius is reduced to 2.5 mm, the largest that fits these edges";
        let composed = feature(&format!("{authored}; {warning}"), &report);
        assert!(!composed.contains("the radius is reduced"), "{composed}");
        assert!(composed.contains("2.5 mm"), "{composed}");
        assert!(composed.contains("1.000000e-6 mm"), "{composed}");
        let mismatch = "Vendor: opaque {name}; arbitrary Boolean retry text";
        assert_eq!(feature(mismatch, &report), mismatch);
    }

    #[test]
    fn standalone_unverified_prose_uses_its_typed_report() {
        let _language = crate::i18n::scope("pt-BR");
        let report = report(BooleanStrategy::UnverifiedShortcut);
        let warning = report.unverified().unwrap();
        let shown = feature(&warning, &report);
        assert!(!shown.contains("not verified"), "{shown}");
        assert!(shown.contains("3.000000e-6 mm"), "{shown}");
    }

    #[test]
    fn exact_retries_do_not_create_a_warning_or_change_authored_text() {
        let _language = crate::i18n::scope("pt-BR");
        let authored = "Vendor: opaque {name}; arbitrary Boolean retry text";
        for strategy in [BooleanStrategy::RefinedTolerance, BooleanStrategy::GrownBoss] {
            let report = report(strategy);
            assert!(!strategy.alters_result());
            assert!(report.warning().is_none());
            assert_eq!(feature(authored, &report), authored);
        }
    }
}
