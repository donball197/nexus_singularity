use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

pub struct CapabilityMonitor {
    pub active_tokens: AtomicUsize,
    pub total_invocations: AtomicUsize,
    pub current_index: AtomicUsize,
    pub current_confidence: AtomicUsize,
    pub distribution: RwLock<Vec<f32>>,
    pub throughput_history: RwLock<Vec<usize>>,
    pub confidence_window: RwLock<Vec<f64>>, 
}

impl CapabilityMonitor {
    pub fn new() -> Self {
        Self {
            active_tokens: AtomicUsize::new(0),
            total_invocations: AtomicUsize::new(0),
            current_index: AtomicUsize::new(0),
            current_confidence: AtomicUsize::new(0),
            distribution: RwLock::new(vec![0.0; 28]),
            throughput_history: RwLock::new(vec![0; 50]),
            confidence_window: RwLock::new(Vec::with_capacity(100)),
        }
    }

    pub fn update_history(&self, val: usize) {
        if let Ok(mut history) = self.throughput_history.write() {
            history.remove(0);
            history.push(val);
        }
    }

    pub fn update_confidence_trend(&self, conf: f64) -> f64 {
        let mut window = self.confidence_window.write().unwrap();
        if window.len() >= 100 { window.remove(0); }
        window.push(conf);
        window.iter().sum::<f64>() / window.len() as f64
    }

    pub fn is_alarm_active(&self) -> bool {
        self.current_confidence.load(Ordering::SeqCst) < 5000
    }

    pub fn get_waveform_html(&self) -> String {
        let history = self.throughput_history.read().unwrap();
        history.iter().map(|&v| {
            let height = (v as f32 / 128.0 * 40.0).min(40.0) as u8;
            format!("<div style='background:#0f0; width:4px; height:{}px; display:inline-block; margin-right:1px; vertical-align:bottom;'></div>", height)
        }).collect::<Vec<_>>().join("")
    }

    pub fn get_heatmap_html(&self) -> String {
        let dist = self.distribution.read().unwrap();
        dist.iter().enumerate().map(|(i, &p)| {
            let intensity = (p * 255.0) as u8;
            format!("<div style='background:rgb(0,{},0); width:18px; height:18px; display:inline-block; margin:1px;' title='Idx {}: {:.2}%'></div>", intensity, i, p * 100.0)
        }).collect::<Vec<_>>().join("")
    }

    pub fn get_state_label(&self, index: usize) -> &str {
        match index {
            27 => "SYSTEM_IDLE (STANDBY)",
            18 => "ACTIVE_ANALYSIS (DATA_LOCKED)",
            _  => "UNKNOWN_TRANSITION",
        }
    }
}
