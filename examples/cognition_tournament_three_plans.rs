use gort::{run_cognition_tournament, CognitionPlan};

fn main() {
    let plans = vec![
        CognitionPlan {
            goal: "Deliver package".to_string(),
            steps: vec![
                "Pick up package".to_string(),
                "Drive north".to_string(),
                "Drop off package".to_string(),
            ],
        },
        CognitionPlan {
            goal: "Deliver package".to_string(),
            steps: vec![
                "Drive north".to_string(),
                "Pick up package".to_string(),
                "Drop off package".to_string(),
            ],
        },
        CognitionPlan {
            goal: "Deliver package".to_string(),
            steps: vec![
                "Pick up package".to_string(),
                "Check fuel".to_string(),
                "Drive north".to_string(),
                "Drop off package".to_string(),
            ],
        },
    ];

    let result = run_cognition_tournament(&plans).expect("cognition tournament");

    println!("winner = {}", result.winner_plan_id);
    println!("semantic_label = {:?}", result.semantic_label);
    println!("arbitration_signature = {}", result.arbitration_signature);
    println!("correction_signature = {}", result.correction_signature);
    println!("stabilization_signature = {}", result.stabilization_signature);
    println!("trajectory_kind = {:?}", result.trajectory_kind);
    println!("telemetry_digest = {}", result.telemetry_digest);
    println!("candidate_count = {}", result.candidate_count);

    for trace in &result.plan_traces {
        println!(
            "plan={} phase16={} phase17={} phase18={}",
            trace.plan_id,
            trace.phase16_trajectory_signature,
            trace.phase17_semantic_signature,
            trace.phase18_inference_signature,
        );
    }
}
