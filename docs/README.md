# Documentation Index

Dokumentasi arsitektur, spesifikasi produk, dan rekayasa perangkat lunak untuk proyek `my-widget`.

---

## Product Requirements (PRD)

| Dokumen | Versi / Status | Tanggal Pembaruan | Ringkasan Cakupan |
| :--- | :--- | :--- | :--- |
| [Desktop Widget Platform](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/prd/20261013_desktop_widget_platform.md) | `Waiting for Review` | 2026-10-13 | Definisi masalah, ruang lingkup MVP (To-Do & Countdown), dan Critical User Journeys (CUJ). |

---

## Software Requirements (SRS)

| Dokumen | Induk PRD | Status | Tanggal Pembaruan | Ringkasan Cakupan |
| :--- | :--- | :--- | :--- | :--- |
| [Desktop Widget Platform Technical Specification](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/srs/20261013_desktop_widget_platform.md) | [PRD](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/prd/20261013_desktop_widget_platform.md) | `Waiting for Review` | 2026-10-13 | Kontrak teknis Win32 pinning, Tauri v2 IPC, skema generic payload Convex, dan matriks verifikasi otomatis. |

---

## Architectural Decisions (ADR)

| ID | Judul Keputusan | Status | Tanggal | Ringkasan Keputusan |
| :--- | :--- | :--- | :--- | :--- |
| [ADR-001](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_001_adopt_tauri_v2_with_rust_host_engine.md) | Adopt Tauri v2 with Rust as Host Desktop Engine | `Approved` | 2026-10-13 | Memilih Tauri v2 (Rust backend + WebView2) untuk efisiensi memori dan akses Win32 API. |
| [ADR-002](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_002_use_convex_for_realtime_cloud_sync.md) | Use Convex as Real-Time Cloud BaaS | `Approved` | 2026-10-13 | Memilih Convex untuk sinkronisasi reaktif live-query otomatis lintas perangkat. |
| [ADR-003](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_003_isolated_multi_window_architecture.md) | Isolated Multi-Window Architecture for Desktop Widgets | `Approved` | 2026-10-13 | Satu jendela OS WebView2 terisolasi per-widget dengan drag handle mandiri. |
| [ADR-004](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_004_generic_polymorphic_payload_storage_pattern.md) | Generic Polymorphic Payload Storage Pattern for Widget Data | `Approved` | 2026-10-13 | Skema tabel generik di Convex dengan validasi tipe data Zod di masing-masing modul widget. |

---

## Konvensi Dokumentasi

- Seluruh spesifikasi produk berada di `docs/prd/` dengan format penamaan `YYYYMMDD_<slug>.md`.
- Seluruh spesifikasi teknis perangkat lunak berada di `docs/srs/` dengan format penamaan `YYYYMMDD_<slug>.md`.
- Seluruh rekaman keputusan arsitektur berada di `docs/decisions/` dengan format penamaan `adr_NNN_<slug>.md`.
- Setiap dokumen baru atau perubahan status wajib diperbarui pada indeks ini untuk mencegah dokumen usang *(anti-rot indexing)*.
