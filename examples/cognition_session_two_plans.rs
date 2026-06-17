use gort::{run_cognition_session, CognitionPlan};

fn main() {
    let plan_a = CognitionPlan {
        goal: "Reach point X".to_string(),
        steps: vec![
            "Move forward".to_string(),
            "Turn left".to_string(),
            "Advance 3 units".to_string(),
        ],
    };

    let plan_b = CognitionPlan {
        goal: "Reach point X".to_string(),
        steps: vec![
            "Turn left".to_string(),
            "Move forward".to_string(),
            "Advance 3 units".to_string(),
        ],
    };

    let result = run_cognition_session(&plan_a, &plan_b).expect("cognition session");

    println!("winner = {}", result.winner_plan_id);
    println!("semantic_label = {:?}", result.semantic_label);
    println!("arbitration_signature = {}", result.arbitration_signature);
    println!("correction_signature = {}", result.correction_signature);
    println!("stabilization_signature = {}", result.stabilization_signature);
    println!("trajectory_kind = {:?}", result.trajectory_kind);
    println!("telemetry_digest = {}", result.telemetry_digest);

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
