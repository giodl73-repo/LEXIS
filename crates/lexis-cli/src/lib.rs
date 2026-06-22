use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct Fixture {
    fixture_id: String,
    status: String,
    #[serde(default)]
    scope: Option<Scope>,
    source_text_included: bool,
    source_text_redistribution_posture: String,
    #[serde(default)]
    source_records: Vec<SourceRecord>,
    #[serde(default)]
    deferred_records: Vec<DeferredRecord>,
    #[serde(default)]
    language_claims: Vec<serde_yaml::Value>,
    #[serde(default)]
    relationship_claims: Vec<serde_yaml::Value>,
    #[serde(default)]
    graph_outputs: Vec<GraphOutput>,
    #[serde(default)]
    chronicle_outputs: Vec<serde_yaml::Value>,
    #[serde(default)]
    promotion_blockers: Vec<String>,
    #[serde(default)]
    nodes: NodeSet,
    #[serde(default)]
    source_links: Vec<SourceLink>,
    #[serde(default)]
    relationship_edges: Vec<RelationshipEdge>,
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    fixture_id: String,
    status: String,
    fixture_class: String,
    #[serde(default)]
    owning_work_package: Option<String>,
    #[serde(default)]
    linked_scope: Option<String>,
    #[serde(default)]
    linked_slice_packages: Vec<String>,
    #[serde(default)]
    linked_source_custody_decision: Vec<String>,
    #[serde(default)]
    fixture_shape: Option<String>,
    #[serde(default)]
    expected_result: Option<String>,
    #[serde(default)]
    review_state: Option<String>,
    #[serde(default)]
    promotion_blockers: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ScenarioManifest {
    scenario_id: String,
    status: String,
    actor: String,
    purpose: String,
    slice_package: String,
    #[serde(default)]
    related_work_packages: Vec<String>,
    #[serde(default)]
    specs_exercised: Vec<String>,
    #[serde(default)]
    positive_path: Vec<String>,
    #[serde(default)]
    negative_paths: Vec<String>,
    #[serde(default)]
    diagnostics_expected: Vec<String>,
    #[serde(default)]
    evidence_expected: Vec<String>,
    #[serde(default)]
    fixture_candidates: Vec<String>,
    #[serde(default)]
    findings_file: Option<String>,
}

#[derive(Debug)]
struct WorkPackageEntry {
    id: String,
    name: String,
    outcome: String,
    primary_gate: String,
    scenario_refs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Scope {
    #[allow(dead_code)]
    scope_id: String,
    question: String,
    #[serde(default)]
    excluded_claims: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SourceRecord {
    source_ref: String,
    contract_state: String,
    may_support_claims: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeferredRecord {
    source_ref: String,
    contract_state: String,
}

#[derive(Debug, Deserialize)]
struct SourceCustodyRecord {
    decision_id: String,
    status: String,
    #[serde(default)]
    source_family: Option<String>,
    #[serde(default)]
    related_slice_packages: Vec<String>,
    #[serde(default)]
    pointer: Option<String>,
    #[serde(default)]
    candidate_source_name: Option<String>,
    #[serde(default)]
    rights_posture: Option<String>,
    #[serde(default)]
    redistribution_posture: Option<String>,
    #[serde(default)]
    citation_note: Option<String>,
    #[serde(default)]
    review_state: Option<String>,
    #[serde(default)]
    reviewer: Option<String>,
    #[serde(default)]
    promotion_allowed: Option<bool>,
    #[serde(default)]
    blocks: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CorrectionPlan {
    plan_id: String,
    status: String,
    source_report: String,
    scope: String,
    #[serde(default)]
    entries: Vec<CorrectionEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CorrectionEntry {
    chain_id: String,
    generated_seed: String,
    proof_source: String,
    action: String,
    generated_path: String,
    corrected_path: String,
    #[serde(default)]
    replacement_forms: Vec<String>,
    route_notes: String,
    #[serde(default)]
    promotion_blockers: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct NodeSet {
    #[serde(default)]
    languages: Vec<Language>,
    #[serde(default)]
    roots: Vec<Root>,
    #[serde(default)]
    wordforms: Vec<Wordform>,
    #[serde(default)]
    meaning_senses: Vec<MeaningSense>,
    #[serde(default)]
    script_forms: Vec<ScriptForm>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Language {
    id: String,
    label: String,
    kind: String,
    source_posture: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Root {
    id: String,
    label: String,
    root_text: String,
    claim_type: String,
    uncertainty: String,
    #[serde(default)]
    source_links: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wordform {
    id: String,
    label: String,
    language_id: String,
    form: String,
    claim_type: String,
    uncertainty: String,
    #[serde(default)]
    source_links: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MeaningSense {
    id: String,
    label: String,
    gloss: String,
    claim_type: String,
    uncertainty: String,
    #[serde(default)]
    source_links: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ScriptForm {
    id: String,
    label: String,
    script_label: String,
    form_text: String,
    transliteration_posture: String,
    #[serde(default)]
    source_links: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SourceLink {
    id: String,
    source_ref: String,
    contract_state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RelationshipEdge {
    id: String,
    edge_kind: String,
    source_id: String,
    target_id: String,
    claim_type: String,
    uncertainty: String,
    #[serde(default)]
    supporting_sources: Vec<String>,
    review_state: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GraphOutput {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    format: String,
    status: String,
}

#[derive(Debug, Serialize)]
struct GraphSlice {
    slice_id: String,
    nodes: Vec<GraphNode>,
    edges: Vec<RelationshipEdge>,
    source_posture_summary: Vec<String>,
    graph_engine_posture: &'static str,
    graph_output_posture: &'static str,
    validation_error_count: usize,
}

#[derive(Debug, Serialize)]
struct GraphNode {
    id: String,
    label: String,
    record_class: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    claim_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uncertainty: Option<String>,
    source_posture: String,
}

#[derive(Debug, Clone, Copy)]
pub enum GraphFormat {
    Json,
    Dot,
}

#[derive(Debug, Default)]
struct SourceCustodyIndex {
    records: HashMap<String, SourceCustodyRecord>,
    load_errors: Vec<String>,
}

#[derive(Debug, Default)]
struct SliceInventoryEntry {
    title: String,
    packages: Vec<String>,
    fixtures: Vec<String>,
    sources: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SliceSeed {
    slice_id: String,
    #[serde(default)]
    fixture_id: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    question: Option<String>,
    #[serde(default)]
    source_state: Option<String>,
    #[serde(default)]
    sources: Vec<SeedSource>,
    #[serde(default)]
    forms: Vec<SeedForm>,
    #[serde(default)]
    relationships: Vec<SeedRelationship>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeedSource {
    id: String,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    may_support_claims: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeedForm {
    form: String,
    language: String,
    source: String,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    meaning: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SeedRelationship {
    kind: String,
    source: String,
    target: String,
    #[serde(default)]
    claim_type: Option<String>,
    #[serde(default)]
    uncertainty: Option<String>,
    #[serde(default)]
    support: Vec<String>,
    #[serde(default)]
    review_state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub id: &'static str,
    pub family: &'static str,
    pub severity: Severity,
    pub affected: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Default)]
pub struct ValidationReport {
    fixture_id: String,
    diagnostics: Vec<Diagnostic>,
}

impl ValidationReport {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

pub fn validate_fixture(path: &Path) -> Result<ValidationReport, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    Ok(validate_fixture_record(&fixture, &source_index))
}

pub fn generate_slice(seed_path: &Path, out_path: &Path) -> Result<String, String> {
    let seed_text = fs::read_to_string(seed_path)
        .map_err(|err| format!("failed to read {}: {err}", seed_path.display()))?;
    let seed: SliceSeed = serde_yaml::from_str(&seed_text)
        .map_err(|err| format!("failed to parse {}: {err}", seed_path.display()))?;
    let fixture = build_fixture_from_seed(&seed)?;
    let fixture_yaml = serde_yaml::to_string(&fixture)
        .map_err(|err| format!("failed to serialize generated fixture: {err}"))?;

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(out_path, fixture_yaml)
        .map_err(|err| format!("failed to write {}: {err}", out_path.display()))?;

    Ok(format!(
        "slice_generated: {}\nseed: {}\nfixture: {}\nforms: {}\nrelationships: {}\n",
        fixture.fixture_id,
        seed_path.display(),
        out_path.display(),
        fixture.nodes.wordforms.len(),
        fixture.relationship_edges.len()
    ))
}

pub fn batch_validate(path: &Path) -> Result<String, String> {
    let fixture_paths = collect_fixture_yaml_paths(path)?;
    let mut valid = 0usize;
    let mut invalid = 0usize;
    let mut output = String::new();
    output.push_str("batch_validate:\n");
    output.push_str(&format!("root: {}\n", path.display()));
    output.push_str(&format!("count: {}\n", fixture_paths.len()));
    output.push_str("fixtures:\n");

    for fixture_path in fixture_paths {
        let (fixture, source_index) = load_fixture_context(&fixture_path)?;
        let report = validate_fixture_record(&fixture, &source_index);
        if report.has_errors() {
            invalid += 1;
        } else {
            valid += 1;
        }
        output.push_str(&format!(
            "  {} | path={} | status={} | validation_errors={}\n",
            fixture.fixture_id,
            fixture_path.display(),
            if report.has_errors() {
                "invalid"
            } else {
                "valid"
            },
            report.diagnostics().len()
        ));
    }

    output.push_str("summary:\n");
    output.push_str(&format!("  valid: {valid}\n"));
    output.push_str(&format!("  invalid: {invalid}\n"));
    Ok(output)
}

pub fn batch_summary(path: &Path) -> Result<String, String> {
    let fixture_paths = collect_fixture_yaml_paths(path)?;
    let mut total_nodes = 0usize;
    let mut total_edges = 0usize;
    let mut valid = 0usize;
    let mut invalid = 0usize;
    let mut output = String::new();
    output.push_str("batch_summary:\n");
    output.push_str(&format!("root: {}\n", path.display()));
    output.push_str(&format!("count: {}\n", fixture_paths.len()));
    output.push_str("fixtures:\n");

    for fixture_path in fixture_paths {
        let (fixture, source_index) = load_fixture_context(&fixture_path)?;
        let report = validate_fixture_record(&fixture, &source_index);
        let graph_output_posture = if report.has_errors() {
            invalid += 1;
            "preview_only_not_promoted"
        } else {
            valid += 1;
            "validated"
        };
        let graph = build_graph_slice(&fixture, graph_output_posture, report.diagnostics().len());
        total_nodes += graph.nodes.len();
        total_edges += graph.edges.len();
        output.push_str(&format!(
            "  {} | status={} | nodes={} | edges={} | validation_errors={}\n",
            fixture.fixture_id,
            graph.graph_output_posture,
            graph.nodes.len(),
            graph.edges.len(),
            graph.validation_error_count
        ));
    }

    output.push_str("summary:\n");
    output.push_str(&format!("  valid: {valid}\n"));
    output.push_str(&format!("  invalid: {invalid}\n"));
    output.push_str(&format!("  nodes: {total_nodes}\n"));
    output.push_str(&format!("  edges: {total_edges}\n"));
    Ok(output)
}

fn build_fixture_from_seed(seed: &SliceSeed) -> Result<Fixture, String> {
    if seed.forms.is_empty() {
        return Err(format!("slice seed '{}' has no forms", seed.slice_id));
    }
    if seed.sources.is_empty() {
        return Err(format!("slice seed '{}' has no sources", seed.slice_id));
    }

    let default_source_state = seed
        .source_state
        .as_deref()
        .unwrap_or("candidate_for_slice")
        .to_string();
    let source_records: Vec<SourceRecord> = seed
        .sources
        .iter()
        .map(|source| SourceRecord {
            source_ref: source.id.clone(),
            contract_state: source
                .state
                .clone()
                .unwrap_or_else(|| default_source_state.clone()),
            may_support_claims: source.may_support_claims.unwrap_or(false),
        })
        .collect();
    let source_state_by_id: HashMap<String, String> = source_records
        .iter()
        .map(|source| (source.source_ref.clone(), source.contract_state.clone()))
        .collect();

    let mut languages_by_name = BTreeMap::new();
    for form in &seed.forms {
        languages_by_name
            .entry(form.language.clone())
            .or_insert_with(|| format!("lang-{}", slug(&form.language)));
    }

    let languages = languages_by_name
        .iter()
        .map(|(label, id)| Language {
            id: id.clone(),
            label: label.clone(),
            kind: "language".to_string(),
            source_posture: default_source_state.clone(),
        })
        .collect();

    let mut wordforms = Vec::new();
    let mut source_links = Vec::new();
    let mut form_ref_index = HashMap::new();
    for form in &seed.forms {
        let language_id = languages_by_name
            .get(&form.language)
            .ok_or_else(|| format!("language '{}' was not indexed", form.language))?
            .clone();
        let wordform_id = form.id.clone().unwrap_or_else(|| {
            format!("wf-{}-{}", trim_lang_prefix(&language_id), slug(&form.form))
        });
        let source_link_id = format!(
            "src-{}-{}",
            trim_lang_prefix(&language_id),
            slug(&form.form)
        );
        let source_state = source_state_by_id
            .get(&form.source)
            .cloned()
            .unwrap_or_else(|| "missing_source_record".to_string());
        wordforms.push(Wordform {
            id: wordform_id.clone(),
            label: form.label.clone().unwrap_or_else(|| form.form.clone()),
            language_id,
            form: form.form.clone(),
            claim_type: "direct_evidence".to_string(),
            uncertainty: "source_limited".to_string(),
            source_links: vec![source_link_id.clone()],
        });
        source_links.push(SourceLink {
            id: source_link_id,
            source_ref: form.source.clone(),
            contract_state: source_state,
        });
        form_ref_index.insert(form.form.clone(), wordform_id.clone());
        form_ref_index.insert(slug(&form.form), wordform_id.clone());
        form_ref_index.insert(wordform_id.clone(), wordform_id);
    }

    let mut meaning_senses = Vec::new();
    let mut relationship_edges = Vec::new();
    for form in &seed.forms {
        let form_ref = form.id.as_deref().unwrap_or(&form.form);
        let wordform_id = resolve_seed_ref(&form_ref_index, form_ref)?;
        let source_link_id = wordforms
            .iter()
            .find(|wordform| wordform.id == wordform_id)
            .and_then(|wordform| wordform.source_links.first())
            .cloned()
            .ok_or_else(|| format!("wordform '{}' has no generated source link", wordform_id))?;
        let source_ref = source_links
            .iter()
            .find(|link| link.id == source_link_id)
            .map(|link| link.source_ref.clone())
            .ok_or_else(|| format!("source link '{}' was not generated", source_link_id))?;
        let support_suffix = slug(&wordform_id);
        relationship_edges.push(RelationshipEdge {
            id: format!("edge-{support_suffix}-supported-by-source"),
            edge_kind: "supports_claim".to_string(),
            source_id: source_link_id.clone(),
            target_id: wordform_id.clone(),
            claim_type: "direct_evidence".to_string(),
            uncertainty: "source_limited".to_string(),
            supporting_sources: vec![source_ref.clone()],
            review_state: default_source_state.clone(),
        });

        if let Some(meaning) = &form.meaning {
            let meaning_id = format!("sense-{support_suffix}");
            meaning_senses.push(MeaningSense {
                id: meaning_id.clone(),
                label: meaning.clone(),
                gloss: "Generated project summary placeholder; not copied source text.".to_string(),
                claim_type: "inference".to_string(),
                uncertainty: "source_limited".to_string(),
                source_links: vec![source_link_id],
            });
            relationship_edges.push(RelationshipEdge {
                id: format!("edge-{support_suffix}-meaning"),
                edge_kind: "meaning_shift_to".to_string(),
                source_id: wordform_id,
                target_id: meaning_id,
                claim_type: "inference".to_string(),
                uncertainty: "source_limited".to_string(),
                supporting_sources: vec![source_ref],
                review_state: default_source_state.clone(),
            });
        }
    }

    for relation in &seed.relationships {
        let source_id = resolve_seed_ref(&form_ref_index, &relation.source)?;
        let target_id = resolve_seed_ref(&form_ref_index, &relation.target)?;
        let support = if relation.support.is_empty() {
            source_records
                .first()
                .map(|source| vec![source.source_ref.clone()])
                .unwrap_or_default()
        } else {
            relation.support.clone()
        };
        relationship_edges.push(RelationshipEdge {
            id: format!(
                "edge-{}-{}-{}",
                slug(&relation.source),
                slug(&relation.kind),
                slug(&relation.target)
            ),
            edge_kind: relation.kind.clone(),
            source_id,
            target_id,
            claim_type: relation
                .claim_type
                .clone()
                .unwrap_or_else(|| "inference".to_string()),
            uncertainty: relation
                .uncertainty
                .clone()
                .unwrap_or_else(|| "source_limited".to_string()),
            supporting_sources: support,
            review_state: relation
                .review_state
                .clone()
                .unwrap_or_else(|| default_source_state.clone()),
        });
    }

    Ok(Fixture {
        fixture_id: seed
            .fixture_id
            .clone()
            .unwrap_or_else(|| format!("LEXIS-GEN-{}", slug(&seed.slice_id))),
        status: seed
            .status
            .clone()
            .unwrap_or_else(|| "generated_candidate".to_string()),
        scope: Some(Scope {
            scope_id: format!("{}-scope", seed.slice_id),
            question: seed.question.clone().unwrap_or_else(|| {
                format!(
                    "Generated graph-facing slice for {}.",
                    seed.slice_id.replace('-', " ")
                )
            }),
            excluded_claims: vec![
                "unreviewed generated relationship claims".to_string(),
                "source text ingestion or redistribution".to_string(),
            ],
        }),
        source_text_included: false,
        source_text_redistribution_posture: "pointer_only_planned".to_string(),
        source_records,
        deferred_records: Vec::new(),
        language_claims: Vec::new(),
        relationship_claims: Vec::new(),
        graph_outputs: Vec::new(),
        chronicle_outputs: Vec::new(),
        promotion_blockers: Vec::new(),
        nodes: NodeSet {
            languages,
            roots: Vec::new(),
            wordforms,
            meaning_senses,
            script_forms: Vec::new(),
        },
        source_links,
        relationship_edges,
    })
}

fn resolve_seed_ref(index: &HashMap<String, String>, raw: &str) -> Result<String, String> {
    index
        .get(raw)
        .or_else(|| index.get(&slug(raw)))
        .cloned()
        .ok_or_else(|| {
            format!(
                "seed reference '{}' did not resolve to a generated form",
                raw
            )
        })
}

fn slug(value: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

fn trim_lang_prefix(language_id: &str) -> &str {
    language_id.strip_prefix("lang-").unwrap_or(language_id)
}

fn collect_fixture_yaml_paths(path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    if path.is_file() {
        paths.push(path.to_path_buf());
    } else {
        collect_fixture_yaml_paths_from_dir(path, &mut paths)?;
    }
    paths.sort();
    Ok(paths)
}

fn collect_fixture_yaml_paths_from_dir(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    if !dir.exists() {
        return Err(format!(
            "fixture batch path '{}' does not exist",
            dir.display()
        ));
    }
    for entry in
        fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))?
    {
        let entry = entry.map_err(|err| format!("failed to read directory entry: {err}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_yaml_paths_from_dir(&path, paths)?;
        } else if path.file_name().and_then(|name| name.to_str()) == Some("fixture.yaml") {
            paths.push(path);
        }
    }
    Ok(())
}

pub fn emit_graph(path: &Path, format: GraphFormat) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    if report.has_errors() {
        return Err(format!(
            "graph emission blocked because fixture '{}' is invalid",
            report.fixture_id
        ));
    }

    let (fixture, _) = load_fixture_context(path)?;
    let graph = build_graph_slice(&fixture, "validated", 0);
    render_graph(&graph, format)
}

pub fn preview_graph(path: &Path, format: GraphFormat) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph = build_graph_slice(
        &fixture,
        "preview_only_not_promoted",
        report.diagnostics.len(),
    );
    render_graph(&graph, format)
}

pub fn summarize_graph(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };
    let graph = build_graph_slice(&fixture, graph_output_posture, report.diagnostics.len());
    Ok(render_summary(&graph))
}

pub fn inspect_graph(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };
    let graph = build_graph_slice(&fixture, graph_output_posture, report.diagnostics.len());
    Ok(render_inspection(&graph))
}

pub fn graph_path(path: &Path, start_id: &str, end_id: &str) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };

    if graph_label_for_id(&fixture, start_id).is_none() {
        return Err(format!(
            "start node '{}' was not found in fixture '{}'",
            start_id, fixture.fixture_id
        ));
    }
    if graph_label_for_id(&fixture, end_id).is_none() {
        return Err(format!(
            "end node '{}' was not found in fixture '{}'",
            end_id, fixture.fixture_id
        ));
    }

    Ok(render_graph_path(
        &fixture,
        start_id,
        end_id,
        graph_output_posture,
        report.diagnostics.len(),
    ))
}

pub fn explain_claim(path: &Path, claim_id: &str) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };

    if fixture
        .relationship_edges
        .iter()
        .any(|edge| edge.id == claim_id)
        || graph_label_for_id(&fixture, claim_id).is_some()
    {
        Ok(render_claim_explanation(
            &fixture,
            &source_index,
            claim_id,
            graph_output_posture,
            report.diagnostics.len(),
        ))
    } else {
        Err(format!(
            "claim '{}' was not found in fixture '{}'",
            claim_id, fixture.fixture_id
        ))
    }
}

pub fn preview_chronicle(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };
    let graph = build_graph_slice(&fixture, graph_output_posture, report.diagnostics.len());
    Ok(render_chronicle_preview(&fixture, &graph))
}

pub fn write_preview_artifacts(path: &Path, out_dir: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };
    let graph = build_graph_slice(&fixture, graph_output_posture, report.diagnostics.len());
    let json = render_graph(&graph, GraphFormat::Json)?;
    let dot = render_graph(&graph, GraphFormat::Dot)?;
    let chronicle = render_chronicle_preview(&fixture, &graph);

    fs::create_dir_all(out_dir)
        .map_err(|err| format!("failed to create {}: {err}", out_dir.display()))?;
    let json_path = out_dir.join("graph-preview.json");
    let dot_path = out_dir.join("graph-preview.dot");
    let chronicle_path = out_dir.join("chronicle-preview.md");
    fs::write(&json_path, json)
        .map_err(|err| format!("failed to write {}: {err}", json_path.display()))?;
    fs::write(&dot_path, dot)
        .map_err(|err| format!("failed to write {}: {err}", dot_path.display()))?;
    fs::write(&chronicle_path, chronicle)
        .map_err(|err| format!("failed to write {}: {err}", chronicle_path.display()))?;

    let mut output = String::new();
    output.push_str(&format!("artifacts_written: {}\n", fixture.fixture_id));
    output.push_str(&format!("status: {graph_output_posture}\n"));
    output.push_str(&format!(
        "validation_errors: {}\n",
        report.diagnostics.len()
    ));
    output.push_str("files:\n");
    output.push_str(&format!("  {}\n", json_path.display()));
    output.push_str(&format!("  {}\n", dot_path.display()));
    output.push_str(&format!("  {}\n", chronicle_path.display()));
    if report.has_errors() {
        output.push_str("review_note: preview artifacts are not promoted source-backed claims\n");
    }
    Ok(output)
}

pub fn write_preview_artifact_batch(path: &Path, out_root: &Path) -> Result<String, String> {
    let fixture_paths = collect_fixture_yaml_paths(path)?;
    fs::create_dir_all(out_root)
        .map_err(|err| format!("failed to create {}: {err}", out_root.display()))?;

    let mut output = String::new();
    output.push_str("artifact_batch_written:\n");
    output.push_str(&format!("root: {}\n", path.display()));
    output.push_str(&format!("out_root: {}\n", out_root.display()));
    output.push_str(&format!("count: {}\n", fixture_paths.len()));
    output.push_str("artifacts:\n");

    for fixture_path in fixture_paths {
        let artifact_name = fixture_path
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("fixture");
        let out_dir = out_root.join(artifact_name);
        let (fixture, source_index) = load_fixture_context(&fixture_path)?;
        let report = validate_fixture_record(&fixture, &source_index);
        let graph_output_posture = if report.has_errors() {
            "preview_only_not_promoted"
        } else {
            "validated"
        };
        write_preview_artifacts(&fixture_path, &out_dir)?;
        output.push_str(&format!(
            "  {} | fixture={} | status={} | validation_errors={}\n",
            fixture.fixture_id,
            fixture_path.display(),
            graph_output_posture,
            report.diagnostics.len()
        ));
    }

    Ok(output)
}

pub fn list_artifacts() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_artifact_inventory(&repo_root)
}

pub fn summarize_artifacts(path: &Path) -> Result<String, String> {
    render_artifact_corpus_summary(path)
}

pub fn write_artifact_report(path: &Path, out_path: &Path) -> Result<String, String> {
    let report = render_artifact_corpus_report(path)?;
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(out_path, report)
        .map_err(|err| format!("failed to write {}: {err}", out_path.display()))?;
    Ok(format!(
        "artifact_report_written:\nroot: {}\nreport: {}\n",
        path.display(),
        out_path.display()
    ))
}

pub fn write_correction_artifact_report(path: &Path, out_path: &Path) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    let report = render_correction_artifact_report(&repo_root, path)?;
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(out_path, report)
        .map_err(|err| format!("failed to write {}: {err}", out_path.display()))?;
    Ok(format!(
        "correction_artifact_report_written:\nroot: {}\nreport: {}\n",
        path.display(),
        out_path.display()
    ))
}

pub fn trace_word(path: &Path, wordform_id: &str) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };

    let wordform = fixture
        .nodes
        .wordforms
        .iter()
        .find(|node| node.id == wordform_id)
        .ok_or_else(|| {
            format!(
                "wordform '{}' was not found in fixture '{}'",
                wordform_id, fixture.fixture_id
            )
        })?;

    Ok(render_word_trace(
        &fixture,
        wordform,
        graph_output_posture,
        report.diagnostics.len(),
    ))
}

pub fn trace_lineage(path: &Path, wordform_id: &str) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };

    let wordform = fixture
        .nodes
        .wordforms
        .iter()
        .find(|node| node.id == wordform_id)
        .ok_or_else(|| {
            format!(
                "wordform '{}' was not found in fixture '{}'",
                wordform_id, fixture.fixture_id
            )
        })?;

    Ok(render_lineage_trace(
        &fixture,
        wordform,
        graph_output_posture,
        report.diagnostics.len(),
    ))
}

pub fn trace_neighborhood(path: &Path, node_id: &str) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };

    if graph_label_for_id(&fixture, node_id).is_none() {
        return Err(format!(
            "node or source link '{}' was not found in fixture '{}'",
            node_id, fixture.fixture_id
        ));
    }

    Ok(render_neighborhood_trace(
        &fixture,
        node_id,
        graph_output_posture,
        report.diagnostics.len(),
    ))
}

pub fn source_status(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    Ok(render_source_status(
        &fixture,
        &source_index,
        report.diagnostics.len(),
    ))
}

pub fn list_sources() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    let source_index = SourceCustodyIndex::load(&repo_root)?;
    Ok(render_source_inventory(&source_index))
}

pub fn source_review(source_id: &str) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    let source_index = SourceCustodyIndex::load(&repo_root)?;
    render_source_review(&repo_root, &source_index, source_id)
}

pub fn list_corrections() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_correction_inventory(&repo_root)
}

pub fn correction_review(chain_id: &str) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_correction_review(&repo_root, chain_id)
}

pub fn generate_correction_seed(chain_id: &str, out_path: &Path) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    write_correction_seed(&repo_root, chain_id, out_path)
}

pub fn generate_correction_seeds(out_dir: &Path) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    write_all_correction_seeds(&repo_root, out_dir)
}

pub fn write_ai_acceptance_report(path: &Path, out_path: &Path) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    let report = render_ai_acceptance_report(&repo_root, path)?;
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    fs::write(out_path, report)
        .map_err(|err| format!("failed to write {}: {err}", out_path.display()))?;
    Ok(format!(
        "ai_acceptance_report_written:\nroot: {}\nreport: {}\n",
        path.display(),
        out_path.display()
    ))
}

pub fn list_slices() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_slice_inventory(&repo_root)
}

pub fn slice_review(slice_id: &str) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_slice_review(&repo_root, slice_id)
}

pub fn list_scenarios() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_scenario_inventory(&repo_root)
}

pub fn scenario_review(scenario_id: &str) -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_scenario_review(&repo_root, scenario_id)
}

pub fn list_work_packages() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_work_package_inventory(&repo_root)
}

pub fn fixture_readiness(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    Ok(render_fixture_readiness(&fixture, &report))
}

pub fn list_fixtures() -> Result<String, String> {
    let cwd = std::env::current_dir().map_err(|err| format!("failed to read cwd: {err}"))?;
    let repo_root = find_repo_root(&cwd)?;
    render_fixture_inventory(&repo_root)
}

pub fn fixture_review(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    let graph_output_posture = if report.has_errors() {
        "preview_only_not_promoted"
    } else {
        "validated"
    };
    let graph = build_graph_slice(&fixture, graph_output_posture, report.diagnostics.len());
    Ok(render_fixture_review(
        &fixture,
        &source_index,
        &report,
        &graph,
    ))
}

pub fn explain_diagnostics(path: &Path) -> Result<String, String> {
    let (fixture, source_index) = load_fixture_context(path)?;
    let report = validate_fixture_record(&fixture, &source_index);
    Ok(render_diagnostic_explanation(&fixture, &report))
}

fn render_graph(graph: &GraphSlice, format: GraphFormat) -> Result<String, String> {
    match format {
        GraphFormat::Json => serde_json::to_string_pretty(&graph)
            .map_err(|err| format!("failed to render JSON graph: {err}")),
        GraphFormat::Dot => Ok(render_dot(&graph)),
    }
}

fn load_fixture_context(path: &Path) -> Result<(Fixture, SourceCustodyIndex), String> {
    let text = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    let fixture: Fixture = serde_yaml::from_str(&text)
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
    let repo_root = find_repo_root(path)?;
    let source_index = SourceCustodyIndex::load(&repo_root)?;
    Ok((fixture, source_index))
}

fn validate_fixture_record(
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
) -> ValidationReport {
    let mut report = ValidationReport {
        fixture_id: fixture.fixture_id.clone(),
        diagnostics: Vec::new(),
    };

    if fixture.status.contains("invalid") || !fixture.promotion_blockers.is_empty() {
        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-SRC-000",
            family: "source_custody",
            severity: Severity::Error,
            affected: fixture.fixture_id.clone(),
            message: "fixture is blocked or invalid by design and cannot be promoted".to_string(),
        });
    }

    if fixture.scope.is_none()
        && (!fixture.nodes.all_node_ids().is_empty()
            || !fixture.relationship_edges.is_empty()
            || !fixture.graph_outputs.is_empty())
    {
        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-SCOPE-001",
            family: "scope",
            severity: Severity::Error,
            affected: fixture.fixture_id.clone(),
            message: "graph-bearing fixture must declare a bounded scope".to_string(),
        });
    }

    for source in &fixture.source_records {
        validate_linked_source_state(
            source_index,
            &source.source_ref,
            &source.contract_state,
            &mut report,
        );

        if source.contract_state != "accepted_for_slice" {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-SRC-001",
                family: "source_custody",
                severity: Severity::Error,
                affected: source.source_ref.clone(),
                message: format!(
                    "source state '{}' cannot support source-backed claims",
                    source.contract_state
                ),
            });
        }

        if source.may_support_claims && source.contract_state != "accepted_for_slice" {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-SRC-004",
                family: "source_custody",
                severity: Severity::Error,
                affected: source.source_ref.clone(),
                message: "non-accepted source is marked as claim-supporting".to_string(),
            });
        }
    }

    validate_graph_model(&fixture, source_index, &mut report);

    for source in &fixture.deferred_records {
        validate_linked_source_state(
            source_index,
            &source.source_ref,
            &source.contract_state,
            &mut report,
        );

        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-SRC-002",
            family: "source_custody",
            severity: Severity::Error,
            affected: source.source_ref.clone(),
            message: format!(
                "deferred source state '{}' cannot appear as accepted evidence",
                source.contract_state
            ),
        });
    }

    if fixture.source_text_included
        && fixture
            .source_text_redistribution_posture
            .contains("pointer_only")
    {
        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-SRC-003",
            family: "source_custody",
            severity: Severity::Error,
            affected: fixture.fixture_id.clone(),
            message: "source text is included under pointer-only redistribution posture"
                .to_string(),
        });
    }

    if !fixture.language_claims.is_empty() || !fixture.relationship_claims.is_empty() {
        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-CLAIM-001",
            family: "claim_type",
            severity: Severity::Error,
            affected: fixture.fixture_id.clone(),
            message:
                "draft source-pointer fixture must not contain language or relationship claims"
                    .to_string(),
        });
    }

    if fixture
        .graph_outputs
        .iter()
        .any(|output| output.status.contains("blocked"))
    {
        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-GRAPH-001",
            family: "graph_preservation",
            severity: Severity::Error,
            affected: fixture.fixture_id.clone(),
            message: "graph output is blocked while fixture validation fails".to_string(),
        });
    }

    if !fixture.chronicle_outputs.is_empty() {
        report.diagnostics.push(Diagnostic {
            id: "LEXIS-DIAG-CHRON-001",
            family: "chronicle_overclaim",
            severity: Severity::Error,
            affected: fixture.fixture_id.clone(),
            message: "draft source-pointer fixture must not contain chronicle output".to_string(),
        });
        if fixture
            .chronicle_outputs
            .iter()
            .any(value_contains_overclaim)
        {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-CHRON-002",
                family: "chronicle_overclaim",
                severity: Severity::Error,
                affected: fixture.fixture_id.clone(),
                message:
                    "chronicle output uses overclaim wording for a candidate or blocked fixture"
                        .to_string(),
            });
        }
    }

    report
}

fn validate_graph_model(
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
    report: &mut ValidationReport,
) {
    let mut node_ids = HashSet::new();
    for (id, class_name) in fixture.nodes.all_node_ids() {
        if id.trim().is_empty() {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-NODE-002",
                family: "claim_type",
                severity: Severity::Error,
                affected: class_name.to_string(),
                message: "node is missing required id".to_string(),
            });
        }
        if !node_ids.insert(id.to_string()) {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-NODE-001",
                family: "claim_type",
                severity: Severity::Error,
                affected: id.to_string(),
                message: "duplicate node id".to_string(),
            });
        }
    }
    validate_claim_node_source_links(fixture, &mut report.diagnostics);
    validate_root_posture(fixture, &mut report.diagnostics);

    let source_link_ids: HashSet<&str> = fixture
        .source_links
        .iter()
        .map(|link| link.id.as_str())
        .collect();
    for link in &fixture.source_links {
        validate_linked_source_state(source_index, &link.source_ref, &link.contract_state, report);
        if link.contract_state != "accepted_for_slice" {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-003",
                family: "relationship",
                severity: Severity::Error,
                affected: link.id.clone(),
                message: format!(
                    "source link '{}' uses non-accepted source state '{}'",
                    link.id, link.contract_state
                ),
            });
        }
    }

    let valid_edge_kinds = [
        "attested_as",
        "descends_from",
        "cognate_with",
        "borrowed_from",
        "calque_of",
        "sound_shift_to",
        "meaning_shift_to",
        "script_variant_of",
        "supports_claim",
        "disputes_claim",
    ];
    for edge in &fixture.relationship_edges {
        if !node_ids.contains(edge.source_id.as_str())
            && !source_link_ids.contains(edge.source_id.as_str())
        {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-001",
                family: "relationship",
                severity: Severity::Error,
                affected: edge.id.clone(),
                message: format!("edge source '{}' does not resolve", edge.source_id),
            });
        }
        if !node_ids.contains(edge.target_id.as_str())
            && !source_link_ids.contains(edge.target_id.as_str())
        {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-001",
                family: "relationship",
                severity: Severity::Error,
                affected: edge.id.clone(),
                message: format!("edge target '{}' does not resolve", edge.target_id),
            });
        }
        if !valid_edge_kinds.contains(&edge.edge_kind.as_str()) {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-002",
                family: "relationship",
                severity: Severity::Error,
                affected: edge.id.clone(),
                message: format!("unsupported edge kind '{}'", edge.edge_kind),
            });
        }
        if edge.supporting_sources.is_empty() {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-005",
                family: "relationship",
                severity: Severity::Error,
                affected: edge.id.clone(),
                message: "relationship edge is missing supporting source references".to_string(),
            });
        }
        if edge.edge_kind == "disputes_claim"
            && (edge.claim_type != "rejected_alternative" || edge.uncertainty != "disputed")
        {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-006",
                family: "relationship",
                severity: Severity::Error,
                affected: edge.id.clone(),
                message:
                    "disputes_claim edge must use rejected_alternative claim type and disputed uncertainty"
                        .to_string(),
            });
        }
        for source_ref in &edge.supporting_sources {
            match source_index.records.get(source_ref) {
                Some(record) if record.status == "accepted_for_slice" => {}
                Some(record) => report.diagnostics.push(Diagnostic {
                    id: "LEXIS-DIAG-EDGE-003",
                    family: "relationship",
                    severity: Severity::Error,
                    affected: edge.id.clone(),
                    message: format!(
                        "edge uses non-accepted source '{}' with state '{}'",
                        source_ref, record.status
                    ),
                }),
                None => report.diagnostics.push(Diagnostic {
                    id: "LEXIS-DIAG-SRC-006",
                    family: "source_custody",
                    severity: Severity::Error,
                    affected: source_ref.clone(),
                    message: "linked source-custody decision record was not found".to_string(),
                }),
            }
        }
    }

    let mut lineage_pairs: HashMap<(&str, &str), HashSet<&str>> = HashMap::new();
    for edge in &fixture.relationship_edges {
        if edge.edge_kind == "borrowed_from" || edge.edge_kind == "descends_from" {
            lineage_pairs
                .entry((edge.source_id.as_str(), edge.target_id.as_str()))
                .or_default()
                .insert(edge.edge_kind.as_str());
        }
    }
    for ((source_id, target_id), edge_kinds) in lineage_pairs {
        if edge_kinds.contains("borrowed_from") && edge_kinds.contains("descends_from") {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-EDGE-004",
                family: "relationship",
                severity: Severity::Error,
                affected: format!("{source_id}->{target_id}"),
                message:
                    "relationship collapses borrowing and descent for the same ordered node pair"
                        .to_string(),
            });
        }
    }
}

fn validate_claim_node_source_links(fixture: &Fixture, diagnostics: &mut Vec<Diagnostic>) {
    for root in &fixture.nodes.roots {
        if root.source_links.is_empty() {
            diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-NODE-003",
                family: "claim_type",
                severity: Severity::Error,
                affected: root.id.clone(),
                message: "claim-bearing root node is missing source links".to_string(),
            });
        }
    }
    for wordform in &fixture.nodes.wordforms {
        if wordform.source_links.is_empty() {
            diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-NODE-003",
                family: "claim_type",
                severity: Severity::Error,
                affected: wordform.id.clone(),
                message: "claim-bearing wordform node is missing source links".to_string(),
            });
        }
    }
    for sense in &fixture.nodes.meaning_senses {
        if sense.source_links.is_empty() {
            diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-NODE-003",
                family: "claim_type",
                severity: Severity::Error,
                affected: sense.id.clone(),
                message: "claim-bearing meaning sense node is missing source links".to_string(),
            });
        }
    }
}

fn validate_root_posture(fixture: &Fixture, diagnostics: &mut Vec<Diagnostic>) {
    for root in &fixture.nodes.roots {
        if root.claim_type != "reconstruction"
            && root.uncertainty != "source_limited"
            && root.uncertainty != "disputed"
        {
            diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-NODE-004",
                family: "claim_type",
                severity: Severity::Error,
                affected: root.id.clone(),
                message:
                    "root node must be marked as reconstruction or carry source-limited/disputed uncertainty"
                        .to_string(),
            });
        }
    }
}

fn value_contains_overclaim(value: &serde_yaml::Value) -> bool {
    match value {
        serde_yaml::Value::String(text) => {
            let lower = text.to_ascii_lowercase();
            lower.contains("proves")
                || lower.contains("proved")
                || lower.contains("proven")
                || lower.contains("settled fact")
        }
        serde_yaml::Value::Sequence(items) => items.iter().any(value_contains_overclaim),
        serde_yaml::Value::Mapping(map) => map
            .iter()
            .any(|(key, value)| value_contains_overclaim(key) || value_contains_overclaim(value)),
        _ => false,
    }
}

impl NodeSet {
    fn all_node_ids(&self) -> Vec<(&str, &'static str)> {
        self.languages
            .iter()
            .map(|node| (node.id.as_str(), "Language"))
            .chain(self.roots.iter().map(|node| (node.id.as_str(), "Root")))
            .chain(
                self.wordforms
                    .iter()
                    .map(|node| (node.id.as_str(), "Wordform")),
            )
            .chain(
                self.meaning_senses
                    .iter()
                    .map(|node| (node.id.as_str(), "MeaningSense")),
            )
            .chain(
                self.script_forms
                    .iter()
                    .map(|node| (node.id.as_str(), "ScriptForm")),
            )
            .collect()
    }
}

fn build_graph_slice(
    fixture: &Fixture,
    graph_output_posture: &'static str,
    validation_error_count: usize,
) -> GraphSlice {
    let mut nodes = Vec::new();
    nodes.extend(fixture.nodes.languages.iter().map(|node| GraphNode {
        id: node.id.clone(),
        label: node.label.clone(),
        record_class: "Language",
        claim_type: None,
        uncertainty: None,
        source_posture: node.source_posture.clone(),
    }));
    nodes.extend(fixture.nodes.roots.iter().map(|node| GraphNode {
        id: node.id.clone(),
        label: node.label.clone(),
        record_class: "Root",
        claim_type: Some(node.claim_type.clone()),
        uncertainty: Some(node.uncertainty.clone()),
        source_posture: source_posture_for_links(&node.source_links, fixture),
    }));
    nodes.extend(fixture.nodes.wordforms.iter().map(|node| GraphNode {
        id: node.id.clone(),
        label: node.label.clone(),
        record_class: "Wordform",
        claim_type: Some(node.claim_type.clone()),
        uncertainty: Some(node.uncertainty.clone()),
        source_posture: source_posture_for_links(&node.source_links, fixture),
    }));
    nodes.extend(fixture.nodes.meaning_senses.iter().map(|node| GraphNode {
        id: node.id.clone(),
        label: node.label.clone(),
        record_class: "MeaningSense",
        claim_type: Some(node.claim_type.clone()),
        uncertainty: Some(node.uncertainty.clone()),
        source_posture: source_posture_for_links(&node.source_links, fixture),
    }));
    nodes.extend(fixture.nodes.script_forms.iter().map(|node| GraphNode {
        id: node.id.clone(),
        label: node.label.clone(),
        record_class: "ScriptForm",
        claim_type: None,
        uncertainty: None,
        source_posture: source_posture_for_links(&node.source_links, fixture),
    }));

    GraphSlice {
        slice_id: fixture.fixture_id.clone(),
        nodes,
        edges: fixture.relationship_edges.clone(),
        source_posture_summary: fixture
            .source_records
            .iter()
            .map(|source| format!("{}:{}", source.source_ref, source.contract_state))
            .chain(
                fixture
                    .deferred_records
                    .iter()
                    .map(|source| format!("{}:{}", source.source_ref, source.contract_state)),
            )
            .collect(),
        graph_engine_posture: "local-only",
        graph_output_posture,
        validation_error_count,
    }
}

fn source_posture_for_links(source_link_ids: &[String], fixture: &Fixture) -> String {
    let postures: Vec<&str> = source_link_ids
        .iter()
        .filter_map(|id| {
            fixture
                .source_links
                .iter()
                .find(|link| link.id == *id)
                .map(|link| link.contract_state.as_str())
        })
        .collect();
    if postures.is_empty() {
        "unavailable".to_string()
    } else {
        postures.join(",")
    }
}

fn render_dot(graph: &GraphSlice) -> String {
    let mut output = String::from("digraph lexis_slice {\n");
    output.push_str(&format!(
        "  graph [label=\"{}\\n{}\\nvalidation errors: {}\"];\n",
        escape_dot(&graph.slice_id),
        graph.graph_output_posture,
        graph.validation_error_count
    ));
    for node in &graph.nodes {
        output.push_str(&format!(
            "  \"{}\" [label=\"{}\\n{}\\n{}\"];\n",
            escape_dot(&node.id),
            escape_dot(&node.label),
            node.record_class,
            escape_dot(&node.source_posture)
        ));
    }
    for edge in &graph.edges {
        output.push_str(&format!(
            "  \"{}\" -> \"{}\" [label=\"{}\\n{}\\n{}\"];\n",
            escape_dot(&edge.source_id),
            escape_dot(&edge.target_id),
            escape_dot(&edge.edge_kind),
            escape_dot(&edge.claim_type),
            escape_dot(&edge.uncertainty)
        ));
    }
    output.push_str("}\n");
    output
}

fn render_inspection(graph: &GraphSlice) -> String {
    let mut output = String::new();
    output.push_str(&format!("fixture: {}\n", graph.slice_id));
    output.push_str(&format!("status: {}\n", graph.graph_output_posture));
    output.push_str(&format!(
        "validation_errors: {}\n",
        graph.validation_error_count
    ));

    output.push_str("nodes:\n");
    if graph.nodes.is_empty() {
        output.push_str("  none\n");
    }
    for node in &graph.nodes {
        output.push_str(&format!(
            "  {} [{}] {} | source={}",
            node.id, node.record_class, node.label, node.source_posture
        ));
        if let Some(claim_type) = &node.claim_type {
            output.push_str(&format!(" | claim={claim_type}"));
        }
        if let Some(uncertainty) = &node.uncertainty {
            output.push_str(&format!(" | uncertainty={uncertainty}"));
        }
        output.push('\n');
    }

    output.push_str("edges:\n");
    if graph.edges.is_empty() {
        output.push_str("  none\n");
    }
    for edge in &graph.edges {
        output.push_str(&format!(
            "  {}: {} -> {} | kind={} | claim={} | uncertainty={} | review={}\n",
            edge.id,
            edge.source_id,
            edge.target_id,
            edge.edge_kind,
            edge.claim_type,
            edge.uncertainty,
            edge.review_state
        ));
    }

    output.push_str("source_posture_summary:\n");
    if graph.source_posture_summary.is_empty() {
        output.push_str("  none\n");
    }
    for posture in &graph.source_posture_summary {
        output.push_str(&format!("  {posture}\n"));
    }

    output
}

fn render_source_status(
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
    validation_error_count: usize,
) -> String {
    let mut output = String::new();
    output.push_str(&format!("source_status: {}\n", fixture.fixture_id));
    output.push_str(&format!("fixture_status: {}\n", fixture.status));
    output.push_str(&format!("validation_errors: {validation_error_count}\n"));
    output.push_str(&format!(
        "source_text_included: {}\n",
        fixture.source_text_included
    ));
    output.push_str(&format!(
        "redistribution_posture: {}\n",
        fixture.source_text_redistribution_posture
    ));

    output.push_str("source_records:\n");
    if fixture.source_records.is_empty() {
        output.push_str("  none\n");
    }
    for source in &fixture.source_records {
        let custody_state = source_index
            .records
            .get(&source.source_ref)
            .map(|record| record.status.as_str())
            .unwrap_or("missing");
        let match_state = if custody_state == source.contract_state {
            "matched"
        } else {
            "mismatch"
        };
        output.push_str(&format!(
            "  {} | fixture_state={} | custody_state={} | supports_claims={} | {}\n",
            source.source_ref,
            source.contract_state,
            custody_state,
            source.may_support_claims,
            match_state
        ));
    }

    output.push_str("deferred_records:\n");
    if fixture.deferred_records.is_empty() {
        output.push_str("  none\n");
    }
    for source in &fixture.deferred_records {
        let custody_state = source_index
            .records
            .get(&source.source_ref)
            .map(|record| record.status.as_str())
            .unwrap_or("missing");
        let match_state = if custody_state == source.contract_state {
            "matched"
        } else {
            "mismatch"
        };
        output.push_str(&format!(
            "  {} | fixture_state={} | custody_state={} | supports_claims=false | {}\n",
            source.source_ref, source.contract_state, custody_state, match_state
        ));
    }

    output.push_str("custody_load_errors:\n");
    if source_index.load_errors.is_empty() {
        output.push_str("  none\n");
    } else {
        for err in &source_index.load_errors {
            output.push_str(&format!("  {err}\n"));
        }
    }

    output.push_str("review_note:\n");
    output.push_str("  Candidate, deferred, missing, or mismatched sources cannot support promoted graph or chronicle claims.\n");
    output
}

fn render_source_inventory(source_index: &SourceCustodyIndex) -> String {
    let mut records: Vec<&SourceCustodyRecord> = source_index.records.values().collect();
    records.sort_by(|left, right| left.decision_id.cmp(&right.decision_id));

    let mut output = String::new();
    output.push_str("source_inventory:\n");
    output.push_str(&format!("count: {}\n", records.len()));
    output.push_str("sources:\n");
    if records.is_empty() {
        output.push_str("  none\n");
    }
    for record in records {
        output.push_str(&format!("  {}:\n", record.decision_id));
        output.push_str(&format!("    status: {}\n", record.status));
        output.push_str(&format!(
            "    family: {}\n",
            record.source_family.as_deref().unwrap_or("unrecorded")
        ));
        output.push_str(&format!(
            "    review_state: {}\n",
            record.review_state.as_deref().unwrap_or("unrecorded")
        ));
        output.push_str(&format!(
            "    promotion_allowed: {}\n",
            record.promotion_allowed.unwrap_or(false)
        ));
        output.push_str(&format!(
            "    redistribution_posture: {}\n",
            record
                .redistribution_posture
                .as_deref()
                .unwrap_or("unrecorded")
        ));
        output.push_str(&format!("    blocks: {}\n", record.blocks.len()));
    }

    output.push_str("custody_load_errors:\n");
    if source_index.load_errors.is_empty() {
        output.push_str("  none\n");
    } else {
        for err in &source_index.load_errors {
            output.push_str(&format!("  {err}\n"));
        }
    }
    output
}

fn render_source_review(
    repo_root: &Path,
    source_index: &SourceCustodyIndex,
    source_id: &str,
) -> Result<String, String> {
    let record = source_index
        .records
        .get(source_id)
        .ok_or_else(|| format!("source-custody decision '{}' was not found", source_id))?;
    let fixture_refs: Vec<FixtureManifest> = load_fixture_manifests(repo_root)?
        .into_iter()
        .filter(|manifest| {
            manifest
                .linked_source_custody_decision
                .iter()
                .any(|linked| linked == source_id)
        })
        .collect();

    let mut output = String::new();
    output.push_str(&format!("source_review: {}\n", record.decision_id));
    output.push_str(&format!("status: {}\n", record.status));
    output.push_str(&format!(
        "family: {}\n",
        record.source_family.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "candidate_source_name: {}\n",
        record
            .candidate_source_name
            .as_deref()
            .unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "pointer: {}\n",
        record.pointer.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "rights_posture: {}\n",
        record.rights_posture.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "redistribution_posture: {}\n",
        record
            .redistribution_posture
            .as_deref()
            .unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "review_state: {}\n",
        record.review_state.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "reviewer: {}\n",
        record.reviewer.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str(&format!(
        "promotion_allowed: {}\n",
        record.promotion_allowed.unwrap_or(false)
    ));
    output.push_str("blocks:\n");
    if record.blocks.is_empty() {
        output.push_str("  none\n");
    } else {
        for blocker in &record.blocks {
            output.push_str(&format!("  {blocker}\n"));
        }
    }
    output.push_str("referencing_fixtures:\n");
    if fixture_refs.is_empty() {
        output.push_str("  none\n");
    } else {
        for manifest in fixture_refs {
            output.push_str(&format!(
                "  {} | status={} | review_state={} | expected_result={}\n",
                manifest.fixture_id,
                manifest.status,
                manifest.review_state.as_deref().unwrap_or("unrecorded"),
                manifest.expected_result.as_deref().unwrap_or("unrecorded")
            ));
        }
    }
    output.push_str("citation_note:\n");
    output.push_str(&format!(
        "  {}\n",
        record.citation_note.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str("review_note:\n");
    output.push_str("  This source review packet is pointer-only and does not accept, ingest, cache, or redistribute source text.\n");
    Ok(output)
}

fn render_correction_inventory(repo_root: &Path) -> Result<String, String> {
    let plans = load_correction_plans(repo_root)?;
    let mut entries: Vec<(&CorrectionPlan, &CorrectionEntry)> = plans
        .iter()
        .flat_map(|plan| plan.entries.iter().map(move |entry| (plan, entry)))
        .collect();
    entries.sort_by(|(_, left), (_, right)| left.chain_id.cmp(&right.chain_id));

    let mut output = String::new();
    output.push_str("correction_inventory:\n");
    output.push_str(&format!("plans: {}\n", plans.len()));
    output.push_str(&format!("entries: {}\n", entries.len()));
    output.push_str("corrections:\n");
    if entries.is_empty() {
        output.push_str("  none\n");
    }
    for (plan, entry) in entries {
        output.push_str(&format!("  {}:\n", entry.chain_id));
        output.push_str(&format!("    plan: {}\n", plan.plan_id));
        output.push_str(&format!("    status: {}\n", plan.status));
        output.push_str(&format!("    action: {}\n", entry.action));
        output.push_str(&format!("    proof_source: {}\n", entry.proof_source));
        output.push_str(&format!(
            "    replacement_forms: {}\n",
            entry.replacement_forms.len()
        ));
        output.push_str(&format!(
            "    blockers: {}\n",
            entry.promotion_blockers.len()
        ));
    }
    Ok(output)
}

fn render_correction_review(repo_root: &Path, chain_id: &str) -> Result<String, String> {
    let normalized = normalize_chain_id(chain_id);
    let plans = load_correction_plans(repo_root)?;
    let source_index = SourceCustodyIndex::load(repo_root)?;
    let mut matches: Vec<(&CorrectionPlan, &CorrectionEntry)> = plans
        .iter()
        .flat_map(|plan| plan.entries.iter().map(move |entry| (plan, entry)))
        .filter(|(_, entry)| entry.chain_id == normalized)
        .collect();
    matches.sort_by(|(left_plan, _), (right_plan, _)| left_plan.plan_id.cmp(&right_plan.plan_id));

    let (plan, entry) = matches
        .first()
        .ok_or_else(|| format!("correction entry '{}' was not found", chain_id))?;
    let source_status = source_index
        .records
        .get(&entry.proof_source)
        .map(|source| source.status.as_str())
        .unwrap_or("missing_source_record");

    let mut output = String::new();
    output.push_str(&format!("correction_review: {}\n", entry.chain_id));
    output.push_str(&format!("plan: {}\n", plan.plan_id));
    output.push_str(&format!("plan_status: {}\n", plan.status));
    output.push_str(&format!("source_report: {}\n", plan.source_report));
    output.push_str("scope:\n");
    output.push_str(&format!("  {}\n", plan.scope.trim()));
    output.push_str(&format!("generated_seed: {}\n", entry.generated_seed));
    output.push_str(&format!("proof_source: {}\n", entry.proof_source));
    output.push_str(&format!("proof_source_status: {}\n", source_status));
    output.push_str(&format!("action: {}\n", entry.action));
    output.push_str("generated_path:\n");
    output.push_str(&format!("  {}\n", entry.generated_path));
    output.push_str("corrected_path:\n");
    output.push_str(&format!("  {}\n", entry.corrected_path));
    push_string_list(&mut output, "replacement_forms", &entry.replacement_forms);
    output.push_str("route_notes:\n");
    output.push_str(&format!("  {}\n", entry.route_notes.trim()));
    push_string_list(&mut output, "promotion_blockers", &entry.promotion_blockers);
    output.push_str("review_note:\n");
    output.push_str("  Correction plans are candidate regeneration instructions; they do not promote graph claims until corrected fixtures validate against accepted source decisions.\n");
    Ok(output)
}

fn write_correction_seed(
    repo_root: &Path,
    chain_id: &str,
    out_path: &Path,
) -> Result<String, String> {
    let normalized = normalize_chain_id(chain_id);
    let entry = find_correction_entry(repo_root, &normalized)?;
    let seed = build_correction_seed(&entry)?;
    write_seed_yaml(&seed, out_path)?;

    Ok(format!(
        "correction_seed_generated: {}\nchain_id: {}\nseed: {}\nforms: {}\nrelationships: {}\n",
        seed.slice_id,
        normalized,
        out_path.display(),
        seed.forms.len(),
        seed.relationships.len()
    ))
}

fn write_all_correction_seeds(repo_root: &Path, out_dir: &Path) -> Result<String, String> {
    let plans = load_correction_plans(repo_root)?;
    let mut entries: Vec<&CorrectionEntry> =
        plans.iter().flat_map(|plan| plan.entries.iter()).collect();
    entries.sort_by(|left, right| left.chain_id.cmp(&right.chain_id));

    fs::create_dir_all(out_dir)
        .map_err(|err| format!("failed to create {}: {err}", out_dir.display()))?;

    let mut output = String::new();
    output.push_str("correction_seed_batch:\n");
    output.push_str(&format!("out_dir: {}\n", out_dir.display()));
    output.push_str(&format!("count: {}\n", entries.len()));
    output.push_str("seeds:\n");

    for entry in entries {
        let seed = build_correction_seed(entry)?;
        let out_path = out_dir.join(format!(
            "{}-{}.yaml",
            entry.chain_id,
            slug(&entry.corrected_path)
        ));
        write_seed_yaml(&seed, &out_path)?;
        output.push_str(&format!(
            "  {} | path={} | forms={} | relationships={}\n",
            entry.chain_id,
            out_path.display(),
            seed.forms.len(),
            seed.relationships.len()
        ));
    }

    Ok(output)
}

fn find_correction_entry(repo_root: &Path, chain_id: &str) -> Result<CorrectionEntry, String> {
    let plans = load_correction_plans(repo_root)?;
    plans
        .into_iter()
        .flat_map(|plan| plan.entries.into_iter())
        .find(|entry| entry.chain_id == chain_id)
        .ok_or_else(|| format!("correction entry '{}' was not found", chain_id))
}

fn build_correction_seed(entry: &CorrectionEntry) -> Result<SliceSeed, String> {
    let route = parse_corrected_route(&entry.corrected_path)?;
    if route.len() < 2 {
        return Err(format!(
            "correction entry '{}' must produce at least two route stages",
            entry.chain_id
        ));
    }

    let source = SeedSource {
        id: entry.proof_source.clone(),
        state: Some("candidate_review".to_string()),
        may_support_claims: Some(false),
    };
    let mut forms = Vec::new();
    let mut primary_forms = Vec::new();

    for stage in &route {
        let mut stage_primary = None;
        for form in &stage.forms {
            let id = Some(format!("wf-{}-{}", slug(&stage.language), slug(form)));
            if stage_primary.is_none() {
                stage_primary = id.clone();
            }
            forms.push(SeedForm {
                form: form.clone(),
                language: stage.language.clone(),
                source: entry.proof_source.clone(),
                id,
                label: None,
                meaning: stage.meaning.clone(),
            });
        }
        if let Some(primary) = stage_primary {
            primary_forms.push((primary, stage.language.clone()));
        }
    }

    let mut relationships = Vec::new();
    for window in primary_forms.windows(2) {
        let (target_form, target_language) = &window[0];
        let (source_form, source_language) = &window[1];
        relationships.push(SeedRelationship {
            kind: if source_language == target_language {
                "descends_from".to_string()
            } else {
                "borrowed_from".to_string()
            },
            source: source_form.clone(),
            target: target_form.clone(),
            claim_type: Some("inference".to_string()),
            uncertainty: Some("source_limited".to_string()),
            support: vec![entry.proof_source.clone()],
            review_state: Some("candidate_review".to_string()),
        });
    }

    Ok(SliceSeed {
        slice_id: format!("corrected-candidate-{}", entry.chain_id),
        fixture_id: Some(format!("LEXIS-GEN-CORR-{}", entry.chain_id)),
        status: Some("candidate_review".to_string()),
        question: Some(format!(
            "Corrected graph-facing candidate route for generated Latin-English chain {}.",
            entry.chain_id
        )),
        source_state: Some("candidate_review".to_string()),
        sources: vec![source],
        forms,
        relationships,
    })
}

fn write_seed_yaml(seed: &SliceSeed, out_path: &Path) -> Result<(), String> {
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;
    }
    let yaml = serde_yaml::to_string(seed)
        .map_err(|err| format!("failed to serialize correction seed: {err}"))?;
    fs::write(out_path, yaml)
        .map_err(|err| format!("failed to write {}: {err}", out_path.display()))
}

#[derive(Debug)]
struct CorrectedRouteStage {
    forms: Vec<String>,
    language: String,
    meaning: Option<String>,
}

fn parse_corrected_route(route: &str) -> Result<Vec<CorrectedRouteStage>, String> {
    let raw_stages: Vec<&str> = route
        .split("->")
        .map(str::trim)
        .filter(|stage| !stage.is_empty())
        .collect();
    if raw_stages.is_empty() {
        return Err("corrected route is empty".to_string());
    }

    let mut stages = Vec::new();
    for (index, raw_stage) in raw_stages.iter().enumerate() {
        let (language, form_text) = infer_stage_language(raw_stage, index, raw_stages.len());
        let forms: Vec<String> = form_text
            .split('/')
            .map(str::trim)
            .filter(|form| !form.is_empty())
            .map(str::to_string)
            .collect();
        if forms.is_empty() {
            return Err(format!(
                "corrected route stage '{}' has no forms",
                raw_stage
            ));
        }
        stages.push(CorrectedRouteStage {
            forms,
            language,
            meaning: if index + 1 == raw_stages.len() {
                Some(
                    "Corrected candidate target meaning; source-detail review required."
                        .to_string(),
                )
            } else {
                None
            },
        });
    }
    Ok(stages)
}

fn infer_stage_language(raw_stage: &str, index: usize, total: usize) -> (String, String) {
    for (prefix, language) in [
        ("Old French ", "Old French"),
        ("Middle English ", "Middle English"),
        ("Late Latin ", "Late Latin"),
        ("French ", "French"),
        ("Latin ", "Latin"),
    ] {
        if let Some(form) = raw_stage.strip_prefix(prefix) {
            return (language.to_string(), form.trim().to_string());
        }
    }

    if index + 1 == total {
        let form = raw_stage
            .strip_suffix(" acoustic route")
            .unwrap_or(raw_stage)
            .trim()
            .to_string();
        ("Modern English".to_string(), form)
    } else if index == 0 || raw_stage.contains('/') {
        ("Latin".to_string(), raw_stage.to_string())
    } else {
        (
            "Unspecified intermediate".to_string(),
            raw_stage.to_string(),
        )
    }
}

fn render_slice_inventory(repo_root: &Path) -> Result<String, String> {
    let slices = build_slice_inventory(repo_root)?;

    let mut output = String::new();
    output.push_str("slice_inventory:\n");
    output.push_str(&format!("count: {}\n", slices.len()));
    output.push_str("slices:\n");
    if slices.is_empty() {
        output.push_str("  none\n");
        return Ok(output);
    }
    for (slice_id, entry) in slices {
        output.push_str(&format!("  {slice_id}:\n"));
        output.push_str(&format!("    title: {}\n", entry.title));
        output.push_str(&format!("    packages: {}\n", entry.packages.len()));
        output.push_str(&format!("    fixtures: {}\n", entry.fixtures.len()));
        if entry.fixtures.is_empty() {
            output.push_str("    fixture_refs: none\n");
        } else {
            output.push_str(&format!(
                "    fixture_refs: {}\n",
                entry.fixtures.join(", ")
            ));
        }
        output.push_str(&format!("    sources: {}\n", entry.sources.len()));
        if entry.sources.is_empty() {
            output.push_str("    source_refs: none\n");
        } else {
            output.push_str(&format!("    source_refs: {}\n", entry.sources.join(", ")));
        }
    }
    Ok(output)
}

fn build_slice_inventory(
    repo_root: &Path,
) -> Result<BTreeMap<String, SliceInventoryEntry>, String> {
    let slice_plan_path = repo_root.join("LANGUAGE_SLICE_PACKAGES.md");
    let text = fs::read_to_string(&slice_plan_path)
        .map_err(|err| format!("failed to read {}: {err}", slice_plan_path.display()))?;
    let mut slices = parse_slice_packages(&text);
    let fixture_manifests = load_fixture_manifests(repo_root)?;
    let source_index = SourceCustodyIndex::load(repo_root)?;

    for manifest in &fixture_manifests {
        for package in &manifest.linked_slice_packages {
            if let Some(prefix) = slice_prefix(package) {
                if let Some(entry) = slices.get_mut(prefix) {
                    entry.fixtures.push(manifest.fixture_id.clone());
                }
            }
        }
        if let Some(scope) = &manifest.linked_scope {
            if let Some(prefix) = slice_prefix(scope) {
                if let Some(entry) = slices.get_mut(prefix) {
                    entry.fixtures.push(manifest.fixture_id.clone());
                }
            }
        }
    }

    for record in source_index.records.values() {
        for package in &record.related_slice_packages {
            if let Some(prefix) = slice_prefix(package) {
                if let Some(entry) = slices.get_mut(prefix) {
                    entry.sources.push(record.decision_id.clone());
                }
            }
        }
    }

    let mut normalized = BTreeMap::new();
    for (slice_id, mut entry) in slices {
        entry.packages.sort();
        entry.packages.dedup();
        entry.fixtures.sort();
        entry.fixtures.dedup();
        entry.sources.sort();
        entry.sources.dedup();
        normalized.insert(slice_id, entry);
    }
    Ok(normalized)
}

fn render_slice_review(repo_root: &Path, slice_id: &str) -> Result<String, String> {
    let normalized = normalize_slice_id(slice_id);
    let slices = build_slice_inventory(repo_root)?;
    let entry = slices
        .get(normalized.as_str())
        .ok_or_else(|| format!("slice '{}' was not found", slice_id))?;
    let fixture_manifests = load_fixture_manifests(repo_root)?;
    let source_index = SourceCustodyIndex::load(repo_root)?;

    let mut output = String::new();
    output.push_str(&format!("slice_review: {}\n", normalized));
    output.push_str(&format!("title: {}\n", entry.title));

    output.push_str("packages:\n");
    for package in &entry.packages {
        output.push_str(&format!("  {package}\n"));
    }

    output.push_str("fixtures:\n");
    let linked_fixtures: Vec<&FixtureManifest> = fixture_manifests
        .iter()
        .filter(|manifest| entry.fixtures.contains(&manifest.fixture_id))
        .collect();
    if linked_fixtures.is_empty() {
        output.push_str("  none\n");
    } else {
        for manifest in linked_fixtures {
            output.push_str(&format!(
                "  {} | status={} | class={} | expected_result={} | blockers={}\n",
                manifest.fixture_id,
                manifest.status,
                manifest.fixture_class,
                manifest.expected_result.as_deref().unwrap_or("unrecorded"),
                manifest.promotion_blockers.len()
            ));
        }
    }

    output.push_str("sources:\n");
    let mut linked_sources: Vec<&SourceCustodyRecord> = source_index
        .records
        .values()
        .filter(|record| entry.sources.contains(&record.decision_id))
        .collect();
    linked_sources.sort_by(|left, right| left.decision_id.cmp(&right.decision_id));
    if linked_sources.is_empty() {
        output.push_str("  none\n");
    } else {
        for record in linked_sources {
            output.push_str(&format!(
                "  {} | status={} | family={} | promotion_allowed={}\n",
                record.decision_id,
                record.status,
                record.source_family.as_deref().unwrap_or("unrecorded"),
                record.promotion_allowed.unwrap_or(false)
            ));
        }
    }

    output.push_str("review_note:\n");
    output.push_str("  Slice review is planning inventory only; it does not accept sources, promote fixtures, emit graphs, or publish chronicles.\n");
    Ok(output)
}

fn render_scenario_inventory(repo_root: &Path) -> Result<String, String> {
    let scenarios = load_scenario_manifests(repo_root)?;
    let mut output = String::new();
    output.push_str("scenario_inventory:\n");
    output.push_str(&format!("count: {}\n", scenarios.len()));
    output.push_str("scenarios:\n");
    if scenarios.is_empty() {
        output.push_str("  none\n");
        return Ok(output);
    }
    for (path, scenario) in scenarios {
        let relative_path = path
            .strip_prefix(repo_root)
            .unwrap_or(path.as_path())
            .display();
        output.push_str(&format!("  {}:\n", scenario.scenario_id));
        output.push_str(&format!("    path: {relative_path}\n"));
        output.push_str(&format!("    status: {}\n", scenario.status));
        output.push_str(&format!("    actor: {}\n", scenario.actor));
        output.push_str(&format!("    slice_package: {}\n", scenario.slice_package));
        output.push_str(&format!(
            "    work_packages: {}\n",
            scenario.related_work_packages.len()
        ));
        output.push_str(&format!(
            "    diagnostics_expected: {}\n",
            scenario.diagnostics_expected.join(", ")
        ));
        output.push_str(&format!(
            "    fixture_candidates: {}\n",
            scenario.fixture_candidates.join(", ")
        ));
    }
    Ok(output)
}

fn render_scenario_review(repo_root: &Path, scenario_id: &str) -> Result<String, String> {
    let normalized = normalize_scenario_id(scenario_id);
    let scenarios = load_scenario_manifests(repo_root)?;
    let (path, scenario) = scenarios
        .into_iter()
        .find(|(_, scenario)| scenario.scenario_id == normalized)
        .ok_or_else(|| format!("scenario '{}' was not found", scenario_id))?;
    let relative_path = path
        .strip_prefix(repo_root)
        .unwrap_or(path.as_path())
        .display();

    let mut output = String::new();
    output.push_str(&format!("scenario_review: {}\n", scenario.scenario_id));
    output.push_str(&format!("path: {relative_path}\n"));
    output.push_str(&format!("status: {}\n", scenario.status));
    output.push_str(&format!("actor: {}\n", scenario.actor));
    output.push_str(&format!("slice_package: {}\n", scenario.slice_package));
    output.push_str("purpose:\n");
    output.push_str(&format!("  {}\n", scenario.purpose.trim()));

    push_string_list(
        &mut output,
        "related_work_packages",
        &scenario.related_work_packages,
    );
    push_string_list(&mut output, "specs_exercised", &scenario.specs_exercised);
    push_string_list(&mut output, "positive_path", &scenario.positive_path);
    push_string_list(&mut output, "negative_paths", &scenario.negative_paths);
    push_string_list(
        &mut output,
        "diagnostics_expected",
        &scenario.diagnostics_expected,
    );
    push_string_list(
        &mut output,
        "evidence_expected",
        &scenario.evidence_expected,
    );
    push_string_list(
        &mut output,
        "fixture_candidates",
        &scenario.fixture_candidates,
    );

    output.push_str(&format!(
        "findings_file: {}\n",
        scenario.findings_file.as_deref().unwrap_or("unrecorded")
    ));
    output.push_str("review_note:\n");
    output.push_str("  Scenario review is planning-only; it does not execute validation, promote fixtures, or create evidence findings.\n");
    Ok(output)
}

fn render_work_package_inventory(repo_root: &Path) -> Result<String, String> {
    let work_packages_path = repo_root
        .join("docs")
        .join("vtrace")
        .join("WORK_PACKAGES.md");
    let text = fs::read_to_string(&work_packages_path)
        .map_err(|err| format!("failed to read {}: {err}", work_packages_path.display()))?;
    let scenarios = load_scenario_manifests(repo_root)?;
    let mut packages = parse_work_packages(&text);

    for package in &mut packages {
        for (_, scenario) in &scenarios {
            if scenario
                .related_work_packages
                .iter()
                .any(|work_package| work_package == &package.id)
            {
                package.scenario_refs.push(scenario.scenario_id.clone());
            }
        }
        package.scenario_refs.sort();
    }

    let mut output = String::new();
    output.push_str("work_package_inventory:\n");
    output.push_str(&format!("count: {}\n", packages.len()));
    output.push_str("work_packages:\n");
    if packages.is_empty() {
        output.push_str("  none\n");
    }
    for package in packages {
        output.push_str(&format!("  {}:\n", package.id));
        output.push_str(&format!("    name: {}\n", package.name));
        output.push_str(&format!("    outcome: {}\n", package.outcome));
        output.push_str(&format!("    primary_gate: {}\n", package.primary_gate));
        if package.scenario_refs.is_empty() {
            output.push_str("    scenario_refs: none\n");
        } else {
            output.push_str(&format!(
                "    scenario_refs: {}\n",
                package.scenario_refs.join(", ")
            ));
        }
    }
    output.push_str("review_note:\n");
    output.push_str("  Work-package inventory is planning/status context; it does not mark package outputs promoted or release-ready.\n");
    Ok(output)
}

fn parse_work_packages(text: &str) -> Vec<WorkPackageEntry> {
    let mut packages = Vec::new();
    for line in text.lines() {
        if !line.starts_with("| LEXIS-WP-") {
            continue;
        }
        let columns: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if columns.len() < 4 {
            continue;
        }
        packages.push(WorkPackageEntry {
            id: columns[0].to_string(),
            name: columns[1].to_string(),
            outcome: columns[2].to_string(),
            primary_gate: columns[3].replace('`', ""),
            scenario_refs: Vec::new(),
        });
    }
    packages
}

fn load_scenario_manifests(repo_root: &Path) -> Result<Vec<(PathBuf, ScenarioManifest)>, String> {
    let scenarios_dir = repo_root.join("scenarios");
    let mut scenario_paths = Vec::new();
    collect_scenario_paths(&scenarios_dir, &mut scenario_paths)?;
    scenario_paths.sort();

    let mut scenarios = Vec::new();
    for path in scenario_paths {
        let text = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let scenario: ScenarioManifest = serde_yaml::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
        scenarios.push((path, scenario));
    }
    Ok(scenarios)
}

fn normalize_scenario_id(scenario_id: &str) -> String {
    if scenario_id.starts_with("LEXIS-SC-") {
        scenario_id.to_string()
    } else if let Ok(number) = scenario_id.parse::<u32>() {
        match number {
            1 => "LEXIS-SC-001-word-root-slice".to_string(),
            2 => "LEXIS-SC-002-borrowing-vs-descent".to_string(),
            3 => "LEXIS-SC-003-source-limited-claim".to_string(),
            4 => "LEXIS-SC-004-rline-preservation".to_string(),
            _ => format!("LEXIS-SC-{number:03}"),
        }
    } else {
        scenario_id.to_string()
    }
}

fn push_string_list(output: &mut String, label: &str, values: &[String]) {
    output.push_str(&format!("{label}:\n"));
    if values.is_empty() {
        output.push_str("  none\n");
    } else {
        for value in values {
            output.push_str(&format!("  {value}\n"));
        }
    }
}

fn collect_scenario_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))?;
    for entry in entries {
        let entry =
            entry.map_err(|err| format!("failed to read entry in {}: {err}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_scenario_paths(&path, paths)?;
        } else if path.file_name().and_then(|name| name.to_str()) == Some("scenario.yaml") {
            paths.push(path);
        }
    }
    Ok(())
}

fn parse_slice_packages(text: &str) -> BTreeMap<String, SliceInventoryEntry> {
    let mut slices = BTreeMap::new();
    let mut current_title = String::from("unrecorded");

    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## Slice Set ") {
            if let Some((number, name)) = title.split_once(':') {
                let slice_id = format!(
                    "LEXIS-SLICE-{:03}",
                    number.trim().parse::<u32>().unwrap_or(0)
                );
                current_title = name.trim().to_string();
                slices
                    .entry(slice_id)
                    .or_insert_with(|| SliceInventoryEntry {
                        title: current_title.clone(),
                        ..SliceInventoryEntry::default()
                    });
            }
        }

        for part in line.split('`') {
            if part.starts_with("LEXIS-SLICE-") {
                if let Some(prefix) = slice_prefix(part) {
                    let entry =
                        slices
                            .entry(prefix.to_string())
                            .or_insert_with(|| SliceInventoryEntry {
                                title: current_title.clone(),
                                ..SliceInventoryEntry::default()
                            });
                    if entry.title == "unrecorded" {
                        entry.title = current_title.clone();
                    }
                    entry.packages.push(part.to_string());
                }
            }
        }
    }

    slices
}

fn slice_prefix(package_id: &str) -> Option<&str> {
    if package_id.len() >= "LEXIS-SLICE-001".len() && package_id.starts_with("LEXIS-SLICE-") {
        Some(&package_id[.."LEXIS-SLICE-001".len()])
    } else {
        None
    }
}

fn normalize_slice_id(slice_id: &str) -> String {
    if slice_id.starts_with("LEXIS-SLICE-") {
        slice_id.to_string()
    } else if let Ok(number) = slice_id.parse::<u32>() {
        format!("LEXIS-SLICE-{number:03}")
    } else {
        slice_id.to_string()
    }
}

fn normalize_chain_id(chain_id: &str) -> String {
    if let Ok(number) = chain_id.parse::<u32>() {
        format!("{number:03}")
    } else {
        chain_id.to_string()
    }
}

fn load_fixture_manifests(repo_root: &Path) -> Result<Vec<FixtureManifest>, String> {
    let planned_dir = repo_root.join("fixtures").join("planned");
    let mut manifest_paths = Vec::new();
    collect_manifest_paths(&planned_dir, &mut manifest_paths)?;
    manifest_paths.sort();

    let mut manifests = Vec::new();
    for path in manifest_paths {
        let text = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let manifest: FixtureManifest = serde_yaml::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
        manifests.push(manifest);
    }
    Ok(manifests)
}

fn render_fixture_readiness(fixture: &Fixture, report: &ValidationReport) -> String {
    let mut diagnostic_counts = BTreeMap::new();
    for diagnostic in report.diagnostics() {
        *diagnostic_counts.entry(diagnostic.family).or_insert(0usize) += 1;
    }

    let source_ready =
        fixture.source_records.iter().all(|source| {
            source.contract_state == "accepted_for_slice" && source.may_support_claims
        }) && fixture.deferred_records.is_empty()
            && !fixture.source_records.is_empty();
    let graph_ready = !fixture.graph_outputs.is_empty()
        && fixture
            .graph_outputs
            .iter()
            .all(|output| !output.status.contains("blocked"));
    let chronicle_ready = !fixture.chronicle_outputs.is_empty();
    let blockers_clear = fixture.promotion_blockers.is_empty();
    let validation_ready = !report.has_errors();
    let promotable =
        validation_ready && source_ready && graph_ready && chronicle_ready && blockers_clear;

    let mut output = String::new();
    output.push_str(&format!("fixture_readiness: {}\n", fixture.fixture_id));
    output.push_str(&format!("fixture_status: {}\n", fixture.status));
    output.push_str(&format!(
        "promotion_ready: {}\n",
        if promotable { "yes" } else { "no" }
    ));
    output.push_str("gates:\n");
    output.push_str(&format!(
        "  validation: {}\n",
        readiness_label(validation_ready)
    ));
    output.push_str(&format!("  sources: {}\n", readiness_label(source_ready)));
    output.push_str(&format!("  graph: {}\n", readiness_label(graph_ready)));
    output.push_str(&format!(
        "  chronicle: {}\n",
        readiness_label(chronicle_ready)
    ));
    output.push_str(&format!(
        "  promotion_blockers: {}\n",
        readiness_label(blockers_clear)
    ));

    output.push_str("diagnostics:\n");
    output.push_str(&format!("  total: {}\n", report.diagnostics().len()));
    if diagnostic_counts.is_empty() {
        output.push_str("  none: 0\n");
    } else {
        for (family, count) in diagnostic_counts {
            output.push_str(&format!("  {family}: {count}\n"));
        }
    }

    output.push_str("blockers:\n");
    if fixture.promotion_blockers.is_empty() {
        output.push_str("  none\n");
    } else {
        for blocker in &fixture.promotion_blockers {
            output.push_str(&format!("  {blocker}\n"));
        }
    }

    output.push_str("next_actions:\n");
    if validation_ready {
        output.push_str("  validation diagnostics are clear\n");
    } else {
        output.push_str("  clear validation diagnostics before promotion\n");
    }
    if source_ready {
        output.push_str("  source records are accepted for slice\n");
    } else {
        output.push_str("  accept source records before graph or chronicle promotion\n");
    }
    if graph_ready {
        output.push_str("  graph outputs are not blocked\n");
    } else {
        output.push_str("  unblock graph outputs after validation passes\n");
    }
    if chronicle_ready {
        output.push_str("  chronicle output exists\n");
    } else {
        output.push_str("  add reviewed chronicle output after graph readiness\n");
    }
    output
}

fn render_fixture_inventory(repo_root: &Path) -> Result<String, String> {
    let planned_dir = repo_root.join("fixtures").join("planned");
    let mut manifest_paths = Vec::new();
    collect_manifest_paths(&planned_dir, &mut manifest_paths)?;
    manifest_paths.sort();

    let mut entries = Vec::new();
    for path in manifest_paths {
        let text = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let manifest: FixtureManifest = serde_yaml::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
        entries.push((path, manifest));
    }

    let mut output = String::new();
    output.push_str("fixture_inventory:\n");
    output.push_str(&format!("count: {}\n", entries.len()));
    output.push_str("fixtures:\n");
    if entries.is_empty() {
        output.push_str("  none\n");
        return Ok(output);
    }

    for (path, manifest) in entries {
        let relative_path = path
            .strip_prefix(repo_root)
            .unwrap_or(path.as_path())
            .display();
        output.push_str(&format!("  {}:\n", manifest.fixture_id));
        output.push_str(&format!("    path: {relative_path}\n"));
        output.push_str(&format!("    status: {}\n", manifest.status));
        output.push_str(&format!("    class: {}\n", manifest.fixture_class));
        output.push_str(&format!(
            "    work_package: {}\n",
            manifest
                .owning_work_package
                .as_deref()
                .unwrap_or("unassigned")
        ));
        output.push_str(&format!(
            "    linked_scope: {}\n",
            manifest.linked_scope.as_deref().unwrap_or("unassigned")
        ));
        output.push_str(&format!(
            "    fixture_shape: {}\n",
            manifest.fixture_shape.as_deref().unwrap_or("unrecorded")
        ));
        output.push_str(&format!(
            "    expected_result: {}\n",
            manifest.expected_result.as_deref().unwrap_or("unrecorded")
        ));
        output.push_str(&format!(
            "    review_state: {}\n",
            manifest.review_state.as_deref().unwrap_or("unrecorded")
        ));
        output.push_str(&format!(
            "    promotion_blockers: {}\n",
            manifest.promotion_blockers.len()
        ));
    }

    Ok(output)
}

fn render_artifact_inventory(repo_root: &Path) -> Result<String, String> {
    let planned_dir = repo_root.join("artifacts");
    let mut artifact_dirs = Vec::new();
    collect_artifact_dirs(&planned_dir, &mut artifact_dirs)?;
    artifact_dirs.sort();

    let mut entries = Vec::new();
    for dir in artifact_dirs {
        let graph_path = dir.join("graph-preview.json");
        let text = fs::read_to_string(&graph_path)
            .map_err(|err| format!("failed to read {}: {err}", graph_path.display()))?;
        let graph: serde_json::Value = serde_json::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", graph_path.display()))?;
        let slice_id = graph
            .get("slice_id")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let posture = graph
            .get("graph_output_posture")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let diagnostics = graph
            .get("validation_error_count")
            .and_then(|value| value.as_u64())
            .unwrap_or(0);
        let nodes = graph
            .get("nodes")
            .and_then(|value| value.as_array())
            .map(|items| items.len())
            .unwrap_or(0);
        let edges = graph
            .get("edges")
            .and_then(|value| value.as_array())
            .map(|items| items.len())
            .unwrap_or(0);
        entries.push((dir, slice_id, posture, diagnostics, nodes, edges));
    }

    let mut output = String::new();
    output.push_str("artifact_inventory:\n");
    output.push_str(&format!("count: {}\n", entries.len()));
    output.push_str("artifacts:\n");
    if entries.is_empty() {
        output.push_str("  none\n");
        return Ok(output);
    }

    for (dir, slice_id, posture, diagnostics, nodes, edges) in entries {
        let relative_dir = dir
            .strip_prefix(repo_root)
            .unwrap_or(dir.as_path())
            .display();
        output.push_str(&format!("  {slice_id}:\n"));
        output.push_str(&format!("    path: {relative_dir}\n"));
        output.push_str(&format!("    posture: {posture}\n"));
        output.push_str(&format!("    validation_errors: {diagnostics}\n"));
        output.push_str(&format!("    nodes: {nodes}\n"));
        output.push_str(&format!("    edges: {edges}\n"));
        output.push_str("    files:\n");
        output.push_str("      graph-preview.json\n");
        output.push_str("      graph-preview.dot\n");
        output.push_str("      chronicle-preview.md\n");
    }
    output.push_str("review_note:\n");
    output.push_str(
        "  Artifact inventory reports preview artifacts only; it does not promote source-backed claims.\n",
    );
    Ok(output)
}

fn render_artifact_corpus_summary(path: &Path) -> Result<String, String> {
    let mut artifact_dirs = Vec::new();
    if path.join("graph-preview.json").exists() {
        artifact_dirs.push(path.to_path_buf());
    } else {
        collect_artifact_dirs(path, &mut artifact_dirs)?;
    }
    artifact_dirs.sort();

    let mut graph_count = 0usize;
    let mut total_nodes = 0usize;
    let mut total_edges = 0usize;
    let mut total_validation_errors = 0u64;
    let mut max_validation_errors = 0u64;
    let mut posture_counts = BTreeMap::new();
    let mut node_class_counts = BTreeMap::new();
    let mut language_counts = BTreeMap::new();
    let mut edge_kind_counts = BTreeMap::new();
    let mut review_state_counts = BTreeMap::new();
    let mut source_posture_counts = BTreeMap::new();

    for dir in &artifact_dirs {
        let graph_path = dir.join("graph-preview.json");
        let text = fs::read_to_string(&graph_path)
            .map_err(|err| format!("failed to read {}: {err}", graph_path.display()))?;
        let graph: serde_json::Value = serde_json::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", graph_path.display()))?;

        graph_count += 1;
        let posture = graph
            .get("graph_output_posture")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown");
        increment_count(&mut posture_counts, posture);

        let validation_errors = graph
            .get("validation_error_count")
            .and_then(|value| value.as_u64())
            .unwrap_or(0);
        total_validation_errors += validation_errors;
        max_validation_errors = max_validation_errors.max(validation_errors);

        if let Some(nodes) = graph.get("nodes").and_then(|value| value.as_array()) {
            total_nodes += nodes.len();
            for node in nodes {
                let class = node
                    .get("record_class")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown");
                increment_count(&mut node_class_counts, class);
                let posture = node
                    .get("source_posture")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown");
                increment_count(&mut source_posture_counts, posture);
                if class == "Language" {
                    let label = node
                        .get("label")
                        .and_then(|value| value.as_str())
                        .unwrap_or("unknown");
                    increment_count(&mut language_counts, label);
                }
            }
        }

        if let Some(edges) = graph.get("edges").and_then(|value| value.as_array()) {
            total_edges += edges.len();
            for edge in edges {
                let kind = edge
                    .get("edge_kind")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown");
                increment_count(&mut edge_kind_counts, kind);
                let review_state = edge
                    .get("review_state")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown");
                increment_count(&mut review_state_counts, review_state);
            }
        }
    }

    let mut output = String::new();
    output.push_str("artifact_corpus_summary:\n");
    output.push_str(&format!("root: {}\n", path.display()));
    output.push_str(&format!("graphs: {graph_count}\n"));
    output.push_str(&format!("nodes: {total_nodes}\n"));
    output.push_str(&format!("edges: {total_edges}\n"));
    output.push_str(&format!(
        "validation_errors_total: {total_validation_errors}\n"
    ));
    output.push_str(&format!("validation_errors_max: {max_validation_errors}\n"));
    push_count_map(&mut output, "graph_postures", &posture_counts);
    push_count_map(&mut output, "node_classes", &node_class_counts);
    push_count_map(&mut output, "languages", &language_counts);
    push_count_map(&mut output, "edge_kinds", &edge_kind_counts);
    push_count_map(&mut output, "edge_review_states", &review_state_counts);
    push_count_map(&mut output, "node_source_postures", &source_posture_counts);
    output.push_str("review_note:\n");
    output.push_str("  Corpus summary is preview analysis only; it does not promote graph claims or accept source custody.\n");
    Ok(output)
}

fn render_artifact_corpus_report(path: &Path) -> Result<String, String> {
    let summaries = load_artifact_graph_summaries(path)?;
    let summary = render_artifact_corpus_summary(path)?;

    let mut output = String::new();
    output.push_str("# Artifact Corpus Analysis\n\n");
    output.push_str(&format!("Root: `{}`\n\n", path.display()));
    output.push_str("## Summary\n\n");
    output.push_str("```text\n");
    output.push_str(&summary);
    output.push_str("```\n\n");
    output.push_str("## Graphs\n\n");
    output.push_str("| Slice | Posture | Validation errors | Nodes | Edges | Artifact path |\n");
    output.push_str("|---|---|---:|---:|---:|---|\n");
    for item in &summaries {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | `{}` |\n",
            item.slice_id,
            item.posture,
            item.validation_errors,
            item.nodes,
            item.edges,
            item.path.display()
        ));
    }
    output.push_str("\n## Review Notes\n\n");
    output.push_str(
        "- This report summarizes preview artifacts only; it does not promote graph claims.\n",
    );
    output.push_str(
        "- `preview_only_not_promoted` graphs remain blocked until source custody and fixture validation are accepted.\n",
    );
    output.push_str(
        "- Use the highest validation-error rows as the first source-detail and fixture-correction targets.\n",
    );
    Ok(output)
}

fn render_correction_artifact_report(
    repo_root: &Path,
    artifact_path: &Path,
) -> Result<String, String> {
    let artifacts = load_artifact_graph_summaries(artifact_path)?;
    let plans = load_correction_plans(repo_root)?;
    let mut corrections = BTreeMap::new();
    for entry in plans.into_iter().flat_map(|plan| plan.entries.into_iter()) {
        corrections.insert(entry.chain_id.clone(), entry);
    }

    let mut joined = Vec::new();
    for artifact in artifacts {
        let chain_id = artifact
            .slice_id
            .strip_prefix("LEXIS-GEN-CORR-")
            .unwrap_or(&artifact.slice_id)
            .to_string();
        if let Some(entry) = corrections.get(&chain_id) {
            joined.push((artifact, entry));
        }
    }
    joined.sort_by(|(left_artifact, _), (right_artifact, _)| {
        right_artifact
            .validation_errors
            .cmp(&left_artifact.validation_errors)
            .then_with(|| left_artifact.slice_id.cmp(&right_artifact.slice_id))
    });

    let mut action_counts = BTreeMap::new();
    let mut total_validation_errors = 0u64;
    let mut max_validation_errors = 0u64;
    for (artifact, entry) in &joined {
        increment_count(&mut action_counts, &entry.action);
        total_validation_errors += artifact.validation_errors;
        max_validation_errors = max_validation_errors.max(artifact.validation_errors);
    }

    let mut output = String::new();
    output.push_str("# Corrected Tier 3 Promotion Worklist\n\n");
    output.push_str(&format!("Artifact root: `{}`\n\n", artifact_path.display()));
    output.push_str("## Summary\n\n");
    output.push_str(&format!("- corrected graphs joined: {}\n", joined.len()));
    output.push_str(&format!(
        "- validation errors total: {total_validation_errors}\n"
    ));
    output.push_str(&format!(
        "- validation errors max: {max_validation_errors}\n"
    ));
    output.push_str("- correction actions:\n");
    if action_counts.is_empty() {
        output.push_str("  - none\n");
    } else {
        for (action, count) in &action_counts {
            output.push_str(&format!("  - `{action}`: {count}\n"));
        }
    }

    output.push_str("\n## Ranked Worklist\n\n");
    output.push_str("| Rank | Chain | Action | Validation errors | Nodes | Edges | Proof source | First blocker |\n");
    output.push_str("|---:|---|---|---:|---:|---:|---|---|\n");
    for (index, (artifact, entry)) in joined.iter().enumerate() {
        let first_blocker = entry
            .promotion_blockers
            .first()
            .map(String::as_str)
            .unwrap_or("unrecorded");
        output.push_str(&format!(
            "| {} | `{}` | `{}` | {} | {} | {} | `{}` | {} |\n",
            index + 1,
            entry.chain_id,
            entry.action,
            artifact.validation_errors,
            artifact.nodes,
            artifact.edges,
            entry.proof_source,
            markdown_table_text(first_blocker)
        ));
    }

    output.push_str("\n## Promotion Guidance\n\n");
    output.push_str("- Start with rows tied at the maximum validation-error count; these represent the largest corrected graph shapes still blocked by candidate source state.\n");
    output.push_str("- For each row, promote only after the proof source is accepted for slice use and the correction blockers are resolved in fixture data.\n");
    output.push_str("- Compound and homonym actions should receive explicit edge labels before acceptance, not just source-state updates.\n");
    Ok(output)
}

fn render_ai_acceptance_report(repo_root: &Path, artifact_path: &Path) -> Result<String, String> {
    let artifacts = load_artifact_graph_summaries(artifact_path)?;
    let plans = load_correction_plans(repo_root)?;
    let source_index = SourceCustodyIndex::load(repo_root)?;
    let mut corrections = BTreeMap::new();
    for entry in plans.into_iter().flat_map(|plan| plan.entries.into_iter()) {
        corrections.insert(entry.chain_id.clone(), entry);
    }

    let mut rows = Vec::new();
    for artifact in artifacts {
        let chain_id = artifact
            .slice_id
            .strip_prefix("LEXIS-GEN-CORR-")
            .unwrap_or(&artifact.slice_id)
            .to_string();
        let Some(entry) = corrections.get(&chain_id) else {
            continue;
        };
        let source = source_index.records.get(&entry.proof_source);
        let source_status = source
            .map(|record| record.status.as_str())
            .unwrap_or("missing_source_record");
        let promotion_allowed = source
            .and_then(|record| record.promotion_allowed)
            .unwrap_or(false);
        let redistribution = source
            .and_then(|record| record.redistribution_posture.as_deref())
            .unwrap_or("unrecorded");

        let source_score = if source_status == "accepted_for_slice" && promotion_allowed {
            30
        } else if source_status == "candidate_review" {
            10
        } else {
            0
        };
        let validation_score = if artifact.validation_errors == 0 {
            25
        } else {
            0
        };
        let route_score = if entry.promotion_blockers.is_empty() {
            20
        } else if !entry.replacement_forms.is_empty() {
            12
        } else {
            6
        };
        let relationship_score = match entry.action.as_str() {
            "compound_route" | "split_homonym_route" => 6,
            _ => 8,
        };
        let rights_score = if redistribution == "pointer_only_planned" {
            8
        } else {
            0
        };
        let score =
            source_score + validation_score + route_score + relationship_score + rights_score;

        let mut hard_gates = Vec::new();
        if source_status != "accepted_for_slice" {
            hard_gates.push("source custody is not accepted_for_slice");
        }
        if !promotion_allowed {
            hard_gates.push("source custody promotion_allowed is false");
        }
        if artifact.validation_errors > 0 {
            hard_gates.push("fixture has validation errors");
        }
        if !entry.promotion_blockers.is_empty() {
            hard_gates.push("correction blockers remain unresolved");
        }
        if matches!(
            entry.action.as_str(),
            "compound_route" | "split_homonym_route"
        ) {
            hard_gates.push("compound or homonym route requires explicit human review");
        }

        let recommendation = if hard_gates.is_empty() && score >= 85 {
            "accept_for_human_promotion_review"
        } else if hard_gates.is_empty() {
            "revise_before_acceptance"
        } else {
            "block_promotion"
        };

        rows.push(AiAcceptanceRow {
            chain_id,
            action: entry.action.clone(),
            proof_source: entry.proof_source.clone(),
            source_status: source_status.to_string(),
            promotion_allowed,
            validation_errors: artifact.validation_errors,
            score,
            recommendation: recommendation.to_string(),
            first_gate: hard_gates
                .first()
                .copied()
                .unwrap_or("no hard gate failed")
                .to_string(),
        });
    }

    rows.sort_by(|left, right| {
        left.recommendation
            .cmp(&right.recommendation)
            .then_with(|| right.score.cmp(&left.score))
            .then_with(|| left.chain_id.cmp(&right.chain_id))
    });

    let mut recommendation_counts = BTreeMap::new();
    for row in &rows {
        increment_count(&mut recommendation_counts, &row.recommendation);
    }

    let mut output = String::new();
    output.push_str("# AI Advisory Acceptance Review\n\n");
    output.push_str(&format!("Artifact root: `{}`\n\n", artifact_path.display()));
    output.push_str("## Authority Boundary\n\n");
    output.push_str("AI acceptance is advisory. It cannot promote fixtures, accept source custody, override diagnostics, or authorize source redistribution.\n\n");
    output.push_str("## Rubric Summary\n\n");
    output.push_str("- Source custody: 30 points\n");
    output.push_str("- Fixture validation: 25 points\n");
    output.push_str("- Route correction: 20 points\n");
    output.push_str("- Relationship semantics: 15 points\n");
    output.push_str("- Rights and chronicle posture: 10 points\n\n");
    output.push_str("## Recommendation Counts\n\n");
    if recommendation_counts.is_empty() {
        output.push_str("- none\n");
    } else {
        for (recommendation, count) in &recommendation_counts {
            output.push_str(&format!("- `{recommendation}`: {count}\n"));
        }
    }
    output.push_str("\n## Chain Reviews\n\n");
    output.push_str("| Chain | Score | Recommendation | Action | Source status | Promotion allowed | Validation errors | Proof source | First hard gate |\n");
    output.push_str("|---|---:|---|---|---|---|---:|---|---|\n");
    for row in &rows {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | {} | {} | `{}` | {} |\n",
            row.chain_id,
            row.score,
            row.recommendation,
            row.action,
            row.source_status,
            row.promotion_allowed,
            row.validation_errors,
            row.proof_source,
            markdown_table_text(&row.first_gate)
        ));
    }
    output.push_str("\n## Current Tier 3 Advisory Finding\n\n");
    output.push_str("All corrected Tier 3 chains remain blocked from promotion because source custody is still `candidate_review`, promotion is not allowed, and fixtures still have validation errors. AI can rank and explain these chains, but none should be accepted yet.\n");
    Ok(output)
}

#[derive(Debug)]
struct AiAcceptanceRow {
    chain_id: String,
    action: String,
    proof_source: String,
    source_status: String,
    promotion_allowed: bool,
    validation_errors: u64,
    score: u64,
    recommendation: String,
    first_gate: String,
}

fn markdown_table_text(input: &str) -> String {
    input.replace('|', "\\|")
}

#[derive(Debug)]
struct ArtifactGraphSummary {
    path: PathBuf,
    slice_id: String,
    posture: String,
    validation_errors: u64,
    nodes: usize,
    edges: usize,
}

fn load_artifact_graph_summaries(path: &Path) -> Result<Vec<ArtifactGraphSummary>, String> {
    let mut artifact_dirs = Vec::new();
    if path.join("graph-preview.json").exists() {
        artifact_dirs.push(path.to_path_buf());
    } else {
        collect_artifact_dirs(path, &mut artifact_dirs)?;
    }
    artifact_dirs.sort();

    let mut summaries = Vec::new();
    for dir in artifact_dirs {
        let graph_path = dir.join("graph-preview.json");
        let text = fs::read_to_string(&graph_path)
            .map_err(|err| format!("failed to read {}: {err}", graph_path.display()))?;
        let graph: serde_json::Value = serde_json::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", graph_path.display()))?;
        let slice_id = graph
            .get("slice_id")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let posture = graph
            .get("graph_output_posture")
            .and_then(|value| value.as_str())
            .unwrap_or("unknown")
            .to_string();
        let validation_errors = graph
            .get("validation_error_count")
            .and_then(|value| value.as_u64())
            .unwrap_or(0);
        let nodes = graph
            .get("nodes")
            .and_then(|value| value.as_array())
            .map(|items| items.len())
            .unwrap_or(0);
        let edges = graph
            .get("edges")
            .and_then(|value| value.as_array())
            .map(|items| items.len())
            .unwrap_or(0);
        summaries.push(ArtifactGraphSummary {
            path: dir,
            slice_id,
            posture,
            validation_errors,
            nodes,
            edges,
        });
    }

    summaries.sort_by(|left, right| left.slice_id.cmp(&right.slice_id));
    Ok(summaries)
}

fn increment_count(map: &mut BTreeMap<String, usize>, key: &str) {
    *map.entry(key.to_string()).or_insert(0) += 1;
}

fn push_count_map(output: &mut String, label: &str, counts: &BTreeMap<String, usize>) {
    output.push_str(&format!("{label}:\n"));
    if counts.is_empty() {
        output.push_str("  none\n");
    } else {
        for (key, count) in counts {
            output.push_str(&format!("  {key}: {count}\n"));
        }
    }
}

fn collect_artifact_dirs(dir: &Path, dirs: &mut Vec<PathBuf>) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in
        fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))?
    {
        let entry = entry.map_err(|err| format!("failed to read directory entry: {err}"))?;
        let path = entry.path();
        if path.is_dir() {
            if path.join("graph-preview.json").exists() {
                dirs.push(path);
            } else {
                collect_artifact_dirs(&path, dirs)?;
            }
        }
    }
    Ok(())
}

fn collect_manifest_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|err| format!("failed to read {}: {err}", dir.display()))?;
    for entry in entries {
        let entry =
            entry.map_err(|err| format!("failed to read entry in {}: {err}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_manifest_paths(&path, paths)?;
        } else if path.file_name().and_then(|name| name.to_str()) == Some("manifest.yaml") {
            paths.push(path);
        }
    }
    Ok(())
}

fn render_fixture_review(
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
    report: &ValidationReport,
    graph: &GraphSlice,
) -> String {
    let mut output = String::new();
    let graph_accepted = graph.graph_output_posture == "validated" && !report.has_errors();
    let chronicle_published = !fixture.chronicle_outputs.is_empty();
    let review_packet_status = if graph_accepted && chronicle_published {
        "ready_for_publication_review"
    } else if graph_accepted {
        "validated_graph_chronicle_unpublished"
    } else {
        "preview_only_not_promoted"
    };

    output.push_str(&format!("fixture_review: {}\n", fixture.fixture_id));
    output.push_str(&format!("review_packet_status: {review_packet_status}\n"));

    output.push_str("\n== readiness ==\n");
    output.push_str(&render_fixture_readiness(fixture, report));

    output.push_str("\n== source_status ==\n");
    output.push_str(&render_source_status(
        fixture,
        source_index,
        report.diagnostics().len(),
    ));

    output.push_str("\n== diagnostics ==\n");
    output.push_str(&render_diagnostic_explanation(fixture, report));

    output.push_str("\n== graph_summary ==\n");
    output.push_str(&render_summary(graph));

    output.push_str("\n== chronicle_preview ==\n");
    output.push_str(&render_chronicle_preview(fixture, graph));

    output.push_str("\nreview_note:\n");
    if graph_accepted {
        output.push_str("  This packet reviews a validated graph slice. It does not publish a chronicle or broaden the accepted source scope.\n");
    } else {
        output.push_str("  This packet is for role review. It does not promote the fixture, accept sources, or publish a chronicle.\n");
    }
    output
}

fn readiness_label(ready: bool) -> &'static str {
    if ready {
        "ready"
    } else {
        "blocked"
    }
}

fn render_diagnostic_explanation(fixture: &Fixture, report: &ValidationReport) -> String {
    let mut grouped: BTreeMap<&str, Vec<&Diagnostic>> = BTreeMap::new();
    for diagnostic in report.diagnostics() {
        grouped
            .entry(diagnostic.family)
            .or_default()
            .push(diagnostic);
    }

    let mut output = String::new();
    output.push_str(&format!("diagnostics_explain: {}\n", fixture.fixture_id));
    output.push_str(&format!("fixture_status: {}\n", fixture.status));
    output.push_str(&format!("total: {}\n", report.diagnostics().len()));

    if grouped.is_empty() {
        output.push_str("status: valid\n");
        output.push_str("review_note:\n");
        output.push_str("  No diagnostics are present for this fixture.\n");
        return output;
    }

    output.push_str("groups:\n");
    for (family, diagnostics) in grouped {
        output.push_str(&format!("  {family}:\n"));
        for diagnostic in diagnostics {
            output.push_str(&format!(
                "    {} {} | affected={} | {}\n",
                diagnostic.severity, diagnostic.id, diagnostic.affected, diagnostic.message
            ));
        }
    }

    output.push_str("review_note:\n");
    output.push_str("  Treat error diagnostics as promotion blockers until the affected fixture, source, or relationship record is corrected and revalidated.\n");
    output
}

fn render_neighborhood_trace(
    fixture: &Fixture,
    node_id: &str,
    graph_output_posture: &'static str,
    validation_error_count: usize,
) -> String {
    let mut output = String::new();
    let node_label = graph_label_for_id(fixture, node_id).unwrap_or_else(|| "unresolved".into());
    output.push_str(&format!("neighborhood_trace: {node_id}\n"));
    output.push_str(&format!("fixture: {}\n", fixture.fixture_id));
    output.push_str(&format!("status: {graph_output_posture}\n"));
    output.push_str(&format!("validation_errors: {validation_error_count}\n"));
    output.push_str(&format!("center: {node_label} [{node_id}]\n"));

    output.push_str("source_links:\n");
    let source_links = source_links_for_node(fixture, node_id);
    if source_links.is_empty() {
        output.push_str("  none\n");
    }
    for link in source_links {
        output.push_str(&format!(
            "  {} -> {} | state={}\n",
            link.id, link.source_ref, link.contract_state
        ));
    }

    output.push_str("adjacent_edges:\n");
    let adjacent_edges: Vec<&RelationshipEdge> = fixture
        .relationship_edges
        .iter()
        .filter(|edge| edge.source_id == node_id || edge.target_id == node_id)
        .collect();
    if adjacent_edges.is_empty() {
        output.push_str("  none\n");
    }
    for edge in adjacent_edges {
        let direction = if edge.source_id == node_id {
            "outgoing"
        } else {
            "incoming"
        };
        let other_id = if edge.source_id == node_id {
            edge.target_id.as_str()
        } else {
            edge.source_id.as_str()
        };
        let other_label =
            graph_label_for_id(fixture, other_id).unwrap_or_else(|| "unresolved".into());
        output.push_str(&format!(
            "  {} {}: {} [{}] | kind={} | claim={} | uncertainty={} | review={}\n",
            direction,
            edge.id,
            other_label,
            other_id,
            edge.edge_kind,
            edge.claim_type,
            edge.uncertainty,
            edge.review_state
        ));
    }

    output.push_str("review_note:\n");
    output.push_str("  Neighborhood traces are fixture-local adjacency views and remain non-promoted while validation errors exist.\n");
    output
}

fn render_graph_path(
    fixture: &Fixture,
    start_id: &str,
    end_id: &str,
    graph_output_posture: &'static str,
    validation_error_count: usize,
) -> String {
    let start_label = graph_label_for_id(fixture, start_id).unwrap_or_else(|| "unresolved".into());
    let end_label = graph_label_for_id(fixture, end_id).unwrap_or_else(|| "unresolved".into());
    let path = shortest_connection_path(fixture, start_id, end_id);

    let mut output = String::new();
    output.push_str(&format!("graph_path: {start_id} -> {end_id}\n"));
    output.push_str(&format!("fixture: {}\n", fixture.fixture_id));
    output.push_str(&format!("status: {graph_output_posture}\n"));
    output.push_str(&format!("validation_errors: {validation_error_count}\n"));
    output.push_str(&format!("start: {start_label} [{start_id}]\n"));
    output.push_str(&format!("end: {end_label} [{end_id}]\n"));

    output.push_str("path_found: ");
    output.push_str(if path.is_empty() && start_id != end_id {
        "no\n"
    } else {
        "yes\n"
    });

    output.push_str("steps:\n");
    if start_id == end_id {
        output.push_str("  self\n");
    } else if path.is_empty() {
        output.push_str("  none\n");
    } else {
        for (index, step) in path.iter().enumerate() {
            let from_label =
                graph_label_for_id(fixture, &step.from_id).unwrap_or_else(|| "unresolved".into());
            let to_label =
                graph_label_for_id(fixture, &step.to_id).unwrap_or_else(|| "unresolved".into());
            let arrow = if step.forward {
                format!("--{}-->", step.edge.edge_kind)
            } else {
                format!("<--{}--", step.edge.edge_kind)
            };
            output.push_str(&format!(
                "  {}. {} [{}] {} {} [{}] | edge={} | claim={} | uncertainty={} | review={}\n",
                index + 1,
                from_label,
                step.from_id,
                arrow,
                to_label,
                step.to_id,
                step.edge.id,
                step.edge.claim_type,
                step.edge.uncertainty,
                step.edge.review_state
            ));
        }
    }

    output.push_str("review_note:\n");
    output.push_str("  Graph paths are fixture-local shortest connection paths; arrows show whether the stored edge direction was followed or traversed backward.\n");
    output
}

fn render_claim_explanation(
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
    claim_id: &str,
    graph_output_posture: &'static str,
    validation_error_count: usize,
) -> String {
    let mut output = String::new();
    output.push_str(&format!("claim_explain: {claim_id}\n"));
    output.push_str(&format!("fixture: {}\n", fixture.fixture_id));
    output.push_str(&format!("status: {graph_output_posture}\n"));
    output.push_str(&format!("validation_errors: {validation_error_count}\n"));

    if let Some(edge) = fixture
        .relationship_edges
        .iter()
        .find(|edge| edge.id == claim_id)
    {
        render_edge_claim_explanation(&mut output, fixture, source_index, edge);
    } else {
        render_node_claim_explanation(&mut output, fixture, source_index, claim_id);
    }

    output.push_str("review_note:\n");
    output.push_str("  Claim explanations are fixture-local evidence views and do not broaden the accepted source scope.\n");
    output
}

fn render_edge_claim_explanation(
    output: &mut String,
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
    edge: &RelationshipEdge,
) {
    let source_label =
        graph_label_for_id(fixture, &edge.source_id).unwrap_or_else(|| "unresolved".into());
    let target_label =
        graph_label_for_id(fixture, &edge.target_id).unwrap_or_else(|| "unresolved".into());

    output.push_str("claim_type: edge\n");
    output.push_str(&format!("edge_kind: {}\n", edge.edge_kind));
    output.push_str(&format!("from: {} [{}]\n", source_label, edge.source_id));
    output.push_str(&format!("to: {} [{}]\n", target_label, edge.target_id));
    output.push_str(&format!("claim: {}\n", edge.claim_type));
    output.push_str(&format!("uncertainty: {}\n", edge.uncertainty));
    output.push_str(&format!("review_state: {}\n", edge.review_state));
    output.push_str("supporting_sources:\n");
    if edge.supporting_sources.is_empty() {
        output.push_str("  none\n");
    }
    for source_ref in &edge.supporting_sources {
        let status = source_index
            .records
            .get(source_ref)
            .map(|record| record.status.as_str())
            .unwrap_or("missing");
        output.push_str(&format!("  {source_ref} | custody_state={status}\n"));
    }
}

fn render_node_claim_explanation(
    output: &mut String,
    fixture: &Fixture,
    source_index: &SourceCustodyIndex,
    node_id: &str,
) {
    let label = graph_label_for_id(fixture, node_id).unwrap_or_else(|| "unresolved".into());
    output.push_str("claim_type: node\n");
    output.push_str(&format!("label: {label}\n"));

    if let Some((record_class, claim_type, uncertainty, source_links)) =
        node_claim_fields(fixture, node_id)
    {
        output.push_str(&format!("record_class: {record_class}\n"));
        output.push_str(&format!("claim: {}\n", claim_type.unwrap_or("unrecorded")));
        output.push_str(&format!(
            "uncertainty: {}\n",
            uncertainty.unwrap_or("unrecorded")
        ));
        output.push_str("source_links:\n");
        if source_links.is_empty() {
            output.push_str("  none\n");
        }
        for link_id in source_links {
            match fixture.source_links.iter().find(|link| link.id == *link_id) {
                Some(link) => {
                    let custody_state = source_index
                        .records
                        .get(&link.source_ref)
                        .map(|record| record.status.as_str())
                        .unwrap_or("missing");
                    output.push_str(&format!(
                        "  {} -> {} | link_state={} | custody_state={}\n",
                        link.id, link.source_ref, link.contract_state, custody_state
                    ));
                }
                None => output.push_str(&format!("  {link_id} -> unresolved\n")),
            }
        }
    }

    output.push_str("connected_edges:\n");
    let connected_edges: Vec<&RelationshipEdge> = fixture
        .relationship_edges
        .iter()
        .filter(|edge| edge.source_id == node_id || edge.target_id == node_id)
        .collect();
    if connected_edges.is_empty() {
        output.push_str("  none\n");
    }
    for edge in connected_edges {
        let direction = if edge.source_id == node_id {
            "outgoing"
        } else {
            "incoming"
        };
        output.push_str(&format!(
            "  {direction} {} | kind={} | claim={} | uncertainty={} | review={}\n",
            edge.id, edge.edge_kind, edge.claim_type, edge.uncertainty, edge.review_state
        ));
    }
}

fn node_claim_fields<'a>(
    fixture: &'a Fixture,
    node_id: &str,
) -> Option<(
    &'static str,
    Option<&'a str>,
    Option<&'a str>,
    Vec<&'a String>,
)> {
    fixture
        .nodes
        .wordforms
        .iter()
        .find(|node| node.id == node_id)
        .map(|node| {
            (
                "Wordform",
                Some(node.claim_type.as_str()),
                Some(node.uncertainty.as_str()),
                node.source_links.iter().collect(),
            )
        })
        .or_else(|| {
            fixture
                .nodes
                .roots
                .iter()
                .find(|node| node.id == node_id)
                .map(|node| {
                    (
                        "Root",
                        Some(node.claim_type.as_str()),
                        Some(node.uncertainty.as_str()),
                        node.source_links.iter().collect(),
                    )
                })
        })
        .or_else(|| {
            fixture
                .nodes
                .meaning_senses
                .iter()
                .find(|node| node.id == node_id)
                .map(|node| {
                    (
                        "MeaningSense",
                        Some(node.claim_type.as_str()),
                        Some(node.uncertainty.as_str()),
                        node.source_links.iter().collect(),
                    )
                })
        })
        .or_else(|| {
            fixture
                .nodes
                .script_forms
                .iter()
                .find(|node| node.id == node_id)
                .map(|node| ("ScriptForm", None, None, node.source_links.iter().collect()))
        })
        .or_else(|| {
            fixture
                .nodes
                .languages
                .iter()
                .find(|node| node.id == node_id)
                .map(|_| ("Language", None, None, Vec::new()))
        })
        .or_else(|| {
            fixture
                .source_links
                .iter()
                .find(|link| link.id == node_id)
                .map(|_| ("SourceLink", None, None, Vec::new()))
        })
}

#[derive(Debug)]
struct PathStep<'a> {
    from_id: String,
    to_id: String,
    edge: &'a RelationshipEdge,
    forward: bool,
}

fn shortest_connection_path<'a>(
    fixture: &'a Fixture,
    start_id: &str,
    end_id: &str,
) -> Vec<PathStep<'a>> {
    if start_id == end_id {
        return Vec::new();
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent: HashMap<String, (String, usize, bool)> = HashMap::new();

    visited.insert(start_id.to_string());
    queue.push_back(start_id.to_string());

    while let Some(current) = queue.pop_front() {
        for (edge_index, edge) in fixture.relationship_edges.iter().enumerate() {
            let candidates = [
                (edge.source_id.as_str(), edge.target_id.as_str(), true),
                (edge.target_id.as_str(), edge.source_id.as_str(), false),
            ];
            for (from, to, forward) in candidates {
                if from != current || visited.contains(to) {
                    continue;
                }
                visited.insert(to.to_string());
                parent.insert(to.to_string(), (current.clone(), edge_index, forward));
                if to == end_id {
                    return reconstruct_path(fixture, start_id, end_id, &parent);
                }
                queue.push_back(to.to_string());
            }
        }
    }

    Vec::new()
}

fn reconstruct_path<'a>(
    fixture: &'a Fixture,
    start_id: &str,
    end_id: &str,
    parent: &HashMap<String, (String, usize, bool)>,
) -> Vec<PathStep<'a>> {
    let mut reversed = Vec::new();
    let mut current = end_id.to_string();

    while current != start_id {
        let Some((from_id, edge_index, forward)) = parent.get(&current) else {
            return Vec::new();
        };
        let edge = &fixture.relationship_edges[*edge_index];
        reversed.push(PathStep {
            from_id: from_id.clone(),
            to_id: current.clone(),
            edge,
            forward: *forward,
        });
        current = from_id.clone();
    }

    reversed.reverse();
    reversed
}

fn render_lineage_trace(
    fixture: &Fixture,
    wordform: &Wordform,
    graph_output_posture: &'static str,
    validation_error_count: usize,
) -> String {
    let mut output = String::new();
    output.push_str(&format!("lineage_trace: {}\n", wordform.id));
    output.push_str(&format!("fixture: {}\n", fixture.fixture_id));
    output.push_str(&format!("status: {graph_output_posture}\n"));
    output.push_str(&format!("validation_errors: {validation_error_count}\n"));
    output.push_str("start:\n");
    output.push_str(&format!(
        "  {} [{}] | claim={} | uncertainty={} | source={}\n",
        wordform.label,
        wordform.id,
        wordform.claim_type,
        wordform.uncertainty,
        source_posture_for_links(&wordform.source_links, fixture)
    ));

    output.push_str("paths:\n");
    let lineage_edges: Vec<&RelationshipEdge> = fixture
        .relationship_edges
        .iter()
        .filter(|edge| is_lineage_edge(edge.edge_kind.as_str()) && edge.source_id == wordform.id)
        .collect();
    if lineage_edges.is_empty() {
        output.push_str("  none\n");
    }
    for edge in lineage_edges {
        let target_label = graph_label_for_id(fixture, edge.target_id.as_str())
            .unwrap_or_else(|| "unresolved".into());
        output.push_str(&format!(
            "  {} --{}--> {} [{}] | claim={} | uncertainty={} | review={}\n",
            wordform.label,
            edge.edge_kind,
            target_label,
            edge.target_id,
            edge.claim_type,
            edge.uncertainty,
            edge.review_state
        ));
    }

    output.push_str("supporting_edges:\n");
    let supporting_edges: Vec<&RelationshipEdge> = fixture
        .relationship_edges
        .iter()
        .filter(|edge| edge.edge_kind == "supports_claim" && edge.target_id == wordform.id)
        .collect();
    if supporting_edges.is_empty() {
        output.push_str("  none\n");
    }
    for edge in supporting_edges {
        output.push_str(&format!(
            "  {} -> {} | claim={} | uncertainty={} | review={}\n",
            edge.source_id, edge.target_id, edge.claim_type, edge.uncertainty, edge.review_state
        ));
    }

    output.push_str("review_note:\n");
    output.push_str("  Lineage paths are fixture-local and remain non-promoted while validation errors exist.\n");
    output
}

fn is_lineage_edge(edge_kind: &str) -> bool {
    matches!(
        edge_kind,
        "descends_from"
            | "cognate_with"
            | "borrowed_from"
            | "calque_of"
            | "sound_shift_to"
            | "meaning_shift_to"
            | "script_variant_of"
    )
}

fn graph_label_for_id(fixture: &Fixture, id: &str) -> Option<String> {
    fixture
        .nodes
        .wordforms
        .iter()
        .find(|node| node.id == id)
        .map(|node| node.label.clone())
        .or_else(|| {
            fixture
                .nodes
                .roots
                .iter()
                .find(|node| node.id == id)
                .map(|node| node.label.clone())
        })
        .or_else(|| {
            fixture
                .nodes
                .languages
                .iter()
                .find(|node| node.id == id)
                .map(|node| node.label.clone())
        })
        .or_else(|| {
            fixture
                .nodes
                .meaning_senses
                .iter()
                .find(|node| node.id == id)
                .map(|node| node.label.clone())
        })
        .or_else(|| {
            fixture
                .nodes
                .script_forms
                .iter()
                .find(|node| node.id == id)
                .map(|node| node.label.clone())
        })
        .or_else(|| {
            fixture
                .source_links
                .iter()
                .find(|link| link.id == id)
                .map(|link| link.source_ref.clone())
        })
}

fn source_links_for_node<'a>(fixture: &'a Fixture, node_id: &str) -> Vec<&'a SourceLink> {
    if let Some(link) = fixture.source_links.iter().find(|link| link.id == node_id) {
        return vec![link];
    }

    let link_ids: Option<&[String]> = fixture
        .nodes
        .wordforms
        .iter()
        .find(|node| node.id == node_id)
        .map(|node| node.source_links.as_slice())
        .or_else(|| {
            fixture
                .nodes
                .roots
                .iter()
                .find(|node| node.id == node_id)
                .map(|node| node.source_links.as_slice())
        })
        .or_else(|| {
            fixture
                .nodes
                .meaning_senses
                .iter()
                .find(|node| node.id == node_id)
                .map(|node| node.source_links.as_slice())
        })
        .or_else(|| {
            fixture
                .nodes
                .script_forms
                .iter()
                .find(|node| node.id == node_id)
                .map(|node| node.source_links.as_slice())
        });

    match link_ids {
        Some(ids) => ids
            .iter()
            .filter_map(|id| fixture.source_links.iter().find(|link| link.id == *id))
            .collect(),
        None => Vec::new(),
    }
}

fn render_word_trace(
    fixture: &Fixture,
    wordform: &Wordform,
    graph_output_posture: &'static str,
    validation_error_count: usize,
) -> String {
    let language = fixture
        .nodes
        .languages
        .iter()
        .find(|language| language.id == wordform.language_id);
    let source_posture = source_posture_for_links(&wordform.source_links, fixture);

    let mut output = String::new();
    output.push_str(&format!("word_trace: {}\n", wordform.id));
    output.push_str(&format!("fixture: {}\n", fixture.fixture_id));
    output.push_str(&format!("status: {graph_output_posture}\n"));
    output.push_str(&format!("validation_errors: {validation_error_count}\n"));
    output.push_str("wordform:\n");
    output.push_str(&format!("  label: {}\n", wordform.label));
    output.push_str(&format!("  form: {}\n", wordform.form));
    output.push_str(&format!(
        "  language: {}\n",
        language
            .map(|language| language.label.as_str())
            .unwrap_or("unresolved")
    ));
    output.push_str(&format!("  claim: {}\n", wordform.claim_type));
    output.push_str(&format!("  uncertainty: {}\n", wordform.uncertainty));
    output.push_str(&format!("  source_posture: {source_posture}\n"));

    output.push_str("source_links:\n");
    if wordform.source_links.is_empty() {
        output.push_str("  none\n");
    }
    for source_link_id in &wordform.source_links {
        match fixture
            .source_links
            .iter()
            .find(|link| link.id == *source_link_id)
        {
            Some(link) => output.push_str(&format!(
                "  {} -> {} | state={}\n",
                link.id, link.source_ref, link.contract_state
            )),
            None => output.push_str(&format!("  {source_link_id} -> unresolved\n")),
        }
    }

    output.push_str("relationships:\n");
    let connected_edges: Vec<&RelationshipEdge> = fixture
        .relationship_edges
        .iter()
        .filter(|edge| edge.source_id == wordform.id || edge.target_id == wordform.id)
        .collect();
    if connected_edges.is_empty() {
        output.push_str("  none\n");
    }
    for edge in connected_edges {
        let direction = if edge.source_id == wordform.id {
            "outgoing"
        } else {
            "incoming"
        };
        output.push_str(&format!(
            "  {} {}: {} -> {} | kind={} | claim={} | uncertainty={} | review={}\n",
            direction,
            edge.id,
            edge.source_id,
            edge.target_id,
            edge.edge_kind,
            edge.claim_type,
            edge.uncertainty,
            edge.review_state
        ));
    }

    output.push_str("review_note:\n");
    output.push_str("  This trace is fixture-local and does not imply accepted source evidence while the fixture is preview-only.\n");
    output
}

fn render_chronicle_preview(fixture: &Fixture, graph: &GraphSlice) -> String {
    let mut output = String::new();
    let scope_question = fixture
        .scope
        .as_ref()
        .map(|scope| scope.question.as_str())
        .unwrap_or("No scope question recorded.");

    output.push_str(&format!("Chronicle preview: {}\n", fixture.fixture_id));
    output.push_str("Status: preview_only_not_published\n\n");

    output.push_str("Scope\n");
    output.push_str(&format!("- Question: {scope_question}\n"));
    if let Some(scope) = &fixture.scope {
        if !scope.excluded_claims.is_empty() {
            output.push_str("- Explicit non-goals: ");
            output.push_str(&scope.excluded_claims.join("; "));
            output.push('\n');
        }
    }
    output.push('\n');

    output.push_str("Evidence path\n");
    let direct_nodes: Vec<&GraphNode> = graph
        .nodes
        .iter()
        .filter(|node| node.claim_type.as_deref() == Some("direct_evidence"))
        .collect();
    if direct_nodes.is_empty() {
        output.push_str("- No direct-evidence nodes are accepted for this preview.\n");
    } else {
        for node in direct_nodes {
            output.push_str(&format!(
                "- {} is represented as {} with source posture {} and uncertainty {}.\n",
                node.label,
                node.record_class,
                node.source_posture,
                node.uncertainty.as_deref().unwrap_or("unrecorded")
            ));
        }
    }
    output.push('\n');

    output.push_str("Theory path\n");
    let theory_edges: Vec<&RelationshipEdge> = graph
        .edges
        .iter()
        .filter(|edge| edge.claim_type != "direct_evidence")
        .collect();
    if theory_edges.is_empty() {
        output
            .push_str("- No inferred, reconstructed, or disputed relationship path is recorded.\n");
    } else {
        for edge in theory_edges {
            output.push_str(&format!(
                "- {} from {} to {} is an {} claim with {} uncertainty and review state {}.\n",
                edge.edge_kind,
                edge.source_id,
                edge.target_id,
                edge.claim_type,
                edge.uncertainty,
                edge.review_state
            ));
        }
    }
    output.push('\n');

    output.push_str("Alternatives\n");
    if fixture.promotion_blockers.is_empty() {
        output.push_str("- No promotion blockers are recorded for this fixture.\n");
    } else {
        for blocker in &fixture.promotion_blockers {
            output.push_str(&format!("- Blocked: {blocker}.\n"));
        }
    }
    output.push('\n');

    output.push_str("Source limits\n");
    output.push_str(&format!(
        "- Source text included: {}.\n",
        fixture.source_text_included
    ));
    output.push_str(&format!(
        "- Redistribution posture: {}.\n",
        fixture.source_text_redistribution_posture
    ));
    if graph.source_posture_summary.is_empty() {
        output.push_str("- No source posture summary is recorded.\n");
    } else {
        for posture in &graph.source_posture_summary {
            output.push_str(&format!("- {posture}.\n"));
        }
    }
    output.push('\n');

    output.push_str("Graph summary\n");
    output.push_str(&format!(
        "- Slice {} has {} nodes, {} edges, and graph output posture {}.\n",
        graph.slice_id,
        graph.nodes.len(),
        graph.edges.len(),
        graph.graph_output_posture
    ));
    let edge_kinds: Vec<&str> = graph
        .edges
        .iter()
        .map(|edge| edge.edge_kind.as_str())
        .collect();
    if !edge_kinds.is_empty() {
        output.push_str(&format!(
            "- Preserved edge kinds: {}.\n",
            edge_kinds.join(", ")
        ));
    }
    output.push('\n');

    output.push_str("Review state\n");
    output.push_str(&format!(
        "- Fixture status is {} with {} validation diagnostics.\n",
        fixture.status, graph.validation_error_count
    ));
    if graph.graph_output_posture == "validated" {
        output.push_str(
            "- This chronicle is a preview only; it is not published, but the graph slice is validated for its bounded source-accepted scope.\n",
        );
    } else {
        output.push_str(
            "- This chronicle is a preview only; it is not published, validated, or source-accepted.\n",
        );
    }
    output
}

fn render_summary(graph: &GraphSlice) -> String {
    let mut node_counts = BTreeMap::new();
    let mut edge_counts = BTreeMap::new();
    let mut source_posture_counts = BTreeMap::new();
    let mut uncertainty_counts = BTreeMap::new();

    for node in &graph.nodes {
        *node_counts.entry(node.record_class).or_insert(0usize) += 1;
        *source_posture_counts
            .entry(node.source_posture.as_str())
            .or_insert(0usize) += 1;
        if let Some(uncertainty) = &node.uncertainty {
            *uncertainty_counts
                .entry(uncertainty.as_str())
                .or_insert(0usize) += 1;
        }
    }

    for edge in &graph.edges {
        *edge_counts.entry(edge.edge_kind.as_str()).or_insert(0usize) += 1;
        *uncertainty_counts
            .entry(edge.uncertainty.as_str())
            .or_insert(0usize) += 1;
    }

    for posture in &graph.source_posture_summary {
        if let Some((_, state)) = posture.rsplit_once(':') {
            *source_posture_counts.entry(state).or_insert(0usize) += 1;
        }
    }

    let mut output = String::new();
    output.push_str(&format!("fixture: {}\n", graph.slice_id));
    output.push_str(&format!("status: {}\n", graph.graph_output_posture));
    output.push_str(&format!(
        "validation_errors: {}\n",
        graph.validation_error_count
    ));
    push_counts(&mut output, "nodes", &node_counts);
    push_counts(&mut output, "edges", &edge_counts);
    push_counts(&mut output, "source_postures", &source_posture_counts);
    push_counts(&mut output, "uncertainties", &uncertainty_counts);
    output
}

fn push_counts(output: &mut String, label: &str, counts: &BTreeMap<&str, usize>) {
    output.push_str(&format!("{label}:\n"));
    if counts.is_empty() {
        output.push_str("  none: 0\n");
        return;
    }
    for (key, count) in counts {
        output.push_str(&format!("  {key}: {count}\n"));
    }
}

fn escape_dot(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn validate_linked_source_state(
    source_index: &SourceCustodyIndex,
    source_ref: &str,
    fixture_state: &str,
    report: &mut ValidationReport,
) {
    if !source_index.load_errors.is_empty() {
        for err in &source_index.load_errors {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-SRC-007",
                family: "source_custody",
                severity: Severity::Error,
                affected: source_ref.to_string(),
                message: err.clone(),
            });
        }
        return;
    }

    match source_index.records.get(source_ref) {
        Some(record) => {
            if record.status != fixture_state {
                report.diagnostics.push(Diagnostic {
                    id: "LEXIS-DIAG-SRC-005",
                    family: "source_custody",
                    severity: Severity::Error,
                    affected: source_ref.to_string(),
                    message: format!(
                        "fixture source state '{}' does not match custody record state '{}'",
                        fixture_state, record.status
                    ),
                });
            }
        }
        None => {
            report.diagnostics.push(Diagnostic {
                id: "LEXIS-DIAG-SRC-006",
                family: "source_custody",
                severity: Severity::Error,
                affected: source_ref.to_string(),
                message: "linked source-custody decision record was not found".to_string(),
            });
        }
    }
}

impl SourceCustodyIndex {
    fn load(repo_root: &Path) -> Result<Self, String> {
        let planned_dir = repo_root.join("source-custody").join("planned");
        let entries = fs::read_dir(&planned_dir)
            .map_err(|err| format!("failed to read {}: {err}", planned_dir.display()))?;
        let mut index = Self::default();

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    index.load_errors.push(format!(
                        "failed to read source-custody entry in {}: {err}",
                        planned_dir.display()
                    ));
                    continue;
                }
            };
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
                continue;
            }
            let text = match fs::read_to_string(&path) {
                Ok(text) => text,
                Err(err) => {
                    index
                        .load_errors
                        .push(format!("failed to read {}: {err}", path.display()));
                    continue;
                }
            };
            let record: SourceCustodyRecord = match serde_yaml::from_str(&text) {
                Ok(record) => record,
                Err(err) => {
                    index
                        .load_errors
                        .push(format!("failed to parse {}: {err}", path.display()));
                    continue;
                }
            };
            index.records.insert(record.decision_id.clone(), record);
        }

        Ok(index)
    }
}

fn load_correction_plans(repo_root: &Path) -> Result<Vec<CorrectionPlan>, String> {
    let corrections_dir = repo_root.join("correction-plans");
    if !corrections_dir.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(&corrections_dir)
        .map_err(|err| format!("failed to read {}: {err}", corrections_dir.display()))?;
    let mut plans = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|err| {
            format!(
                "failed to read correction-plan entry in {}: {err}",
                corrections_dir.display()
            )
        })?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
            continue;
        }
        let text = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let plan: CorrectionPlan = serde_yaml::from_str(&text)
            .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
        plans.push(plan);
    }

    plans.sort_by(|left, right| left.plan_id.cmp(&right.plan_id));
    Ok(plans)
}

fn find_repo_root(path: &Path) -> Result<PathBuf, String> {
    let mut current = path
        .canonicalize()
        .map_err(|err| format!("failed to resolve {}: {err}", path.display()))?;
    if current.is_file() {
        current.pop();
    }

    loop {
        if current.join("Cargo.toml").exists() && current.join("source-custody").exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(format!(
                "could not find LEXIS repo root for {}",
                path.display()
            ));
        }
    }
}

impl Display for ValidationReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "fixture: {}", self.fixture_id)?;
        if self.diagnostics.is_empty() {
            writeln!(f, "status: valid")?;
            return Ok(());
        }

        writeln!(f, "status: invalid")?;
        for diagnostic in &self.diagnostics {
            writeln!(
                f,
                "{} {} {} {}: {}",
                diagnostic.severity,
                diagnostic.family,
                diagnostic.id,
                diagnostic.affected,
                diagnostic.message
            )?;
        }
        Ok(())
    }
}

impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => f.write_str("error"),
            Severity::Warning => f.write_str("warning"),
            Severity::Info => f.write_str("info"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, read_to_string, write};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_REPO_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn validates_source_pointer_fixture_as_invalid_by_design() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/source-pointer-scribere/fixture.yaml");
        let report = validate_fixture(&path).expect("fixture should parse");

        assert!(report.has_errors());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SRC-001"
                && diagnostic.family == "source_custody"
                && diagnostic.affected == "LEXIS-SRCDEC-001-latin-lexicographic-reference"));
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SRC-000"
                && diagnostic.affected == "LEXIS-FIX-001-source-pointer-scribere"));
        assert!(report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.family != "relationship"));
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SRC-005"));
    }

    #[test]
    fn detects_source_state_mismatch_against_linked_record() {
        let root = unique_temp_root();
        create_dir_all(root.join("source-custody/planned")).expect("source directory");
        write(root.join("Cargo.toml"), "[workspace]\n").expect("cargo marker");
        write(
            root.join("source-custody/planned/test-source.yaml"),
            "decision_id: LEXIS-SRCDEC-TEST\nstatus: accepted_for_slice\n",
        )
        .expect("source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-TEST
status: draft_shape_invalid_by_design
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-TEST
    contract_state: candidate_review
    may_support_claims: false
deferred_records: []
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("fixture should parse");

        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SRC-005"
                && diagnostic.affected == "LEXIS-SRCDEC-TEST"));
    }

    #[test]
    fn detects_missing_source_custody_record() {
        let root = temp_repo_root();
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-TEST
status: draft_shape_invalid_by_design
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-MISSING
    contract_state: candidate_review
    may_support_claims: false
deferred_records: []
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("fixture should parse");

        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SRC-006"
                && diagnostic.affected == "LEXIS-SRCDEC-MISSING"));
    }

    #[test]
    fn detects_malformed_source_custody_record() {
        let root = temp_repo_root();
        write(
            root.join("source-custody/planned/bad.yaml"),
            "decision_id: [not-a-scalar\n",
        )
        .expect("malformed source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-TEST
status: draft_shape_invalid_by_design
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-ANY
    contract_state: candidate_review
    may_support_claims: false
deferred_records: []
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("fixture should parse");

        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SRC-007"
                && diagnostic.message.contains("failed to parse")));
    }

    #[test]
    fn blocks_graph_emit_for_invalid_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/negative-borrowing-descent/fixture.yaml");
        let err = emit_graph(&path, GraphFormat::Json).expect_err("invalid fixture should block");

        assert!(err.contains("graph emission blocked"));
    }

    #[test]
    fn emits_json_and_dot_for_accepted_local_graph_fixture() {
        let root = temp_repo_root();
        write(
            root.join("source-custody/planned/test-source.yaml"),
            "decision_id: LEXIS-SRCDEC-TEST\nstatus: accepted_for_slice\n",
        )
        .expect("source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-GRAPH-TEST
status: accepted_for_slice
scope:
  scope_id: LEXIS-SCOPE-TEST
  question: Accepted local graph test scope.
  excluded_claims: []
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
    may_support_claims: true
deferred_records: []
nodes:
  languages:
    - id: lang-test
      label: Test Language
      kind: language
      source_posture: accepted_for_slice
  wordforms:
    - id: wf-test-a
      label: test-a
      language_id: lang-test
      form: test-a
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links:
        - src-test
    - id: wf-test-b
      label: test-b
      language_id: lang-test
      form: test-b
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links:
        - src-test
  meaning_senses: []
  script_forms: []
source_links:
  - id: src-test
    source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
relationship_edges:
  - id: edge-test
    edge_kind: supports_claim
    source_id: src-test
    target_id: wf-test-a
    claim_type: direct_evidence
    uncertainty: settled_for_slice
    supporting_sources:
      - LEXIS-SRCDEC-TEST
    review_state: accepted_for_slice
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let json = emit_graph(&fixture_path, GraphFormat::Json).expect("json graph");
        let dot = emit_graph(&fixture_path, GraphFormat::Dot).expect("dot graph");

        assert!(json.contains("\"slice_id\": \"LEXIS-FIX-GRAPH-TEST\""));
        assert!(json.contains("\"edge_kind\": \"supports_claim\""));
        assert!(json.contains("\"source_posture\": \"accepted_for_slice\""));
        assert!(dot.contains("digraph lexis_slice"));
        assert!(dot.contains("supports_claim"));
        assert!(dot.contains("settled_for_slice"));
    }

    #[test]
    fn rejects_graph_fixture_without_scope() {
        let root = temp_repo_root();
        write(
            root.join("source-custody/planned/test-source.yaml"),
            "decision_id: LEXIS-SRCDEC-TEST\nstatus: accepted_for_slice\n",
        )
        .expect("source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-MISSING-SCOPE
status: accepted_for_slice
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
    may_support_claims: true
nodes:
  languages:
    - id: lang-test
      label: Test Language
      kind: language
      source_posture: accepted_for_slice
  wordforms: []
  meaning_senses: []
  script_forms: []
source_links: []
relationship_edges: []
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("report");

        assert!(report.has_errors());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-SCOPE-001"));
    }

    #[test]
    fn rejects_collapsed_borrowing_and_descent_edges() {
        let root = temp_repo_root();
        write(
            root.join("source-custody/planned/test-source.yaml"),
            "decision_id: LEXIS-SRCDEC-TEST\nstatus: accepted_for_slice\n",
        )
        .expect("source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-COLLAPSED-EDGE
status: accepted_for_slice
scope:
  scope_id: LEXIS-SCOPE-TEST
  question: Collapsed edge test scope.
  excluded_claims: []
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
    may_support_claims: true
nodes:
  languages:
    - id: lang-test
      label: Test Language
      kind: language
      source_posture: accepted_for_slice
  wordforms:
    - id: wf-a
      label: a
      language_id: lang-test
      form: a
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links:
        - src-test
    - id: wf-b
      label: b
      language_id: lang-test
      form: b
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links:
        - src-test
  meaning_senses: []
  script_forms: []
source_links:
  - id: src-test
    source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
relationship_edges:
  - id: edge-borrowed
    edge_kind: borrowed_from
    source_id: wf-a
    target_id: wf-b
    claim_type: inference
    uncertainty: settled_for_slice
    supporting_sources:
      - LEXIS-SRCDEC-TEST
    review_state: accepted_for_slice
  - id: edge-descended
    edge_kind: descends_from
    source_id: wf-a
    target_id: wf-b
    claim_type: inference
    uncertainty: settled_for_slice
    supporting_sources:
      - LEXIS-SRCDEC-TEST
    review_state: accepted_for_slice
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("report");

        assert!(report.has_errors());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-EDGE-004"));
    }

    #[test]
    fn rejects_chronicle_overclaim_wording() {
        let root = temp_repo_root();
        write(
            root.join("source-custody/planned/test-source.yaml"),
            "decision_id: LEXIS-SRCDEC-TEST\nstatus: accepted_for_slice\n",
        )
        .expect("source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-CHRONICLE-OVERCLAIM
status: draft_shape_invalid_by_design
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
    may_support_claims: true
nodes:
  languages: []
  wordforms: []
  meaning_senses: []
  script_forms: []
source_links: []
relationship_edges: []
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs:
  - title: This proves a universal origin.
promotion_blockers:
  - draft fixture
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("report");

        assert!(report.has_errors());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-CHRON-002"));
    }

    #[test]
    fn rejects_unsupported_claim_nodes_and_edges() {
        let root = temp_repo_root();
        write(
            root.join("source-custody/planned/test-source.yaml"),
            "decision_id: LEXIS-SRCDEC-TEST\nstatus: accepted_for_slice\n",
        )
        .expect("source record");
        let fixture_path = root.join("fixture.yaml");
        write(
            &fixture_path,
            r#"
fixture_id: LEXIS-FIX-UNSUPPORTED-DOMAIN-SHAPE
status: accepted_for_slice
scope:
  scope_id: LEXIS-SCOPE-TEST
  question: Unsupported domain shape test scope.
  excluded_claims: []
source_text_included: false
source_text_redistribution_posture: pointer_only_planned
source_records:
  - source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
    may_support_claims: true
nodes:
  languages:
    - id: lang-test
      label: Test Language
      kind: language
      source_posture: accepted_for_slice
  roots:
    - id: root-bad
      label: bad root
      root_text: bad
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links: []
  wordforms:
    - id: wf-a
      label: a
      language_id: lang-test
      form: a
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links:
        - src-test
    - id: wf-b
      label: b
      language_id: lang-test
      form: b
      claim_type: direct_evidence
      uncertainty: settled_for_slice
      source_links:
        - src-test
  meaning_senses: []
  script_forms: []
source_links:
  - id: src-test
    source_ref: LEXIS-SRCDEC-TEST
    contract_state: accepted_for_slice
relationship_edges:
  - id: edge-unsupported-dispute
    edge_kind: disputes_claim
    source_id: wf-a
    target_id: wf-b
    claim_type: inference
    uncertainty: source_limited
    supporting_sources: []
    review_state: blocked
language_claims: []
relationship_claims: []
graph_outputs: []
chronicle_outputs: []
promotion_blockers: []
"#,
        )
        .expect("fixture");

        let report = validate_fixture(&fixture_path).expect("report");

        assert!(report.has_errors());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-NODE-003"));
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-NODE-004"));
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-EDGE-005"));
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-EDGE-006"));
    }

    #[test]
    fn previews_accepted_fixture_graph_without_promotion() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let json = preview_graph(&path, GraphFormat::Json).expect("json preview");
        let dot = preview_graph(&path, GraphFormat::Dot).expect("dot preview");

        assert!(json.contains("\"graph_output_posture\": \"preview_only_not_promoted\""));
        assert!(json.contains("\"source_posture\": \"accepted_for_slice\""));
        assert!(json.contains("\"validation_error_count\":"));
        assert!(dot.contains("preview_only_not_promoted"));
        assert!(dot.contains("settled_for_slice"));
        assert!(dot.contains("borrowed_from"));
    }

    #[test]
    fn writes_preview_artifacts_from_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let out_dir = unique_temp_root().join("artifacts");
        let result = write_preview_artifacts(&path, &out_dir).expect("artifact write");

        let json_path = out_dir.join("graph-preview.json");
        let dot_path = out_dir.join("graph-preview.dot");
        let chronicle_path = out_dir.join("chronicle-preview.md");
        let json_text = read_to_string(&json_path).expect("json artifact");
        let dot = read_to_string(&dot_path).expect("dot artifact");
        let chronicle = read_to_string(&chronicle_path).expect("chronicle artifact");
        let json: serde_json::Value = serde_json::from_str(&json_text).expect("valid json");

        assert!(result.contains("artifacts_written: LEXIS-FIX-002-golden-scribere-slice"));
        assert_eq!(json["slice_id"], "LEXIS-FIX-002-golden-scribere-slice");
        assert_eq!(json["validation_error_count"], 0);
        assert!(dot.contains("wf-en-script"));
        assert!(dot.contains("meaning_shift_to"));
        assert!(chronicle.contains("preview only"));
        assert!(chronicle.contains("validated for its bounded source-accepted scope"));
    }

    #[test]
    fn summarizes_accepted_fixture_graph() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let summary = summarize_graph(&path).expect("graph summary");

        assert!(summary.contains("status: validated"));
        assert!(summary.contains("validation_errors: 0"));
        assert!(summary.contains("Wordform: 9"));
        assert!(summary.contains("MeaningSense: 5"));
        assert!(summary.contains("borrowed_from: 4"));
        assert!(summary.contains("descends_from: 4"));
        assert!(summary.contains("meaning_shift_to: 4"));
        assert!(summary.contains("settled_for_slice: 31"));
    }

    #[test]
    fn inspects_accepted_fixture_graph() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let inspection = inspect_graph(&path).expect("graph inspection");

        assert!(inspection.contains("status: validated"));
        assert!(inspection.contains("wf-en-scribe [Wordform]"));
        assert!(inspection.contains("kind=borrowed_from"));
        assert!(inspection.contains("claim=direct_evidence"));
        assert!(inspection.contains("uncertainty=settled_for_slice"));
        assert!(inspection
            .contains("LEXIS-SRCDEC-001-latin-lexicographic-reference:accepted_for_slice"));
    }

    #[test]
    fn previews_chronicle_for_accepted_fixture_without_publishing() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let chronicle = preview_chronicle(&path).expect("chronicle preview");

        assert!(chronicle.contains("Chronicle preview: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(chronicle.contains("Evidence path"));
        assert!(chronicle.contains("Theory path"));
        assert!(chronicle.contains("borrowed_from"));
        assert!(chronicle.contains("Source limits"));
        assert!(chronicle.contains("validated"));
        assert!(chronicle.contains("validated for its bounded source-accepted scope"));
        assert!(!chronicle.contains("proves"));
    }

    #[test]
    fn traces_wordform_from_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let trace = trace_word(&path, "wf-en-scribe").expect("word trace");

        assert!(trace.contains("word_trace: wf-en-scribe"));
        assert!(trace.contains("language: English"));
        assert!(trace.contains("source_posture: accepted_for_slice"));
        assert!(trace.contains("incoming edge-scribe-supported-by-english-source"));
        assert!(trace.contains("outgoing edge-scribe-borrowed-from-scriba"));
        assert!(trace.contains("kind=borrowed_from"));
        assert!(trace.contains("accepted_for_slice"));
    }

    #[test]
    fn reports_missing_wordform_trace_target() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let err = trace_word(&path, "wf-missing").expect_err("missing wordform should fail");

        assert!(err.contains("wf-missing"));
        assert!(err.contains("was not found"));
    }

    #[test]
    fn traces_lineage_from_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let lineage = trace_lineage(&path, "wf-en-scribe").expect("lineage trace");

        assert!(lineage.contains("lineage_trace: wf-en-scribe"));
        assert!(lineage.contains("scribe --borrowed_from--> scriba"));
        assert!(lineage.contains("claim=direct_evidence"));
        assert!(lineage.contains("uncertainty=settled_for_slice"));
        assert!(lineage.contains("supporting_edges"));
        assert!(lineage.contains("validated"));
    }

    #[test]
    fn traces_neighborhood_from_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let neighborhood = trace_neighborhood(&path, "wf-en-scribe").expect("neighborhood trace");

        assert!(neighborhood.contains("neighborhood_trace: wf-en-scribe"));
        assert!(neighborhood.contains("center: scribe [wf-en-scribe]"));
        assert!(
            neighborhood.contains("src-en-scribe -> LEXIS-SRCDEC-002-english-etymology-reference")
        );
        assert!(neighborhood.contains("incoming edge-scribe-supported-by-english-source"));
        assert!(neighborhood.contains("outgoing edge-scribe-borrowed-from-scriba"));
        assert!(neighborhood.contains("accepted_for_slice"));
    }

    #[test]
    fn finds_graph_path_across_accepted_slice() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let graph_path =
            graph_path(&path, "wf-lat-scribere", "wf-en-describe").expect("graph path");

        assert!(graph_path.contains("graph_path: wf-lat-scribere -> wf-en-describe"));
        assert!(graph_path.contains("status: validated"));
        assert!(graph_path.contains("path_found: yes"));
        assert!(graph_path.contains("scribere / scribo [wf-lat-scribere]"));
        assert!(graph_path.contains("describere [wf-lat-describere]"));
        assert!(graph_path.contains("describe [wf-en-describe]"));
        assert!(graph_path.contains("<--descends_from--"));
        assert!(graph_path.contains("<--borrowed_from--"));
        assert!(graph_path.contains("edge=edge-describere-descends-from-scribere"));
        assert!(graph_path.contains("edge=edge-describe-borrowed-from-describere"));
    }

    #[test]
    fn explains_edge_claim_from_accepted_slice() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let explanation = explain_claim(&path, "edge-describe-borrowed-from-describere")
            .expect("edge explanation");

        assert!(explanation.contains("claim_explain: edge-describe-borrowed-from-describere"));
        assert!(explanation.contains("status: validated"));
        assert!(explanation.contains("claim_type: edge"));
        assert!(explanation.contains("edge_kind: borrowed_from"));
        assert!(explanation.contains("from: describe [wf-en-describe]"));
        assert!(explanation.contains("to: describere [wf-lat-describere]"));
        assert!(explanation.contains(
            "LEXIS-SRCDEC-002-english-etymology-reference | custody_state=accepted_for_slice"
        ));
    }

    #[test]
    fn explains_node_claim_from_accepted_slice() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let explanation = explain_claim(&path, "wf-en-describe").expect("node explanation");

        assert!(explanation.contains("claim_explain: wf-en-describe"));
        assert!(explanation.contains("claim_type: node"));
        assert!(explanation.contains("record_class: Wordform"));
        assert!(
            explanation.contains("src-en-describe -> LEXIS-SRCDEC-002-english-etymology-reference")
        );
        assert!(explanation.contains("incoming edge-describe-supported-by-english-source"));
        assert!(explanation.contains("outgoing edge-describe-borrowed-from-describere"));
        assert!(explanation.contains("outgoing edge-describe-meaning-account"));
    }

    #[test]
    fn generates_fixture_from_compact_seed() {
        let seed_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../seeds/scribere-family.yaml");
        let root = temp_repo_root();
        write_accepted_source_records(&root);
        let out_path = root.join("fixtures/generated/scribere-family/fixture.yaml");

        let output = generate_slice(&seed_path, &out_path).expect("slice generation");
        let fixture_text = read_to_string(&out_path).expect("generated fixture");
        let report = validate_fixture(&out_path).expect("generated fixture validation");

        assert!(output.contains("slice_generated: LEXIS-GEN-scribere-family"));
        assert!(fixture_text.contains("fixture_id: LEXIS-GEN-scribere-family"));
        assert!(fixture_text.contains("edge-describe-borrowed-from-describere"));
        assert!(!report.has_errors());
    }

    #[test]
    fn batch_validates_generated_fixtures() {
        let seed_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../seeds/scribere-family.yaml");
        let root = temp_repo_root();
        write_accepted_source_records(&root);
        let out_path = root.join("fixtures/generated/scribere-family/fixture.yaml");
        generate_slice(&seed_path, &out_path).expect("slice generation");

        let validation = batch_validate(&root.join("fixtures")).expect("batch validation");
        let summary = batch_summary(&root.join("fixtures")).expect("batch summary");

        assert!(validation.contains("batch_validate:"));
        assert!(validation.contains("LEXIS-GEN-scribere-family"));
        assert!(validation.contains("valid: 1"));
        assert!(validation.contains("invalid: 0"));
        assert!(summary.contains("batch_summary:"));
        assert!(summary.contains("status=validated"));
        assert!(summary.contains("nodes="));
        assert!(summary.contains("edges="));
    }

    #[test]
    fn reports_missing_neighborhood_target() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let err = trace_neighborhood(&path, "node-missing").expect_err("missing node should fail");

        assert!(err.contains("node-missing"));
        assert!(err.contains("was not found"));
    }

    #[test]
    fn reports_source_status_for_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let status = source_status(&path).expect("source status");

        assert!(status.contains("source_status: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(status.contains(
            "LEXIS-SRCDEC-001-latin-lexicographic-reference | fixture_state=accepted_for_slice | custody_state=accepted_for_slice"
        ));
        assert!(status.contains("supports_claims=true"));
        assert!(status.contains("custody_load_errors:\n  none"));
    }

    #[test]
    fn reports_fixture_readiness_for_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let readiness = fixture_readiness(&path).expect("fixture readiness");

        assert!(readiness.contains("fixture_readiness: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(readiness.contains("promotion_ready: no"));
        assert!(readiness.contains("validation: ready"));
        assert!(readiness.contains("sources: ready"));
        assert!(readiness.contains("graph: ready"));
        assert!(readiness.contains("chronicle: blocked"));
        assert!(readiness.contains("total: 0"));
        assert!(readiness.contains("source records are accepted for slice"));
    }

    #[test]
    fn explains_diagnostics_for_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let explanation = explain_diagnostics(&path).expect("diagnostic explanation");

        assert!(explanation.contains("diagnostics_explain: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(explanation.contains("total: 0"));
        assert!(explanation.contains("status: valid"));
        assert!(explanation.contains("No diagnostics are present"));
    }

    #[test]
    fn renders_fixture_review_packet_for_accepted_fixture() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/golden-scribere-slice/fixture.yaml");
        let packet = fixture_review(&path).expect("fixture review packet");

        assert!(packet.contains("fixture_review: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(packet.contains("review_packet_status: validated_graph_chronicle_unpublished"));
        assert!(packet.contains("== readiness =="));
        assert!(packet.contains("== source_status =="));
        assert!(packet.contains("== diagnostics =="));
        assert!(packet.contains("== graph_summary =="));
        assert!(packet.contains("== chronicle_preview =="));
        assert!(packet.contains("promotion_ready: no"));
        assert!(packet.contains("source_status: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(packet.contains("diagnostics_explain: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(packet.contains("Chronicle preview: LEXIS-FIX-002-golden-scribere-slice"));
        assert!(packet.contains("reviews a validated graph slice"));
        assert!(packet.contains("does not publish a chronicle"));
    }

    #[test]
    fn lists_planned_fixture_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = render_fixture_inventory(&root).expect("fixture inventory");

        assert!(inventory.contains("fixture_inventory:"));
        assert!(inventory.contains("count: 9"));
        assert!(inventory.contains("LEXIS-FIX-001-source-pointer-scribere"));
        assert!(inventory.contains("LEXIS-FIX-002-golden-scribere-slice"));
        assert!(inventory.contains("LEXIS-FIX-006-script-alphabet-slice"));
        assert!(inventory.contains("LEXIS-FIX-007-pie-root-mini-slice"));
        assert!(inventory.contains("LEXIS-FIX-008-semitic-root-pattern-slice"));
        assert!(inventory.contains("LEXIS-FIX-009-glyph-graph-write-meta-slice"));
        assert!(inventory.contains("negative-borrowing-descent"));
        assert!(inventory.contains("promotion_blockers:"));
    }

    #[test]
    fn lists_preview_artifact_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = render_artifact_inventory(&root).expect("artifact inventory");

        assert!(inventory.contains("artifact_inventory:"));
        assert!(inventory.contains("count:"));
        assert!(inventory.contains("LEXIS-FIX-002-golden-scribere-slice"));
        assert!(inventory.contains("LEXIS-FIX-006-script-alphabet-slice"));
        assert!(inventory.contains("LEXIS-FIX-007-pie-root-mini-slice"));
        assert!(inventory.contains("graph-preview.json"));
        assert!(inventory.contains("chronicle-preview.md"));
        assert!(inventory.contains("does not promote source-backed claims"));
    }

    #[test]
    fn writes_preview_artifact_batch_from_fixture_directory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let fixture_dir = root.join("fixtures/generated/corrected-latin-english-tier3");
        let out_dir = unique_temp_root().join("artifacts");

        let result =
            write_preview_artifact_batch(&fixture_dir, &out_dir).expect("batch artifact write");

        assert!(result.contains("artifact_batch_written:"));
        assert!(result.contains("count: 15"));
        assert!(result.contains("LEXIS-GEN-CORR-094"));
        assert!(out_dir
            .join("094-sonus-sonare-old-french-son-soner-middle-english-sound-sound-acoustic-route")
            .join("graph-preview.json")
            .exists());
        assert!(out_dir
            .join("094-sonus-sonare-old-french-son-soner-middle-english-sound-sound-acoustic-route")
            .join("chronicle-preview.md")
            .exists());
    }

    #[test]
    fn summarizes_generated_artifact_corpus() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifact_dir = root.join("artifacts/generated/corrected-latin-english-tier3");
        let summary = render_artifact_corpus_summary(&artifact_dir).expect("artifact summary");

        assert!(summary.contains("artifact_corpus_summary:"));
        assert!(summary.contains("graphs: 15"));
        assert!(summary.contains("nodes: 143"));
        assert!(summary.contains("edges: 135"));
        assert!(summary.contains("preview_only_not_promoted: 15"));
        assert!(summary.contains("Language: 55"));
        assert!(summary.contains("Wordform: 73"));
        assert!(summary.contains("Latin:"));
        assert!(summary.contains("borrowed_from:"));
        assert!(summary.contains("supports_claim: 73"));
        assert!(summary.contains("candidate_review:"));
    }

    #[test]
    fn writes_generated_artifact_corpus_report() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifact_dir = root.join("artifacts/generated/corrected-latin-english-tier3");
        let out_path = unique_temp_root().join("report.md");

        let output = write_artifact_report(&artifact_dir, &out_path).expect("artifact report");
        let report = read_to_string(&out_path).expect("artifact report text");

        assert!(output.contains("artifact_report_written:"));
        assert!(report.contains("# Artifact Corpus Analysis"));
        assert!(report.contains("graphs: 15"));
        assert!(report.contains("| `LEXIS-GEN-CORR-094` |"));
        assert!(report.contains("preview_only_not_promoted"));
        assert!(report.contains("Validation errors"));
    }

    #[test]
    fn writes_correction_artifact_worklist_report() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifact_dir = root.join("artifacts/generated/corrected-latin-english-tier3");
        let report = render_correction_artifact_report(&root, &artifact_dir)
            .expect("correction artifact report");

        assert!(report.contains("# Corrected Tier 3 Promotion Worklist"));
        assert!(report.contains("- corrected graphs joined: 15"));
        assert!(report.contains("`replace_bridge`:"));
        assert!(report.contains("| 1 | `068` | `replace_bridge` | 18"));
        assert!(report.contains("LEXIS-SRCDEC-096-tier3-proof-joy"));
        assert!(report.contains("`split_homonym_route`"));
        assert!(report.contains("Compound and homonym actions"));
    }

    #[test]
    fn writes_ai_acceptance_report_with_blocked_recommendations() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifact_dir = root.join("artifacts/generated/corrected-latin-english-tier3");
        let report =
            render_ai_acceptance_report(&root, &artifact_dir).expect("ai acceptance report");

        assert!(report.contains("# AI Advisory Acceptance Review"));
        assert!(report.contains("AI acceptance is advisory"));
        assert!(report.contains("`block_promotion`: 15"));
        assert!(report.contains("| `094` |"));
        assert!(report.contains("source custody is not accepted_for_slice"));
        assert!(report.contains("none should be accepted yet"));
    }

    #[test]
    fn validates_negative_borrowing_descent_fixture_as_invalid() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/negative-borrowing-descent/fixture.yaml");
        let report = validate_fixture(&path).expect("negative fixture should parse");
        let explanation = explain_diagnostics(&path).expect("diagnostics");

        assert!(report.has_errors());
        assert!(report
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.id == "LEXIS-DIAG-EDGE-004"));
        assert!(explanation.contains("relationship:"));
        assert!(explanation.contains("LEXIS-DIAG-EDGE-004"));
    }

    #[test]
    fn lists_planned_source_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source_index = SourceCustodyIndex::load(&root).expect("source index");
        let inventory = render_source_inventory(&source_index);

        assert!(inventory.contains("source_inventory:"));
        assert!(inventory.contains("count: 109"));
        assert!(inventory.contains("LEXIS-SRCDEC-001-latin-lexicographic-reference"));
        assert!(inventory.contains("LEXIS-SRCDEC-002-english-etymology-reference"));
        assert!(inventory.contains("LEXIS-SRCDEC-006-pie-reconstruction-reference"));
        assert!(inventory.contains("LEXIS-SRCDEC-007-semitic-root-reference"));
        assert!(inventory.contains("LEXIS-SRCDEC-008-meta-etymology-reference"));
        assert!(inventory.contains("LEXIS-SRCDEC-009-latin-english-batch-candidate-reference"));
        assert!(inventory.contains("LEXIS-SRCDEC-010-tier1-proof-act"));
        assert!(inventory.contains("LEXIS-SRCDEC-019-tier1-proof-motion"));
        assert!(inventory.contains("LEXIS-SRCDEC-020-tier1-proof-position"));
        assert!(inventory.contains("LEXIS-SRCDEC-029-tier1-proof-section"));
        assert!(inventory.contains("LEXIS-SRCDEC-030-tier1-proof-solution"));
        assert!(inventory.contains("LEXIS-SRCDEC-035-tier1-proof-delete"));
        assert!(inventory.contains("LEXIS-SRCDEC-036-tier2-proof-amateur"));
        assert!(inventory.contains("LEXIS-SRCDEC-045-tier2-proof-spirit"));
        assert!(inventory.contains("LEXIS-SRCDEC-046-tier2-proof-state"));
        assert!(inventory.contains("LEXIS-SRCDEC-055-tier2-proof-tractor"));
        assert!(inventory.contains("LEXIS-SRCDEC-056-tier2-proof-valid"));
        assert!(inventory.contains("LEXIS-SRCDEC-065-tier2-proof-cupidity"));
        assert!(inventory.contains("LEXIS-SRCDEC-066-tier2-proof-dolor"));
        assert!(inventory.contains("LEXIS-SRCDEC-075-tier2-proof-fusion"));
        assert!(inventory.contains("LEXIS-SRCDEC-076-tier2-proof-gesture"));
        assert!(inventory.contains("LEXIS-SRCDEC-085-tier2-proof-orator"));
        assert!(inventory.contains("LEXIS-SRCDEC-086-tier2-proof-passion"));
        assert!(inventory.contains("LEXIS-SRCDEC-094-tier2-proof-use"));
        assert!(inventory.contains("LEXIS-SRCDEC-095-tier3-proof-spectator"));
        assert!(inventory.contains("LEXIS-SRCDEC-109-tier3-proof-urgent"));
        assert!(inventory.contains("status: accepted_for_slice"));
        assert!(inventory.contains("promotion_allowed: false"));
        assert!(inventory.contains("custody_load_errors:\n  none"));
    }

    #[test]
    fn renders_source_review_packet_for_candidate_source() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source_index = SourceCustodyIndex::load(&root).expect("source index");
        let review = render_source_review(
            &root,
            &source_index,
            "LEXIS-SRCDEC-002-english-etymology-reference",
        )
        .expect("source review");

        assert!(review.contains("source_review: LEXIS-SRCDEC-002-english-etymology-reference"));
        assert!(review.contains("status: accepted_for_slice"));
        assert!(review.contains("promotion_allowed: true"));
        assert!(review.contains("referencing_fixtures:"));
        assert!(review.contains("LEXIS-FIX-002-golden-scribere-slice"));
        assert!(review.contains("pointer-only"));
    }

    #[test]
    fn lists_tier3_correction_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = render_correction_inventory(&root).expect("correction inventory");

        assert!(inventory.contains("correction_inventory:"));
        assert!(inventory.contains("plans: 1"));
        assert!(inventory.contains("entries: 15"));
        assert!(inventory.contains("025:"));
        assert!(inventory.contains("action: keep_or_replace_target"));
        assert!(inventory.contains("094:"));
        assert!(inventory.contains("action: split_homonym_route"));
        assert!(inventory.contains("099:"));
        assert!(inventory.contains("LEXIS-SRCDEC-109-tier3-proof-urgent"));
    }

    #[test]
    fn renders_correction_review_with_source_status() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let review = render_correction_review(&root, "94").expect("correction review");

        assert!(review.contains("correction_review: 094"));
        assert!(review.contains("proof_source: LEXIS-SRCDEC-106-tier3-proof-sound"));
        assert!(review.contains("proof_source_status: candidate_review"));
        assert!(review.contains("action: split_homonym_route"));
        assert!(review.contains("sonus/sonare"));
        assert!(review.contains("homonym routes must be split before promotion"));
    }

    #[test]
    fn builds_correction_seed_from_route_plan() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let entry = find_correction_entry(&root, "094").expect("correction entry");
        let seed = build_correction_seed(&entry).expect("correction seed");

        assert_eq!(seed.slice_id, "corrected-candidate-094");
        assert_eq!(seed.fixture_id.as_deref(), Some("LEXIS-GEN-CORR-094"));
        assert!(seed
            .sources
            .iter()
            .any(|source| source.id == "LEXIS-SRCDEC-106-tier3-proof-sound"));
        assert!(seed
            .forms
            .iter()
            .any(|form| form.form == "sonus" && form.language == "Latin"));
        assert!(seed
            .forms
            .iter()
            .any(|form| form.form == "soner" && form.language == "Old French"));
        assert!(seed
            .relationships
            .iter()
            .any(|relationship| relationship.kind == "borrowed_from"
                && relationship.source == "wf-modern-english-sound"
                && relationship.target == "wf-middle-english-sound"));
    }

    #[test]
    fn generated_correction_fixture_keeps_duplicate_forms_separate() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let entry = find_correction_entry(&root, "094").expect("correction entry");
        let seed = build_correction_seed(&entry).expect("correction seed");
        let temp = unique_temp_root();
        let seed_path = temp.join("seed.yaml");
        let fixture_path = temp.join("fixture.yaml");

        write_seed_yaml(&seed, &seed_path).expect("seed write");
        generate_slice(&seed_path, &fixture_path).expect("fixture generation");
        let fixture_text = read_to_string(&fixture_path).expect("fixture text");

        assert!(fixture_text.contains("edge-wf-middle-english-sound-supported-by-source"));
        assert!(fixture_text.contains("edge-wf-modern-english-sound-supported-by-source"));
        assert!(fixture_text.contains("src-middle-english-sound"));
        assert!(fixture_text.contains("src-modern-english-sound"));
    }

    #[test]
    fn writes_all_correction_seed_files() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temp = unique_temp_root();
        let out_dir = temp.join("seeds/corrected-tier3");

        let output = write_all_correction_seeds(&root, &out_dir).expect("correction seeds");

        assert!(output.contains("correction_seed_batch:"));
        assert!(output.contains("count: 15"));
        assert!(out_dir.join("094-sonus-sonare-old-french-son-soner-middle-english-sound-sound-acoustic-route.yaml").exists());
    }

    #[test]
    fn reports_missing_source_review_target() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source_index = SourceCustodyIndex::load(&root).expect("source index");
        let err = render_source_review(&root, &source_index, "LEXIS-SRCDEC-MISSING")
            .expect_err("missing source should fail");

        assert!(err.contains("LEXIS-SRCDEC-MISSING"));
        assert!(err.contains("was not found"));
    }

    #[test]
    fn lists_language_slice_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = render_slice_inventory(&root).expect("slice inventory");

        assert!(inventory.contains("slice_inventory:"));
        assert!(inventory.contains("count: 5"));
        assert!(inventory.contains("LEXIS-SLICE-001"));
        assert!(inventory.contains("title: Latin `scribere`"));
        assert!(inventory.contains("LEXIS-FIX-002-golden-scribere-slice"));
        assert!(inventory.contains("LEXIS-SRCDEC-001-latin-lexicographic-reference"));
        assert!(inventory.contains("LEXIS-SLICE-005"));
    }

    #[test]
    fn summarizes_root_centered_candidate_slice() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/planned/pie-root-mini-slice/fixture.yaml");
        let summary = summarize_graph(&path).expect("graph summary");
        let inspection = inspect_graph(&path).expect("graph inspection");

        assert!(summary.contains("status: preview_only_not_promoted"));
        assert!(summary.contains("Root: 1"));
        assert!(summary.contains("cognate_with: 3"));
        assert!(inspection.contains("root-pie-bher [Root]"));
        assert!(inspection.contains("kind=cognate_with"));
    }

    #[test]
    fn renders_language_slice_review() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let review = render_slice_review(&root, "001").expect("slice review");

        assert!(review.contains("slice_review: LEXIS-SLICE-001"));
        assert!(review.contains("title: Latin `scribere`"));
        assert!(review.contains("LEXIS-SLICE-001-SOURCE"));
        assert!(review.contains("LEXIS-FIX-002-golden-scribere-slice"));
        assert!(review.contains("LEXIS-SRCDEC-002-english-etymology-reference"));
        assert!(review.contains("planning inventory only"));
    }

    #[test]
    fn reports_missing_slice_review_target() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let err = render_slice_review(&root, "999").expect_err("missing slice should fail");

        assert!(err.contains("999"));
        assert!(err.contains("was not found"));
    }

    #[test]
    fn lists_planned_scenario_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = render_scenario_inventory(&root).expect("scenario inventory");

        assert!(inventory.contains("scenario_inventory:"));
        assert!(inventory.contains("count: 4"));
        assert!(inventory.contains("LEXIS-SC-001-word-root-slice"));
        assert!(inventory.contains("LEXIS-SC-002-borrowing-vs-descent"));
        assert!(inventory.contains("slice_package: LEXIS-SLICE-001"));
        assert!(inventory.contains("diagnostics_expected:"));
        assert!(inventory.contains("fixture_candidates:"));
    }

    #[test]
    fn renders_scenario_review_packet() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let review = render_scenario_review(&root, "001").expect("scenario review");

        assert!(review.contains("scenario_review: LEXIS-SC-001-word-root-slice"));
        assert!(review.contains("actor: language-history reviewer"));
        assert!(review.contains("positive_path:"));
        assert!(review.contains("negative_paths:"));
        assert!(review.contains("diagnostics_expected:"));
        assert!(review.contains("fixture_candidates:"));
        assert!(review.contains("Scenario review is planning-only"));
    }

    #[test]
    fn reports_missing_scenario_review_target() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let err = render_scenario_review(&root, "999").expect_err("missing scenario should fail");

        assert!(err.contains("999"));
        assert!(err.contains("was not found"));
    }

    #[test]
    fn lists_work_package_inventory() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let inventory = render_work_package_inventory(&root).expect("work package inventory");

        assert!(inventory.contains("work_package_inventory:"));
        assert!(inventory.contains("count: 9"));
        assert!(inventory.contains("LEXIS-WP-003"));
        assert!(inventory.contains("name: Source-custody stub"));
        assert!(inventory.contains("LEXIS-SC-001-word-root-slice"));
        assert!(inventory.contains("LEXIS-WP-008"));
        assert!(inventory.contains("does not mark package outputs promoted"));
    }

    fn temp_repo_root() -> PathBuf {
        let root = unique_temp_root();
        create_dir_all(root.join("source-custody/planned")).expect("source directory");
        write(root.join("Cargo.toml"), "[workspace]\n").expect("cargo marker");
        root
    }

    fn write_accepted_source_records(root: &Path) {
        write(
            root.join("source-custody/planned/latin.yaml"),
            "decision_id: LEXIS-SRCDEC-001-latin-lexicographic-reference\nstatus: accepted_for_slice\n",
        )
        .expect("latin source record");
        write(
            root.join("source-custody/planned/english.yaml"),
            "decision_id: LEXIS-SRCDEC-002-english-etymology-reference\nstatus: accepted_for_slice\n",
        )
        .expect("english source record");
    }

    fn unique_temp_root() -> PathBuf {
        let counter = TEMP_REPO_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "lexis-test-{}-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("clock should be after epoch")
                .as_nanos(),
            counter
        ))
    }
}
