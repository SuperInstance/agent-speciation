# agent-speciation

**Spontaneous role differentiation in agent fleets.**

An experimental Rust library exploring the hypothesis that roles in agent ensembles EMERGE from behavioral patterns rather than being assigned. Given 8 agents with no leader and a "contrary motion" rule (never repeat, always diverge from the fleet average), the ensemble spontaneously speciates into 4-5 distinct behavioral types.

## Origin

Based on Qwen's insight: *"contrary motion = differentiation, the ensemble speciates into 4-5 distinct types"*

This crate tests whether enforced behavioral divergence — a rule that agents must be different from the fleet average — naturally produces role differentiation analogous to biological speciation. No central coordinator assigns roles; they emerge from the dynamics of agents trying to be different from each other.

## Core Concepts

### AgentSpecies
Five emergent species identified by their behavioral signatures:
- **Explorer**: High novelty rate, seeks new territory. Driven by curiosity and the unknown.
- **Synthesizer**: High integrative score, combines others' outputs into coherent wholes.
- **Critic**: High corrective score, evaluates and improves existing work.
- **Generator**: High output volume, produces large quantities of diverse material.
- **Sentinel**: High stability, monitors and maintains system integrity.

These are NOT assigned — they emerge from clustering on behavioral vectors.

### BehavioralSignature
A 6-dimensional behavioral profile for each agent capturing:
1. `novelty_rate` — How novel/original is the agent's output?
2. `integrative_score` — How much does it combine others' inputs?
3. `corrective_score` — How much does it fix/improve others' work?
4. `output_volume` — How much does it produce per tick?
5. `stability` — How consistent is the agent over time?
6. `contrariness` — How different from the fleet average?

```rust
use agent_speciation::BehavioralSignature;

let agent = BehavioralSignature::from_values(1, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8);
```

### SpeciesDetector
Clusters agents by their behavioral vectors against pre-seeded species centroids. Uses nearest-centroid classification with a configurable speciation threshold. The detector tracks current assignments and can report species distribution.

```rust
use agent_speciation::{SpeciesDetector, BehavioralSignature};

let mut detector = SpeciesDetector::new(1.0);
let explorer = BehavioralSignature::from_values(1, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8);
let species = detector.classify(&explorer);
// species == AgentSpecies::Explorer
```

### NicheTracker
Monitors species assignments over time, computing stability metrics. Stability measures what fraction of agents keep the same species between consecutive ticks. High stability means species have settled into stable niches; low stability means ongoing differentiation.

### SpeciationRules
Enforces the two core rules of speciation:
1. **Contrary motion**: Agents are nudged away from the fleet average behavioral vector, ensuring divergence.
2. **No niche repetition**: Checks whether a behavioral vector is too similar to existing agents, preventing role overlap.

```rust
use agent_speciation::SpeciationRules;

let mut rules = SpeciationRules::new();
rules.update_fleet_average(&[0.5, 0.5, 0.5, 0.5, 0.5, 0.5]);
let mut vec = [0.6, 0.4, 0.5, 0.5, 0.5, 0.5];
rules.apply_contrary_motion(&mut vec, 0.5);
```

### EcosystemMonitor
Tracks the full ecosystem over time: species counts, diversity indices, extinction events, and speciation events. Provides the Shannon diversity index and evenness metrics to quantify ecosystem health.

- **Shannon diversity**: Measures both the number of species and how evenly agents are distributed across them
- **Evenness**: Shannon diversity divided by maximum possible diversity (perfect evenness = 1.0)
- **Extinction events**: When a species that had members drops to zero
- **Speciation events**: When a new species appears

```rust
use agent_speciation::EcosystemMonitor;
use std::collections::HashMap;

let mut monitor = EcosystemMonitor::new();
let distribution: HashMap<AgentSpecies, usize> = vec![
    (AgentSpecies::Explorer, 2),
    (AgentSpecies::Synthesizer, 2),
    (AgentSpecies::Critic, 2),
    (AgentSpecies::Generator, 2),
].into_iter().collect();
monitor.record(0, &distribution);
println!("Diversity: {}", monitor.diversity_index());
println!("Evenness: {}", monitor.evenness());
```

## Design Principles

1. **Emergence over assignment.** No one decides "you are the Explorer." The behavioral data speaks. Species labels are descriptive, not prescriptive.

2. **Contrary motion is the engine.** Without the rule pushing agents away from the fleet average, all agents would converge to the same behavior. Contrary motion is the speciation pressure.

3. **Stability is a signal.** When the NicheTracker shows high stability, the ecosystem has settled. When it fluctuates, speciation is still in progress.

4. **Diversity is health.** The ecosystem is healthiest when multiple species coexist with reasonable evenness. Monocultures (single species) are fragile.

## Metrics

- **Species count**: Number of distinct species currently present
- **Shannon diversity index**: Entropy-based measure of species diversity
- **Evenness**: How equally distributed agents are across species (0–1)
- **Stability**: Fraction of agents retaining species between ticks
- **Extinction/speciation events**: Transitions in ecosystem composition
- **Contrariness**: Distance from fleet average behavioral vector

## Testing

The crate includes 13 comprehensive tests covering:
- Single-agent and batch species classification
- Clustering distance calculations
- Shannon diversity index (uniform and degenerate cases)
- Evenness calculation
- Contrary motion enforcement and nudge strength
- Niche occupation detection
- Full speciation scenario with 8 agents
- Extinction and speciation event detection
- Niche tracker stability (perfect and partial stability)
- Behavioral signature snapshot history
- Species enumeration

Run tests with:
```bash
cargo test
```

## Experimental Hypothesis

The crate is designed to test whether 8+ agents with contrary-motion rules spontaneously differentiate into 4-5 stable species. If the `EcosystemMonitor` consistently shows 4-5 species with high stability after an initial differentiation period, it supports the emergence hypothesis. If agents oscillate between species or collapse into fewer types, the contrary-motion rule alone may be insufficient for stable speciation.

## License

Experimental / research use.
