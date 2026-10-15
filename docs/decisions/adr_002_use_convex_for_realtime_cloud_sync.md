---
title: "ADR-002: Use Convex as Real-Time Cloud BaaS"
type: adr
status: Approved
date: 2026-10-13
deciders: [Eds, Antigravity]
tags: [architecture, backend, cloud, sync, convex]
parent_srs: docs/srs/20261016_desktop_widget_platform.md
parent_prd: docs/prd/20261013_desktop_widget_platform.md
supersedes: null
superseded_by: null
---

# ADR-002: Use Convex as Real-Time Cloud BaaS

## 1. Status & Context
- **Status**: Approved (2026-10-13)
- **Context**:
  Data widget (seperti tugas to-do list dan tanggal countdown) perlu disinkronkan secara mulus antar perangkat pengguna (PC desktop, laptop, dan perangkat mobile di masa depan). Sistem membutuhkan basis data berbasis cloud *(Backend-as-a-Service / BaaS)* yang:
  1. Mendukung reaktivitas data instan dua arah *(live reactive subscriptions)* tanpa perlu polling HTTP manual.
  2. Memiliki beban pemeliharaan infrastruktur dan boilerplate manajemen state lokal yang minimal.
  3. Terintegrasi mulus dengan lingkungan ekosistem TypeScript dan React di sisi frontend.

## 2. Decision
Kita memutuskan untuk menggunakan **Convex** sebagai penyedia BaaS cloud utama untuk sinkronisasi data widget.

Klien frontend React di dalam Tauri akan memanfaatkan pustaka resmi `@convex-dev/react` untuk mengeksekusi kueri reaktif live (`useQuery`) dan mutasi data (`useMutation`) melalui koneksi persisten WebSocket (`wss://`).

## 3. Alternatives Considered

### Alternative 1: Supabase (PostgreSQL + Realtime CDC)
- **Pros**: Relasional PostgreSQL standar, open-source, mendukung self-hosting, memiliki SDK untuk berbagai bahasa termasuk C#.
- **Cons**: Mekanisme reaktivitas berbasis aliran perubahan baris *(Postgres Change Data Capture events: INSERT/UPDATE/DELETE)*, bukan kueri otomatis. Memerlukan penulisan logika manajemen state lokal manual di klien untuk memproses mutasi baris data.
- **Why Rejected**: Menambah kompleksitas boilerplate di sisi klien widget untuk menyusun ulang state lokal, sedangkan Convex menghitung ulang kueri di server dan langsung mengirimkan state akhir yang utuh ke komponen UI.

### Alternative 2: Backend Kustom Mandiri (Node.js/FastAPI + WebSocket + SQLite/Postgres)
- **Pros**: Kontrol penuh atas seluruh lapisan protokol dan penyimpanan.
- **Cons**: Membutuhkan deployment, hosting server, pemantauan uptime, manajemen koneksi WebSocket pool, dan pemeliharaan server secara manual.
- **Why Rejected**: Biaya pemeliharaan operasional terlalu besar untuk kebutuhan widget personal/multi-device.

### Alternative 3: Firebase Firestore
- **Pros**: Database dokumen real-time yang sudah teruji lama dengan SDK web yang matang.
- **Cons**: Model harga dan aturan keamanan (*Security Rules*) berbasis dokumen yang sering membingungkan, serta ketiadaan TypeScript schema definition terpusat yang se-ergonomis Convex.
- **Why Rejected**: Convex menawarkan developer experience TypeScript-native yang jauh lebih bersih dengan fungsi server bertipe aman *(type-safe server functions)*.

## 4. Consequences

### Positive
- Komponen widget langsung bereaksi terhadap perubahan data di cloud secara otomatis *(zero boilerplate state syncing)*.
- Skema data dan fungsi backend didefinisikan secara deklaratif dalam TypeScript.
- Mendukung sinkronisasi instan ke aplikasi mobile atau web di masa mendatang karena Convex adalah backend terpusat.

### Negative & Operational Costs
- Ketergantungan pada layanan cloud pihak ketiga *(Convex Cloud)*.
- Pengguna harus memiliki koneksi internet untuk sinkronisasi data baru (meskipun cache memori lokal tetap menampilkan snapshot terakhir).

### Risks & Mitigations
- **Risiko**: Latensi jaringan atau putusnya koneksi internet saat perangkat offline.
- **Mitigasi**: Klien Convex secara otomatis menangani reconnect WebSocket dan mengantrikan mutasi di memori hingga koneksi pulih kembali.

## 5. Verification & Compliance
- Kepatuhan diuji melalui Contract Tests (`REQ-SYNC-001`) yang memverifikasi bahwa mutasi data dari satu klien langsung tercermin pada kueri klien lain dalam waktu kurang dari 500 milidetik.

## 6. Related Decisions
- Berpasangan dengan skema generic payload di [`docs/decisions/adr_004_generic_polymorphic_payload_storage_pattern.md`](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_004_generic_polymorphic_payload_storage_pattern.md).
