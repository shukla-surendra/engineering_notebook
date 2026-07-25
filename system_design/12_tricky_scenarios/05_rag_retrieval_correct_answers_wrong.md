# 5. Retrieval Looks Right, Answers Are Wrong

**Primary topic:** [RAG + LLM-Serving at Scale](../06_rag_llm_serving_at_scale/tutorial.md)

## The Situation

A RAG-based internal documentation assistant is getting complaints: answers are
confidently wrong. When you manually test the exact questions users report, the retrieved
chunks *look* correct and relevant. But users insist the answers are outdated or wrong.

## First Questions to Ask

- When you manually test, are you using the **same embedding model version** the
  production index was built with, or the current/latest one?
- How recently was the vector index last rebuilt, versus how recently did the source
  documents change?
- Are users' actual queries similar in phrasing to your manual tests, or could
  phrasing/vocabulary differences be retrieving different (worse) chunks for them than for
  you?
- Is there a reranking step, and is it active for real user traffic the same way it is in
  your manual test path?

## Likely Root Causes (ranked)

1. **Embedding model version mismatch between index-time and query-time.** If the vector
   index was built with embedding model version A, and the query-time embedding service
   was since upgraded to version B (a routine-seeming dependency bump), query embeddings
   and indexed embeddings now live in *different vector spaces* — similarity scores
   become meaningless, but retrieval doesn't *fail*, it just silently returns poor
   matches. Your manual test might coincidentally use a cached/pinned older embedding
   client, masking the issue.
2. **Stale index relative to source documents.** If documents are updated frequently but
   re-indexing runs on a slower schedule (or has been silently failing), retrieval
   confidently returns chunks that were correct *when indexed* but are now outdated — the
   retrieval mechanism is working exactly as designed, on stale data.
3. **Query phrasing sensitivity** — real users phrase questions differently than a
   developer manually testing (informal language, typos, different vocabulary for the
   same concept) — if retrieval quality is more phrasing-sensitive than realized, your
   manual tests (which you naturally phrase "well") aren't representative of the actual
   failure population.

## Diagnostic Path

1. **Verify embedding model version parity** — check the exact model version/checkpoint
   used to build the current index versus the model version the live query-embedding
   service is currently calling. This is the highest-leverage single check, since it's
   both the most common root cause and the fastest to rule in/out.
2. **Check index rebuild timestamps against source document modification timestamps** —
   if there's a meaningful gap, that's a strong signal for root cause #2, regardless of
   whether #1 is also true.
3. **Pull a sample of *actual* failing user queries** (not your own test queries) and
   manually inspect retrieval quality for those specific phrasings — compare retrieval
   relevance scores between your manual tests and real complaint queries.
4. **If reranking exists, verify it's actually applied identically in the production path**
   as in your manual test path — a common gap is a manual/debug tool that bypasses a
   production-only reranking step, making manual testing look artificially better.

## The Fix

- **Immediate mitigation**: if embedding version mismatch is confirmed, either roll back
  the query-time embedding service to match the index, or trigger an emergency full
  re-index with the new embedding model — there's no way to "convert" old-embedding-space
  vectors to new-embedding-space compatibility after the fact.
- **Long-term fix**: pin embedding model version explicitly as a versioned dependency of
  the index itself (not a floating "latest" dependency), and treat an embedding-model
  upgrade as requiring a full re-index as a hard, enforced step — never an independent,
  silently-compatible deploy.

## Prevention

The systemic lesson: **retrieval can fail silently** — a broken embedding-space match
doesn't throw an error or return zero results, it just returns *plausible-looking but
wrong* matches, which is precisely why this class of bug is hard to catch without
deliberately checking version parity. See the embedding-model discussion in the
[RAG tutorial](../06_rag_llm_serving_at_scale/tutorial.md#embedding-vector-search) and
the retrieval-hallucination failure mode it calls out.

---

**Previous:** [4. Overnight GPU Cost Spike](04_gpu_cost_spike.md)  |  **Next:** [6. Silently Duplicated Training Data](06_duplicate_training_data.md)
