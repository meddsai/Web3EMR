-- Create patients table
CREATE TABLE patients (
    id SERIAL PRIMARY KEY,
    fhir_id VARCHAR(64) UNIQUE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    dob DATE NOT NULL,
    gender VARCHAR(16),
    address TEXT,
    phone VARCHAR(32),
    email VARCHAR(128),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_patients_last_name ON patients(last_name);

-- Create encounters table
CREATE TABLE encounters (
    id SERIAL PRIMARY KEY,
    fhir_id VARCHAR(64) UNIQUE,
    patient_id INTEGER NOT NULL REFERENCES patients(id) ON DELETE CASCADE,
    encounter_type VARCHAR(64),
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_encounters_patient_id ON encounters(patient_id);

-- Create vital_signs table
CREATE TABLE vital_signs (
    id SERIAL PRIMARY KEY,
    encounter_id INTEGER NOT NULL REFERENCES encounters(id) ON DELETE CASCADE,
    recorded_at TIMESTAMP NOT NULL,
    vital_type VARCHAR(32) NOT NULL,
    value VARCHAR(64) NOT NULL,
    unit VARCHAR(16),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_vitals_encounter_id ON vital_signs(encounter_id);

-- Create diagnoses table
CREATE TABLE diagnoses (
    id SERIAL PRIMARY KEY,
    encounter_id INTEGER NOT NULL REFERENCES encounters(id) ON DELETE CASCADE,
    icd_code VARCHAR(16) NOT NULL,
    description TEXT,
    diagnosed_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_diagnoses_encounter_id ON diagnoses(encounter_id);

-- Create treatments table
CREATE TABLE treatments (
    id SERIAL PRIMARY KEY,
    encounter_id INTEGER NOT NULL REFERENCES encounters(id) ON DELETE CASCADE,
    procedure_code VARCHAR(32) NOT NULL,
    description TEXT,
    performed_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_treatments_encounter_id ON treatments(encounter_id);
