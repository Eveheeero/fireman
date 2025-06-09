# ML Integration Blueprint for Fireman

## Practical ML/AI Integration for Production Decompilers

This blueprint shows how to integrate modern ML techniques into Fireman while maintaining performance and reliability.

## Architecture Overview

```rust
// Modular ML integration - can disable for pure traditional analysis
pub struct FiremanML {
    // Core ML components
    embedder: Option<CodeEmbedder>,
    pattern_matcher: Option<NeuralPatternMatcher>,
    type_predictor: Option<TypePredictor>,
    name_suggester: Option<NameSuggester>,
    
    // Configuration
    config: MLConfig,
}

pub struct MLConfig {
    pub use_embeddings: bool,
    pub use_local_models: bool,
    pub use_cloud_apis: bool,
    pub max_api_cost_per_binary: f64,
    pub cache_embeddings: bool,
}
```

## Phase 1: Embeddings for Code Similarity

### Implementation Plan

```rust
use ort::{Environment, SessionBuilder, Value};  // ONNX Runtime for local inference

pub struct CodeEmbedder {
    model: ort::Session,
    tokenizer: Tokenizer,
    cache: DashMap<u64, Vec<f32>>,
}

impl CodeEmbedder {
    pub fn new() -> Result<Self> {
        // Use ONNX for cross-platform compatibility
        let env = Environment::builder()
            .with_name("fireman-embedder")
            .build()?;
            
        let model = SessionBuilder::new(&env)?
            .with_model_from_file("models/codebert-asm.onnx")?;
            
        Ok(Self {
            model,
            tokenizer: Tokenizer::new(),
            cache: DashMap::new(),
        })
    }
    
    pub fn embed_function(&self, func: &Function) -> Result<Embedding> {
        // Check cache
        let hash = hash_function(func);
        if let Some(cached) = self.cache.get(&hash) {
            return Ok(Embedding::from_vec(cached.clone()));
        }
        
        // Convert assembly to tokens
        let tokens = self.tokenizer.tokenize_assembly(&func.instructions)?;
        
        // Run inference
        let input = Array2::from_shape_vec((1, tokens.len()), tokens)?;
        let outputs = self.model.run(vec![Value::from_array(input)?])?;
        
        let embedding = outputs[0].try_extract::<f32>()?.to_vec();
        
        // Cache result
        self.cache.insert(hash, embedding.clone());
        
        Ok(Embedding::from_vec(embedding))
    }
}
```

### Pattern Database with Embeddings

```rust
use hnsw::{Hnsw, Searcher};  // Hierarchical Navigable Small World for fast similarity

pub struct EmbeddingPatternDB {
    index: Hnsw<f32, DistCosine>,
    patterns: Vec<KnownPattern>,
    
    // Pre-computed embeddings for common libraries
    stdlib_embeddings: HashMap<String, Embedding>,
}

impl EmbeddingPatternDB {
    pub fn build() -> Result<Self> {
        let mut index = Hnsw::new(32, 768, 16, 200, DistCosine);
        let mut patterns = Vec::new();
        
        // Load pre-computed embeddings
        let stdlib_embeddings = Self::load_stdlib_embeddings()?;
        
        // Add to index
        for (name, embedding) in &stdlib_embeddings {
            let id = patterns.len();
            patterns.push(KnownPattern {
                name: name.clone(),
                library: detect_library(name),
                confidence: 1.0,
            });
            index.insert(embedding.as_slice(), id);
        }
        
        Ok(Self { index, patterns, stdlib_embeddings })
    }
    
    pub fn find_similar(&self, embedding: &Embedding, k: usize) -> Vec<Match> {
        let mut searcher = Searcher::default();
        let results = self.index.search(embedding.as_slice(), k, &mut searcher);
        
        results.into_iter()
            .map(|(id, distance)| Match {
                pattern: &self.patterns[id],
                similarity: 1.0 - distance,  // Convert distance to similarity
            })
            .collect()
    }
}
```

## Phase 2: Local SLM Integration

### Using Small Language Models Locally

```rust
use candle::{Device, Tensor};  // Rust-native ML framework

pub struct LocalLLM {
    model: CodeLlamaModel,
    device: Device,
    max_tokens: usize,
}

impl LocalLLM {
    pub fn new() -> Result<Self> {
        // Use quantized model for efficiency
        let device = Device::cuda_if_available(0)?;
        let model = CodeLlamaModel::load_quantized("models/codellama-7b-q4.bin", &device)?;
        
        Ok(Self {
            model,
            device,
            max_tokens: 512,
        })
    }
    
    pub fn improve_code(&self, decompiled: &str) -> Result<ImprovedCode> {
        let prompt = format!(
            "Improve this decompiled C code by adding meaningful variable names and comments:\n\n{}",
            decompiled
        );
        
        // Tokenize
        let tokens = self.model.tokenize(&prompt)?;
        
        // Generate
        let output = self.model.generate(
            &tokens,
            self.max_tokens,
            temperature: 0.3,  // Low temperature for consistency
            top_p: 0.9,
        )?;
        
        // Parse response
        Ok(self.parse_improvement(output))
    }
    
    pub fn suggest_names(&self, context: &NamingContext) -> Vec<NameSuggestion> {
        // Specialized prompt for variable naming
        let prompt = self.build_naming_prompt(context);
        
        let output = self.model.generate_quick(&prompt, max_tokens: 50)?;
        
        // Extract suggestions
        self.parse_name_suggestions(output)
    }
}
```

### Efficient Batching for SLM

```rust
pub struct BatchedLLM {
    llm: LocalLLM,
    pending: Arc<Mutex<Vec<LLMRequest>>>,
    results: Arc<DashMap<RequestId, LLMResult>>,
}

impl BatchedLLM {
    pub fn process_batch(&self) {
        loop {
            // Collect batch
            let batch = {
                let mut pending = self.pending.lock().unwrap();
                if pending.len() >= 8 || 
                   (pending.len() > 0 && pending[0].age() > Duration::from_millis(100)) {
                    pending.drain(..).collect::<Vec<_>>()
                } else {
                    continue;
                }
            };
            
            // Process batch efficiently
            let prompts: Vec<_> = batch.iter().map(|r| &r.prompt).collect();
            let results = self.llm.generate_batch(&prompts)?;
            
            // Store results
            for (req, result) in batch.into_iter().zip(results) {
                self.results.insert(req.id, result);
                req.notify.notify_one();
            }
        }
    }
}
```

## Phase 3: Smart Pattern Recognition

### ML-Enhanced Pattern Matching

```rust
pub struct NeuralPatternMatcher {
    // Traditional pattern matching
    exact_matcher: AhoCorasick,
    
    // ML for fuzzy matching
    pattern_embedder: PatternEmbedder,
    pattern_index: HnswIndex,
    
    // Confidence calibration
    calibrator: ConfidenceCalibrator,
}

impl NeuralPatternMatcher {
    pub fn match_patterns(&self, code: &[u8]) -> Vec<PatternMatch> {
        let mut matches = Vec::new();
        
        // Stage 1: Exact matches (fast)
        for m in self.exact_matcher.find_iter(code) {
            matches.push(PatternMatch {
                pattern_id: m.pattern(),
                offset: m.start(),
                length: m.len(),
                confidence: 1.0,
                method: MatchMethod::Exact,
            });
        }
        
        // Stage 2: Fuzzy matching for unmatched regions
        for region in self.find_unmatched_regions(code, &matches) {
            if region.len() < 20 {
                continue;  // Too small for meaningful embedding
            }
            
            // Embed region
            let embedding = self.pattern_embedder.embed(&code[region.clone()])?;
            
            // Find similar patterns
            let similar = self.pattern_index.search(&embedding, k=3);
            
            for (pattern_id, similarity) in similar {
                if similarity > 0.8 {
                    matches.push(PatternMatch {
                        pattern_id,
                        offset: region.start,
                        length: region.len(),
                        confidence: self.calibrator.calibrate(similarity),
                        method: MatchMethod::Fuzzy,
                    });
                }
            }
        }
        
        matches
    }
}
```

### Learning from User Feedback

```rust
pub struct AdaptivePatternLearner {
    feedback_db: sled::Db,
    model_updater: ModelUpdater,
}

impl AdaptivePatternLearner {
    pub fn record_feedback(&self, pattern: &Pattern, feedback: UserFeedback) {
        // Store feedback
        let key = pattern.id.to_be_bytes();
        let mut feedbacks: Vec<UserFeedback> = self.feedback_db
            .get(&key)?
            .map(|b| bincode::deserialize(&b).unwrap())
            .unwrap_or_default();
            
        feedbacks.push(feedback);
        self.feedback_db.insert(key, bincode::serialize(&feedbacks)?)?;
        
        // Trigger retraining if enough feedback
        if feedbacks.len() % 100 == 0 {
            self.model_updater.schedule_update(pattern.id);
        }
    }
    
    pub fn update_patterns(&self) {
        // Collect feedback data
        let feedback_data = self.collect_feedback_data();
        
        // Fine-tune embeddings based on user corrections
        let updated_embeddings = self.fine_tune_embeddings(feedback_data);
        
        // Update pattern database
        self.update_pattern_db(updated_embeddings);
    }
}
```

## Phase 4: Type Inference Enhancement

### ML-Assisted Type Recovery

```rust
pub struct MLTypeInference {
    // Traditional constraint solver
    constraint_solver: Z3Solver,
    
    // ML for ambiguous cases
    type_predictor: TypePredictor,
    
    // Confidence threshold
    confidence_threshold: f32,
}

impl MLTypeInference {
    pub fn infer_types(&self, function: &Function) -> TypeMap {
        // Step 1: Collect constraints
        let constraints = self.collect_constraints(function);
        
        // Step 2: Solve deterministic constraints
        let mut type_map = self.constraint_solver.solve(&constraints)?;
        
        // Step 3: Use ML for remaining unknowns
        for (var, _) in type_map.iter().filter(|(_, ty)| ty.is_unknown()) {
            let context = self.build_context(function, var);
            let prediction = self.type_predictor.predict(&context)?;
            
            if prediction.confidence > self.confidence_threshold {
                type_map.insert(var.clone(), prediction.predicted_type);
            }
        }
        
        type_map
    }
}

pub struct TypePredictor {
    model: ort::Session,
    feature_extractor: TypeFeatureExtractor,
}

impl TypePredictor {
    pub fn predict(&self, context: &TypeContext) -> TypePrediction {
        // Extract features
        let features = self.feature_extractor.extract(context);
        
        // Run inference
        let input = Value::from_array(features)?;
        let output = self.model.run(vec![input])?;
        
        // Parse prediction
        let probs = output[0].try_extract::<f32>()?;
        let (type_id, confidence) = probs.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
            
        TypePrediction {
            predicted_type: Type::from_id(type_id),
            confidence: *confidence,
            alternatives: self.get_alternatives(probs),
        }
    }
}
```

## Phase 5: API Integration (When Needed)

### Cost-Aware Cloud API Usage

```rust
pub struct CloudMLIntegration {
    clients: HashMap<Provider, Box<dyn MLProvider>>,
    budget_tracker: BudgetTracker,
    cache: ResponseCache,
}

impl CloudMLIntegration {
    pub async fn enhance_decompilation(&self, code: &str, budget: f64) -> Result<String> {
        // Check cache first
        let cache_key = hash(code);
        if let Some(cached) = self.cache.get(&cache_key) {
            return Ok(cached);
        }
        
        // Check budget
        if !self.budget_tracker.can_spend(budget) {
            return Err(Error::BudgetExceeded);
        }
        
        // Select provider based on cost/quality tradeoff
        let provider = self.select_provider(code.len(), budget);
        
        // Make request
        let enhanced = provider.enhance_code(code).await?;
        
        // Track spending
        self.budget_tracker.record_expense(provider.cost_for(code.len()));
        
        // Cache result
        self.cache.insert(cache_key, enhanced.clone());
        
        Ok(enhanced)
    }
}
```

## Implementation Roadmap

### Phase 1: Foundation (Months 1-2)

- [ ] Integrate ONNX Runtime for local inference
- [ ] Build pattern embedding system
- [ ] Create similarity search infrastructure
- [ ] Add caching layer

### Phase 2: Local Models (Months 3-4)

- [ ] Integrate quantized CodeLlama model
- [ ] Implement batching system
- [ ] Add variable name suggestion
- [ ] Create feedback collection

### Phase 3: Production Features (Months 5-6)

- [ ] Add confidence calibration
- [ ] Implement cost tracking
- [ ] Create model update pipeline
- [ ] Add A/B testing framework

### Phase 4: Advanced Features (Months 7-8)

- [ ] Multi-modal analysis (CFG + assembly)
- [ ] Custom model training pipeline
- [ ] Distributed inference
- [ ] Real-time learning

## Performance Considerations

```rust
// Benchmark different approaches
#[cfg(test)]
mod benchmarks {
    use criterion::{black_box, criterion_group, Criterion};
    
    fn bench_embedding_speed(c: &mut Criterion) {
        let embedder = CodeEmbedder::new().unwrap();
        let function = load_test_function();
        
        c.bench_function("embed_function", |b| {
            b.iter(|| embedder.embed_function(black_box(&function)))
        });
    }
    
    fn bench_pattern_matching(c: &mut Criterion) {
        let matcher = NeuralPatternMatcher::new().unwrap();
        let code = vec![0u8; 1024];
        
        c.bench_function("neural_pattern_match", |b| {
            b.iter(|| matcher.match_patterns(black_box(&code)))
        });
    }
}
```

## Monitoring and Metrics

```rust
pub struct MLMetrics {
    // Performance metrics
    inference_latency: Histogram,
    cache_hit_rate: Counter,
    
    // Quality metrics
    user_acceptance_rate: Gauge,
    pattern_match_accuracy: Histogram,
    
    // Cost metrics
    api_calls_per_day: Counter,
    total_api_cost: Counter,
}

impl MLMetrics {
    pub fn record_inference(&self, start: Instant, hit_cache: bool) {
        self.inference_latency.observe(start.elapsed().as_secs_f64());
        if hit_cache {
            self.cache_hit_rate.inc();
        }
    }
}
```

## Best Practices

1. **Always provide fallbacks**: ML can fail - traditional analysis must work
2. **Cache aggressively**: Embeddings and predictions are expensive
3. **Monitor costs**: Cloud APIs can surprise you
4. **Version models**: Track which model produced which result
5. **Collect feedback**: Improve models with real user data
6. **Profile everything**: ML can be a performance bottleneck
7. **Test determinism**: Same input should give same output

## Conclusion

This blueprint provides a practical path to integrate ML into Fireman while maintaining the reliability and performance
users expect from a production decompiler.