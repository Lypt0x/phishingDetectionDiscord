use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};

pub struct Metrics {
    registry: Registry,
    
    unhealthy: Counter,
    healthy: Counter,
    cached: Counter
}

pub enum MetricType {
    Unhealthy,
    Healthy,
    Cached
}

impl Metrics {
    pub fn new() -> Self {
        let mut metrics = Self {
            registry: Registry::new(),
            unhealthy: self::counter("unhealthy", "harmful links submitted"),
            healthy: self::counter("healthy", "healthy links submitted"),
            cached: self::counter("cached", "cached links submitted")
        };

        
        let registry = &mut metrics.registry;
        self::register(registry, &[&metrics.unhealthy, &metrics.healthy, &metrics.cached]);

        metrics
    }

    pub fn increment(&mut self, metric_type: MetricType) {
        match metric_type {
            MetricType::Cached => self.cached.inc(),
            MetricType::Healthy => self.healthy.inc(),
            MetricType::Unhealthy => self.unhealthy.inc()
        }
    }

    pub fn gather(&self) -> Vec<u8> {
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        
        encoder.encode(&metric_families, &mut buffer).expect("encode");
        buffer
    }

}

fn counter(name: &str, help: &str) -> Counter {
    Counter::with_opts(Opts::new(name, help)).expect("counter")
}

fn register(registry: &mut Registry, counters: &[&Counter]) {
    for counter in counters.into_iter() {
        let counter = *counter;
        registry.register(Box::new(counter.clone())).expect("register");
    }
}
