---
title: Desktop Widget Platform
type: prd
status: Approved
last_updated: 2026-10-15
---

# Product Requirements Document: Desktop Widget Platform

## Section 1: Metadata & Status

- **Project Name**: Desktop Widget Platform (`my-widget`)
- **Document Type**: Product Requirements Document (PRD)
- **Status**: Approved
- **Lead / Owner**: Eds (Product & Engineering Owner)
- **Target OS**: Windows 10 / Windows 11 (64-bit)
- **Initial Target Widgets**: To-Do List (dengan notifikasi batas waktu) dan Days-To-Go Countdown
- **Target Release**: MVP v0.1.0

---

## Section 2: Overview & Context

### 2.1 Problem Statement
Pengguna desktop sering kali membutuhkan akses visual cepat ke informasi penting sehari-hari seperti daftar tugas mendesak *(to-do list)* dan pengingat tanggal krusial *(countdown days-to-go)* langsung di layar utama kerja mereka tanpa harus membuka jendela browser atau aplikasi penuh yang menghabiskan memori dan mengganggu alur fokus.

Aplikasi pencatat konvensional umumnya berbentuk aplikasi jendela standar yang:
1. Memenuhi taskbar dan daftar jendela aktif (`Alt + Tab`), sehingga mengganggu navigasi jendela kerja.
2. Memerlukan pembukaan manual berulang kali setiap kali ingin melihat atau mencentang progres.
3. Aplikasi desktop berbasis web tradisional (seperti Electron wrapper) memakan memori RAM sangat besar meskipun hanya dalam kondisi diam *(idle)*.

### 2.2 Target Personas & Core Need
- **Primary Persona**: Power user desktop yang menginginkan informasi personal selalu tampil di atas wallpaper desktop seperti widget Rainmeter, ringan di memori, dapat digeser secara bebas, serta otomatis tersinkronisasi ke perangkat lain (laptop, mobile di masa depan).
- **Core Value Proposition**: Widget desktop native yang menempel di wallpaper, hemat sumber daya, responsif secara instan terhadap perubahan data di cloud, dan memiliki arsitektur modular yang memudahkan penambahan widget baru di kemudian hari.

---

## Section 3: Goals & Success Metrics

### 3.1 Business & Product Goals
1. Menyediakan platform host widget Windows yang stabil, tidak intrusif, dan menyatu secara visual dengan desktop.
2. Memfasilitasi ekstensibilitas widget berbasis add-on sehingga penambahan modul widget baru tidak merombak aplikasi utama.
3. Menjamin sinkronisasi data dua arah secara instan *(real-time reactivity)* antar perangkat via cloud BaaS.

### 3.2 Success Metrics (Key Results)
- **Memory Footprint (Idle)**: Konsumsi RAM pada kondisi diam berada di bawah 80 MB untuk host engine dan 2 widget aktif.
- **CPU Utilization (Idle)**: Konsumsi CPU 0% saat tidak ada interaksi pengguna atau animasi aktif.
- **Sync Latency**: Mutasi data dari cloud tersinkronisasi dan ter-render di widget desktop dalam waktu kurang dari 500 milidetik saat terhubung ke jaringan internet stabil.
- **Notification Reliability**: 100% notifikasi pengingat batas waktu tugas terkirim tepat waktu melalui banner notifikasi native Windows.

---

## Section 4: Scope & Constraints

### 4.1 In-Scope (MVP - P0)
- **Host Engine Foundation**:
  - Jendela widget frameless, transparan, dan disematkan di level desktop (`HWND_BOTTOM`).
  - Jendela tidak muncul di Taskbar (`skip_taskbar: true`) dan tidak muncul di daftar `Alt + Tab` (`WS_EX_TOOLWINDOW`).
  - System Tray Icon di area notifikasi Windows untuk kontrol aplikasi (membuka widget, mengunci posisi, menutup aplikasi).
- **Interaksi Widget**:
  - Drag handle khusus untuk menggeser posisi jendela widget secara bebas.
  - Menu klik kanan pada widget untuk membuka panel pengaturan konfigurasi lokal widget.
  - Penyimpanan posisi fisik (X, Y) widget di penyimpanan lokal komputer pengguna.
- **Modul Widget 1 (To-Do List)**:
  - Melihat daftar tugas aktif dan tugas selesai.
  - Menambah tugas baru, menandai selesai *(toggle complete)*, dan menghapus tugas.
  - Menentukan batas waktu tugas *(due date / due time)* dan memicu notifikasi native Windows saat waktu tercapai.
- **Modul Widget 2 (Days-To-Go Countdown)**:
  - Menampilkan hitung mundur hari/jam menuju tanggal target tertentu.
  - Mengatur nama kegiatan dan target waktu via form pengaturan widget.
- **Cloud Synchronization**:
  - Sinkronisasi data konten widget dua arah menggunakan Convex dengan skema dokumen generik *(Pola A)*.

### 4.2 Out-of-Scope (Deferred - P2)
- Aplikasi klien untuk platform Android/iOS (fokus MVP adalah fungsionalitas Windows desktop terlebih dahulu).
- Sistem autentikasi pengguna multi-tenant yang rumit (pada fase MVP menggunakan single-user instance ID atau workspace ID sederhana).
- Marketplace / Dynamic online plugin installer (widget masih berupa modul terdaftar di dalam struktur kode proyek).
- Pengaturan tema visual dinamis yang kompleks per-widget.

### 4.3 Technical Constraints
- Sistem operasi target: Windows 10 build 19041 ke atas dan Windows 11.
- Runtime backend menggunakan Rust (Tauri v2).
- Runtime tampilan menggunakan WebView2 yang telah terpasang secara bawaan di Windows.

---

## Section 5: Requirements & Critical User Journeys

### 5.1 Critical User Journeys (CUJs)

#### CUJ-01: Membuka dan Memposisikan Widget di Desktop
- **Given**: Aplikasi host engine berjalan di latar belakang (ikon terlihat di System Tray).
- **When**: Pengguna mengklik opsi buka widget To-Do dari menu tray.
- **Then**: Jendela widget To-Do muncul di layar desktop tanpa border, tanpa muncul di taskbar. Pengguna dapat menekan drag handle untuk menggeser widget ke pojok kanan bawah layar. Saat dilepas, posisi (X, Y) tersimpan di konfigurasi lokal komputer.

#### CUJ-02: Menambah Tugas dan Menerima Notifikasi
- **Given**: Widget To-Do aktif di desktop dan terhubung ke Convex.
- **When**: Pengguna mengetik nama tugas baru "Review Laporan" dan menyetel batas waktu 15 menit ke depan.
- **Then**: Tugas tersimpan di Convex secara instan. Ketika waktu batas tiba, sistem memicu notifikasi toast native Windows yang berbunyi dan menampilkan nama tugas.

#### CUJ-03: Menyetel Target Waktu pada Widget Days-To-Go
- **Given**: Widget Days-To-Go aktif di desktop.
- **When**: Pengguna melakukan klik kanan pada widget, memilih menu "Settings", dan memasukkan nama "Liburan Akhir Tahun" serta tanggal target 31 Desember.
- **Then**: Tampilan widget langsung memperbarui angka hari tersisa (*e.g., "78 Days to go"*), dan data tersimpan ke cloud.

---

## Section 6: Design & Technical References

- **Arsitektur Teknis**: Terhubung langsung ke Software Requirements Specification di [`docs/srs/20261016_desktop_widget_platform.md`](file:///D:/Project%20Hub/my-widget/docs/srs/20261016_desktop_widget_platform.md).
- **Brand & Visual**: Menggunakan pendekatan clean, translucent/frosted glass (backdrop-blur), typography tegas, dan kontras tinggi untuk keterbacaan di atas berbagai wallpaper Windows.

---

## Section 7: Launch, Risks & Open Decisions

### 7.1 Risks & Mitigation
- **Risk 1: Z-Order Reset saat Windows Desktop Refresh**:
  - *Skenario*: Menekan `Win + D` (Show Desktop) kadang menyembunyikan jendela biasa atau mengubah urutan jendela sistem.
  - *Mitigasi*: Engine Rust menangani event window level dan memastikan jendela disematkan ke level desktop atau menanggapi event aktivasi jendela secara tepat.
- **Risk 2: Kehilangan Koneksi Internet**:
  - *Skenario*: Widget tidak dapat menghubungi server Convex saat komputer offline.
  - *Mitigasi*: Menampilkan indikator status koneksi yang halus, dan mempertahankan data kueri terakhir yang ada di memori klien.

### 7.2 Decision Log
- [x] Memilih **Tauri v2 + Convex** untuk menyeimbangkan performa native Rust dengan kemudahan reaktivitas data web TypeScript.
- [x] Memilih **Multi-Window terisolasi** per widget untuk fleksibilitas pergeseran koordinat bebas di desktop.
- [x] Memilih **Pola A (Generic Payload Table)** di Convex untuk memastikan penambahan modul widget di masa depan bersifat *plug-and-play*.
