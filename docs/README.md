# Documentation Index

Dokumentasi arsitektur, spesifikasi produk, dan rekayasa perangkat lunak untuk proyek `my-widget`.

---

## Product Requirements (PRD)

| Dokumen | Versi / Status | Tanggal Pembaruan | Ringkasan Cakupan |
| :--- | :--- | :--- | :--- |
| [Desktop Widget Platform](file:///D:/Project%20Hub/my-widget/docs/prd/20261013_desktop_widget_platform.md) | `Approved` | 2026-10-15 | Definisi masalah, ruang lingkup MVP (To-Do & Countdown), dan Critical User Journeys (CUJ). |

---

## Software Requirements (SRS)

| Dokumen | Induk PRD | Status | Tanggal Pembaruan | Ringkasan Cakupan |
| :--- | :--- | :--- | :--- | :--- |
| [Desktop Widget Platform Technical Specification](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md) | [PRD](file:///D:/Project%20Hub/my-widget/docs/prd/20261013_desktop_widget_platform.md) | `Approved` | 2026-10-16 | Kontrak teknis Win32 pinning, Tray Flyout Hub, tema mono gray, Tauri v2 IPC, generic payload Convex, dan dekomposisi 5 rencana kerja. |

---

## Implementation Plans (Plan)

| Dokumen | Induk SRS | Status | Progress | Tanggal Pembaruan | Ringkasan Cakupan |
| :--- | :--- | :--- | :--- | :--- | :--- |
| [Phase 1: Core Host Platform Engine](file:///D:/Project%20Hub/my-widget/docs/plan/20261015_phase1_core_host_engine.md) | [SRS](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md) | `Completed` | `100% (28/28)` | 2026-10-15 | Inisialisasi Tauri v2, Win32 desktop pinning, multi-window runner, System Tray, dan persistensi config (Audit Remediation). |
| [Phase 2: Cloud Backend & Sync Pipeline](file:///D:/Project%20Hub/my-widget/docs/plan/20261015_phase2_cloud_backend_sync.md) | [SRS](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md) | `Completed` | `100% (24/24)` | 2026-10-16 | Setup Convex BaaS, skema generic payload `widget_data`, kueri/mutasi, dan contract tests (Audit Remediation). |
| `phase3_tray_flyout_widget_hub.md` | [SRS](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md) | `Planned` | `0%` | 2026-10-16 | Jendela flyout tray kompak, antarmuka mono gray (Hub & Config view), toggle visibilitas, dan sinkronisasi setting in-place. |
| `phase4_todo_widget_notification.md` | [SRS](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md) | `Planned` | `0%` | 2026-10-16 | Modul UI To-Do, Zod schema, task CRUD, deadline monitor interval, dan Windows native toast bridge. |
| `phase5_countdown_widget_polish.md` | [SRS](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md) | `Planned` | `0%` | 2026-10-16 | Modul UI Countdown, kalkulasi delta waktu, audit visual mono gray frosted glass, dan tolok ukur RAM/CPU. |

---

## Architectural Decisions (ADR)

| ID | Judul Keputusan | Status | Tanggal | Ringkasan Keputusan |
| :--- | :--- | :--- | :--- | :--- |
| [ADR-001](file:///D:/Project%20Hub/my-widget/docs/decisions/adr_001_adopt_tauri_v2_with_rust_host_engine.md) | Adopt Tauri v2 with Rust as Host Desktop Engine | `Approved` | 2026-10-13 | Memilih Tauri v2 (Rust backend + WebView2) untuk efisiensi memori dan akses Win32 API. |
| [ADR-002](file:///D:/Project%20Hub/my-widget/docs/decisions/adr_002_use_convex_for_realtime_cloud_sync.md) | Use Convex as Real-Time Cloud BaaS | `Approved` | 2026-10-13 | Memilih Convex untuk sinkronisasi reaktif live-query otomatis lintas perangkat. |
| [ADR-003](file:///D:/Project%20Hub/my-widget/docs/decisions/adr_003_isolated_multi_window_architecture.md) | Isolated Multi-Window Architecture for Desktop Widgets | `Approved` | 2026-10-13 | Satu jendela OS WebView2 terisolasi per-widget dengan drag handle mandiri. |
| [ADR-004](file:///D:/Project%20Hub/my-widget/docs/decisions/adr_004_generic_polymorphic_payload_storage_pattern.md) | Generic Polymorphic Payload Storage Pattern for Widget Data | `Approved` | 2026-10-13 | Skema tabel generik di Convex dengan validasi tipe data Zod di masing-masing modul widget. |

---

## Konvensi Dokumentasi

- Seluruh spesifikasi produk berada di `docs/prd/` dengan format penamaan `YYYYMMDD_<slug>.md`.
- Seluruh spesifikasi teknis perangkat lunak berada di `docs/srs/` dengan format penamaan `YYYYMMDD_<slug>.md`.
- Seluruh rekaman keputusan arsitektur berada di `docs/decisions/` dengan format penamaan `adr_NNN_<slug>.md`.
- Setiap dokumen baru atau perubahan status wajib diperbarui pada indeks ini untuk mencegah dokumen usang *(anti-rot indexing)*.
