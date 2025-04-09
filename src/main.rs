// src/main.rs
// Charcot EMR: Command-line interface for the EMR system

use std::path::Path;
use anyhow::{Result, anyhowfn load_patient(emr: &mut EMR, args: &ArgMatches) -> Result<()> {
    let filename = args.get_one::<String>("filename").unwrap();
    let key = args.get_one::<String>("key").unwrap();
    
    let patient_id = emr.load_patient(filename, key)?;
    println!("Loaded patient {} from {}", patient_id, filename);
    
    // Display basic info
    if let Some(bundle) = emr.bundles.get(&patient_id) {
        if let Some(BundleEntry { resource: Resource::Patient(patient), .. }) = bundle.entry.first() {
            if let Some(name) = patient.name.first() {
                let given = name.given.join(" ");
                let family = name.family.clone().unwrap_or_default();
                println!("Name: {} {}", given, family);
                println!("Gender: {}", patient.gender);
                println!("Birth date: {}", patient.birth_date);
            }
        }
        
        println!("Version history:");
        for (i, version) in bundle.version_history.iter().enumerate() {
            println!("  {}: {} - {}", i+1, version.timestamp, version.message);
        }
    }
    
    Ok(())
}

fn print_usage() {
    println!("Charcot EMR System");
    println!("Usage:");
    println!("  emr_cli create-patient <id> <given_name> <family_name> <gender> <birth_date> <key>");
    println!("  emr_cli add-vital <patient_id> bp <systolic> <diastolic> <key>");
    println!("  emr_cli prescribe <patient_id> <medication> <dose_mg> <frequency> <key>");
    println!("  emr_cli connect-device <patient_id> <device_type> <key>");
    println!("  emr_cli load <filename> <key>");
};
use clap::{Command, Arg, ArgMatches, value_parser};
use charcot_emr::*;

fn main() -> Result<()> {
    // Set up command-line interface
    let matches = Command::new("Charcot EMR")
        .version("0.1.0")
        .author("Charcot Team")
        .about("A medical EMR system for the Charcot language")
        .subcommand(
            Command::new("create-patient")
                .about("Create a new patient record")
                .arg(Arg::new("id").required(true).help("Patient ID"))
                .arg(Arg::new("given_name").required(true).help("Given name"))
                .arg(Arg::new("family_name").required(true).help("Family name"))
                .arg(Arg::new("gender").required(true).help("Gender (male/female/other)"))
                .arg(Arg::new("birth_date").required(true).help("Birth date (YYYY-MM-DD)"))
                .arg(Arg::new("key").required(true).help("Encryption key for the patient file"))
        )
        .subcommand(
            Command::new("add-vital")
                .about("Add vital signs to a patient record")
                .arg(Arg::new("patient_id").required(true).help("Patient ID"))
                .arg(Arg::new("type").required(true).help("Type of vital (bp for blood pressure)"))
                .arg(Arg::new("value1").required(true).value_parser(value_parser!(i32)).help("First value (systolic for bp)"))
                .arg(Arg::new("value2").required(true).value_parser(value_parser!(i32)).help("Second value (diastolic for bp)"))
                .arg(Arg::new("key").required(true).help("Encryption key for the patient file"))
        )
        .subcommand(
            Command::new("prescribe")
                .about("Prescribe medication for a patient")
                .arg(Arg::new("patient_id").required(true).help("Patient ID"))
                .arg(Arg::new("medication").required(true).help("Medication name"))
                .arg(Arg::new("dose_mg").required(true).value_parser(value_parser!(f64)).help("Dose in mg"))
                .arg(Arg::new("frequency").required(true).help("Frequency (e.g., daily, twice daily)"))
                .arg(Arg::new("key").required(true).help("Encryption key for the patient file"))
        )
        .subcommand(
            Command::new("connect-device")
                .about("Connect a medical device to a patient")
                .arg(Arg::new("patient_id").required(true).help("Patient ID"))
                .arg(Arg::new("device_type").required(true).help("Type of device (e.g., glucometer)"))
                .arg(Arg::new("key").required(true).help("Encryption key for the patient file"))
        )
        .subcommand(
            Command::new("load")
                .about("Load a patient record from a .med file")
                .arg(Arg::new("filename").required(true).help("Path to the .med file"))
                .arg(Arg::new("key").required(true).help("Encryption key for the patient file"))
        )
        .get_matches();

    let mut emr = EMR::new()?;
    
    match matches.subcommand() {
        Some(("create-patient", args)) => create_patient(&mut emr, args),
        Some(("add-vital", args)) => add_vital(&mut emr, args),
        Some(("prescribe", args)) => prescribe_medication(&mut emr, args),
        Some(("connect-device", args)) => connect_device(&mut emr, args),
        Some(("load", args)) => load_patient(&mut emr, args),
        _ => {
            print_usage();
            Ok(())
        }
    }
}

fn create_patient(emr: &mut EMR, args: &ArgMatches) -> Result<()> {
    let id = args.get_one::<String>("id").unwrap();
    let given_name = args.get_one::<String>("given_name").unwrap();
    let family_name = args.get_one::<String>("family_name").unwrap();
    let gender = args.get_one::<String>("gender").unwrap();
    let birth_date = args.get_one::<String>("birth_date").unwrap();
    let key = args.get_one::<String>("key").unwrap();
    
    emr.create_patient(id, given_name, family_name, gender, birth_date)?;
    emr.commit_changes(id, "Initial patient creation")?;
    emr.save_patient(id, key)?;
    
    println!("Patient created and saved to patient_{}.med", id);
    Ok(())
}

fn add_vital(emr: &mut EMR, args: &ArgMatches) -> Result<()> {
    let patient_id = args.get_one::<String>("patient_id").unwrap();
    let vital_type = args.get_one::<String>("type").unwrap();
    let value1 = args.get_one::<i32>("value1").unwrap();
    let value2 = args.get_one::<i32>("value2").unwrap();
    let key = args.get_one::<String>("key").unwrap();
    
    if vital_type == "bp" {
        // Load patient first
        let filename = format!("patient_{}.med", patient_id);
        if Path::new(&filename).exists() {
            emr.load_patient(&filename, key)?;
        } else {
            return Err(anyhow!("Patient file not found: {}", filename));
        }
        
        // Add blood pressure
        emr.add_blood_pressure(patient_id, *value1, *value2)?;
        emr.commit_changes(patient_id, &format!("Added BP: {}/{}", value1, value2))?;
        emr.save_patient(patient_id, key)?;
        
        println!("Added blood pressure {}/{} to patient {}", value1, value2, patient_id);
    } else {
        println!("Only blood pressure (bp) vital is supported at this time");
    }
    
    Ok(())
}

fn prescribe_medication(emr: &mut EMR, args: &ArgMatches) -> Result<()> {
    let patient_id = args.get_one::<String>("patient_id").unwrap();
    let medication = args.get_one::<String>("medication").unwrap();
    let dose_mg = args.get_one::<f64>("dose_mg").unwrap();
    let frequency = args.get_one::<String>("frequency").unwrap();
    let key = args.get_one::<String>("key").unwrap();
    
    // Load patient first
    let filename = format!("patient_{}.med", patient_id);
    if Path::new(&filename).exists() {
        emr.load_patient(&filename, key)?;
    } else {
        return Err(anyhow!("Patient file not found: {}", filename));
    }
    
    // Prescribe medication
    emr.prescribe_medication(patient_id, medication, *dose_mg, frequency)?;
    emr.commit_changes(patient_id, &format!("Prescribed {} {}mg {}", medication, dose_mg, frequency))?;
    emr.save_patient(patient_id, key)?;
    
    println!("Prescribed {} {}mg {} to patient {}", medication, dose_mg, frequency, patient_id);
    Ok(())
}

fn connect_device(emr: &mut EMR, args: &ArgMatches) -> Result<()> {
    let patient_id = args.get_one::<String>("patient_id").unwrap();
    let device_type = args.get_one::<String>("device_type").unwrap();
    let key = args.get_one::<String>("key").unwrap();
    
    // Load patient first
    let filename = format!("patient_{}.med", patient_id);
    if Path::new(&filename).exists() {
        emr.load_patient(&filename, key)?;
    } else {
        return Err(anyhow!("Patient file not found: {}", filename));
    }
    
    // Connect device
    emr.connect_device(patient_id, device_type)?;
    emr.commit_changes(patient_id, &format!("Connected device: {}", device_type))?;
    emr.save_patient(patient_id, key)?;
    
    println!("Connected device {} to patient {}", device_type, patient_id);
    Ok(())