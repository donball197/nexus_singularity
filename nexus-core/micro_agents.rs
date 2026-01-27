use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel;
use ort::value::Value;
use anyhow::Result;

pub struct MicroAgent {
    pub session: Session,
}

impl MicroAgent {
    pub fn init() -> Result<Self> {
        let _ = ort::init()
            .with_name("nexus_sovereign")
            .commit();

        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(1)?
            .commit_from_file("models/model.onnx")?;

        Ok(MicroAgent { session })
    }

    pub fn prepare_and_run(&mut self, ids: Vec<i64>, seq_len: usize) -> Result<Vec<f32>> {
        let input_ids = Value::from_array(([1, seq_len], ids.clone()))?;
        let token_type_ids = Value::from_array(([1, seq_len], vec![0i64; seq_len]))?;
        let attention_mask = Value::from_array(([1, seq_len], vec![1i64; seq_len]))?;

        let outputs = self.session.run(ort::inputs![
            "input_ids" => input_ids,
            "token_type_ids" => token_type_ids,
            "attention_mask" => attention_mask
        ])?;

        let (_shape, data) = outputs[0].try_extract_tensor::<f32>()?;
        Ok(data.to_vec())
    }
}
