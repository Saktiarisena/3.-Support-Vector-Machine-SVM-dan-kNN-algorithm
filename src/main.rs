use linfa::prelude::*;
use linfa_svm::Svm;
use ndarray::{Array, Array2};
use std::error::Error;
use std::io::Write;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::time::Duration;
use std::thread;
use linfa_clustering::KMeans;
use plotters::prelude::*;
use calamine::{open_workbook, Reader, Xlsx};

fn main() -> Result<(), Box<dyn Error>> {
    // Baca dataset dari file Excel
    let mut workbook: Xlsx<_> = open_workbook("Rice_MSC_Dataset_sample.xlsx")?;
    let range = workbook.worksheet_range("Sheet1")
        .ok_or("Sheet 'Sheet1' tidak ditemukan")??;

    let mut records = Vec::new();

    println!("=== Data Asli ===");
    println!("No\tASPECT_RATIO (Fitur)\tCOMPACTNESS (Label)");
    println!("------------------------------------------------");

    for (i, row) in range.rows().skip(1).enumerate() { // Skip header
        let aspect_ratio: f64 = row[1].to_string().parse()?;
        let compactness: f64 = row[3].to_string().parse()?;
        println!("{}\t{:.4}\t\t\t{:.4}", i + 1, aspect_ratio, compactness);
        records.push((aspect_ratio, compactness));
    }

    // Konversi ke Array2
    let mut features = Array2::zeros((records.len(), 1));
    let mut labels = Array::zeros(records.len());

    for (i, (aspect_ratio, compactness)) in records.iter().enumerate() {
        features[[i, 0]] = *aspect_ratio;
        labels[i] = *compactness;
    }

    // Bagi dataset menjadi training dan testing
    let (train, test) = linfa::dataset::Dataset::new(features, labels.clone()).split_with_ratio(0.2);

    // SVM
    println!("\n=== Training SVM ===");
    let svm = Svm::params()
        .linear_kernel()        
        .fit(&train)?;

    let svm_pred = svm.predict(&test);
    
    println!("\n=== Hasil Prediksi SVM ===");
    println!("Data Test\tPrediksi");
    println!("--------------------------");
    for (i, pred) in svm_pred.iter().enumerate() {
        let x = test.records()[[i, 0]];
        println!("{:.4}\t\t{:.4}", x, pred);
    }

    // K-Means
    println!("\n=== Training K-Means ===");
    let running_knn = Arc::new(AtomicBool::new(true));
    let running_knn_clone = running_knn.clone();

    let handle_knn = thread::spawn(move || {
        let mut dots = 0;
        while running_knn_clone.load(Ordering::Relaxed) {
            print!("\rTraining K-Means{}   ", ".".repeat(dots));
            dots = (dots + 1) % 4;
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(500));
        }
        println!("\rTraining K-Means selesai!        ");
    });

    let knn_model = KMeans::params(3)
        .max_n_iterations(100)
        .fit(&train)?;

    running_knn.store(false, Ordering::Relaxed);
    handle_knn.join().unwrap();

    let knn_pred = knn_model.predict(&test);
    
    println!("\n=== Hasil Prediksi K-Means ===");
    println!("Data Test\tCluster");
    println!("--------------------------");
    for (i, &cluster) in knn_pred.iter().enumerate() {
        let x = test.records()[[i, 0]];
        println!("{:.4}\t\t{}", x, cluster);
    }

    // Buat grafik
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Hasil Prediksi Model", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..10f64, 0f64..1f64)?;

    chart.configure_mesh()
        .x_desc("ASPECT_RATIO")
        .y_desc("COMPACTNESS")
        .draw()?;

    // Plot data asli
    chart.draw_series(
        records.iter().map(|(x, y)| Circle::new((*x, *y), 3, BLUE.filled())),
    )?.label("Data Asli")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE.filled()));

    // Plot prediksi SVM
    chart.draw_series(
        test.records().outer_iter().zip(svm_pred.iter()).map(|(x, y)| {
            Circle::new((x[0], *y), 3, RED.filled())
        })
    )?.label("Prediksi SVM")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));

    // Plot prediksi K-Means
    let cluster_colors = [GREEN, MAGENTA, CYAN];
    chart.draw_series(
        test.records().outer_iter().zip(knn_pred.iter()).map(|(x, &cluster)| {
            TriangleMarker::new((x[0], labels[x[0] as usize]), 8, cluster_colors[cluster].filled())
        })
    )?.label("Prediksi K-Means")
      .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN.filled()));

    chart.configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    println!("\nGrafik telah disimpan sebagai plot.png");

    Ok(())
}