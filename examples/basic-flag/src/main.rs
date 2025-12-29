//! Example: Basic Flag Logic
//!
//! This example demonstrates that "Logic" in BI is simply the topology of connections.
//! We implement standard logic gates (AND, OR, NOT/Inhibition) using only
//! the Flag Linkage system.
//!
//! It includes "Sanity Checks" to prove the BI doesn't just react to everything,
//! but strictly follows the logical constraints.

use console::style;
use std::collections::{HashMap, HashSet};

// ============================================================================
// Core Definitions
// ============================================================================

type FlagId = u32;

/// A Logic Link represents a synaptic connection.
/// It defines what stimulates a result and what inhibits it.
struct Link {
    triggers: Vec<FlagId>, // All must be present (AND logic)
    forbids: Vec<FlagId>,  // Any presence prevents output (NOT logic)
    output: FlagId,        // The resulting concept
}

/// The Thinking Engine
struct Mind {
    // Symbol Table
    label_to_id: HashMap<String, FlagId>,
    id_to_label: HashMap<FlagId, String>,
    next_id: FlagId,

    // Long-term Memory (Static Rules)
    links: Vec<Link>,

    // Short-term Memory (Active Context)
    active_flags: HashSet<FlagId>,
}

impl Mind {
    fn new() -> Self {
        Self {
            label_to_id: HashMap::new(),
            id_to_label: HashMap::new(),
            next_id: 1,
            links: Vec::new(),
            active_flags: HashSet::new(),
        }
    }

    // --- Knowledge Definition ---

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

    /// Define a rule: (A + B) - (C) -> D
    /// Logic: IF (A and B exist) AND (C does NOT exist) THEN D exists.
    fn rule(&mut self, triggers: &[&str], forbids: &[&str], output: &str) {
        let t_ids: Vec<FlagId> = triggers.iter().map(|n| self.id(n)).collect();
        let f_ids: Vec<FlagId> = forbids.iter().map(|n| self.id(n)).collect();
        let o_id = self.id(output);

        self.links.push(Link {
            triggers: t_ids,
            forbids: f_ids,
            output: o_id,
        });
    }

    // --- Runtime ---

    fn reset_memory(&mut self) {
        self.active_flags.clear();
    }

    fn inject(&mut self, inputs: &[&str]) {
        for name in inputs {
            let id = self.id(name);
            self.active_flags.insert(id);
            println!("[Input] + `{}`", style(name).green());
        }
    }

    fn tick(&mut self, tick_count: usize) -> bool {
        let mut new_activations = Vec::new();

        for link in &self.links {
            // 1. Check Output redundancy
            if self.active_flags.contains(&link.output) {
                continue;
            }

            // 2. Check Triggers (AND Gate)
            let triggers_met = link.triggers.iter().all(|t| self.active_flags.contains(t));
            if !triggers_met {
                continue;
            }

            // 3. Check Forbids (Inhibition / NOT Gate)
            let inhibition_active = link.forbids.iter().any(|f| self.active_flags.contains(f));
            if inhibition_active {
                continue;
            }

            // All conditions met
            new_activations.push((link.output, link.triggers.clone()));
        }

        if new_activations.is_empty() {
            return false;
        }

        // Commit Logic
        for (out_id, causes) in new_activations {
            let cause_str = causes
                .iter()
                .map(|c| format!("`{}`", self.label(*c)))
                .collect::<Vec<_>>()
                .join(", ");
            let out_str = style(format!("`{}`", self.label(out_id))).yellow().bold();

            println!("[Tick {}] {} ---> {}", tick_count, cause_str, out_str);

            self.active_flags.insert(out_id);
        }

        true
    }

    /// Runs until logic stabilizes
    fn ponder(&mut self) {
        let mut tick = 1;
        while self.tick(tick) {
            tick += 1;
        }
        println!(); // Spacer
    }
}

// ============================================================================
// Main Simulation
// ============================================================================

fn main() {
    let mut mind = Mind::new();

    println!("=== BI Logic Gate Test ===\n");

    // ---------------------------------------------------------
    // Case 1: AND Gate (Security System)
    // Concept: Strict requirement. All inputs must be present.
    // ---------------------------------------------------------
    println!("{}", style("--- Case 1: AND Logic (Security) ---").bold());

    // Logic: KeyCard + Fingerprint -> AccessGranted
    mind.rule(&["KeyCard", "Fingerprint"], &[], "AccessGranted");

    println!("Test A: Incomplete Input (Failure Expected)");
    mind.reset_memory();
    mind.inject(&["KeyCard"]); // Missing Fingerprint
    mind.ponder(); // Should produce NOTHING

    println!("Test B: Complete Input (Success Expected)");
    mind.reset_memory();
    mind.inject(&["KeyCard", "Fingerprint"]);
    mind.ponder(); // Should derive AccessGranted

    // ---------------------------------------------------------
    // Case 2: OR Gate (Alarm System)
    // Concept: Multiple paths to the same result.
    // ---------------------------------------------------------
    println!("{}", style("--- Case 2: OR Logic (Emergency) ---").bold());

    // Logic: Smoke -> Alarm
    // Logic: Heat -> Alarm
    // In BI, "OR" is just defining multiple separate rules for the same output.
    mind.rule(&["Smoke"], &[], "Alarm");
    mind.rule(&["Heat"], &[], "Alarm");

    println!("Test A: Path One");
    mind.reset_memory();
    mind.inject(&["Smoke"]);
    mind.ponder();

    println!("Test B: Path Two");
    mind.reset_memory();
    mind.inject(&["Heat"]);
    mind.ponder();

    // ---------------------------------------------------------
    // Case 3: NOT / Inhibition (Smart Light)
    // Concept: A flag can block a connection.
    // ---------------------------------------------------------
    println!(
        "{}",
        style("--- Case 3: Inhibition (Smart Light) ---").bold()
    );

    // Logic: SwitchOn + (NOT PowerOutage) -> LightOn
    mind.rule(&["SwitchOn"], &["PowerOutage"], "LightOn");

    println!("Test A: Switch On but Power Outage (Failure Expected)");
    mind.reset_memory();
    mind.inject(&["SwitchOn", "PowerOutage"]);
    mind.ponder(); // Should NOT turn light on

    println!("Test B: Normal Operation (Success Expected)");
    mind.reset_memory();
    mind.inject(&["SwitchOn"]);
    mind.ponder(); // Should turn light on
}
