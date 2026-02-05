//! Model comparison utilities

use crate::types::{Model, ModelCapability};
use std::fmt;

/// Result of comparing two models
#[derive(Debug, Clone)]
pub struct ModelComparison<'a> {
    pub model_a: &'a Model,
    pub model_b: &'a Model,
    pub pricing: PricingComparison,
    pub context: ContextComparison,
    pub capabilities: CapabilityComparison,
}

/// Pricing comparison between two models
#[derive(Debug, Clone)]
pub struct PricingComparison {
    pub input_diff: Option<f64>,
    pub output_diff: Option<f64>,
    pub total_diff: f64,
    /// Which model is cheaper (A, B, or Equal)
    pub cheaper: ComparisonResult,
}

/// Context window comparison
#[derive(Debug, Clone)]
pub struct ContextComparison {
    pub input_diff: Option<i64>,
    pub output_diff: Option<i64>,
    /// Which model has larger context (A, B, or Equal)
    pub larger: ComparisonResult,
}

/// Capability comparison
#[derive(Debug, Clone)]
pub struct CapabilityComparison {
    /// Capabilities only model A has
    pub only_a: Vec<ModelCapability>,
    /// Capabilities only model B has
    pub only_b: Vec<ModelCapability>,
    /// Capabilities both models have
    pub shared: Vec<ModelCapability>,
}

/// Result of a comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonResult {
    /// Model A is better/larger/cheaper
    A,
    /// Model B is better/larger/cheaper
    B,
    /// Both are equal or not comparable
    Equal,
}

impl fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::Equal => write!(f, "="),
        }
    }
}

/// Compare two models and return structured comparison
pub fn compare_models<'a>(model_a: &'a Model, model_b: &'a Model) -> ModelComparison<'a> {
    let pricing = compare_pricing(model_a, model_b);
    let context = compare_context(model_a, model_b);
    let capabilities = compare_capabilities(model_a, model_b);

    ModelComparison {
        model_a,
        model_b,
        pricing,
        context,
        capabilities,
    }
}

fn compare_pricing(a: &Model, b: &Model) -> PricingComparison {
    let input_diff = match (a.input_cost_per_million, b.input_cost_per_million) {
        (Some(a_cost), Some(b_cost)) => Some(b_cost - a_cost),
        _ => None,
    };

    let output_diff = match (a.output_cost_per_million, b.output_cost_per_million) {
        (Some(a_cost), Some(b_cost)) => Some(b_cost - a_cost),
        _ => None,
    };

    let total_a = a.total_cost_per_million();
    let total_b = b.total_cost_per_million();
    let total_diff = total_b - total_a;

    let cheaper = if total_a < total_b {
        ComparisonResult::A
    } else if total_b < total_a {
        ComparisonResult::B
    } else {
        ComparisonResult::Equal
    };

    PricingComparison {
        input_diff,
        output_diff,
        total_diff,
        cheaper,
    }
}

fn compare_context(a: &Model, b: &Model) -> ContextComparison {
    let input_diff = match (a.max_input_tokens, b.max_input_tokens) {
        (Some(a_tokens), Some(b_tokens)) => Some(b_tokens as i64 - a_tokens as i64),
        _ => None,
    };

    let output_diff = match (a.max_output_tokens, b.max_output_tokens) {
        (Some(a_tokens), Some(b_tokens)) => Some(b_tokens as i64 - a_tokens as i64),
        _ => None,
    };

    let larger = match (a.max_input_tokens, b.max_input_tokens) {
        (Some(a_tokens), Some(b_tokens)) => {
            if a_tokens > b_tokens {
                ComparisonResult::A
            } else if b_tokens > a_tokens {
                ComparisonResult::B
            } else {
                ComparisonResult::Equal
            }
        }
        _ => ComparisonResult::Equal,
    };

    ContextComparison {
        input_diff,
        output_diff,
        larger,
    }
}

fn compare_capabilities(a: &Model, b: &Model) -> CapabilityComparison {
    let only_a: Vec<ModelCapability> = a
        .capabilities
        .iter()
        .filter(|cap| !b.capabilities.contains(cap))
        .cloned()
        .collect();

    let only_b: Vec<ModelCapability> = b
        .capabilities
        .iter()
        .filter(|cap| !a.capabilities.contains(cap))
        .cloned()
        .collect();

    let shared: Vec<ModelCapability> = a
        .capabilities
        .iter()
        .filter(|cap| b.capabilities.contains(cap))
        .cloned()
        .collect();

    CapabilityComparison {
        only_a,
        only_b,
        shared,
    }
}

impl<'a> ModelComparison<'a> {
    /// Format comparison as a diff-style string with +/- indicators
    pub fn format_diff(&self) -> String {
        let mut lines = Vec::new();

        lines.push(format!(
            "Comparing: {} vs {}",
            self.model_a.id, self.model_b.id
        ));
        lines.push(String::new());

        // Pricing
        lines.push("Pricing (per 1M tokens):".to_string());
        if let Some(diff) = self.pricing.input_diff {
            let indicator = if diff > 0.0 { "+" } else { "" };
            lines.push(format!(
                "  Input:  ${:.2} vs ${:.2} ({}{:.2})",
                self.model_a.input_cost_per_million.unwrap_or(0.0),
                self.model_b.input_cost_per_million.unwrap_or(0.0),
                indicator,
                diff
            ));
        }
        if let Some(diff) = self.pricing.output_diff {
            let indicator = if diff > 0.0 { "+" } else { "" };
            lines.push(format!(
                "  Output: ${:.2} vs ${:.2} ({}{:.2})",
                self.model_a.output_cost_per_million.unwrap_or(0.0),
                self.model_b.output_cost_per_million.unwrap_or(0.0),
                indicator,
                diff
            ));
        }
        let total_indicator = if self.pricing.total_diff > 0.0 {
            "+"
        } else {
            ""
        };
        lines.push(format!(
            "  Total:  ${:.2} vs ${:.2} ({}{:.2}) [Cheaper: {}]",
            self.model_a.total_cost_per_million(),
            self.model_b.total_cost_per_million(),
            total_indicator,
            self.pricing.total_diff,
            self.pricing.cheaper
        ));

        lines.push(String::new());

        // Context
        lines.push("Context Window:".to_string());
        if let Some(diff) = self.context.input_diff {
            let indicator = if diff > 0 { "+" } else { "" };
            lines.push(format!(
                "  Input:  {} vs {} ({}{}) [Larger: {}]",
                self.model_a.max_input_tokens.unwrap_or(0),
                self.model_b.max_input_tokens.unwrap_or(0),
                indicator,
                diff,
                self.context.larger
            ));
        }
        if let Some(diff) = self.context.output_diff {
            let indicator = if diff > 0 { "+" } else { "" };
            lines.push(format!(
                "  Output: {} vs {} ({}{})",
                self.model_a.max_output_tokens.unwrap_or(0),
                self.model_b.max_output_tokens.unwrap_or(0),
                indicator,
                diff
            ));
        }

        lines.push(String::new());

        // Capabilities
        lines.push("Capabilities:".to_string());
        if !self.capabilities.only_a.is_empty() {
            for cap in &self.capabilities.only_a {
                lines.push(format!("  - {} (only A)", cap));
            }
        }
        if !self.capabilities.only_b.is_empty() {
            for cap in &self.capabilities.only_b {
                lines.push(format!("  + {} (only B)", cap));
            }
        }
        if !self.capabilities.shared.is_empty() {
            for cap in &self.capabilities.shared {
                lines.push(format!("  = {} (both)", cap));
            }
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ModelModalities, Provider};

    fn make_model(
        id: &str,
        input_cost: f64,
        output_cost: f64,
        max_input: u64,
        capabilities: Vec<ModelCapability>,
    ) -> Model {
        Model {
            id: id.to_string(),
            provider: Provider {
                id: "test".to_string(),
                name: "Test".to_string(),
            },
            name: id.to_string(),
            family: None,
            input_cost_per_million: Some(input_cost),
            output_cost_per_million: Some(output_cost),
            cache_read_cost_per_million: None,
            cache_write_cost_per_million: None,
            max_input_tokens: Some(max_input),
            max_output_tokens: Some(4096),
            modalities: ModelModalities::default(),
            capabilities,
        }
    }

    #[test]
    fn compare_models_pricing() {
        let a = make_model("cheap", 1.0, 2.0, 100000, vec![]);
        let b = make_model("expensive", 5.0, 10.0, 100000, vec![]);

        let cmp = compare_models(&a, &b);

        assert_eq!(cmp.pricing.cheaper, ComparisonResult::A);
        assert_eq!(cmp.pricing.input_diff, Some(4.0));
        assert_eq!(cmp.pricing.output_diff, Some(8.0));
    }

    #[test]
    fn compare_models_context() {
        let a = make_model("small", 1.0, 2.0, 50000, vec![]);
        let b = make_model("large", 1.0, 2.0, 200000, vec![]);

        let cmp = compare_models(&a, &b);

        assert_eq!(cmp.context.larger, ComparisonResult::B);
        assert_eq!(cmp.context.input_diff, Some(150000));
    }

    #[test]
    fn compare_models_capabilities() {
        let a = make_model(
            "vision-model",
            1.0,
            2.0,
            100000,
            vec![ModelCapability::Vision, ModelCapability::FunctionCalling],
        );
        let b = make_model(
            "reasoning-model",
            1.0,
            2.0,
            100000,
            vec![ModelCapability::Reasoning, ModelCapability::FunctionCalling],
        );

        let cmp = compare_models(&a, &b);

        assert!(cmp.capabilities.only_a.contains(&ModelCapability::Vision));
        assert!(cmp
            .capabilities
            .only_b
            .contains(&ModelCapability::Reasoning));
        assert!(cmp
            .capabilities
            .shared
            .contains(&ModelCapability::FunctionCalling));
    }
}
