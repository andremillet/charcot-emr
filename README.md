# Charcot EMR System

A prototype implementation of the Electronic Medical Record (EMR) system described in the Charcot medical programming language specification.

## Overview

This project implements a minimal EMR system based on the specifications in the Charcot language design documents. It provides:

- FHIR-aligned data structures for medical information
- Secure .med file format with AES-256 encryption
- Version history for patient records
- CLI and GUI interfaces
- Support for basic clinical workflows

## Features

- **Patient Management**: Create and load patient records
- **Vital Signs**: Record blood pressure measurements
- **Medication Management**: Prescribe medications with dosing instructions
- **Device Integration**: Connect medical devices (mock implementation)
- **Security**: HIPAA-compliant encryption with patient-controlled keys
- **Versioning**: Track changes to patient records with commit messages
- **Interoperability**: FHIR-compatible data structures

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Installation

1. Clone the repository
2. Build the project:

```bash
cargo build --release
```

### Running the CLI

```bash
# Create a new patient
cargo run --bin emr_cli -- create-patient 123 John Doe male 1980-01-01 mypassword

# Add blood pressure vital signs
cargo run --bin emr_cli -- add-vital 123 bp 120 80 mypassword

# Prescribe medication
cargo run --bin emr_cli -- prescribe 123 "lisinopril" 10 "daily" mypassword

# Connect a device
cargo run --bin emr_cli -- connect-device 123 "glucometer" mypassword

# Load a patient record
cargo run --bin emr_cli -- load patient_123.med mypassword
```

### Running the GUI

```bash
cargo run --bin emr_gui
```

## File Format

The `.med` file format is a JSON structure with the following elements:

- `iv`: Base64-encoded initialization vector for AES-256 encryption
- `data`: Base64-encoded encrypted FHIR Bundle containing patient data
- `hash`: SHA-256 hash of the unencrypted data for integrity verification
- `created`: Timestamp of initial creation
- `modified`: Timestamp of last modification

## Security

- Patient data is encrypted using AES-256-GCM
- Encryption keys are derived from user-provided passwords
- Audit logging of all operations
- Data integrity verification via SHA-256 hashing

## Next Steps

This prototype demonstrates the core functionality described in the Charcot language specification. Future development will:

1. Expand the EMR functionality with more vital signs and workflows
2. Implement a custom domain-specific language (DSL) for medical programming
3. Create a compiler that translates Charcot code to Rust
4. Enhance interoperability with full FHIR server integration

## License

This project is licensed under the MIT License - see the LICENSE file for details.
