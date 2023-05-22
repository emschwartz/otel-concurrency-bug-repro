This demonstrates a concurrency bug in the `opentelemetry_sdk` crate in version 0.19.0.

To reproduce the bug, you may need to run the main function of this library a few times. However, you should be able to see a run where the expected count value is less than the number of threads spawned that attempt to increment the counter.
