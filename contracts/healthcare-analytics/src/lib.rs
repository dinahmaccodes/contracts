#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, BytesN, Env, Map,
    String, Symbol, Vec,
};

/// --------------------
/// Data Structures
/// --------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenderStats {
    pub male: u64,
    pub female: u64,
    pub other: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreatmentOutcome {
    pub treatment: String,
    pub count: u64,
    pub success_rate: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeDistribution {
    pub outcome: Symbol,
    pub count: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopulationStats {
    pub condition: String,
    pub total_cases: u64,
    pub average_age: u32,
    pub gender_distribution: GenderStats,
    pub common_treatments: Vec<TreatmentOutcome>,
    pub outcome_distribution: Vec<OutcomeDistribution>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualityScore {
    pub metric_name: String,
    pub score: u32,
    pub target: u32,
    pub percentile: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EfficiencyMetric {
    pub name: String,
    pub value: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerComparison {
    pub peer_group: Symbol,
    pub rank: u32,
    pub total_peers: u32,
    pub has_data: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderScorecard {
    pub provider_id: Address,
    pub quality_metrics: Vec<QualityScore>,
    pub patient_satisfaction: u32,
    pub efficiency_metrics: Vec<EfficiencyMetric>,
    pub peer_comparison: PeerComparison,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadmissionStats {
    pub facility_id: Address,
    pub condition: String,
    pub total_admissions: u64,
    pub readmissions: u64,
    pub readmission_rate: u32,
    pub days: u32,
    pub reporting_period: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplianceReport {
    pub provider_id: Address,
    pub compliance_type: Symbol,
    pub period: u64,
    pub compliant_cases: u64,
    pub total_cases: u64,
    pub compliance_rate: u32,
    pub issues_identified: Vec<String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BenchmarkResult {
    pub provider_id: Address,
    pub metric: String,
    pub provider_value: u32,
    pub peer_group: Symbol,
    pub peer_average: u32,
    pub peer_median: u32,
    pub percentile: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnonymizedOutcome {
    pub outcome_type: Symbol,
    pub condition: String,
    pub treatment: String,
    pub result: Symbol,
    pub age_group: Symbol,
    pub gender: Symbol,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualityMetric {
    pub provider_id: Address,
    pub metric_name: String,
    pub numerator: u32,
    pub denominator: u32,
    pub reporting_period: u64,
    pub calculated_rate: u32,
}

// --- Referral workflow types ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReferralStatus {
    Pending,
    Accepted,
    Declined,
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Referral {
    pub referral_id: u64,
    pub referring_provider: Address,
    pub receiving_provider: Address,
    pub patient_id: Address,
    pub specialty: Symbol,
    pub reason: String,
    pub priority: Symbol,
    pub status: ReferralStatus,
    pub created_at: u64,
    pub accepted_at: Option<u64>,
    pub completed_at: Option<u64>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ReferralError {
    NotFound = 1,
    Unauthorized = 2,
    InvalidState = 3,
    InvalidInput = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SatisfactionRecord {
    pub visit_id: u64,
    pub patient_id: Address,
    pub satisfaction_score: u32,
    pub feedback_hash: Option<BytesN<32>>,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadmissionRecord {
    pub facility_id: Address,
    pub condition: String,
    pub admission_count: u64,
    pub readmission_count: u64,
    pub days: u32,
    pub reporting_period: u64,
}

/// --------------------
/// Storage Keys
/// --------------------
/// Extended referral data (decline info, completion, etc.) stored separately
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferralDeclineInfo {
    pub decline_reason: String,
    pub suggest_alternative: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferralCompletionInfo {
    pub consultation_summary_hash: BytesN<32>,
    pub recommendations: String,
    pub followup_required: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CareSummaryRecord {
    pub from_provider: Address,
    pub summary_type: Symbol,
    pub summary_hash: BytesN<32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CareSummaryRequestRecord {
    pub requesting_provider: Address,
    pub information_needed: Vec<Symbol>,
}

#[contracttype]
pub enum DataKey {
    Outcomes(String),
    QualityMetrics(Address, String, u64),
    ProviderMetrics(Address),
    PopulationData(String),
    Readmissions(Address, String, u64),
    Satisfaction(u64),
    ProviderSatisfaction(Address),
    ComplianceData(Address, Symbol, u64),
    BenchmarkData(Symbol, String),
    // Referral workflow
    NextReferralId,
    Referral(u64),
    EstimatedAppointment(u64),
    ReferralDecline(u64),
    ReferralStatusNote(u64),
    ReferralCompletion(u64),
    CareSummary(u64, Symbol),
    CareSummaryRequest(u64),
}

#[contract]
pub struct HealthcareAnalytics;

#[contractimpl]
impl HealthcareAnalytics {
    /// Record anonymized patient outcome for population health tracking
    pub fn record_anonymized_outcome(
        env: Env,
        outcome_type: Symbol,
        condition: String,
        treatment: String,
        result: Symbol,
        age_group: Symbol,
        gender: Symbol,
        timestamp: u64,
    ) {
        let outcome = AnonymizedOutcome {
            outcome_type: outcome_type.clone(),
            condition: condition.clone(),
            treatment,
            result,
            age_group,
            gender,
            timestamp,
        };

        let key = DataKey::Outcomes(condition.clone());
        let mut outcomes: Vec<AnonymizedOutcome> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        outcomes.push_back(outcome);
        env.storage().persistent().set(&key, &outcomes);

        env.events()
            .publish((symbol_short!("rec_out"), condition), outcome_type);
    }

    /// Record quality metric for a provider
    pub fn record_quality_metric(
        env: Env,
        provider_id: Address,
        metric_name: String,
        numerator: u32,
        denominator: u32,
        reporting_period: u64,
    ) {
        provider_id.require_auth();

        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }

        let calculated_rate = (numerator as u64 * 10000 / denominator as u64) as u32;

        let metric = QualityMetric {
            provider_id: provider_id.clone(),
            metric_name: metric_name.clone(),
            numerator,
            denominator,
            reporting_period,
            calculated_rate,
        };

        let key =
            DataKey::QualityMetrics(provider_id.clone(), metric_name.clone(), reporting_period);
        env.storage().persistent().set(&key, &metric);

        env.events()
            .publish((symbol_short!("rec_qm"), provider_id), metric_name);
    }

    /// Calculate comprehensive provider scorecard
    pub fn calculate_provider_scorecard(
        env: Env,
        provider_id: Address,
        metrics: Vec<String>,
        period_start: u64,
        period_end: u64,
    ) -> ProviderScorecard {
        let mut quality_metrics: Vec<QualityScore> = Vec::new(&env);

        for i in 0..metrics.len() {
            let metric_name = metrics.get(i).unwrap();

            for period in period_start..=period_end {
                let key = DataKey::QualityMetrics(provider_id.clone(), metric_name.clone(), period);
                if let Some(metric) = env.storage().persistent().get::<_, QualityMetric>(&key) {
                    let score = metric.calculated_rate;
                    let target = 8500u32;
                    let percentile = if score >= target {
                        90
                    } else {
                        (score as u64 * 90 / target as u64) as u32
                    };

                    quality_metrics.push_back(QualityScore {
                        metric_name: metric_name.clone(),
                        score,
                        target,
                        percentile,
                    });

                    break;
                }
            }
        }

        let satisfaction =
            Self::get_avg_satisfaction(&env, provider_id.clone(), period_start, period_end);

        let efficiency_metrics: Vec<EfficiencyMetric> = Vec::new(&env);

        ProviderScorecard {
            provider_id,
            quality_metrics,
            patient_satisfaction: satisfaction,
            efficiency_metrics,
            peer_comparison: PeerComparison {
                peer_group: symbol_short!("none"),
                rank: 0,
                total_peers: 0,
                has_data: false,
            },
        }
    }

    /// Get population statistics for a condition
    pub fn get_population_statistics(
        env: Env,
        condition: String,
        age_range: Option<Symbol>,
        time_period: u64,
    ) -> PopulationStats {
        let key = DataKey::Outcomes(condition.clone());
        let outcomes: Vec<AnonymizedOutcome> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        let mut total_cases: u64 = 0;
        let mut total_age: u64 = 0;
        let mut male_count: u64 = 0;
        let mut female_count: u64 = 0;
        let mut other_count: u64 = 0;

        let mut treatment_map: Map<String, u64> = Map::new(&env);
        let mut outcome_map: Map<Symbol, u64> = Map::new(&env);

        let cutoff_time = if time_period > 0 {
            env.ledger().timestamp().saturating_sub(time_period)
        } else {
            0
        };

        for i in 0..outcomes.len() {
            let outcome = outcomes.get(i).unwrap();

            if outcome.timestamp < cutoff_time {
                continue;
            }

            if let Some(ref age_filter) = age_range {
                if outcome.age_group != *age_filter {
                    continue;
                }
            }

            total_cases += 1;

            let age = Self::age_group_to_midpoint(&outcome.age_group);
            total_age += age as u64;

            if outcome.gender == symbol_short!("male") {
                male_count += 1;
            } else if outcome.gender == symbol_short!("female") {
                female_count += 1;
            } else {
                other_count += 1;
            }

            let treatment_count = treatment_map.get(outcome.treatment.clone()).unwrap_or(0);
            treatment_map.set(outcome.treatment.clone(), treatment_count + 1);

            let outcome_count = outcome_map.get(outcome.result.clone()).unwrap_or(0);
            outcome_map.set(outcome.result.clone(), outcome_count + 1);
        }

        let average_age = if total_cases > 0 {
            (total_age / total_cases) as u32
        } else {
            0
        };

        let mut common_treatments: Vec<TreatmentOutcome> = Vec::new(&env);
        let treatment_keys = treatment_map.keys();
        for i in 0..treatment_keys.len() {
            let treatment = treatment_keys.get(i).unwrap();
            let count = treatment_map.get(treatment.clone()).unwrap();
            common_treatments.push_back(TreatmentOutcome {
                treatment,
                count,
                success_rate: 8500,
            });
        }

        let mut outcome_distribution: Vec<OutcomeDistribution> = Vec::new(&env);
        let outcome_keys = outcome_map.keys();
        for i in 0..outcome_keys.len() {
            let outcome = outcome_keys.get(i).unwrap();
            let count = outcome_map.get(outcome.clone()).unwrap();
            outcome_distribution.push_back(OutcomeDistribution { outcome, count });
        }

        PopulationStats {
            condition,
            total_cases,
            average_age,
            gender_distribution: GenderStats {
                male: male_count,
                female: female_count,
                other: other_count,
            },
            common_treatments,
            outcome_distribution,
        }
    }

    /// Track readmission rates for a facility
    pub fn track_readmission_rate(
        env: Env,
        facility_id: Address,
        condition: String,
        days: u32,
        reporting_period: u64,
    ) -> ReadmissionStats {
        facility_id.require_auth();

        let key = DataKey::Readmissions(facility_id.clone(), condition.clone(), reporting_period);

        let record: ReadmissionRecord =
            env.storage()
                .persistent()
                .get(&key)
                .unwrap_or(ReadmissionRecord {
                    facility_id: facility_id.clone(),
                    condition: condition.clone(),
                    admission_count: 0,
                    readmission_count: 0,
                    days,
                    reporting_period,
                });

        let readmission_rate = if record.admission_count > 0 {
            (record.readmission_count * 10000 / record.admission_count) as u32
        } else {
            0
        };

        let stats = ReadmissionStats {
            facility_id: facility_id.clone(),
            condition: condition.clone(),
            total_admissions: record.admission_count,
            readmissions: record.readmission_count,
            readmission_rate,
            days,
            reporting_period,
        };

        env.events()
            .publish((symbol_short!("track_r"), facility_id), condition);

        stats
    }

    /// Record patient satisfaction score
    pub fn record_patient_satisfaction(
        env: Env,
        visit_id: u64,
        patient_id: Address,
        satisfaction_score: u32,
        feedback_hash: Option<BytesN<32>>,
    ) {
        patient_id.require_auth();

        if satisfaction_score > 100 {
            panic!("Satisfaction score must be 0-100");
        }

        let record = SatisfactionRecord {
            visit_id,
            patient_id: patient_id.clone(),
            satisfaction_score,
            feedback_hash,
            timestamp: env.ledger().timestamp(),
        };

        let key = DataKey::Satisfaction(visit_id);
        env.storage().persistent().set(&key, &record);

        env.events()
            .publish((symbol_short!("rec_sat"), patient_id), satisfaction_score);
    }

    /// Generate compliance report for a provider
    pub fn generate_compliance_report(
        env: Env,
        provider_id: Address,
        compliance_type: Symbol,
        period: u64,
    ) -> ComplianceReport {
        let key = DataKey::ComplianceData(provider_id.clone(), compliance_type.clone(), period);

        let (compliant_cases, total_cases) = env
            .storage()
            .persistent()
            .get::<_, (u64, u64)>(&key)
            .unwrap_or((0, 0));

        let compliance_rate = if total_cases > 0 {
            (compliant_cases * 10000 / total_cases) as u32
        } else {
            0
        };

        let issues_identified: Vec<String> = Vec::new(&env);

        ComplianceReport {
            provider_id,
            compliance_type,
            period,
            compliant_cases,
            total_cases,
            compliance_rate,
            issues_identified,
        }
    }

    /// Benchmark provider performance against peers
    pub fn benchmark_performance(
        env: Env,
        provider_id: Address,
        metric: String,
        peer_group: Symbol,
    ) -> BenchmarkResult {
        let current_period = env.ledger().timestamp() / 86400;

        let provider_key =
            DataKey::QualityMetrics(provider_id.clone(), metric.clone(), current_period);
        let provider_metric: QualityMetric = env
            .storage()
            .persistent()
            .get(&provider_key)
            .unwrap_or(QualityMetric {
                provider_id: provider_id.clone(),
                metric_name: metric.clone(),
                numerator: 0,
                denominator: 1,
                reporting_period: current_period,
                calculated_rate: 0,
            });

        let benchmark_key = DataKey::BenchmarkData(peer_group.clone(), metric.clone());
        let (peer_avg, peer_median) = env
            .storage()
            .persistent()
            .get::<_, (u32, u32)>(&benchmark_key)
            .unwrap_or((8000, 8200));

        let provider_value = provider_metric.calculated_rate;

        let percentile = if provider_value >= peer_avg {
            50 + ((provider_value - peer_avg) as u64 * 50 / peer_avg.max(1) as u64) as u32
        } else {
            ((provider_value as u64 * 50) / peer_avg.max(1) as u64) as u32
        };

        BenchmarkResult {
            provider_id,
            metric,
            provider_value,
            peer_group,
            peer_average: peer_avg,
            peer_median,
            percentile: percentile.min(100),
        }
    }

    // Helper functions

    fn age_group_to_midpoint(age_group: &Symbol) -> u32 {
        if *age_group == symbol_short!("age0_18") {
            9
        } else if *age_group == symbol_short!("age19_35") {
            27
        } else if *age_group == symbol_short!("age36_50") {
            43
        } else if *age_group == symbol_short!("age51_65") {
            58
        } else {
            70
        }
    }

    fn get_avg_satisfaction(env: &Env, provider_id: Address, _start: u64, _end: u64) -> u32 {
        let key = DataKey::ProviderSatisfaction(provider_id);
        let scores: Vec<u32> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(env));

        if scores.is_empty() {
            return 75;
        }

        let mut total: u64 = 0;
        for i in 0..scores.len() {
            total += scores.get(i).unwrap() as u64;
        }

        (total / scores.len() as u64) as u32
    }

    /// Admin function to update readmission data
    pub fn update_readmission_data(
        env: Env,
        facility_id: Address,
        condition: String,
        admission_count: u64,
        readmission_count: u64,
        days: u32,
        reporting_period: u64,
    ) {
        facility_id.require_auth();

        let record = ReadmissionRecord {
            facility_id: facility_id.clone(),
            condition: condition.clone(),
            admission_count,
            readmission_count,
            days,
            reporting_period,
        };

        let key = DataKey::Readmissions(facility_id, condition, reporting_period);
        env.storage().persistent().set(&key, &record);
    }

    /// Admin function to update compliance data
    pub fn update_compliance_data(
        env: Env,
        provider_id: Address,
        compliance_type: Symbol,
        period: u64,
        compliant_cases: u64,
        total_cases: u64,
    ) {
        provider_id.require_auth();

        let key = DataKey::ComplianceData(provider_id, compliance_type, period);
        env.storage()
            .persistent()
            .set(&key, &(compliant_cases, total_cases));
    }

    /// Admin function to update benchmark data
    pub fn update_benchmark_data(
        env: Env,
        peer_group: Symbol,
        metric: String,
        peer_average: u32,
        peer_median: u32,
    ) {
        let key = DataKey::BenchmarkData(peer_group, metric);
        env.storage()
            .persistent()
            .set(&key, &(peer_average, peer_median));
    }

    /// Link satisfaction to provider
    pub fn link_satisfaction_to_provider(env: Env, provider_id: Address, visit_id: u64) {
        provider_id.require_auth();

        let visit_key = DataKey::Satisfaction(visit_id);
        let record: SatisfactionRecord = env
            .storage()
            .persistent()
            .get(&visit_key)
            .expect("Satisfaction record not found");

        let provider_key = DataKey::ProviderSatisfaction(provider_id);
        let mut scores: Vec<u32> = env
            .storage()
            .persistent()
            .get(&provider_key)
            .unwrap_or(Vec::new(&env));

        scores.push_back(record.satisfaction_score);
        env.storage().persistent().set(&provider_key, &scores);
    }

    // --- Referral workflow ---

    /// Create a new referral from referring provider to receiving provider.
    pub fn create_referral(
        env: Env,
        referring_provider: Address,
        patient_id: Address,
        referred_to: Address,
        specialty: Symbol,
        reason: String,
        priority: Symbol,
        clinical_summary_hash: BytesN<32>,
        requested_services: Vec<Symbol>,
    ) -> Result<u64, ReferralError> {
        referring_provider.require_auth();

        let next_id_key = DataKey::NextReferralId;
        let next_id: u64 = env
            .storage()
            .persistent()
            .get(&next_id_key)
            .unwrap_or(0);
        let referral_id = next_id + 1;
        env.storage().persistent().set(&next_id_key, &referral_id);

        let now = env.ledger().timestamp();
        let referral = Referral {
            referral_id,
            referring_provider: referring_provider.clone(),
            receiving_provider: referred_to.clone(),
            patient_id,
            specialty,
            reason,
            priority,
            status: ReferralStatus::Pending,
            created_at: now,
            accepted_at: None,
            completed_at: None,
        };

        let key = DataKey::Referral(referral_id);
        env.storage().persistent().set(&key, &referral);

        // Store initial clinical summary as first shared summary
        let summary_key = DataKey::CareSummary(referral_id, symbol_short!("clinical"));
        env.storage().persistent().set(
            &summary_key,
            &CareSummaryRecord {
                from_provider: referring_provider.clone(),
                summary_type: symbol_short!("clinical"),
                summary_hash: clinical_summary_hash,
            },
        );

        env.events().publish(
            (symbol_short!("ref_creat"), referring_provider),
            (referral_id, referred_to, requested_services),
        );

        Ok(referral_id)
    }

    /// Accept a referral (receiving provider only).
    pub fn accept_referral(
        env: Env,
        referral_id: u64,
        receiving_provider: Address,
        estimated_appointment_date: Option<u64>,
    ) -> Result<(), ReferralError> {
        receiving_provider.require_auth();

        let key = DataKey::Referral(referral_id);
        let mut referral: Referral = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)?;

        if referral.receiving_provider != receiving_provider {
            return Err(ReferralError::Unauthorized);
        }
        if referral.status != ReferralStatus::Pending {
            return Err(ReferralError::InvalidState);
        }

        let now = env.ledger().timestamp();
        referral.status = ReferralStatus::Accepted;
        referral.accepted_at = Some(now);
        env.storage().persistent().set(&key, &referral);

        if let Some(date) = estimated_appointment_date {
            env.storage()
                .persistent()
                .set(&DataKey::EstimatedAppointment(referral_id), &date);
        }

        env.events().publish(
            (symbol_short!("ref_acc"), receiving_provider),
            (referral_id, estimated_appointment_date),
        );

        Ok(())
    }

    /// Decline a referral (receiving provider only).
    pub fn decline_referral(
        env: Env,
        referral_id: u64,
        receiving_provider: Address,
        decline_reason: String,
        suggest_alternative: Option<Address>,
    ) -> Result<(), ReferralError> {
        receiving_provider.require_auth();

        let key = DataKey::Referral(referral_id);
        let mut referral: Referral = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)?;

        if referral.receiving_provider != receiving_provider {
            return Err(ReferralError::Unauthorized);
        }
        if referral.status != ReferralStatus::Pending {
            return Err(ReferralError::InvalidState);
        }

        referral.status = ReferralStatus::Declined;
        env.storage().persistent().set(&key, &referral);
        env.storage().persistent().set(
            &DataKey::ReferralDecline(referral_id),
            &ReferralDeclineInfo {
                decline_reason,
                suggest_alternative,
            },
        );

        env.events().publish(
            (symbol_short!("ref_decl"), receiving_provider),
            referral_id,
        );

        Ok(())
    }

    /// Update referral status (referring or receiving provider).
    pub fn update_referral_status(
        env: Env,
        referral_id: u64,
        provider_id: Address,
        status: Symbol,
        status_note: Option<String>,
    ) -> Result<(), ReferralError> {
        provider_id.require_auth();

        let key = DataKey::Referral(referral_id);
        let mut referral: Referral = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)?;

        let is_participant = referral.referring_provider == provider_id
            || referral.receiving_provider == provider_id;
        if !is_participant {
            return Err(ReferralError::Unauthorized);
        }
        if referral.status == ReferralStatus::Declined || referral.status == ReferralStatus::Cancelled
        {
            return Err(ReferralError::InvalidState);
        }

        let new_status = symbol_to_referral_status(&status);
        referral.status = new_status.clone();
        env.storage().persistent().set(&key, &referral);

        if let Some(note) = status_note {
            env.storage()
                .persistent()
                .set(&DataKey::ReferralStatusNote(referral_id), &note);
        }

        env.events().publish(
            (symbol_short!("ref_st_up"), provider_id),
            (referral_id, status),
        );

        Ok(())
    }

    /// Complete a referral with consultation summary (receiving provider only).
    pub fn complete_referral(
        env: Env,
        referral_id: u64,
        receiving_provider: Address,
        consultation_summary_hash: BytesN<32>,
        recommendations: String,
        followup_required: bool,
    ) -> Result<(), ReferralError> {
        receiving_provider.require_auth();

        let key = DataKey::Referral(referral_id);
        let mut referral: Referral = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)?;

        if referral.receiving_provider != receiving_provider {
            return Err(ReferralError::Unauthorized);
        }
        if referral.status == ReferralStatus::Declined
            || referral.status == ReferralStatus::Cancelled
            || referral.status == ReferralStatus::Completed
        {
            return Err(ReferralError::InvalidState);
        }

        let now = env.ledger().timestamp();
        referral.status = ReferralStatus::Completed;
        referral.completed_at = Some(now);
        env.storage().persistent().set(&key, &referral);

        env.storage().persistent().set(
            &DataKey::ReferralCompletion(referral_id),
            &ReferralCompletionInfo {
                consultation_summary_hash,
                recommendations,
                followup_required,
            },
        );

        env.events().publish(
            (symbol_short!("ref_done"), receiving_provider),
            (referral_id, followup_required),
        );

        Ok(())
    }

    /// Share a care summary for a referral (referring or receiving provider).
    pub fn share_care_summary(
        env: Env,
        referral_id: u64,
        from_provider: Address,
        summary_type: Symbol,
        summary_hash: BytesN<32>,
    ) -> Result<(), ReferralError> {
        from_provider.require_auth();

        let key = DataKey::Referral(referral_id);
        let referral: Referral = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)?;

        let is_participant = referral.referring_provider == from_provider
            || referral.receiving_provider == from_provider;
        if !is_participant {
            return Err(ReferralError::Unauthorized);
        }
        if referral.status == ReferralStatus::Declined || referral.status == ReferralStatus::Cancelled
        {
            return Err(ReferralError::InvalidState);
        }

        let summary_key = DataKey::CareSummary(referral_id, summary_type.clone());
        env.storage().persistent().set(
            &summary_key,
            &CareSummaryRecord {
                from_provider: from_provider.clone(),
                summary_type: summary_type.clone(),
                summary_hash,
            },
        );

        env.events().publish(
            (symbol_short!("care_shr"), from_provider),
            (referral_id, summary_type),
        );

        Ok(())
    }

    /// Request care summary / information from the other provider.
    pub fn request_care_summary(
        env: Env,
        referral_id: u64,
        requesting_provider: Address,
        information_needed: Vec<Symbol>,
    ) -> Result<(), ReferralError> {
        requesting_provider.require_auth();

        let key = DataKey::Referral(referral_id);
        let referral: Referral = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)?;

        let is_participant = referral.referring_provider == requesting_provider
            || referral.receiving_provider == requesting_provider;
        if !is_participant {
            return Err(ReferralError::Unauthorized);
        }
        if referral.status == ReferralStatus::Declined || referral.status == ReferralStatus::Cancelled
        {
            return Err(ReferralError::InvalidState);
        }

        let request_key = DataKey::CareSummaryRequest(referral_id);
        env.storage().persistent().set(
            &request_key,
            &CareSummaryRequestRecord {
                requesting_provider: requesting_provider.clone(),
                information_needed: information_needed.clone(),
            },
        );

        env.events().publish(
            (symbol_short!("care_req"), requesting_provider),
            (referral_id, information_needed),
        );

        Ok(())
    }

    /// Get referral by id (for clients and status tracking).
    pub fn get_referral(env: Env, referral_id: u64) -> Result<Referral, ReferralError> {
        let key = DataKey::Referral(referral_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(ReferralError::NotFound)
    }
}

fn symbol_to_referral_status(s: &Symbol) -> ReferralStatus {
    if *s == symbol_short!("pending") {
        ReferralStatus::Pending
    } else if *s == symbol_short!("accepted") {
        ReferralStatus::Accepted
    } else if *s == symbol_short!("declined") {
        ReferralStatus::Declined
    } else if *s == symbol_short!("sched") {
        ReferralStatus::Scheduled
    } else if *s == symbol_short!("in_prog") {
        ReferralStatus::InProgress
    } else if *s == symbol_short!("completed") {
        ReferralStatus::Completed
    } else if *s == symbol_short!("cancelled") {
        ReferralStatus::Cancelled
    } else {
        ReferralStatus::Pending
    }
}

#[cfg(test)]
mod test;
