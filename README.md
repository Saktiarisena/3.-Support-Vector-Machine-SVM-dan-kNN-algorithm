## Fitur

* Membaca dataset dari file Excel (`.xlsx`).
* Melatih model:

  * **SVM (Support Vector Machine)** untuk regresi.
  * **K-Means Clustering** untuk melakukan pengelompokan.
* Menampilkan data asli, hasil prediksi SVM, dan cluster K-Means pada terminal.
* Menyimpan hasil visualisasi dalam bentuk file gambar (`plot.png`).
* Menampilkan progress training K-Means dengan animasi sederhana di terminal.

---

## Prasyarat

Pastikan Anda telah menginstal:

* **Rust** ([https://www.rust-lang.org/](https://www.rust-lang.org/))
* File dataset: `Rice_MSC_Dataset_sample.xlsx`

---

## Dependencies

Berikut adalah crates yang digunakan pada project ini:

```toml
[dependencies]
linfa = "0.7.1"
linfa-svm = "0.7.2"
linfa-nn = "0.7.1"
linfa-clustering = "0.7.1"
ndarray = "0.15.6"
csv = "1.1"
rand = "0.9.0"
plotters = "0.3.0"
calamine = "0.19"
```

---

## Cara Menjalankan

### 1. Clone Repository

```bash
git clone https://github.com/username/project-name.git
cd project-name
```

### 2. Menambahkan Dataset

Pastikan file `Rice_MSC_Dataset_sample.xlsx` berada di direktori root proyek.

### 3. Build & Run Program

```bash
cargo run
```

---

## Penjelasan Alur Program

1. **Membaca Dataset:**

   * Program membaca file Excel (`Rice_MSC_Dataset_sample.xlsx`) dari sheet `Sheet1`.
   * Field yang digunakan:

     * `ASPECT_RATIO` (fitur/input)
     * `COMPACTNESS` (label/target)

2. **Pra-pemrosesan Data:**

   * Dataset di-split menjadi data training (80%) dan testing (20%).

3. **Training Model:**

   * **SVM (Support Vector Machine)**

     * Menggunakan linear kernel untuk memodelkan hubungan antara `ASPECT_RATIO` dan `COMPACTNESS`.
   * **K-Means Clustering**

     * Mengelompokkan data ke dalam 3 cluster.
     * Menampilkan progress training dengan animasi sederhana.

4. **Prediksi & Output:**

   * Menampilkan prediksi SVM dan cluster K-Means pada data testing di terminal.

5. **Visualisasi Data:**

   * Membuat plot `plot.png` yang berisi:

     * Data asli (warna biru)
     * Hasil prediksi SVM (warna merah)
     * Cluster K-Means (warna hijau, magenta, cyan)

---

## Output Contoh

```
=== Data Asli ===
No      ASPECT_RATIO (Fitur)    COMPACTNESS (Label)
------------------------------------------------
1       2.3450                  0.6500
2       3.1200                  0.7000
...

=== Training SVM ===

=== Hasil Prediksi SVM ===
Data Test       Prediksi
--------------------------
2.3450          0.6450
...

=== Training K-Means ===
Training K-Means...
Training K-Means selesai!

=== Hasil Prediksi K-Means ===
Data Test       Cluster
--------------------------
2.3450          1
...

Grafik telah disimpan sebagai plot.png
```

---

## Struktur Proyek

```
project-name/
│
├── Cargo.toml
├── src/
│   └── main.rs
├── Rice_MSC_Dataset_sample.xlsx
└── plot.png (setelah program dijalankan)
```

---

## Catatan

* Pastikan dataset dengan header yang sesuai:

  * Kolom ke-2 (`index 1`): `ASPECT_RATIO`
  * Kolom ke-4 (`index 3`): `COMPACTNESS`
* Plot akan otomatis digenerate setelah proses training dan prediksi selesai.

---
