// Diesel models for core tables
use diesel::prelude::*;
use chrono::NaiveDate;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, Debug)]
#[table_name = "patients"]
pub struct Patient {
    pub id: i32,
    pub fhir_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Patient)]
#[table_name = "encounters"]
pub struct Encounter {
    pub id: i32,
    pub fhir_id: Option<String>,
    pub patient_id: i32,
    pub encounter_type: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Encounter)]
#[table_name = "vital_signs"]
pub struct VitalSign {
    pub id: i32,
    pub encounter_id: i32,
    pub recorded_at: NaiveDateTime,
    pub vital_type: String,
    pub value: String,
    pub unit: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Encounter)]
#[table_name = "diagnoses"]
pub struct Diagnosis {
    pub id: i32,
    pub encounter_id: i32,
    pub icd_code: String,
    pub description: Option<String>,
    pub diagnosed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Insertable, AsChangeset, Debug)]
#[belongs_to(Encounter)]
#[table_name = "treatments"]
pub struct Treatment {
    pub id: i32,
    pub encounter_id: i32,
    pub procedure_code: String,
    pub description: Option<String>,
    pub performed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}