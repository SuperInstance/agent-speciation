//! # agent-speciation
//!
//! Spontaneous role differentiation in agent fleets.
//!
//! Based on the hypothesis that contrary motion leads to differentiation,
//! and an ensemble of agents with no leader will spontaneously speciate
//! into 4-5 distinct types. Roles EMERGE from behavior, not assignment.

use std::collections::HashMap;

/// An emergent species/role type that arises from agent behavior patterns.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AgentSpecies {
    /// Explorer: seeks new territory, high novelty output.
    Explorer,
    /// Synthesizer: combines inputs from others, integrative output.
    Synthesizer,
    /// Critic: evaluates and refines, corrective output.
    Critic,
    /// Generator: produces large volumes of diverse output.
    Generator,
    /// Sentinel: monitors and maintains, stability-focused output.
    Sentinel,
    /// Unknown: not yet classified.
    Unclassified,
}

impl AgentSpecies {
    /// Get all defined species variants.
    pub fn all() -> Vec<AgentSpecies> {
        vec![
            AgentSpecies::Explorer,
            AgentSpecies::Synthesizer,
            AgentSpecies::Critic,
            AgentSpecies::Generator,
            AgentSpecies::Sentinel,
        ]
    }

    /// Get a human-readable name for the species.
    pub fn name(&self) -> &str {
        match self {
            AgentSpecies::Explorer => "Explorer",
            AgentSpecies::Synthesizer => "Synthesizer",
            AgentSpecies::Critic => "Critic",
            AgentSpecies::Generator => "Generator",
            AgentSpecies::Sentinel => "Sentinel",
            AgentSpecies::Unclassified => "Unclassified",
        }
    }
}

/// Behavioral signature of an agent: captures output patterns for species detection.
#[derive(Clone, Debug)]
pub struct BehavioralSignature {
    /// The agent's unique identifier.
    pub agent_id: u64,
    /// Average novelty of outputs (0.0–1.0).
    pub novelty_rate: f64,
    /// Average integrative score (combining others' inputs).
    pub integrative_score: f64,
    /// Average corrective score (fixing/improving others' work).
    pub corrective_score: f64,
    /// Volume of output (units per tick).
    pub output_volume: f64,
    /// Stability score (consistency over time).
    pub stability: f64,
    /// Contrary motion score: how different from the fleet average.
    pub contrariness: f64,
    /// History of behavioral vectors for temporal analysis.
    pub history: Vec<[f64; 6]>,
}

impl BehavioralSignature {
    /// Create a new behavioral signature.
    pub fn new(agent_id: u64) -> Self {
        Self {
            agent_id,
            novelty_rate: 0.0,
            integrative_score: 0.0,
            corrective_score: 0.0,
            output_volume: 0.0,
            stability: 0.5,
            contrariness: 0.0,
            history: vec![],
        }
    }

    /// Create from explicit behavioral values.
    pub fn from_values(
        agent_id: u64,
        novelty_rate: f64,
        integrative_score: f64,
        corrective_score: f64,
        output_volume: f64,
        stability: f64,
        contrariness: f64,
    ) -> Self {
        let sig = Self {
            agent_id,
            novelty_rate,
            integrative_score,
            corrective_score,
            output_volume,
            stability,
            contrariness,
            history: vec![],
        };
        sig
    }

    /// Get the behavioral vector for clustering.
    pub fn vector(&self) -> [f64; 6] {
        [
            self.novelty_rate,
            self.integrative_score,
            self.corrective_score,
            self.output_volume,
            self.stability,
            self.contrariness,
        ]
    }

    /// Record current state to history.
    pub fn snapshot(&mut self) {
        self.history.push(self.vector());
    }

    /// Compute Euclidean distance to another signature's behavioral vector.
    pub fn distance_to(&self, other: &BehavioralSignature) -> f64 {
        self.vector()
            .iter()
            .zip(other.vector().iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

/// Detects species by clustering agents based on their behavioral signatures.
#[derive(Clone, Debug)]
pub struct SpeciesDetector {
    /// Current species assignments: agent_id → species.
    pub assignments: HashMap<u64, AgentSpecies>,
    /// Cluster centroids: species → behavioral vector.
    pub centroids: HashMap<String, [f64; 6]>,
    /// Minimum distance to consider an agent a new species.
    pub speciation_threshold: f64,
}

impl SpeciesDetector {
    /// Create a new species detector.
    pub fn new(speciation_threshold: f64) -> Self {
        let mut centroids = HashMap::new();
        // Pre-seed centroids for known species
        centroids.insert("Explorer".to_string(), [0.9, 0.2, 0.1, 0.6, 0.3, 0.8]);
        centroids.insert("Synthesizer".to_string(), [0.4, 0.9, 0.3, 0.5, 0.6, 0.4]);
        centroids.insert("Critic".to_string(), [0.3, 0.4, 0.9, 0.4, 0.7, 0.5]);
        centroids.insert("Generator".to_string(), [0.5, 0.3, 0.2, 0.95, 0.2, 0.6]);
        centroids.insert("Sentinel".to_string(), [0.2, 0.3, 0.4, 0.3, 0.95, 0.3]);

        Self {
            assignments: HashMap::new(),
            centroids,
            speciation_threshold,
        }
    }

    /// Classify an agent based on its behavioral signature.
    pub fn classify(&mut self, signature: &BehavioralSignature) -> AgentSpecies {
        let vec = signature.vector();

        let mut best_species = AgentSpecies::Unclassified;
        let mut best_distance = f64::INFINITY;

        for (name, centroid) in &self.centroids {
            let dist = euclidean_distance(&vec, centroid);
            if dist < best_distance {
                best_distance = dist;
                best_species = match name.as_str() {
                    "Explorer" => AgentSpecies::Explorer,
                    "Synthesizer" => AgentSpecies::Synthesizer,
                    "Critic" => AgentSpecies::Critic,
                    "Generator" => AgentSpecies::Generator,
                    "Sentinel" => AgentSpecies::Sentinel,
                    _ => AgentSpecies::Unclassified,
                };
            }
        }

        self.assignments.insert(signature.agent_id, best_species.clone());
        best_species
    }

    /// Classify a batch of agents.
    pub fn classify_batch(&mut self, signatures: &[BehavioralSignature]) -> Vec<(u64, AgentSpecies)> {
        signatures.iter().map(|s| {
            let species = self.classify(s);
            (s.agent_id, species)
        }).collect()
    }

    /// Get current species distribution.
    pub fn distribution(&self) -> HashMap<AgentSpecies, usize> {
        let mut dist = HashMap::new();
        for species in self.assignments.values() {
            *dist.entry(species.clone()).or_insert(0) += 1;
        }
        dist
    }

    /// Count distinct species currently assigned.
    pub fn species_count(&self) -> usize {
        self.assignments
            .values()
            .collect::<std::collections::HashSet<_>>()
            .len()
    }
}

fn euclidean_distance(a: &[f64; 6], b: &[f64; 6]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}

/// Tracks species stability over time.
#[derive(Clone, Debug)]
pub struct NicheTracker {
    /// History of species assignments at each tick.
    pub timeline: Vec<(u64, HashMap<u64, AgentSpecies>)>,
    /// Current assignments.
    pub current: HashMap<u64, AgentSpecies>,
}

impl NicheTracker {
    /// Create a new niche tracker.
    pub fn new() -> Self {
        Self {
            timeline: vec![],
            current: HashMap::new(),
        }
    }

    /// Record the current state at a given tick.
    pub fn record(&mut self, tick: u64, assignments: &HashMap<u64, AgentSpecies>) {
        self.current = assignments.clone();
        self.timeline.push((tick, assignments.clone()));
    }

    /// Compute stability: fraction of agents that kept the same species between last two ticks.
    pub fn stability(&self) -> f64 {
        if self.timeline.len() < 2 {
            return 1.0;
        }
        let prev = &self.timeline[self.timeline.len() - 2].1;
        let curr = &self.timeline[self.timeline.len() - 1].1;

        if curr.is_empty() {
            return 1.0;
        }

        let same = curr.iter().filter(|(id, sp)| prev.get(id) == Some(sp)).count();
        same as f64 / curr.len() as f64
    }

    /// Get the number of ticks recorded.
    pub fn len(&self) -> usize {
        self.timeline.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.timeline.is_empty()
    }
}

impl Default for NicheTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Rules that enforce speciation: never repeat, always contrary motion.
#[derive(Clone, Debug)]
pub struct SpeciationRules {
    /// Whether to enforce contrary motion (agents should diverge from fleet average).
    pub enforce_contrary_motion: bool,
    /// Whether to prevent two agents from occupying the same niche.
    pub enforce_no_repeat: bool,
    /// The fleet behavioral average, updated as agents report.
    pub fleet_average: [f64; 6],
    /// Number of agents contributing to the average.
    pub fleet_size: usize,
}

impl SpeciationRules {
    /// Create new speciation rules.
    pub fn new() -> Self {
        Self {
            enforce_contrary_motion: true,
            enforce_no_repeat: true,
            fleet_average: [0.5; 6],
            fleet_size: 0,
        }
    }

    /// Update fleet average with a new behavioral vector.
    pub fn update_fleet_average(&mut self, vector: &[f64; 6]) {
        self.fleet_size += 1;
        let n = self.fleet_size as f64;
        for i in 0..6 {
            self.fleet_average[i] = self.fleet_average[i] * (n - 1.0) / n + vector[i] / n;
        }
    }

    /// Compute how contrary a behavioral vector is to the fleet average.
    pub fn contrariness(&self, vector: &[f64; 6]) -> f64 {
        euclidean_distance(vector, &self.fleet_average)
    }

    /// Apply contrary motion: nudge a behavioral vector away from the fleet average.
    pub fn apply_contrary_motion(&self, vector: &mut [f64; 6], strength: f64) {
        if !self.enforce_contrary_motion || self.fleet_size == 0 {
            return;
        }
        for i in 0..6 {
            let diff = vector[i] - self.fleet_average[i];
            vector[i] += diff * strength;
            // Clamp to [0, 1]
            vector[i] = vector[i].clamp(0.0, 1.0);
        }
    }

    /// Check if an agent is too similar to existing species assignments.
    /// Returns true if the agent's niche is already occupied.
    pub fn is_niche_occupied(&self, agent_vector: &[f64; 6], existing: &[[f64; 6]], threshold: f64) -> bool {
        if !self.enforce_no_repeat {
            return false;
        }
        existing.iter().any(|v| euclidean_distance(agent_vector, v) < threshold)
    }
}

impl Default for SpeciationRules {
    fn default() -> Self {
        Self::new()
    }
}

/// Monitors the ecosystem: species count, diversity, extinction events.
#[derive(Clone, Debug)]
pub struct EcosystemMonitor {
    /// Species distribution history over time.
    pub history: Vec<(u64, HashMap<AgentSpecies, usize>)>,
    /// Extinction events: (tick, species that went extinct).
    pub extinctions: Vec<(u64, AgentSpecies)>,
    /// Speciation events: (tick, new species that appeared).
    pub speciations: Vec<(u64, AgentSpecies)>,
}

impl EcosystemMonitor {
    /// Create a new ecosystem monitor.
    pub fn new() -> Self {
        Self {
            history: vec![],
            extinctions: vec![],
            speciations: vec![],
        }
    }

    /// Record a snapshot of the ecosystem at a given tick.
    pub fn record(&mut self, tick: u64, distribution: &HashMap<AgentSpecies, usize>) {
        // Check for extinctions and speciations
        if let Some((_, prev_dist)) = self.history.last() {
            for (species, count) in prev_dist {
                if *count > 0 && distribution.get(species).copied().unwrap_or(0) == 0 {
                    self.extinctions.push((tick, species.clone()));
                }
            }
            for (species, count) in distribution {
                if *count > 0 && prev_dist.get(species).copied().unwrap_or(0) == 0 {
                    self.speciations.push((tick, species.clone()));
                }
            }
        } else {
            // First snapshot: all present species are speciation events
            for (species, count) in distribution {
                if *count > 0 {
                    self.speciations.push((tick, species.clone()));
                }
            }
        }

        self.history.push((tick, distribution.clone()));
    }

    /// Compute Shannon diversity index of the current ecosystem.
    pub fn diversity_index(&self) -> f64 {
        let current = match self.history.last() {
            Some((_, dist)) => dist,
            None => return 0.0,
        };

        let total: usize = current.values().sum();
        if total == 0 {
            return 0.0;
        }

        let mut h = 0.0;
        for &count in current.values() {
            if count > 0 {
                let p = count as f64 / total as f64;
                h -= p * p.ln();
            }
        }
        h
    }

    /// Get the current species count.
    pub fn species_count(&self) -> usize {
        match self.history.last() {
            Some((_, dist)) => dist.values().filter(|&&c| c > 0).count(),
            None => 0,
        }
    }

    /// Get the dominant species (highest count).
    pub fn dominant_species(&self) -> Option<AgentSpecies> {
        let current = self.history.last()?.1.iter()
            .filter(|(_, c)| **c > 0)
            .max_by_key(|(_, c)| **c)?;
        Some(current.0.clone())
    }

    /// Total extinction events recorded.
    pub fn extinction_count(&self) -> usize {
        self.extinctions.len()
    }

    /// Total speciation events recorded.
    pub fn speciation_count(&self) -> usize {
        self.speciations.len()
    }

    /// Get evenness: how equally distributed agents are across species (0–1).
    pub fn evenness(&self) -> f64 {
        let count = self.species_count();
        if count == 0 {
            return 0.0;
        }
        let max_diversity = (count as f64).ln();
        if max_diversity == 0.0 {
            return 1.0;
        }
        self.diversity_index() / max_diversity
    }
}

impl Default for EcosystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_detection() {
        let mut detector = SpeciesDetector::new(1.0);

        // Explorer-like: high novelty, low integrative
        let explorer = BehavioralSignature::from_values(1, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8);
        let species = detector.classify(&explorer);
        assert_eq!(species, AgentSpecies::Explorer);

        // Synthesizer-like: high integrative
        let synth = BehavioralSignature::from_values(2, 0.4, 0.9, 0.3, 0.5, 0.6, 0.4);
        let species2 = detector.classify(&synth);
        assert_eq!(species2, AgentSpecies::Synthesizer);
    }

    #[test]
    fn test_batch_classification() {
        let mut detector = SpeciesDetector::new(1.0);
        let sigs = vec![
            BehavioralSignature::from_values(1, 0.9, 0.2, 0.1, 0.6, 0.3, 0.8),
            BehavioralSignature::from_values(2, 0.3, 0.4, 0.9, 0.4, 0.7, 0.5),
            BehavioralSignature::from_values(3, 0.5, 0.3, 0.2, 0.95, 0.2, 0.6),
        ];

        let results = detector.classify_batch(&sigs);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].0, 1);
        assert_eq!(results[1].0, 2);
        assert_eq!(results[2].0, 3);
    }

    #[test]
    fn test_clustering_distance() {
        let a = BehavioralSignature::from_values(1, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let b = BehavioralSignature::from_values(2, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let dist = a.distance_to(&b);
        assert!((dist - 2.0_f64.sqrt()).abs() < 1e-10);

        let c = BehavioralSignature::from_values(3, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!((a.distance_to(&c)).abs() < 1e-10);
    }

    #[test]
    fn test_diversity_index() {
        let mut monitor = EcosystemMonitor::new();

        // Uniform distribution = high diversity
        let uniform: HashMap<AgentSpecies, usize> = vec![
            (AgentSpecies::Explorer, 2),
            (AgentSpecies::Synthesizer, 2),
            (AgentSpecies::Critic, 2),
            (AgentSpecies::Generator, 2),
        ].into_iter().collect();
        monitor.record(0, &uniform);
        let div = monitor.diversity_index();
        assert!(div > 1.0, "Uniform distribution should have high diversity, got {}", div);

        // Single species = zero diversity
        let mut monitor2 = EcosystemMonitor::new();
        let single: HashMap<AgentSpecies, usize> = vec![
            (AgentSpecies::Explorer, 8),
        ].into_iter().collect();
        monitor2.record(0, &single);
        assert!((monitor2.diversity_index() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_contrary_motion_enforcement() {
        let mut rules = SpeciationRules::new();
        assert!(rules.enforce_contrary_motion);

        // Simulate fleet: all at 0.5
        for _ in 0..4 {
            rules.update_fleet_average(&[0.5; 6]);
        }

        // An agent at 0.6 should be pushed further from 0.5
        let mut vec = [0.6, 0.4, 0.5, 0.5, 0.5, 0.5];
        let original_contrariness = rules.contrariness(&vec);
        rules.apply_contrary_motion(&mut vec, 0.5);
        let new_contrariness = rules.contrariness(&vec);
        assert!(new_contrariness >= original_contrariness);
    }

    #[test]
    fn test_niche_occupied_detection() {
        let rules = SpeciationRules::new();
        let existing = vec![[0.9, 0.2, 0.1, 0.6, 0.3, 0.8]];
        let same = [0.9, 0.2, 0.1, 0.6, 0.3, 0.8];
        assert!(rules.is_niche_occupied(&same, &existing, 0.1));

        let different = [0.1, 0.9, 0.9, 0.1, 0.9, 0.1];
        assert!(!rules.is_niche_occupied(&different, &existing, 0.1));
    }

    #[test]
    fn test_speciation_over_time() {
        let mut monitor = EcosystemMonitor::new();
        let mut detector = SpeciesDetector::new(1.0);
        let mut tracker = NicheTracker::new();

        // 8 agents, each with different behavioral profiles
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

        // Tick 0: classify all
        let _ = detector.classify_batch(&agents);
        let dist = detector.distribution();
        monitor.record(0, &dist);
        tracker.record(0, &detector.assignments);

        // Should have multiple species
        assert!(detector.species_count() >= 2, "Should speciate into multiple species");
        assert!(monitor.species_count() >= 2);
    }

    #[test]
    fn test_extinction_and_speciation_events() {
        let mut monitor = EcosystemMonitor::new();

        let tick0: HashMap<AgentSpecies, usize> = vec![
            (AgentSpecies::Explorer, 3),
            (AgentSpecies::Synthesizer, 3),
            (AgentSpecies::Critic, 2),
        ].into_iter().collect();
        monitor.record(0, &tick0);

        let tick1: HashMap<AgentSpecies, usize> = vec![
            (AgentSpecies::Explorer, 4),
            (AgentSpecies::Synthesizer, 4),
            // Critic went extinct
        ].into_iter().collect();
        monitor.record(1, &tick1);

        assert_eq!(monitor.extinction_count(), 1);
        assert_eq!(monitor.extinctions[0].1, AgentSpecies::Critic);
        assert_eq!(monitor.speciation_count(), 3); // 3 species at tick 0
    }

    #[test]
    fn test_evenness() {
        let mut monitor = EcosystemMonitor::new();

        // Perfect evenness
        let even: HashMap<AgentSpecies, usize> = vec![
            (AgentSpecies::Explorer, 2),
            (AgentSpecies::Synthesizer, 2),
            (AgentSpecies::Critic, 2),
        ].into_iter().collect();
        monitor.record(0, &even);
        assert!((monitor.evenness() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_dominant_species() {
        let mut monitor = EcosystemMonitor::new();
        let dist: HashMap<AgentSpecies, usize> = vec![
            (AgentSpecies::Explorer, 1),
            (AgentSpecies::Synthesizer, 5),
            (AgentSpecies::Critic, 2),
        ].into_iter().collect();
        monitor.record(0, &dist);
        assert_eq!(monitor.dominant_species(), Some(AgentSpecies::Synthesizer));
    }

    #[test]
    fn test_niche_tracker_stability() {
        let mut tracker = NicheTracker::new();

        let a1: HashMap<u64, AgentSpecies> = vec![
            (1, AgentSpecies::Explorer),
            (2, AgentSpecies::Synthesizer),
        ].into_iter().collect();
        tracker.record(0, &a1);

        // Same assignments
        tracker.record(1, &a1);
        assert!((tracker.stability() - 1.0).abs() < 1e-10);

        // Changed one
        let a2: HashMap<u64, AgentSpecies> = vec![
            (1, AgentSpecies::Critic),
            (2, AgentSpecies::Synthesizer),
        ].into_iter().collect();
        tracker.record(2, &a2);
        assert!((tracker.stability() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_behavioral_signature_snapshot() {
        let mut sig = BehavioralSignature::new(1);
        assert!(sig.history.is_empty());
        sig.snapshot();
        assert_eq!(sig.history.len(), 1);
        sig.novelty_rate = 0.8;
        sig.snapshot();
        assert_eq!(sig.history.len(), 2);
    }

    #[test]
    fn test_species_all_variants() {
        let all = AgentSpecies::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&AgentSpecies::Explorer));
        assert!(all.contains(&AgentSpecies::Synthesizer));
    }
}
