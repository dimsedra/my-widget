---
title: Implementation Plan - Phase 2 Cloud Backend & Sync Pipeline
type: plan
parent_prd: docs/prd/20261013_desktop_widget_platform.md
parent_srs: docs/srs/20261013_desktop_widget_platform.md
status: in_progress
total_tasks: 14
completed_tasks: 6
last_updated: 2026-10-15
---

# Implementation Plan: Phase 2 - Cloud Backend & Sync Pipeline

> **Progress Tracker:** `[█████░░░░░░░░░░░]` **6 / 14 Tasks Completed (43%)**

Dokumen rencana kerja ini mendefinisikan langkah implementasi bertahap *(commit-driven TDD roadmap)* untuk membangun pipa backend cloud dan sinkronisasi data reaktif dua arah *(real-time reactivity)* berbasis Convex BaaS sesuai spesifikasi kebutuhan pada `REQ-SYNC-001`, `NFR-REL-001`, serta keputusan arsitektur `ADR-002` dan `ADR-004`.

Setiap skenario perilaku disusun dengan pendekatan dual-task TDD (Task a: Test [RED] dan Task b: Feature [GREEN]) guna memastikan verifikasi perilaku publik, isolasi data instan, dan kebersihan log pengujian.

---

## Phase 1: Database Schema & In-Memory Test Setup

Fokus fase ini adalah menyiapkan infrastruktur pengujian otomatis backend di memori via `convex-test` dan mendefinisikan skema tabel polimorfik generik `widget_data`.

### Scenario 1.1: Backend Test Runner Setup (Normal Path / Infra)
- [x] **Task 1.1a: [TEST] Add failing test asserting convex-test runner configuration** (`a46be22`)
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[NEW] convex/test.setup.test.ts`
  - **Commit**: `test(convex): verify convex-test test runner setup`
  - **Verification**: `pnpm test convex/test.setup.test.ts` (FAILED as expected)

- [x] **Task 1.1b: [FEAT] Install convex-test and configure Vitest inclusion for convex** (`6cbc9ec`)
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] package.json`, `[MODIFY] vite.config.ts`, `[MODIFY] convex/test.setup.test.ts`
  - **Commit**: `feat(convex): install convex-test and configure vitest runner`
  - **Verification**: `pnpm test convex/test.setup.test.ts` (PASSED / GREEN)

### Scenario 1.2: Generic Polymorphic Schema Definition (Core Invariant)
- [x] **Task 1.2a: [TEST] Add failing test asserting widget_data table schema and indexes** (`039c6aa`)
  - **Fulfills**: `REQ-SYNC-001`, `ADR-004`
  - **Files**: `[NEW] convex/schema.test.ts`
  - **Commit**: `test(schema): assert widget_data schema and indexes definition`
  - **Verification**: `pnpm test convex/schema.test.ts` (FAILED as expected)

- [x] **Task 1.2b: [FEAT] Implement generic polymorphic schema in convex/schema.ts** (`438384d`)
  - **Fulfills**: `REQ-SYNC-001`, `ADR-004`
  - **Files**: `[NEW] convex/schema.ts`
  - **Commit**: `feat(schema): implement generic widget_data schema with indexes`
  - **Verification**: `pnpm test convex/schema.test.ts` (PASSED / GREEN)

---

## Phase 2: Convex Server Functions (Queries & Mutations)

Fokus fase ini adalah membangun fungsi server Convex untuk kueri data reaktif dan mutasi data polimorfik *(upsert)* dengan perlindungan data multi-instance dan batasan pembacaan terukur.

### Scenario 2.1: Initial Widget Data Upsert via updatePayload (Normal Path - Creation)
- [x] **Task 2.1a: [TEST] Add failing test asserting new widget record creation via updatePayload** (`9832ca0`)
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[NEW] convex/widgets.test.ts`
  - **Commit**: `test(widgets): assert initial widget creation via updatePayload`
  - **Verification**: `pnpm test convex/widgets.test.ts` (FAILED as expected)

- [x] **Task 2.1b: [FEAT] Implement get query and updatePayload mutation insert branch** (`0bb7b6a`)
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[NEW] convex/widgets.ts`
  - **Commit**: `feat(widgets): implement get query and updatePayload creation logic`
  - **Verification**: `pnpm test convex/widgets.test.ts` (PASSED / GREEN)

### Scenario 2.2: Existing Widget Data Update without Row Duplication (Normal Path - Mutation)
- [ ] **Task 2.2a: [TEST] Add failing test asserting updatePayload modifies existing row without duplication**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] convex/widgets.test.ts`
  - **Commit**: `test(widgets): assert updatePayload patches existing row without duplication`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: FAIL / RED)

- [ ] **Task 2.2b: [FEAT] Implement patch branch in updatePayload mutation**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] convex/widgets.ts`
  - **Commit**: `feat(widgets): implement patch branch in updatePayload mutation`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: PASS / GREEN)

### Scenario 2.3: Multi-Instance Data Isolation (Core Invariant)
- [ ] **Task 2.3a: [TEST] Add failing test asserting strict data isolation across distinct instanceIds**
  - **Fulfills**: `REQ-SYNC-001`, `ADR-003`
  - **Files**: `[MODIFY] convex/widgets.test.ts`
  - **Commit**: `test(widgets): assert data isolation between distinct widget instances`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: FAIL / RED)

- [ ] **Task 2.3b: [FEAT] Enforce index-based unique instance querying in get query**
  - **Fulfills**: `REQ-SYNC-001`, `ADR-003`
  - **Files**: `[MODIFY] convex/widgets.ts`
  - **Commit**: `feat(widgets): enforce by_instance index querying for data isolation`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: PASS / GREEN)

### Scenario 2.4: Non-Existent Instance Handling (Boundary - Empty/Null)
- [ ] **Task 2.4a: [TEST] Add failing test asserting get returns null cleanly for non-existent instanceId**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] convex/widgets.test.ts`
  - **Commit**: `test(widgets): assert get query returns null for unknown instanceId`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: FAIL / RED)

- [ ] **Task 2.4b: [FEAT] Ensure graceful null return in get query**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] convex/widgets.ts`
  - **Commit**: `feat(widgets): handle null result gracefully for unknown instanceId`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: PASS / GREEN)

### Scenario 2.5: Bounded Query List by Widget Type (Boundary & Scale)
- [ ] **Task 2.5a: [TEST] Add failing test asserting listByType returns bounded list of instances**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] convex/widgets.test.ts`
  - **Commit**: `test(widgets): assert listByType returns bounded results`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: FAIL / RED)

- [ ] **Task 2.5b: [FEAT] Implement bounded listByType query using take()**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[MODIFY] convex/widgets.ts`
  - **Commit**: `feat(widgets): implement bounded listByType query`
  - **Verification**: `pnpm test convex/widgets.test.ts` (EXPECTED: PASS / GREEN)

---

## Phase 3: Frontend Client Integration & Sync Hook (Client Pipeline)

Fokus fase ini adalah menghubungkan frontend React ke provider Convex serta menyediakan custom hook `useWidgetSync` dengan ketahanan saat offline.

### Scenario 3.1: ConvexClientProvider Context Wiring (Normal Path)
- [ ] **Task 3.1a: [TEST] Add failing test asserting ConvexClientProvider renders children with Convex context**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[NEW] src/providers/ConvexClientProvider.test.tsx`
  - **Commit**: `test(client): assert ConvexClientProvider provides valid context`
  - **Verification**: `pnpm test src/providers/ConvexClientProvider.test.tsx` (EXPECTED: FAIL / RED)

- [ ] **Task 3.1b: [FEAT] Implement ConvexClientProvider reading VITE_CONVEX_URL and wrap App root**
  - **Fulfills**: `REQ-SYNC-001`
  - **Files**: `[NEW] src/providers/ConvexClientProvider.tsx`, `[MODIFY] src/App.tsx`
  - **Commit**: `feat(client): implement ConvexClientProvider and wrap App root`
  - **Verification**: `pnpm test src/providers/ConvexClientProvider.test.tsx` (EXPECTED: PASS / GREEN)

### Scenario 3.2: useWidgetSync Custom Hook with Offline Resilience (Boundary & Error Resilience)
- [ ] **Task 3.2a: [TEST] Add failing test asserting useWidgetSync handles loading, data return, and mutation invocation**
  - **Fulfills**: `REQ-SYNC-001`, `NFR-REL-001`
  - **Files**: `[NEW] src/hooks/useWidgetSync.test.tsx`
  - **Commit**: `test(client): assert useWidgetSync hook lifecycle and mutation calls`
  - **Verification**: `pnpm test src/hooks/useWidgetSync.test.tsx` (EXPECTED: FAIL / RED)

- [ ] **Task 3.2b: [FEAT] Implement useWidgetSync hook wrapping useQuery and useMutation**
  - **Fulfills**: `REQ-SYNC-001`, `NFR-REL-001`
  - **Files**: `[NEW] src/hooks/useWidgetSync.ts`
  - **Commit**: `feat(client): implement useWidgetSync hook with reactive query and mutation`
  - **Verification**: `pnpm test src/hooks/useWidgetSync.test.tsx` (EXPECTED: PASS / GREEN)
