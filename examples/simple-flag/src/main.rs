//! Example: Simple Flag System
//!
//! This example demonstrates the fundamental concept of Bionic Intelligence:
//! The "Flag" system. It shows how static logical rules combined with dynamic
//! inputs allow the system to "think" and derive new facts deterministically.

use console::style;
use ptree::{TreeBuilder, print_tree};
use std::collections::HashMap;

// ============================================================================
// Type Definitions
// ============================================================================

type FlagId = u32;

/// Logic Rule: Defines how Flags connect.
/// In a real BI, this is stored in the static "LogicRom".
struct Rule {
    triggers: Vec<FlagId>, // Conditions (Inputs)
    output: FlagId,        // Result (Output)
}

/// Source of a Flag's activation.
/// Essential for "White Box" debugging and logic tracing.
#[derive(Clone, Debug)]
enum Source {
    Input,                           // Axiom injected by user
    Derived { causes: Vec<FlagId> }, // Reasoned from rules
}

/// The simplest BI Runtime Kernel.
struct Mind {
    // --- Symbol Table (Human <-> Machine) ---
    label_to_id: HashMap<String, FlagId>,
    id_to_label: HashMap<FlagId, String>,
    next_id: FlagId,

    // --- Static Memory (The Brain Structure) ---
    rules: Vec<Rule>,

    // --- Dynamic Memory (Consciousness / RAM) ---
    // Stores the active flags and the reason WHY they are active.
    active_memory: HashMap<FlagId, Source>,
}

impl Mind {
    fn new() -> Self {
        Self {
            label_to_id: HashMap::new(),
            id_to_label: HashMap::new(),
            next_id: 1,
            rules: Vec::new(),
            active_memory: HashMap::new(),
        }
    }

    // ========================================================================
    // Compile-time Helper (Knowledge Construction)
    // ========================================================================

    /// Get ID for a label, creating it if necessary.
    fn id(&mut self, label: &str) -> FlagId {
        if let Some(&id) = self.label_to_id.get(label) {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.label_to_id.insert(label.to_string(), id);
            self.id_to_label.insert(id, label.to_string());
            id
        }
    }

    fn label(&self, id: FlagId) -> String {
        self.id_to_label
            .get(&id)
            .cloned()
            .unwrap_or(format!("?{}", id))
    }

    /// Add a logical rule: A + B + ... -> C
    fn learn(&mut self, inputs: &[&str], output: &str) {
        let t_ids: Vec<FlagId> = inputs.iter().map(|n| self.id(n)).collect();
        let o_id = self.id(output);

        self.rules.push(Rule {
            triggers: t_ids,
            output: o_id,
        });
    }

    // ========================================================================
    // Runtime Execution (Inference)
    // ========================================================================

    /// Inject initial facts (Stimuli).
    fn inject(&mut self, inputs: &[&str]) {
        for name in inputs {
            let id = self.id(name);
            self.active_memory.insert(id, Source::Input);
            println!("[Input] + `{}`", style(name).green());
        }
    }

    /// Run one cycle of thought processing.
    /// Returns true if the mind state changed (new thoughts derived).
    fn tick(&mut self, tick_count: usize) -> bool {
        let mut new_facts: Vec<(FlagId, Vec<FlagId>)> = Vec::new();

        // 1. Scan Rules against Active Memory
        for rule in &self.rules {
            // Optimization: Don't re-derive known facts
            if self.active_memory.contains_key(&rule.output) {
                continue;
            }

            // Logic Gate: AND (All triggers must be present)
            let all_met = rule
                .triggers
                .iter()
                .all(|t| self.active_memory.contains_key(t));

            if all_met {
                new_facts.push((rule.output, rule.triggers.clone()));
            }
        }

        if new_facts.is_empty() {
            return false;
        }

        // 2. Commit new facts to memory (Neuron Activation)
        let header = format!("[Tick {tick_count}]");
        let padding = " ".repeat(header.len());

        for (i, (output_id, causes)) in new_facts.iter().enumerate() {
            // Log format: `CauseA`, `CauseB` ---> `Result`
            let cause_str = causes
                .iter()
                .map(|id| format!("`{}`", self.label(*id)))
                .collect::<Vec<_>>()
                .join(", ");

            let out_str = format!("`{}`", self.label(*output_id));

            let message = format!("{} ---> {}", cause_str, style(out_str).yellow().bold());
            if i == 0 {
                println!("{header} {message}");
            } else {
                println!("{padding} {message}");
            }

            self.active_memory.insert(
                *output_id,
                Source::Derived {
                    causes: causes.clone(),
                },
            );
        }

        true
    }

    // ========================================================================
    // Analysis (White Box Debugging)
    // ========================================================================

    /// Visualizes the logic chain for a specific concept.
    fn trace(&self, target: &str) {
        println!("\n=== Trace: `{}` ===", target);

        if let Some(&id) = self.label_to_id.get(target) {
            if self.active_memory.contains_key(&id) {
                let mut builder = TreeBuilder::new(self.node_text(id));
                self.build_tree_recursive(id, &mut builder);
                print_tree(&builder.build()).unwrap();
            } else {
                println!("Memory does not contain `{}`", target);
            }
        } else {
            println!("Unknown concept: `{}`", target);
        }
    }

    fn node_text(&self, id: FlagId) -> String {
        let label = self.label(id);
        match self.active_memory.get(&id) {
            Some(Source::Input) => format!("`{}` (Input)", label),
            Some(Source::Derived { .. }) => format!("`{}`", label),
            None => format!("`{}` (MISSING)", label),
        }
    }

    fn build_tree_recursive(&self, id: FlagId, builder: &mut TreeBuilder) {
        if let Some(Source::Derived { causes }) = self.active_memory.get(&id) {
            for &cause_id in causes {
                builder.begin_child(self.node_text(cause_id));
                self.build_tree_recursive(cause_id, builder);
                builder.end_child();
            }
        }
    }
}

fn main() {
    let mut mind = Mind::new();

    // ------------------------------------------------------------------------
    // Phase 1: Pre-compilation (Hardcoding the Static Logic Graph)
    // ------------------------------------------------------------------------
    // In a real scenario, the Compiler generates this from config files.

    // Inheritance: A implies B
    mind.learn(&["Knife"], "Sharp");
    mind.learn(&["Apple"], "Fruit");
    mind.learn(&["Fruit"], "Solid");

    // Interaction: Logic calculation
    // "If something Sharp cuts something Solid, Separation occurs"
    mind.learn(&["Sharp", "Solid", "Cut"], "Separation");

    // Concept Discovery:
    // "If a Fruit is Separated, it becomes Fruit Slices"
    mind.learn(&["Separation", "Fruit"], "Fruit Slices");

    // ------------------------------------------------------------------------
    // Phase 2: Runtime (Inference)
    // ------------------------------------------------------------------------
    println!("--- Simulation Start ---");

    // Scenario: User holds a Knife, an Apple, and performs Cut action.
    mind.inject(&["Knife", "Apple", "Cut"]);

    let mut tick = 1;
    loop {
        if !mind.tick(tick) {
            break;
        }
        tick += 1;
    }
    println!("--- Simulation Stable ---\n");

    // ------------------------------------------------------------------------
    // Phase 3: Explainability (White Box Check)
    // ------------------------------------------------------------------------
    // Prove that the AI knows WHY it thinks "Fruit Slices" exist.
    mind.trace("Fruit Slices");
}
