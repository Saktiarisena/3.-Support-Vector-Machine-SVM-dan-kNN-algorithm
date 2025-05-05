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

Sumber Refrensi :
- https://www.kaggle.com/code/satyaprakashshukl/rice-classification 
[1]	K. H. Ng, S. C. Liew, and F. Ernawan, “An Improved Image Steganography Scheme Based on RDWT and QR Decomposition,” IOP Conf. Ser. Mater. Sci. Eng., vol. 769, no. 1, pp. 222–231, 2020. 

[2]	B. Ando, S. Baglio, S. Castorina, R. Crispino, and V. Marletta, “A Methodology for the Development of Low-Cost, Flexible Touch Sensor for Application in Assistive Technology,” IEEE Trans. Instrum. Meas., vol. 70, 2021. 

[3]	V. Krishnasamy and S. Venkatachalam, “An efficient data flow material model based cloud authentication data security and reduce a cloud storage cost using Index-level Boundary Pattern Convergent Encryption algorithm,” Mater. Today Proc., vol. 81, no. 2, pp. 931–936, 2021. 

[4]	X. Yang et al., “A Survey on Smart Agriculture: Development Modes, Technologies, and Security and Privacy Challenges,” IEEE/CAA J. Autom. Sin., vol. 8, no. 2, pp. 273–302, 2021. 

[5]	S. Ibrahim, S. B. A. Kamaruddin, A. Zabidi, and N. A. M. Ghani, “Contrastive analysis of rice grain classification techniques: Multi-class support vector machine vs artificial neural network,” IAES Int. J. Artif. Intell., vol. 9, no. 4, pp. 616–622, 2020. 

[6]	A. S. Hamzah and A. Mohamed, “Classification of white rice grain quality using ann: A review,” IAES Int. J. Artif. Intell., vol. 9, no. 4, pp. 600–608, 2020. 

[7]	MUH ZAINAL ALTIM, FAISAL, SALMIAH, KASMAN, ANDI YUDHISTIRA, and RITA AMALIA SYAMSU, “Pengklasifikasi Beras Menggunakan Metode Cnn (Convolutional Neural Network),” J. INSTEK (Informatika Sains dan Teknol., vol. 7, no. 1, pp. 151–155, 2022. 

[8]	P. S. Sampaio, A. S. Almeida, and C. M. Brites, “Use of artificial neural network model for rice quality prediction based on grain physical parameters,” Foods, vol. 10, no. 12, 2021. 

[9]	W. Xia, R. Peng, H. Chu, X. Zhu, Z. Yang, and ..., “An Overall Real-Time Mechanism for Classification and Quality Evaluation of Rice,” Available SSRN …. 

[10]	A. Bhattacharjee, K. R. Singh, T. S. Singh, S. Datta, U. R. Singh, and G. Thingbaijam, “INTELLIGENT SYSTEMS AND APPLICATIONS IN ENGINEERING A Comparative Study on Rice Grain Classification Using Convolutional Neural Network and Other Machine Learning Techniques,” pp. 0–1, 2024. 

[11]	T. T. H. Phan, Q. T. Vo, and H. Du Nguyen, “A novel method for identifying rice seed purity using hybrid machine learning algorithms,” Heliyon, vol. 10, no. 14, 2024. 

[12]	Y. Wang, H. Wang, and Z. Peng, “Rice diseases detection and classification using attention based neural network and bayesian optimization,” Expert Syst. Appl., vol. 178, 2021. 

[13]	Y. Haddad, K. Salonitis, and C. Emmanouilidis, “A decision-making framework for the design of local production networks under largescale disruptions,” Procedia Manuf., vol. 55, no. C, pp. 393–400, 2021. 

[14]	I. Samarakoon and P. Liyanage, “Impact of Data Analytics on Operations Success of Apparel Sector ABC Clothing Pvt Limited (Sri Lanka),” Int. J. Comput. Appl., vol. 184, no. 33, pp. 1–15, 2022. 

[15]	Q. W. Kong, J. He, Z. W. Zhang, H. Zheng, and P. Z. Wang, “Projection as a way of thinking to find factors in factor space,” Procedia Comput. Sci., vol. 199, pp. 503–508,

---
