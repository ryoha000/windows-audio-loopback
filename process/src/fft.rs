use std::collections::VecDeque;

use rustfft::{num_complex::Complex32, FftPlanner};

use plotters::prelude::*;

pub struct Freq {
    pub RecTimes: Vec<i32>,
    pub PCMs: Vec<VecDeque<f32>>,
}

pub fn aaa() {
    let freq = 48000;
    let window_size = freq / 1000 * 5; // 5ms
    let hop_size = freq / 1000 * 1; // 1ms

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1);
    let mut buffer = vec![Complex32::new(0.0, 0.0); window_size];
    fft.process(&mut buffer);
}

pub fn kakko_kari(pcms: Vec<Vec<f32>>) {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(pcms[0].len());
    for (chan, pcm) in pcms.iter().enumerate() {
        let mut buffer = pcm
            .into_iter()
            .map(move |v| Complex32::new(*v, 0.0))
            .collect::<Vec<Complex32>>();
        fft.process(&mut buffer);

        let x_freq = (0..buffer.len()).collect::<Vec<usize>>();
        let y_db = buffer
            .iter()
            .map(|v| 20.0 * (v.norm_sqr() / (buffer.len() as f32).sqrt()).log10())
            .collect::<Vec<f32>>();

        let image_width = 1080;
        let image_height = 720;
        let filename = format!("plot{}.png", chan);
        // 描画先を指定。画像出力する場合はBitMapBackend
        let root = BitMapBackend::new(&filename, (image_width, image_height)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let caption = "Sample Plot";
        let font = ("sans-serif", 20);

        let (y_min, y_max) = y_db
            .iter()
            .fold((0.0 / 0.0, 0.0 / 0.0), |(m, n), v| (v.min(m), v.max(n)));
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, font.into_font()) // キャプションのフォントやサイズ
            .margin(10) // 上下左右全ての余白
            .x_label_area_size(16) // x軸ラベル部分の余白
            .y_label_area_size(42) // y軸ラベル部分の余白
            .build_cartesian_2d(
                // x軸とy軸の数値の範囲を指定する
                *x_freq.first().unwrap()..*x_freq.last().unwrap(), // x軸の範囲
                y_min..y_max,                                      // y軸の範囲
            )
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        // 折れ線グラフの定義＆描画
        let line_series =
            LineSeries::new(x_freq.iter().zip(y_db.iter()).map(|(x, y)| (*x, *y)), &RED);
        chart.draw_series(line_series).unwrap();
    }
}