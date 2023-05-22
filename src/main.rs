use opentelemetry_api::{global, Context, KeyValue};
use opentelemetry_prometheus::exporter;
use opentelemetry_sdk::export::metrics::aggregation;
use opentelemetry_sdk::metrics::{controllers, processors, selectors};
use std::thread::spawn;

pub(crate) const HISTOGRAM_BUCKETS: [f64; 14] = [
    0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
];

fn main() {
    let controller = controllers::basic(processors::factory(
        selectors::simple::histogram(HISTOGRAM_BUCKETS),
        aggregation::cumulative_temporality_selector(),
    ))
    .build();

    let prometheus_exporter = exporter(controller).init();

    let mut handles = vec![];
    let count = 10;
    for _i in 0..count {
        handles.push(spawn(|| {
            global::meter("").u64_counter("my_counter").init().add(
                &Context::current(),
                1,
                &[KeyValue::new("key", "value"), KeyValue::new("foo", "bar")],
            );
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Count should be: {}", count);

    let metrics = prometheus_exporter.registry().gather();
    let metric = metrics
        .iter()
        .find(|m| m.get_name() == "my_counter_total")
        .unwrap();
    println!(
        "Count is: {}",
        metric.get_metric()[0].get_counter().get_value()
    );
}
