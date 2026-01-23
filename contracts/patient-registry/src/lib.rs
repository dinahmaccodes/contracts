#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Bytes, Env, String,
};

/// --------------------
/// Patient Structures
/// --------------------
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatientData {
    pub name: String,
    pub dob: u64,
    pub metadata: String, // IPFS / encrypted medical refs
}

/// --------------------
/// Doctor Structures
/// --------------------
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoctorData {
    pub name: String,
    pub specialization: String,
    pub certificate_hash: Bytes,
    pub verified: bool,
}

/// --------------------
/// Storage Keys
/// --------------------
#[contracttype]
pub enum DataKey {
    Patient(Address),
    Doctor(Address),
    Institution(Address),
}

#[contract]
pub struct MedicalRegistry;

#[contractimpl]
impl MedicalRegistry {
    // =====================================================
    //                    PATIENT LOGIC
    // =====================================================

    pub fn register_patient(
        env: Env,
        wallet: Address,
        name: String,
        dob: u64,
        metadata: String,
    ) {
        wallet.require_auth();

        let key = DataKey::Patient(wallet.clone());
        if env.storage().persistent().has(&key) {
            panic!("Patient already registered");
        }

        let patient = PatientData { name, dob, metadata };
        env.storage().persistent().set(&key, &patient);

        env.events()
            .publish((symbol_short!("reg_pat"), wallet), symbol_short!("success"));
    }

    pub fn update_patient(env: Env, wallet: Address, metadata: String) {
        wallet.require_auth();

        let key = DataKey::Patient(wallet.clone());
        let mut patient: PatientData = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Patient not found");

        patient.metadata = metadata;
        env.storage().persistent().set(&key, &patient);

        env.events()
            .publish((symbol_short!("upd_pat"), wallet), symbol_short!("success"));
    }

    pub fn get_patient(env: Env, wallet: Address) -> PatientData {
        let key = DataKey::Patient(wallet);
        env.storage()
            .persistent()
            .get(&key)
            .expect("Patient not found")
    }

    // =====================================================
    //                    DOCTOR LOGIC
    // =====================================================

    pub fn register_doctor(
        env: Env,
        wallet: Address,
        name: String,
        specialization: String,
        certificate_hash: Bytes,
    ) {
        wallet.require_auth();

        let key = DataKey::Doctor(wallet.clone());
        if env.storage().persistent().has(&key) {
            panic!("Doctor already registered");
        }

        let doctor = DoctorData {
            name,
            specialization,
            certificate_hash,
            verified: false,
        };

        env.storage().persistent().set(&key, &doctor);

        env.events()
            .publish((symbol_short!("reg_doc"), wallet), symbol_short!("success"));
    }

    pub fn verify_doctor(
        env: Env,
        wallet: Address,
        institution_wallet: Address,
    ) {
        institution_wallet.require_auth();

        let inst_key = DataKey::Institution(institution_wallet);
        if !env.storage().persistent().has(&inst_key) {
            panic!("Unauthorized institution");
        }

        let doc_key = DataKey::Doctor(wallet.clone());
        let mut doctor: DoctorData = env
            .storage()
            .persistent()
            .get(&doc_key)
            .expect("Doctor not found");

        doctor.verified = true;
        env.storage().persistent().set(&doc_key, &doctor);

        env.events()
            .publish((symbol_short!("ver_doc"), wallet), symbol_short!("verified"));
    }

    pub fn get_doctor(env: Env, wallet: Address) -> DoctorData {
        let key = DataKey::Doctor(wallet);
        env.storage()
            .persistent()
            .get(&key)
            .expect("Doctor not found")
    }

    // =====================================================
    //              INSTITUTION MANAGEMENT
    // =====================================================

    pub fn register_institution(env: Env, institution_wallet: Address) {
        institution_wallet.require_auth();
        let key = DataKey::Institution(institution_wallet);
        env.storage().persistent().set(&key, &true);
    }
}
#[cfg(test)]
mod test;
