# Hands-On: DVC

A practical companion to the [GitOps & CI/CD for ML tutorial](tutorial.md#dvc-vs-delta-lakeunity-catalog-the-actual-difference)
— that tutorial covers *when* DVC is the right tool versus Delta Lake/Unity Catalog; this
covers *how* to actually use it: tracking data, pushing it to remote storage, versioning
it alongside code, and building a reproducible pipeline.

## What You'll Set Up

A Git repo with DVC tracking a dataset stored in S3, versioned across commits the same way
code is — then a small multi-stage pipeline (prepare → train → evaluate) that DVC can
re-run only when its actual inputs change.

## Prerequisites

- A Git repo (`git init` if starting fresh)
- Python with `pip install "dvc[s3]"` (swap `s3` for `gs`/`azure` depending on your cloud)
- An S3 bucket (or any supported remote) to act as data storage

## 1. Initialize DVC

```bash
git init                 # if not already a git repo
dvc init
git add .dvc .dvcignore
git commit -m "Initialize DVC"
```

## 2. Track a Dataset

```bash
dvc add data/train.csv
```

This creates `data/train.csv.dvc` — a small text pointer file (containing a content hash,
size, and path) — and adds the *actual* `data/train.csv` to `.gitignore` automatically.
**This pointer file is what you commit to Git**, not the data itself:

```bash
git add data/train.csv.dvc data/.gitignore
git commit -m "Track train.csv with DVC"
```

This is the core mechanic: Git tracks a small pointer, DVC tracks the actual (potentially
huge) file content, keyed by that pointer's hash.

## 3. Configure a Remote

```bash
dvc remote add -d storage s3://your-bucket/dvc-store
git add .dvc/config
git commit -m "Configure S3 remote for DVC"

# if not using an IAM role, configure credentials:
dvc remote modify storage access_key_id <your-access-key>
dvc remote modify storage secret_access_key <your-secret-key>
```

## 4. Push and Pull Data

```bash
dvc push          # upload tracked data to the configured remote
dvc pull          # download data referenced by the current .dvc pointer files
```

Anyone who clones the Git repo gets the pointer files immediately via `git clone`, then
runs `dvc pull` to fetch the actual data — the same mental model as Git LFS, but with
DVC's pipeline features layered on top (below).

## 5. Version Data Alongside Code

This is the payoff: switching Git branches/commits also switches which data version is
"current," via `dvc checkout`:

```bash
# update the dataset
echo "new,row,here" >> data/train.csv
dvc add data/train.csv          # re-hash, update the pointer file
git add data/train.csv.dvc
git commit -m "Update train.csv with new data"
dvc push

# later, go back to the previous data version:
git checkout HEAD~1 -- data/train.csv.dvc
dvc checkout                    # pulls the OLD data version matching that pointer
```

`git log -p data/train.csv.dvc` gives you a full audit trail of every data version ever
committed — directly useful for the lineage/audit concerns in the
[cost & governance tutorial](../10_cost_security_multiregion/tutorial.md#security-compliance-in-ml-pipelines)
and the [audit-lineage tricky scenario](../12_tricky_scenarios/10_audit_lineage_reconstruction.md).

## 6. Build a Reproducible Pipeline

DVC pipelines declare stages with explicit dependencies and outputs, so `dvc repro` only
re-runs stages whose actual inputs changed — not the whole pipeline blindly.

```yaml
# dvc.yaml
stages:
  prepare:
    cmd: python src/prepare.py data/train.csv data/prepared.csv
    deps:
      - src/prepare.py
      - data/train.csv
    outs:
      - data/prepared.csv

  train:
    cmd: python src/train.py data/prepared.csv model.pkl
    deps:
      - src/train.py
      - data/prepared.csv
    outs:
      - model.pkl

  evaluate:
    cmd: python src/evaluate.py model.pkl data/prepared.csv metrics.json
    deps:
      - src/evaluate.py
      - model.pkl
    metrics:
      - metrics.json:
          cache: false
```

```bash
dvc repro                 # runs only the stages whose deps changed since last run
dvc dag                   # visualize the pipeline's dependency graph in the terminal
git add dvc.yaml dvc.lock
git commit -m "Add training pipeline"
```

`dvc.lock` (auto-generated) pins the exact hashes of every dependency/output from the last
successful run — this is what lets `dvc repro` skip stages whose inputs provably haven't
changed, the same incremental-build idea as `make`, applied to ML pipeline stages.

## 7. Compare Experiments (Optional, More Advanced)

```bash
dvc exp run --set-param train.learning_rate=0.01
dvc exp run --set-param train.learning_rate=0.1
dvc exp show              # tabular comparison of metrics across experiment runs
```

Each `dvc exp run` runs the pipeline with a parameter override in a lightweight, disposable
workspace — useful for hyperparameter sweeps without committing a new Git commit per
attempt; only promote the winning experiment to a real commit once you've picked it.

## Common Commands Cheat Sheet

| Command | What it does |
|---|---|
| `dvc add <file>` | Start tracking a file/directory with DVC |
| `dvc push` / `dvc pull` | Upload/download tracked data to/from the remote |
| `dvc checkout` | Sync the working directory's data to match the current Git-checked-out `.dvc` pointers |
| `dvc status` | Show what's changed (data or pipeline stages) since the last commit/run |
| `dvc repro` | Re-run pipeline stages whose dependencies changed |
| `dvc dag` | Visualize the pipeline dependency graph |
| `dvc exp run` | Run a disposable experiment (optionally with parameter overrides) |
| `dvc exp show` | Compare metrics across experiment runs |
| `dvc remote list` | Show configured remotes |

## Troubleshooting

- **`dvc pull` fails with a permissions error**: check remote credentials
  (`dvc remote modify` values, or the IAM role attached to the compute running the
  command) — this is the most common first-time setup issue.
- **`dvc repro` reruns everything even though nothing changed**: check that `dvc.lock` was
  actually committed to Git — if it's missing or gitignored by mistake, DVC has no record
  of the last successful run's exact hashes to compare against.
- **Large files make `dvc add` slow**: DVC hashes the full file content to generate the
  pointer — for very large datasets, consider tracking a directory with many smaller files
  instead of one huge file, since DVC can checkout/pull individual files within a tracked
  directory incrementally.
- **Team members get merge conflicts on `.dvc` files**: this is expected when two people
  update the same tracked dataset independently — resolve it like any other merge
  conflict (decide which data version wins, or re-run the pipeline stage that produced it)
  rather than trying to "merge" the pointer file's hash directly.

---

**See also:** [GitOps & CI/CD for ML tutorial](tutorial.md) · [Hands-On: ArgoCD](argocd_hands_on.md)
