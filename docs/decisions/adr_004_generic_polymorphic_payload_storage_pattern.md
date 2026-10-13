---
title: "ADR-004: Generic Polymorphic Payload Storage Pattern for Widget Data"
type: adr
status: Waiting for Review
date: 2026-10-13
deciders: [Eds, Antigravity]
tags: [architecture, database, schema, convex, storage, zod]
parent_srs: docs/srs/20261013_desktop_widget_platform.md
parent_prd: docs/prd/20261013_desktop_widget_platform.md
supersedes: null
superseded_by: null
---

# ADR-004: Generic Polymorphic Payload Storage Pattern for Widget Data

## 1. Status & Context
- **Status**: Waiting for Review (2026-10-13)
- **Context**:
  Sistem dirancang sebagai platform widget berbasis add-on yang dapat diperluas sewaktu-waktu. Setiap jenis widget (To-Do List, Days-To-Go Countdown, catatan, pelacak kebiasaan, dsb.) memiliki kebutuhan atribut dan struktur data yang sangat berbeda satu sama lain. Kita memerlukan strategi penyimpanan data di Convex yang:
  1. Mendukung penambahan widget baru tanpa perlu mengubah skema database global setiap kali modul baru dibuat *(Zero Schema Mutation on Add-on)*.
  2. Tetap menjamin keamanan tipe data *(type safety)* dan validasi input yang ketat agar tidak ada data korup yang tersimpan.

## 2. Decision
Kita memutuskan untuk menggunakan **Pola Penyimpanan Dokumen Polimorfik Generik *(Pola A - Generic Polymorphic Payload Storage Pattern)*** di Convex.

Di tingkat database Convex, kita hanya mendefinisikan satu tabel tunggal `widget_data` dengan struktur amplop *(envelope)*:
- `instanceId: v.string()`
- `widgetType: v.string()`
- `title: v.string()`
- `payload: v.any()`
- `updatedAt: v.number()`

Validasi skema spesifik dan penegakan tipe data ketat dialihkan ke **modul masing-masing widget di sisi frontend** menggunakan skema Zod (misalnya `src/widgets/todo/schema.ts` dan `src/widgets/countdown/schema.ts`).

## 3. Alternatives Considered

### Alternative 1: Dedicated Tables per Widget Module di Convex (`todos`, `countdowns`, dll)
- **Pros**: Validasi tipe data ketat langsung dieksekusi di level internal database Convex.
- **Cons**: Mengorbankan kemandirian add-on. Setiap kali pengembang ingin membuat widget baru, berkas skema backend sentral `convex/schema.ts` harus diubah dan dideploy ulang.
- **Why Rejected**: Bertentangan dengan prinsip desain sistem add-on yang mandiri dan terisolasi per folder widget.

### Alternative 2: Local SQLite dengan Sync Engine Kustom
- **Pros**: Data tersimpan penuh secara lokal dalam tabel relasional SQL.
- **Cons**: Memerlukan penulisan logika sinkronisasi multi-device manual (resolusi konflik, penanda timestamp mutasi, polling, atau sync engine rumit seperti CRDT).
- **Why Rejected**: Beban implementasi dan pemeliharaan sinkronisasi terlalu berat dibandingkan memanfaatkan mekanisme reaktif bawaan Convex.

## 4. Consequences

### Positive
- **100% Plug-and-Play**: Modul widget baru di masa depan cukup dibuat di dalam folder `src/widgets/<nama-widget>/` dengan skema Zod sendiri, tanpa perlu memodifikasi berkas database sentral Convex.
- Mendukung pembuatan banyak instance dari satu jenis widget yang sama (misal 2 widget countdown untuk acara berbeda) dengan `instanceId` yang unik.
- Pengujian validasi data menjadi sangat modular dan terisolasi per-widget.

### Negative & Operational Costs
- Database Convex tidak memvalidasi isi field `payload` secara langsung; integritas data sepenuhnya dijaga oleh pustaka Zod di sisi klien sebelum memanggil mutasi.
- Jika ada perubahan format data drastis *(schema breaking changes)* pada versi widget berikutnya, migrasi data lama harus ditangani di lapisan validasi klien (misal dengan nilai default Zod).

### Risks & Mitigations
- **Risiko**: Data korup jika klien yang tidak mematuhi skema mengirim mutasi dengan format payload salah.
- **Mitigasi**: Menjalankan parser Zod (`TodoPayloadSchema.parse(data)`) tepat sebelum fungsi mutasi Convex dipanggil di antarmuka pengguna, serta fallback ke default state jika data kueri tidak sesuai.

## 5. Verification & Compliance
- Diverifikasi melalui Unit Test TypeScript (`REQ-TODO-001`, `REQ-COUNTDOWN-001`) yang menguji keakuratan validasi Zod untuk payload normal, batas nilai, dan payload malformed.

## 6. Related Decisions
- Berpasangan dengan pilihan Convex di [`docs/decisions/adr_002_use_convex_for_realtime_cloud_sync.md`](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_002_use_convex_for_realtime_cloud_sync.md).
