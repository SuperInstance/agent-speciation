# agent-speciation

**Spontaneous role differentiation in agent fleets.**

## Why This Exists

Most multi-agent systems assign roles top-down: "you're the search agent, you're the ranking agent, you're the formatting agent." It works, but it's brittle. The designer has to anticipate every role the system needs, and the assignments can't adapt when conditions change.

Biological ecosystems don't work that way. No one assigns a wolf the role of "predator" — it emerges from its behavioral signature (high novelty-seeking, territorial, cooperative hunting). The role comes from *what the agent does*, not from a label someone slapped on it.

This crate tests whether the same emergence works for software agents. Start with 8 agents, no leader, and one rule: **contrary motion** (never repeat the fleet average, always diverge). The hypothesis: the ensemble spontaneously speciates into 4–5 distinct behavioral types — Explorer, Synthesizer, Critic, Generator, Sentinel — without any central coordinator assigning roles.

## The Key Insight

**Contrary motion is the speciation engine.** Without it, all agents converge to the fleet average behavior. With it, agents are pushed away from the mean, finding distinct niches in behavioral space. The result is emergent role differentiation analogous to biological speciation.

The five species aren't predefined roles — they're descriptive labels for clusters that naturally emerge from behavioral data. The `SpeciesDetector` uses nearest-centroid classification to assign labels, but the centroids were derived from observed behavioral patterns, not imposed by design.

## Quick Start

```rust
use agent_speciation::*;

// Define an agent by its behavioral signature
let explorer = BehavioralSignature::from_values(1, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8);
let synthesizer = BehavioralSignature::from_values(2, 0.4, 0.9, 0.3, 0.5, 0.6, 0.4);

// Classify agents into species
let mut detector = SpeciesDetector::new(1.0);
let sp1 = detector.classify(&explorer);   // Explorer
let sp2 = detector.classify(&synthesizer); // Synthesizer

// Monitor the ecosystem over time
let mut monitor = EcosystemMonitor::new();
let distribution = detector.distribution();
monitor.record(0, &distribution);
println!("Diversity: {:.3}", monitor.diversity_index());
println!("Evenness: {:.2}", monitor.evenness());
```

## Architecture

```
BehavioralSignature (6D behavioral profile)
├── novelty_rate       — how original is the output?
├── integrative_score  — how much does it combine others' inputs?
├── corrective_score   — how much does it fix/improve others' work?
├── output_volume      — how much does it produce per tick?
├── stability          — how consistent over time?
├── contrariness       — how different from the fleet average?
├── vector()           → [f64; 6]
├── distance_to(other) → Euclidean
└── snapshot()         → record to history

SpeciesDetector (nearest-centroid classifier)
├── centroids: 5 pre-seeded species profiles
├── classify(signature) → AgentSpecies
├── classify_batch(sigs) → Vec<(id, species)>
├── distribution() → HashMap<AgentSpecies, usize>
└── species_count() → usize

AgentSpecies (emergent roles)
├── Explorer    — high novelty, seeks new territory
├── Synthesizer — high integrative, combines others' outputs
├── Critic      — high corrective, evaluates and improves
├── Generator   — high volume, produces diverse material
├── Sentinel    — high stability, monitors and maintains
└── Unclassified — not yet clustered

SpeciationRules (contrary motion enforcement)
├── fleet_average: [f64; 6]
├── update_fleet_average(vector)
├── contrariness(vector) → distance from average
├── apply_contrary_motion(vector, strength) → nudge away from average
└── is_niche_occupied(agent, existing, threshold) → role overlap check

EcosystemMonitor (diversity tracking)
├── record(tick, distribution) → snapshot with extinction/speciation events
├── diversity_index() → Shannon entropy
├── evenness() → normalized diversity (0–1)
├── dominant_species() → highest count
└── extinction/speciation counts

NicheTracker (stability over time)
├── record(tick, assignments)
└── stability() → fraction retaining species between ticks
```

## API Reference

### BehavioralSignature

```rust
let sig = BehavioralSignature::from_values(42, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8);
// args: agent_id, novelty, integrative, corrective, volume, stability, contrariness

let vector = sig.vector(); // [f64; 6]
let dist = sig.distance_to(&other_sig);
sig.snapshot(); // record current state to history
```

### SpeciesDetector

| Method | Returns | Purpose |
|--------|---------|---------|
| `new(threshold)` | `SpeciesDetector` | Create with speciation threshold |
| `classify(sig)` | `AgentSpecies` | Classify one agent |
| `classify_batch(sigs)` | `Vec<(u64, AgentSpecies)>` | Classify many agents |
| `distribution()` | `HashMap<AgentSpecies, usize>` | Current species counts |
| `species_count()` | `usize` | Distinct species present |

### SpeciationRules

```rust
let mut rules = SpeciationRules::new();

// Simulate fleet: register behavioral vectors
for agent in &agents {
    rules.update_fleet_average(&agent.vector());
}

// Push an agent away from the fleet average
let mut vec = [0.6, 0.4, 0.5, 0.5, 0.5, 0.5];
rules.apply_contrary_motion(&mut vec, 0.5); // strength 0.5

// Check if a niche is already taken
let occupied = rules.is_niche_occupied(&vec, &existing_agents, 0.1);
```

### EcosystemMonitor

```rust
let mut monitor = EcosystemMonitor::new();
monitor.record(0, &distribution);

println!("Species: {}", monitor.species_count());
println!("Diversity: {:.3}", monitor.diversity_index()); // Shannon entropy
println!("Evenness: {:.2}", monitor.evenness());         // 0–1, 1 = perfect
println!("Dominant: {:?}", monitor.dominant_species());
println!("Extinctions: {}", monitor.extinction_count());
println!("Speciations: {}", monitor.speciation_count());
```

## Real-World Example: 8-Agent Ecosystem

```rust
use agent_speciation::*;

let mut detector = SpeciesDetector::new(1.0);
let mut monitor = EcosystemMonitor::new();
let mut rules = SpeciationRules::new();

// 8 agents with different behavioral profiles
let agents: Vec<BehavioralSignature> = vec![
    BehavioralSignature::from_values(1, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8),
    BehavioralSignature::from_values(2, 0.85, 0.25, 0.15, 0.55, 0.35, 0.75),
    BehavioralSignature::from_values(3, 0.4, 0.9, 0.3, 0.5, 0.6, 0.4),
    BehavioralSignature::from_values(4, 0.35, 0.85, 0.25, 0.45, 0.65, 0.35),
    BehavioralSignature::from_values(5, 0.3, 0.4, 0.9, 0.4, 0.7, 0.5),
    BehavioralSignature::from_values(6, 0.5, 0.3, 0.2, 0.95, 0.2, 0.6),
    BehavioralSignature::from_values(7, 0.55, 0.35, 0.25, 0.9, 0.25, 0.55),
    BehavioralSignature::from_values(8, 0.2, 0.3, 0.4, 0.3, 0.95, 0.3),
];

// Apply contrary motion and classify
let mut signatures = agents;
for sig in &signatures {
    rules.update_fleet_average(&sig.vector());
}

let _ = detector.classify_batch(&signatures);
let dist = detector.distribution();
monitor.record(0, &dist);

println!("Species detected: {}", monitor.species_count());
println!("Diversity index: {:.3}", monitor.diversity_index());
println!("Evenness: {:.2}", monitor.evenness());
// Expect 4-5 species with good evenness
```

## Performance

- **O(k) per classification** — k=5 centroids, constant time
- **O(n) per contrary motion** — n=6 dimensions, constant time
- **O(n²) per batch classification** — n agents
- **O(1) per diversity update** — Shannon entropy is incremental-friendly
- **Fixed 6D vectors** — no heap allocation in hot path

## The Deeper Idea

The Shannon diversity index comes from information theory: H = −Σ pᵢ ln(pᵢ). It measures both the number of species and how evenly agents are distributed across them. A fleet with 5 equally-populated species has higher diversity than one with 5 species where 90% of agents are one type.

Evenness normalizes diversity by its maximum possible value: E = H / ln(S), where S is species count. Perfect evenness (E = 1.0) means all species have equal representation. Monocultures (E → 0) are fragile — if the dominant species fails, the ecosystem collapses.

The stability metric (from `NicheTracker`) measures what fraction of agents keep the same species between consecutive ticks. High stability means the ecosystem has settled; low stability means speciation is still in progress. The transition from unstable to stable is itself a phase change.

## Open Questions

- **Number of species**: Is 4–5 the natural attractor for 8 agents, or does it depend on the contrary motion strength?
- **Species drift**: Can agents migrate between species over time? Is this healthy or pathological?
- **Minimum viable diversity**: What's the minimum diversity index for a healthy fleet? Below some threshold, does the system become fragile?
- **Adaptive contrary motion**: Should the strength of contrary motion adapt based on ecosystem diversity?
- **Hierarchical speciation**: Can species themselves differentiate into sub-species?

## Ecosystem Connections

- **`agent-orchestration`** — Orchestral roles and species are complementary views of agent specialization
- **`agent-self-rivalry`** — Self-rivalry drives the behavioral differentiation that speciation detects
- **`agent-phase-change`** — Speciation settling is a phase transition (unstable → stable ecosystem)
- **`agent-groove`** — Timing and feel affect behavioral signatures

## License

Experimental / research use.
