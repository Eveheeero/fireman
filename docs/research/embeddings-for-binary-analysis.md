# Code Embeddings for Binary Analysis: A Practical Guide

## Introduction

Code embeddings transform binary code into dense vector representations that capture semantic meaning. This guide shows
how to leverage embeddings for practical decompilation tasks.

## Why Embeddings Work for Binary Code

### The Key Insight

Assembly instructions, like natural language, have:

- **Context**: Instructions gain meaning from surrounding code
- **Patterns**: Common sequences appear across programs
- **Semantics**: Different sequences can have same meaning

```python
# Example: Different assembly, same semantics
# Pattern 1: Clear register
xor eax, eax

# Pattern 2: Also clears register  
mov eax, 0

# Pattern 3: Also clears register
sub eax, eax

# All three should have similar embeddings!
```

## Building Practical Embedding Systems

### 1. Assembly-to-Vector Pipeline

```python
import torch
from transformers import AutoTokenizer, AutoModel
import numpy as np

class AssemblyEmbedder:
    def __init__(self, model_name="microsoft/codebert-base"):
        self.tokenizer = AutoTokenizer.from_pretrained(model_name)
        self.model = AutoModel.from_pretrained(model_name)
        self.model.eval()
        
    def embed_assembly(self, assembly_code):
        """Convert assembly to embedding vector"""
        # Tokenize
        inputs = self.tokenizer(
            assembly_code,
            return_tensors="pt",
            max_length=512,
            truncation=True,
            padding=True
        )
        
        # Get embeddings
        with torch.no_grad():
            outputs = self.model(**inputs)
            # Use CLS token embedding
            embedding = outputs.last_hidden_state[:, 0, :].numpy()
        
        return embedding[0]  # Shape: (768,)
    
    def embed_function(self, instructions):
        """Embed entire function"""
        # Convert instructions to text
        asm_text = '\n'.join(str(inst) for inst in instructions)
        return self.embed_assembly(asm_text)
```

### 2. Training Custom Embeddings

```python
# Dataset preparation for assembly embeddings
class AssemblyDataset:
    def __init__(self):
        self.pairs = []
    
    def add_equivalent_pair(self, asm1, asm2):
        """Add functionally equivalent assembly"""
        self.pairs.append((asm1, asm2, 1.0))  # Similar
    
    def add_different_pair(self, asm1, asm2):
        """Add functionally different assembly"""
        self.pairs.append((asm1, asm2, 0.0))  # Different
    
    def create_training_data(self):
        """Generate training data from compiled binaries"""
        # Compile same function with different optimizations
        source = "int add(int a, int b) { return a + b; }"
        
        asm_o0 = compile_and_disasm(source, "-O0")
        asm_o2 = compile_and_disasm(source, "-O2") 
        asm_o3 = compile_and_disasm(source, "-O3")
        
        # These should be similar
        self.add_equivalent_pair(asm_o0, asm_o2)
        self.add_equivalent_pair(asm_o2, asm_o3)
        
        # Different function - should be different
        other = "int mul(int a, int b) { return a * b; }"
        asm_other = compile_and_disasm(other, "-O2")
        self.add_different_pair(asm_o2, asm_other)
```

### 3. Contrastive Learning for Binary Code

```python
import torch.nn.functional as F

class ContrastiveAssemblyModel(torch.nn.Module):
    def __init__(self, base_model):
        super().__init__()
        self.encoder = base_model
        self.projection = torch.nn.Linear(768, 256)
        
    def forward(self, anchor, positive, negative):
        # Encode all inputs
        anchor_emb = self.projection(self.encoder(anchor))
        pos_emb = self.projection(self.encoder(positive))
        neg_emb = self.projection(self.encoder(negative))
        
        # Normalize
        anchor_emb = F.normalize(anchor_emb, p=2, dim=1)
        pos_emb = F.normalize(pos_emb, p=2, dim=1)
        neg_emb = F.normalize(neg_emb, p=2, dim=1)
        
        return anchor_emb, pos_emb, neg_emb
    
    def triplet_loss(self, anchor, positive, negative, margin=0.5):
        """Contrastive loss for learning similarities"""
        pos_dist = F.pairwise_distance(anchor, positive)
        neg_dist = F.pairwise_distance(anchor, negative)
        loss = F.relu(pos_dist - neg_dist + margin)
        return loss.mean()
```

## Practical Applications

### 1. Function Similarity Search

```python
import faiss
import numpy as np

class FunctionSearchEngine:
    def __init__(self, embedding_dim=768):
        # Use efficient similarity search
        self.index = faiss.IndexFlatIP(embedding_dim)  # Inner product
        self.embedder = AssemblyEmbedder()
        self.function_db = []
        
    def add_functions(self, functions):
        """Add functions to searchable index"""
        embeddings = []
        
        for func in functions:
            # Embed function
            emb = self.embedder.embed_function(func['assembly'])
            embeddings.append(emb)
            
            # Store metadata
            self.function_db.append({
                'name': func['name'],
                'library': func['library'],
                'source': func.get('source', None)
            })
        
        # Add to index
        embeddings = np.array(embeddings).astype('float32')
        faiss.normalize_L2(embeddings)  # Normalize for cosine similarity
        self.index.add(embeddings)
    
    def search(self, query_function, k=5):
        """Find similar functions"""
        # Embed query
        query_emb = self.embedder.embed_function(query_function)
        query_emb = np.array([query_emb]).astype('float32')
        faiss.normalize_L2(query_emb)
        
        # Search
        scores, indices = self.index.search(query_emb, k)
        
        # Return results with metadata
        results = []
        for idx, score in zip(indices[0], scores[0]):
            func = self.function_db[idx]
            results.append({
                **func,
                'similarity': float(score),
                'confidence': self._score_to_confidence(score)
            })
        
        return results
    
    def _score_to_confidence(self, score):
        """Convert similarity score to confidence"""
        # Empirically determined thresholds
        if score > 0.95:
            return "very_high"
        elif score > 0.85:
            return "high"
        elif score > 0.70:
            return "medium"
        else:
            return "low"
```

### 2. Library Function Identification

```python
class LibraryIdentifier:
    def __init__(self):
        self.search_engine = FunctionSearchEngine()
        self._build_library_database()
    
    def _build_library_database(self):
        """Build database of known library functions"""
        libraries = [
            ('libc', '/usr/lib/libc.so'),
            ('libssl', '/usr/lib/libssl.so'),
            ('libz', '/usr/lib/libz.so'),
        ]
        
        for lib_name, lib_path in libraries:
            functions = extract_functions(lib_path)
            
            # Add with metadata
            lib_functions = [{
                'assembly': f['asm'],
                'name': f['name'],
                'library': lib_name,
            } for f in functions]
            
            self.search_engine.add_functions(lib_functions)
    
    def identify(self, unknown_function):
        """Identify if function is from known library"""
        results = self.search_engine.search(unknown_function, k=3)
        
        # High confidence match?
        if results[0]['similarity'] > 0.9:
            return {
                'identified': True,
                'library': results[0]['library'],
                'function': results[0]['name'],
                'confidence': results[0]['confidence']
            }
        
        return {'identified': False, 'candidates': results}
```

### 3. Clustering Similar Code

```python
from sklearn.cluster import DBSCAN
import numpy as np

class CodeClusterer:
    def __init__(self, embedder):
        self.embedder = embedder
        
    def cluster_functions(self, functions, eps=0.3):
        """Group similar functions together"""
        # Embed all functions
        embeddings = []
        for func in functions:
            emb = self.embedder.embed_function(func['assembly'])
            embeddings.append(emb)
        
        embeddings = np.array(embeddings)
        
        # Cluster using DBSCAN
        clustering = DBSCAN(eps=eps, min_samples=2, metric='cosine')
        labels = clustering.fit_predict(embeddings)
        
        # Group results
        clusters = {}
        for idx, label in enumerate(labels):
            if label not in clusters:
                clusters[label] = []
            clusters[label].append(functions[idx])
        
        return clusters
    
    def find_code_families(self, malware_samples):
        """Identify malware families through clustering"""
        clusters = self.cluster_functions(malware_samples)
        
        families = []
        for label, samples in clusters.items():
            if label == -1:  # Noise
                continue
                
            # Analyze cluster characteristics
            family = {
                'id': f'family_{label}',
                'samples': samples,
                'size': len(samples),
                'characteristic': self._extract_characteristics(samples)
            }
            families.append(family)
        
        return families
```

### 4. Zero-Shot Algorithm Recognition

```python
class AlgorithmRecognizer:
    def __init__(self):
        self.embedder = AssemblyEmbedder()
        self.algorithm_embeddings = self._create_algorithm_embeddings()
    
    def _create_algorithm_embeddings(self):
        """Pre-compute embeddings for known algorithms"""
        algorithms = {
            'quicksort': self._get_quicksort_embedding(),
            'binary_search': self._get_binary_search_embedding(),
            'bubble_sort': self._get_bubble_sort_embedding(),
            'crc32': self._get_crc32_embedding(),
            'aes': self._get_aes_embedding(),
        }
        return algorithms
    
    def recognize(self, assembly_code):
        """Identify algorithm without training"""
        # Embed unknown code
        unknown_emb = self.embedder.embed_assembly(assembly_code)
        
        # Compare with known algorithms
        scores = {}
        for algo_name, algo_emb in self.algorithm_embeddings.items():
            similarity = cosine_similarity(unknown_emb, algo_emb)
            scores[algo_name] = similarity
        
        # Return top matches
        sorted_algos = sorted(scores.items(), key=lambda x: x[1], reverse=True)
        
        return {
            'best_match': sorted_algos[0][0],
            'confidence': sorted_algos[0][1],
            'alternatives': sorted_algos[1:3]
        }
```

## Optimization Techniques

### 1. Efficient Batch Processing

```python
class BatchEmbedder:
    def __init__(self, model, batch_size=32):
        self.model = model
        self.batch_size = batch_size
        
    def embed_many(self, assembly_list):
        """Efficiently embed multiple functions"""
        all_embeddings = []
        
        # Process in batches
        for i in range(0, len(assembly_list), self.batch_size):
            batch = assembly_list[i:i + self.batch_size]
            
            # Batch tokenization
            inputs = self.tokenizer(
                batch,
                return_tensors="pt",
                max_length=512,
                truncation=True,
                padding=True
            )
            
            # Batch inference
            with torch.no_grad():
                outputs = self.model(**inputs)
                embeddings = outputs.last_hidden_state[:, 0, :].numpy()
            
            all_embeddings.extend(embeddings)
        
        return np.array(all_embeddings)
```

### 2. Caching and Persistence

```python
import pickle
import hashlib

class CachedEmbedder:
    def __init__(self, embedder, cache_dir="./embedding_cache"):
        self.embedder = embedder
        self.cache_dir = Path(cache_dir)
        self.cache_dir.mkdir(exist_ok=True)
        
    def embed_with_cache(self, assembly_code):
        """Embed with caching"""
        # Create cache key
        cache_key = hashlib.sha256(assembly_code.encode()).hexdigest()
        cache_file = self.cache_dir / f"{cache_key}.pkl"
        
        # Check cache
        if cache_file.exists():
            with open(cache_file, 'rb') as f:
                return pickle.load(f)
        
        # Compute embedding
        embedding = self.embedder.embed_assembly(assembly_code)
        
        # Cache result
        with open(cache_file, 'wb') as f:
            pickle.dump(embedding, f)
        
        return embedding
```

### 3. Dimensionality Reduction

```python
from sklearn.decomposition import PCA
import umap

class CompactEmbedder:
    def __init__(self, base_embedder, target_dim=128):
        self.base_embedder = base_embedder
        self.target_dim = target_dim
        self.reducer = None
        
    def fit_reducer(self, training_samples):
        """Train dimensionality reduction"""
        # Get full embeddings
        embeddings = [
            self.base_embedder.embed_assembly(s) 
            for s in training_samples
        ]
        embeddings = np.array(embeddings)
        
        # Fit UMAP for better preservation of structure
        self.reducer = umap.UMAP(
            n_components=self.target_dim,
            metric='cosine',
            n_neighbors=15,
            min_dist=0.1
        )
        self.reducer.fit(embeddings)
        
    def embed_compact(self, assembly_code):
        """Get compact embedding"""
        # Full embedding
        full_emb = self.base_embedder.embed_assembly(assembly_code)
        
        # Reduce
        compact_emb = self.reducer.transform([full_emb])[0]
        
        return compact_emb  # Shape: (128,) instead of (768,)
```

## Advanced Techniques

### 1. Multi-Modal Embeddings

```python
class MultiModalEmbedder:
    """Combine assembly with other signals"""
    
    def __init__(self):
        self.asm_embedder = AssemblyEmbedder()
        self.cfg_embedder = CFGEmbedder()
        self.data_embedder = DataEmbedder()
        
    def embed_function(self, function):
        # Assembly embedding
        asm_emb = self.asm_embedder.embed_function(function.assembly)
        
        # Control flow graph embedding
        cfg_emb = self.cfg_embedder.embed_cfg(function.cfg)
        
        # Data/string embedding
        data_emb = self.data_embedder.embed_data(function.data_refs)
        
        # Concatenate or combine
        combined = np.concatenate([
            asm_emb,
            cfg_emb,
            data_emb
        ])
        
        return combined
```

### 2. Hierarchical Embeddings

```python
class HierarchicalEmbedder:
    """Embed at multiple granularities"""
    
    def embed_program(self, binary):
        # Instruction level
        inst_embeddings = [
            self.embed_instruction(inst)
            for inst in binary.instructions
        ]
        
        # Basic block level
        block_embeddings = [
            self.embed_block(block)
            for block in binary.blocks
        ]
        
        # Function level
        func_embeddings = [
            self.embed_function(func)
            for func in binary.functions
        ]
        
        # Module level
        module_embedding = self.aggregate_embeddings(func_embeddings)
        
        return {
            'instructions': inst_embeddings,
            'blocks': block_embeddings,
            'functions': func_embeddings,
            'module': module_embedding
        }
```

## Common Pitfalls and Solutions

### 1. Architecture Differences

**Problem**: x86 and ARM embeddings don't align  
**Solution**: Use architecture-agnostic intermediate representation

### 2. Optimization Variations

**Problem**: Same code optimized differently has different embeddings  
**Solution**: Train with multiple optimization levels

### 3. Embedding Drift

**Problem**: Embeddings change when model updated  
**Solution**: Version embeddings and maintain compatibility layer

## Future Directions

1. **Graph Neural Networks**: Better CFG representation
2. **Transformer Architectures**: Attention over instruction sequences
3. **Self-Supervised Learning**: Learn from massive binary corpora
4. **Cross-Architecture Transfer**: Universal binary embeddings

## Conclusion

Embeddings provide a powerful tool for binary analysis, enabling:

- Fast similarity search
- Pattern recognition
- Clustering and classification
- Cross-binary knowledge transfer

The key is choosing the right embedding approach for your specific use case and constraints.