# Charcot EMR SaaS Platform

A comprehensive, scalable, multi-tenant Electronic Medical Record system built with Rust and React.

## Overview

Charcot EMR SaaS is a country-agnostic, FHIR-compliant Electronic Medical Record system designed for global deployment. Built with performance, security, and scalability in mind, it leverages Rust for the backend and React for the frontend to provide a robust healthcare data management solution.

## Key Features

- Multi-tenant architecture supporting healthcare organizations worldwide
- FHIR-compliant data model for interoperability
- Real-time updates via WebSockets
- Advanced encryption for data security and privacy
- Comprehensive audit logging
- Internationalization support
- Responsive design for desktop and mobile access
- Offline capability for unstable network environments

## Project Structure

### Backend (Rust/Axum)

#### Core Files

- `src/main.rs` - Application entry point, server setup, and route configuration
- `src/lib.rs` - Core library exports and shared functionality
- `src/config/mod.rs` - Configuration module exports
- `src/config/settings.rs` - Application settings and environment variable handling

#### API Endpoints

- `src/api/mod.rs` - API module exports and shared handlers
- `src/api/patients/mod.rs` - Patient management endpoints (CRUD operations)
- `src/api/clinical/mod.rs` - Clinical data endpoints (observations, medications)
- `src/api/admin/mod.rs` - Administrative endpoints for system management
- `src/api/billing/mod.rs` - Billing and payment processing endpoints
- `src/api/auth/mod.rs` - Authentication and authorization endpoints
- `src/api/search/mod.rs` - Advanced search functionality for clinical data
- `src/api/analytics/mod.rs` - Analytics and reporting endpoints

#### Database Layer

- `src/db/mod.rs` - Database connection management and shared utilities
- `src/db/migrations/` - Database schema migration files
- `src/db/models/mod.rs` - Data model module exports
- `src/db/models/patient.rs` - Patient data structures
- `src/db/models/observation.rs` - Clinical observation data structures
- `src/db/models/medication.rs` - Medication and prescription data structures
- `src/db/models/organization.rs` - Healthcare organization data structures
- `src/db/models/user.rs` - User account data structures
- `src/db/models/tenant.rs` - Multi-tenancy data structures
- `src/db/repositories/mod.rs` - Repository pattern implementations for data access
- `src/db/repositories/patient_repository.rs` - Patient data access layer
- `src/db/repositories/clinical_repository.rs` - Clinical data access layer

#### Middleware

- `src/middleware/mod.rs` - Middleware module exports
- `src/middleware/auth.rs` - Authentication and authorization middleware
- `src/middleware/tenant.rs` - Multi-tenancy middleware for tenant isolation
- `src/middleware/logging.rs` - Request logging and audit trail middleware
- `src/middleware/cache.rs` - Response caching middleware

#### Services

- `src/services/mod.rs` - Service module exports
- `src/services/notifications/mod.rs` - Notification service for alerts and reminders
- `src/services/reporting/mod.rs` - Reporting and analytics service
- `src/services/integrations/mod.rs` - Third-party integration services
- `src/services/scheduler/mod.rs` - Scheduled task management service

#### Real-time and Background Processing

- `src/realtime/mod.rs` - WebSocket handlers for real-time updates
- `src/jobs/mod.rs` - Background job processing system

#### Utilities

- `src/utils/mod.rs` - Utility module exports
- `src/utils/error.rs` - Error handling utilities
- `src/utils/validation.rs` - Data validation utilities
- `src/utils/security.rs` - Encryption and security utilities
- `src/utils/i18n.rs` - Internationalization utilities

### Frontend (React/TypeScript)

#### Core Files

- `frontend/package.json` - Node.js dependencies and scripts
- `frontend/tsconfig.json` - TypeScript configuration
- `frontend/index.html` - HTML entry point
- `frontend/.env` - Environment variable configuration
- `frontend/src/main.tsx` - Application entry point
- `frontend/src/App.tsx` - Root component
- `frontend/src/index.css` - Global styles

#### Components

- `frontend/src/components/common/Layout.tsx` - Main application layout
- `frontend/src/components/common/Sidebar.tsx` - Navigation sidebar
- `frontend/src/components/common/NavBar.tsx` - Top navigation bar
- `frontend/src/components/common/Button.tsx` - Reusable button component
- `frontend/src/components/patients/PatientList.tsx` - Patient list component
- `frontend/src/components/patients/PatientDetail.tsx` - Patient detail view
- `frontend/src/components/clinical/VitalSigns.tsx` - Vital signs display component
- `frontend/src/components/clinical/MedicationList.tsx` - Medication list component

#### Pages

- `frontend/src/pages/Dashboard.tsx` - Main dashboard page
- `frontend/src/pages/PatientListPage.tsx` - Patient directory page
- `frontend/src/pages/PatientDetailPage.tsx` - Detailed patient view page
- `frontend/src/pages/Login.tsx` - Authentication page
- `frontend/src/pages/Settings.tsx` - User and organization settings page

#### State Management

- `frontend/src/contexts/AuthContext.tsx` - Authentication context provider
- `frontend/src/contexts/TenantContext.tsx` - Tenant context provider
- `frontend/src/hooks/useApi.ts` - API interaction hook
- `frontend/src/hooks/useWebSocket.ts` - WebSocket connection hook
- `frontend/src/hooks/useTranslation.ts` - Internationalization hook

#### API Clients

- `frontend/src/api/client.ts` - Base API client with authentication
- `frontend/src/api/patients.ts` - Patient API endpoints
- `frontend/src/api/clinical.ts` - Clinical data API endpoints
- `frontend/src/api/auth.ts` - Authentication API endpoints

#### Internationalization

- `frontend/src/i18n/index.ts` - Internationalization setup
- `frontend/src/i18n/translations/en.json` - English translations
- `frontend/src/i18n/translations/es.json` - Spanish translations
- `frontend/src/i18n/translations/fr.json` - French translations

### Infrastructure

#### Docker

- `docker-compose.yml` - Local development environment setup
- `Dockerfile` - Main application container definition
- `docker/api.Dockerfile` - Backend service container definition
- `docker/frontend.Dockerfile` - Frontend application container definition
- `docker/nginx.conf` - Nginx configuration for serving the frontend

#### Kubernetes

- `kubernetes/base/deployment.yaml` - Base Kubernetes deployment configuration
- `kubernetes/base/service.yaml` - Service definitions for internal routing
- `kubernetes/base/ingress.yaml` - Ingress configuration for external access
- `kubernetes/base/kustomization.yaml` - Kustomize base configuration
- `kubernetes/overlays/dev/kustomization.yaml` - Development environment configuration
- `kubernetes/overlays/staging/kustomization.yaml` - Staging environment configuration
- `kubernetes/overlays/prod/kustomization.yaml` - Production environment configuration

#### Terraform

- `terraform/main.tf` - Main Terraform configuration
- `terraform/variables.tf` - Input variables for infrastructure
- `terraform/outputs.tf` - Output variables for infrastructure
- `terraform/modules/vpc/main.tf` - VPC network configuration
- `terraform/modules/eks/main.tf` - Kubernetes cluster configuration
- `terraform/modules/rds/main.tf` - Database configuration
- `terraform/modules/elasticache/main.tf` - Redis cache configuration
- `terraform/modules/s3/main.tf` - S3 storage configuration

### CI/CD

- `.github/workflows/ci.yml` - Continuous integration workflow
- `.github/workflows/cd.yml` - Continuous deployment workflow

### Documentation

- `docs/README.md` - Documentation overview
- `docs/api/openapi.yaml` - OpenAPI specification for the API
- `docs/architecture/overview.md` - Architecture documentation
- `docs/deployment/kubernetes.md` - Kubernetes deployment guide
- `docs/deployment/aws.md` - AWS deployment guide

### Scripts

- `scripts/setup.sh` - Initial project setup script
- `scripts/dev.sh` - Local development environment script
- `scripts/migrate.sh` - Database migration script

### Database Migrations

- `migrations/00001_initial_schema.sql` - Initial database schema
- `migrations/00002_create_patients.sql` - Patient table migrations
- `migrations/00003_create_observations.sql` - Clinical observation migrations

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v16+)
- Docker and Docker Compose
- Kubernetes CLI (optional for local development)

### Development Setup

1. Clone the repository:
   ```
   git clone https://github.com/your-org/charcot-emr-saas.git
   cd charcot-emr-saas
   ```

2. Run the setup script:
   ```
   ./scripts/setup.sh
   ```

3. Start the development environment:
   ```
   ./scripts/dev.sh
   ```

4. Access the application:
   - Backend API: http://localhost:3000
   - Frontend: http://localhost:8080

## Deployment

See the deployment guides in the `docs/deployment` directory for detailed instructions on deploying to Kubernetes and AWS.

## Contributing

Please see CONTRIBUTING.md for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
