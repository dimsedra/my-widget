---
title: "ADR-001: Adopt Tauri v2 with Rust as Host Desktop Engine"
type: adr
status: Waiting for Review
date: 2026-10-13
deciders: [Eds, Antigravity]
tags: [architecture, desktop, engine, rust, tauri]
parent_srs: docs/srs/20261013_desktop_widget_platform.md
parent_prd: docs/prd/20261013_desktop_widget_platform.md
supersedes: null
superseded_by: null
---

# ADR-001: Adopt Tauri v2 with Rust as Host Desktop Engine

## 1. Status & Context
- **Status**: Waiting for Review (2026-10-13)
- **Context**: 
  Aplikasi widget desktop ini dirancang untuk berjalan terus-menerus di latar belakang *(always-on background process)* menempel di wallpaper desktop Windows ala Rainmeter. Dibutuhkan engine desktop yang:
  1. Sangat hemat memori dan CPU saat dalam kondisi diam *(idle)*.
  2. Mampu berinteraksi langsung dengan API sistem operasi Windows *(Win32 API)* untuk menyematkan jendela ke level terbawah (`HWND_BOTTOM`), menyembunyikan jendela dari taskbar dan `Alt + Tab`, serta mengelola System Tray.
  3. Memiliki fleksibilitas tinggi dalam perancangan antarmuka komponen UI yang modern dan estetis.

## 2. Decision
Kita memutuskan untuk menggunakan **Tauri v2** dengan backend **Rust** sebagai host engine desktop, dan **WebView2 (React + Vite)** sebagai lapisan tampilan widget.

Rust menangani siklus hidup aplikasi, System Tray, pergeseran jendela, manipulasi Win32 API, serta penyimpanan konfigurasi lokal. Lapisan antarmuka web menangani tata letak widget dan interaksi pengguna.

## 3. Alternatives Considered

### Alternative 1: Electron (Node.js + Bundled Chromium)
- **Pros**: Ekosistem web sangat matang, mudah membuat window kustom.
- **Cons**: Mengemas salinan penuh browser Chromium dan Node.js ke dalam aplikasi.
- **Why Rejected**: Konsumsi memori RAM saat idle terlalu boros (biasanya 150–300 MB), bertentangan dengan prinsip widget desktop yang harus ringan dan tidak membebani sistem saat berjalan 24/7.

### Alternative 2: Pure C# / .NET (WPF atau WinUI 3)
- **Pros**: Integrasi first-class dengan Windows Win32 dan DirectX rendering tanpa web engine.
- **Cons**: Terbatas hanya untuk Windows (kode UI tidak dapat digunakan ulang jika ingin ekspansi ke platform mobile di masa depan), dan Convex tidak memiliki SDK resmi tingkat pertama untuk .NET.
- **Why Rejected**: Mengharuskan kita menulis jembatan protokol kustom untuk Convex, serta membatasi portabilitas komponen UI di masa mendatang.

### Alternative 3: Pure Rust GUI (Slint / egui)
- **Pros**: Konsumsi memori sangat minim (< 20 MB RAM) dan performa komputasi murni.
- **Cons**: Ekosistem perancangan UI berbasis deklaratif web terbatas; integrasi klien WebSocket Convex harus dibangun dari nol.
- **Why Rejected**: Kompleksitas perakitan antarmuka dan pengelolaan protokol Convex terlalu tinggi dibandingkan manfaatnya.

## 4. Consequences

### Positive
- Memanfaatkan WebView2 bawaan Windows sehingga ukuran paket instalasi sangat kecil dan konsumsi memori idle jauh lebih rendah daripada Electron (~30–70 MB per instance).
- Backend Rust berjalan dengan 0% CPU saat idle tanpa siklus pengumpulan sampah memori *(Garbage Collector)* berkala.
- Akses penuh ke API Windows via crate `windows-rs` di Rust.
- Komponen frontend dapat memanfaatkan ekosistem web modern (React, Tailwind CSS, Convex React SDK).

### Negative & Operational Costs
- Memerlukan perkakas kompilasi Rust *(Rust toolchain + C++ Build Tools)* pada mesin pengembangan.
- Performa tampilan bergantung pada ketersediaan runtime Microsoft Edge WebView2 di sistem Windows pengguna (standar di Windows 10/11).

### Risks & Mitigations
- **Risiko**: Variasi perilaku WebView2 antar versi Windows.
- **Mitigasi**: Menargetkan Windows 10 build 19041+ dan Windows 11 di mana WebView2 Evergreen selalu diperbarui otomatis oleh sistem operasi.

## 5. Verification & Compliance
- Verifikasi batas penggunaan RAM saat diam melalui pengujian performa (`NFR-PERF-001`, ambang batas total < 90 MB).
- Pemeriksaan berkala bahwa proses Rust berada di 0% CPU saat tidak ada event kursor aktif.

## 6. Related Decisions
- Terkait langsung dengan [`docs/srs/20261013_desktop_widget_platform.md`](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/srs/20261013_desktop_widget_platform.md) (Bagian `REQ-HOST-001` hingga `REQ-HOST-003`).
