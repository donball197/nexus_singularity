pub fn get_hud_html(is_stress: bool, avg_conf: f64, heatmap: String) -> String {
    let bg_base = if is_stress { "#900" } else { "#112" };
    let pulse_speed = if is_stress { "0.1" } else { "1.5" };
    
    format!(
        "<html><head><meta http-equiv='refresh' content='1'><style>
        body {{ font-family:monospace; background:{}; color:#0f0; padding:20px; transition: background 0.1s; animation: heartbeat {}s infinite alternate; }}
        @keyframes heartbeat {{ from {{ filter: brightness(1); }} to {{ filter: brightness(2); }} }}
        </style></head><body>
        <h2>NEXUS HUD: MODULAR SENSORY</h2>
        <div style='background:#111; padding:10px; border:2px solid #0f0;'>{}</div><br>
        <b>Stability Index:</b> {:.2}%
        <script>window.navigator.vibrate(200);</script>
        </body></html>",
        bg_base, pulse_speed, heatmap, avg_conf
    )
}
