use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, AxisScale, PlotConfiguration};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    use implementation::FogNetwork::{Instance::*, Nodes::{FGN, AvailableFCN, StatusMetricParameter}};
    use implementation::User::{Application, ExpectationMetricParameter};
    use implementation::Constants;
    let mut rng = rand::thread_rng();

    let mut new_instance = black_box(Instance::new_empty(
        vec![
           Parameter::new(
                String::from("RoundTripTime"),
                1.0f32,
                false,
            ),
            Parameter::new(
                String::from("ResourceAvailability"),
                1.0f32,
                true,
            ),
            Parameter::new(
                String::from("ProcessingSpeed"),
                1.0f32,
                true,
            ),
        ],
    ));

    // let test_fgn = black_box(new_instance.get_FGN(String::from("Test_FGN_2")).unwrap());
    // new_instance.add_FGN(
    //     FGN::new(String::from("Test_FGN_1"))
    // );

    for n in 3..=50 {
        let mut fgn = black_box(FGN::new(String::from(n.to_string())));
        for i in 0..n {
            let _ = black_box(Application::new(
                i.to_string(),
                vec![
                    ExpectationMetricParameter::new(
                        Constants::Application::ExpectationMetricParameterType::AccessRate,
                        rng.gen_range::<f32,_>(2.0..=10.0).floor()),
                    ExpectationMetricParameter::new(
                        Constants::Application::ExpectationMetricParameterType::RequiredResources,
                        rng.gen_range::<f32,_>(1.0..=8.0).floor()),
                    ExpectationMetricParameter::new(
                        Constants::Application::ExpectationMetricParameterType::ProcessingTime,
                        rng.gen_range::<f32,_>(3.0..=12.0).floor()*10.0),
                ],
            ).send_placement_request(&mut fgn));

            let _ = black_box(fgn.add_available_fcn(
                AvailableFCN {
                    id: i.to_string(),
                    params: vec![
                        StatusMetricParameter::new(
                            Constants::FCN::StatusMetricParameterType::RoundTripTime,
                            rng.gen_range::<f32,_>(1.0..=6.0).floor()*100.0),
                        StatusMetricParameter::new(
                            Constants::FCN::StatusMetricParameterType::ResourceAvailability,
                            rng.gen_range::<f32,_>(1.0..=10.0).floor()),
                        StatusMetricParameter::new(
                            Constants::FCN::StatusMetricParameterType::ProcessingSpeed,
                            rng.gen_range::<f32,_>(1.0..=7.0).floor()*10.0),
                    ]
                }
            ));
        }

        let _ = new_instance.add_FGN(fgn);
    }

    let mut group = c.benchmark_group("time_complexity");
    group.significance_level(0.1).sample_size(1000);
    for n in 3..=50 {
        group.bench_with_input(BenchmarkId::new("no_lookup", n), &n, |b, &n| {
            b.iter(|| black_box(new_instance.get_FGN(n.to_string()).unwrap()).application_placement())
        });
        group.bench_with_input(BenchmarkId::new("with_lookup", n), &n, |b, &n| {
            b.iter(|| black_box(new_instance.get_FGN(n.to_string()).unwrap()).application_placement_with_lookup())
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
