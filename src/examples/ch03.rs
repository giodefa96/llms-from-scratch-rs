use crate::Example;

/// Example 02.01
pub struct EG01 {}

impl Example for EG01 {
    fn description(&self) -> String {
        String::from("Computing attention scores as a dot product.")
    }

    fn page_source(&self) -> usize {
        57_usize
    }

    fn main(&self) {
        use candle_core::{Device, Tensor};

        let dev = Device::cuda_if_available(0).unwrap();
        let inputs = Tensor::new(
            &[
                [0.43_f32, 0.15, 0.89], // Your
                [0.55, 0.87, 0.66],     // journey
                [0.57, 0.85, 0.64],     // starts
                [0.22, 0.58, 0.33],     // with
                [0.77, 0.25, 0.10],     // one
                [0.05, 0.80, 0.55],     // step
            ],
            &dev,
        )
        .unwrap();

        let query = inputs
            .index_select(&Tensor::new(&[1u32], &dev).unwrap(), 0)
            .unwrap();

        // compute attention scores
        let mut optional_attn_scores_2: Option<Tensor> = None;
        for i in 0..inputs.dims()[0] {
            let x_i = inputs
                .index_select(&Tensor::new(&[i as u32], &dev).unwrap(), 0)
                .unwrap();
            let a_i = x_i
                .matmul(&query.t().unwrap())
                .unwrap()
                .flatten_all()
                .unwrap();
            optional_attn_scores_2 = match optional_attn_scores_2 {
                Some(attn_scores_2) => Some(Tensor::cat(&[&attn_scores_2, &a_i], 0).unwrap()),
                None => Some(a_i),
            }
        }

        if let Some(attn_scores_2) = optional_attn_scores_2 {
            // raw attention scores
            println!("Raw attention scores: {:?}", attn_scores_2);

            // basic normalization
            let sum = attn_scores_2.sum_all().unwrap();
            let normalized_attn_scores = (attn_scores_2.broadcast_div(&sum))
                .unwrap()
                .to_vec1::<f32>();
            println!("Normalized attention scores: {:?}", normalized_attn_scores);

            // softmax normalization
            let exponentiator = attn_scores_2.exp().unwrap();
            let exponentiator_sum = exponentiator.sum_all().unwrap();
            let softmax_attn_scores = exponentiator.broadcast_div(&exponentiator_sum).unwrap();
            println!(
                "Softmax-normalized attention scores: {:?}",
                softmax_attn_scores
            );
        }
    }
}
