pub struct Cycle {
    f2i: f64,
    buffer: Vec<f64>,
    phasei: u32,
}

impl Cycle {
    pub fn new() -> Self {
        // 16384ポイントのテーブルを作成
        let mut buffer = Vec::with_capacity(16384);

        // サイン波のルックアップテーブルを生成
        for i in 0..16384 {
            let phase = 2.0 * std::f64::consts::PI * (i as f64) / 16384.0;
            buffer.push(phase.sin());
        }

        Self {
            f2i: 4294967296.0 / 48000.0, // サンプリングレート48kHzの場合
            buffer,
            phasei: 0,
        }
    }

    pub fn perform(
        &mut self,
        _frequency: f64,
        _phase_offset: f64,
        signal_out: &mut [f32],
        phase_out: &mut [f32],
        n: usize,
    ) {
        for i in 0..n {
            let uint_phase = self.phasei;

            // インデックスと補間係数の計算
            // 位相からインデックスへの変換（32-18 = 14ビット 上位14ビットを使用）
            let idx = (uint_phase >> 18) as usize;
            // 補間用の小数部（下位18ビット）
            let frac = (uint_phase & 0x3FFFF) as f64 * 3.81471181759574e-6;

            // 線形補間
            let y0 = self.buffer[idx];
            // インデックスの範囲を0-16383に制限（高速なビットマスク）
            let y1 = self.buffer[((idx + 1) & 16383) as usize];
            let y = y0 + frac * (y1 - y0);

            // 位相の更新
            let pincr = (440.0 * self.f2i) as u32;
            self.phasei = self.phasei.wrapping_add(pincr);

            // 出力の設定
            signal_out[i] = y as f32;
            phase_out[i] = (uint_phase as f64 * 0.232830643653869629e-9) as f32;
        }
    }
}
