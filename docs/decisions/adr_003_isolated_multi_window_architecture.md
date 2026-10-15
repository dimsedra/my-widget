---
title: "ADR-003: Isolated Multi-Window Architecture for Desktop Widgets"
type: adr
status: Approved
date: 2026-10-13
deciders: [Eds, Antigravity]
tags: [architecture, desktop, windowing, tauri, webview]
parent_srs: docs/srs/20261016_desktop_widget_platform.md
parent_prd: docs/prd/20261013_desktop_widget_platform.md
supersedes: null
superseded_by: null
---

# ADR-003: Isolated Multi-Window Architecture for Desktop Widgets

## 1. Status & Context
- **Status**: Approved (2026-10-13)
- **Context**:
  Sistem widget desktop memerlukan cara untuk menampilkan beberapa widget (To-Do, Countdown, dan widget di masa depan) di atas layar Windows secara simultan. Masing-masing widget harus bisa ditempatkan di koordinat bebas di desktop (misalnya satu di pojok kiri atas dan satu di pojok kanan bawah), memiliki ukuran independen, dan dapat digeser tanpa mengganggu area desktop di sekitarnya.

## 2. Decision
Kita memutuskan untuk menggunakan **Model Jendela Terisolasi *(Isolated Multi-Window Architecture)***, di mana setiap widget yang aktif berjalan sebagai satu jendela sistem operasi terpisah *(individual WebviewWindow instance)* yang dikelola oleh host engine Rust Tauri.

Setiap jendela widget memiliki:
1. Koordinat fisik (X, Y) dan ukuran (width, height) sendiri di level OS Windows.
2. Drag handle khusus dengan atribut `data-tauri-drag-region` untuk pergeseran bebas.
3. Menu konteks klik kanan untuk membuka antarmuka konfigurasi spesifik widget tersebut.

## 3. Alternatives Considered

### Alternative 1: Single-Window Fullscreen Canvas Overlay
- **Pros**: Hanya membuat satu proses WebView2 untuk seluruh layar, sehingga konsumsi memori RAM lebih rendah (~40–60 MB total untuk semua widget).
- **Cons**: Mengharuskan kita menangani deteksi klik tembus pandang *(per-pixel click-through hit-testing / WS_EX_TRANSPARENT toggle)* secara rumit di Win32 API agar klik pada area kosong antara widget-widget tembus ke desktop Windows secara tepat.
- **Why Rejected**: Kompleksitas teknis hit-testing sangat tinggi dan rentan terhadap bug di mana pengguna tidak bisa mengklik ikon desktop Windows biasa di bawah area transparan.

### Alternative 2: Single Window Modular Floating Pod (Dock/Sidebar)
- **Pros**: Seluruh widget disatukan di dalam satu jendela baris atau dock mengambang tunggal.
- **Cons**: Mengorbankan kebebasan penataan desktop ala Rainmeter. Pengguna tidak bisa meletakkan widget secara terpisah di sudut-sudut monitor yang berbeda.
- **Why Rejected**: Tidak memenuhi visi produk untuk menyediakan widget bebas seperti Rainmeter di mana widget dapat ditempatkan di mana saja di layar.

## 4. Consequences

### Positive
- Interaksi pengguna sangat alami: area di luar bingkai widget adalah desktop Windows asli 100%, tanpa perlu manipulasi tembus klik rumit.
- Isolasi kesalahan *(fault isolation)*: jika satu widget mengalami kegagalan render atau error JavaScript, widget lainnya tetap berjalan normal.
- Pergeseran jendela menggunakan akselerasi native OS yang sangat mulus via atribut drag Tauri.

### Negative & Operational Costs
- Konsumsi memori bertambah secara linear dengan jumlah widget aktif (tiap instance WebView2 memakan sekitar 30–40 MB RAM tambahan).
- Host engine Rust bertanggung jawab mengelola koordinat dan siklus hidup banyak jendela secara bersamaan.

### Risks & Mitigations
- **Risiko**: Konsumsi RAM melonjak jika pengguna membuka terlalu banyak widget sekaligus.
- **Mitigasi**: Untuk fase MVP (2 widget aktif: To-Do dan Countdown), batas memori tetap aman di bawah 90 MB total. Kita dapat menambahkan batas maksimal instance widget aktif jika diperlukan di masa depan.

## 5. Verification & Compliance
- Diverifikasi melalui Unit dan Integration Test (`REQ-HOST-001`, `REQ-HOST-002`, `REQ-CONFIG-001`) yang memastikan pembuatan jendela terpisah, persistensi koordinat independen, dan penutupan jendela tanpa memory leak.

## 6. Related Decisions
- Terkait langsung dengan [`docs/decisions/adr_001_adopt_tauri_v2_with_rust_host_engine.md`](file:///C:/Users/Dimas%20Edra%20Ar%20Rafi/ProjectHub/my-widget/docs/decisions/adr_001_adopt_tauri_v2_with_rust_host_engine.md).
