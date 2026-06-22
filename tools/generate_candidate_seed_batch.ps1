param(
    [string]$OutputDir = "seeds/candidate-latin-english-100"
)

$ErrorActionPreference = "Stop"

$sourceId = "LEXIS-SRCDEC-009-latin-english-batch-candidate-reference"
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)

$families = @(
    @{ Root = "agere"; Bridge = "actum"; English = "act"; Meaning = "doing or acting" },
    @{ Root = "amare"; Bridge = "amator"; English = "amateur"; Meaning = "love or devoted pursuit" },
    @{ Root = "audire"; Bridge = "auditum"; English = "auditor"; Meaning = "hearing or listening" },
    @{ Root = "capere"; Bridge = "captum"; English = "capture"; Meaning = "taking or seizing" },
    @{ Root = "cedere"; Bridge = "cessum"; English = "cession"; Meaning = "going or yielding" },
    @{ Root = "credere"; Bridge = "creditum"; English = "credit"; Meaning = "trust or belief" },
    @{ Root = "ducere"; Bridge = "ductum"; English = "conduct"; Meaning = "leading or drawing" },
    @{ Root = "facere"; Bridge = "factum"; English = "factor"; Meaning = "making or doing" },
    @{ Root = "ferre"; Bridge = "transferre"; English = "transfer"; Meaning = "bearing or carrying across" },
    @{ Root = "finire"; Bridge = "finitum"; English = "finite"; Meaning = "ending or bounding" },
    @{ Root = "flos"; Bridge = "florem"; English = "flower"; Meaning = "flowering or blooming" },
    @{ Root = "habere"; Bridge = "habitum"; English = "habit"; Meaning = "holding or having" },
    @{ Root = "jacere"; Bridge = "projectum"; English = "project"; Meaning = "throwing or casting forward" },
    @{ Root = "jungere"; Bridge = "junctum"; English = "junction"; Meaning = "joining or linking" },
    @{ Root = "legere"; Bridge = "lectum"; English = "lecture"; Meaning = "reading or gathering" },
    @{ Root = "mittere"; Bridge = "missum"; English = "mission"; Meaning = "sending or dispatching" },
    @{ Root = "movere"; Bridge = "motum"; English = "motion"; Meaning = "moving or changing position" },
    @{ Root = "ponere"; Bridge = "positum"; English = "position"; Meaning = "placing or setting" },
    @{ Root = "portare"; Bridge = "portatum"; English = "portable"; Meaning = "carrying or bearing" },
    @{ Root = "premere"; Bridge = "pressum"; English = "pressure"; Meaning = "pressing or compressing" },
    @{ Root = "regere"; Bridge = "rectum"; English = "rector"; Meaning = "ruling or guiding" },
    @{ Root = "rumpere"; Bridge = "ruptum"; English = "rupture"; Meaning = "breaking or bursting" },
    @{ Root = "scribere"; Bridge = "scriptum"; English = "script"; Meaning = "writing or marking" },
    @{ Root = "sentire"; Bridge = "sensum"; English = "sense"; Meaning = "feeling or perceiving" },
    @{ Root = "spectare"; Bridge = "spectator"; English = "spectator"; Meaning = "looking or observing" },
    @{ Root = "spirare"; Bridge = "spiritum"; English = "spirit"; Meaning = "breathing or animating" },
    @{ Root = "stare"; Bridge = "statum"; English = "state"; Meaning = "standing or setting" },
    @{ Root = "tangere"; Bridge = "tactum"; English = "contact"; Meaning = "touching or reaching" },
    @{ Root = "tenere"; Bridge = "tentum"; English = "tenant"; Meaning = "holding or keeping" },
    @{ Root = "videre"; Bridge = "visum"; English = "vision"; Meaning = "seeing or viewing" },
    @{ Root = "vocare"; Bridge = "vocatum"; English = "vocation"; Meaning = "calling or naming" },
    @{ Root = "volvere"; Bridge = "volutum"; English = "volume"; Meaning = "rolling or turning" },
    @{ Root = "currere"; Bridge = "cursum"; English = "course"; Meaning = "running or flowing" },
    @{ Root = "pellere"; Bridge = "pulsum"; English = "pulse"; Meaning = "driving or pushing" },
    @{ Root = "tendere"; Bridge = "tensum"; English = "tension"; Meaning = "stretching or extending" },
    @{ Root = "vertere"; Bridge = "versum"; English = "version"; Meaning = "turning or changing" },
    @{ Root = "venire"; Bridge = "ventum"; English = "advent"; Meaning = "coming or arriving" },
    @{ Root = "vincere"; Bridge = "victum"; English = "victory"; Meaning = "conquering or overcoming" },
    @{ Root = "secare"; Bridge = "sectum"; English = "section"; Meaning = "cutting or dividing" },
    @{ Root = "sedere"; Bridge = "sessum"; English = "session"; Meaning = "sitting or settling" },
    @{ Root = "solvere"; Bridge = "solutum"; English = "solution"; Meaning = "loosening or resolving" },
    @{ Root = "struere"; Bridge = "structum"; English = "structure"; Meaning = "building or arranging" },
    @{ Root = "trahere"; Bridge = "tractum"; English = "tractor"; Meaning = "drawing or pulling" },
    @{ Root = "valere"; Bridge = "validum"; English = "valid"; Meaning = "strength or worth" },
    @{ Root = "velle"; Bridge = "volitio"; English = "volition"; Meaning = "willing or choosing" },
    @{ Root = "dicere"; Bridge = "dictum"; English = "dictate"; Meaning = "saying or declaring" },
    @{ Root = "docere"; Bridge = "doctum"; English = "doctor"; Meaning = "teaching or showing" },
    @{ Root = "dare"; Bridge = "datum"; English = "data"; Meaning = "giving or granting" },
    @{ Root = "cavere"; Bridge = "cautum"; English = "caution"; Meaning = "guarding or taking care" },
    @{ Root = "celare"; Bridge = "celatum"; English = "conceal"; Meaning = "hiding or covering" },
    @{ Root = "clamare"; Bridge = "clamatum"; English = "claim"; Meaning = "calling or crying out" },
    @{ Root = "claudere"; Bridge = "clausum"; English = "clause"; Meaning = "closing or enclosing" },
    @{ Root = "colere"; Bridge = "cultum"; English = "culture"; Meaning = "cultivating or tending" },
    @{ Root = "componere"; Bridge = "compositum"; English = "compose"; Meaning = "putting together" },
    @{ Root = "condicere"; Bridge = "condicio"; English = "condition"; Meaning = "agreement or stipulation" },
    @{ Root = "cupere"; Bridge = "cupitum"; English = "cupidity"; Meaning = "desiring or longing" },
    @{ Root = "delere"; Bridge = "deletum"; English = "delete"; Meaning = "destroying or erasing" },
    @{ Root = "dolere"; Bridge = "dolorem"; English = "dolor"; Meaning = "pain or grief" },
    @{ Root = "eximere"; Bridge = "exemptum"; English = "exempt"; Meaning = "taking out or freeing" },
    @{ Root = "errare"; Bridge = "erratum"; English = "error"; Meaning = "wandering or straying" },
    @{ Root = "esse"; Bridge = "essentia"; English = "essence"; Meaning = "being or existence" },
    @{ Root = "fallere"; Bridge = "falsum"; English = "false"; Meaning = "deceiving or failing" },
    @{ Root = "fateri"; Bridge = "fassum"; English = "confess"; Meaning = "acknowledging or declaring" },
    @{ Root = "flectere"; Bridge = "flexum"; English = "flex"; Meaning = "bending or turning" },
    @{ Root = "fluere"; Bridge = "fluxum"; English = "flux"; Meaning = "flowing or moving" },
    @{ Root = "frangere"; Bridge = "fractum"; English = "fracture"; Meaning = "breaking or shattering" },
    @{ Root = "fundere"; Bridge = "fusum"; English = "fusion"; Meaning = "pouring or spreading" },
    @{ Root = "gaudere"; Bridge = "gaudium"; English = "joy"; Meaning = "rejoicing or gladness" },
    @{ Root = "gerere"; Bridge = "gestum"; English = "gesture"; Meaning = "carrying or performing" },
    @{ Root = "gradi"; Bridge = "gressum"; English = "progress"; Meaning = "stepping or going" },
    @{ Root = "haerere"; Bridge = "haesum"; English = "adhesion"; Meaning = "sticking or clinging" },
    @{ Root = "ire"; Bridge = "itum"; English = "itinerary"; Meaning = "going or traveling" },
    @{ Root = "laedere"; Bridge = "laesum"; English = "lesion"; Meaning = "hurting or damaging" },
    @{ Root = "laudare"; Bridge = "laudatum"; English = "laud"; Meaning = "praising or commending" },
    @{ Root = "linquere"; Bridge = "lictum"; English = "relic"; Meaning = "leaving or abandoning" },
    @{ Root = "loqui"; Bridge = "locutum"; English = "locution"; Meaning = "speaking or saying" },
    @{ Root = "lucere"; Bridge = "lucem"; English = "lucid"; Meaning = "shining or being clear" },
    @{ Root = "manere"; Bridge = "mansum"; English = "mansion"; Meaning = "remaining or staying" },
    @{ Root = "monere"; Bridge = "monitum"; English = "monitor"; Meaning = "warning or advising" },
    @{ Root = "nasci"; Bridge = "natum"; English = "nation"; Meaning = "being born or arising" },
    @{ Root = "nocere"; Bridge = "nocitum"; English = "noxious"; Meaning = "harming or injuring" },
    @{ Root = "novare"; Bridge = "novatum"; English = "novel"; Meaning = "making new" },
    @{ Root = "orare"; Bridge = "oratum"; English = "orator"; Meaning = "speaking or pleading" },
    @{ Root = "pati"; Bridge = "passum"; English = "passion"; Meaning = "suffering or enduring" },
    @{ Root = "placere"; Bridge = "placitum"; English = "placid"; Meaning = "pleasing or calming" },
    @{ Root = "plicare"; Bridge = "plicatum"; English = "complicate"; Meaning = "folding or layering" },
    @{ Root = "quaerere"; Bridge = "quaesitum"; English = "question"; Meaning = "seeking or asking" },
    @{ Root = "rapere"; Bridge = "raptum"; English = "rapture"; Meaning = "seizing or carrying off" },
    @{ Root = "ridere"; Bridge = "risum"; English = "risible"; Meaning = "laughing or smiling" },
    @{ Root = "salire"; Bridge = "saltum"; English = "salient"; Meaning = "leaping or jumping" },
    @{ Root = "sanare"; Bridge = "sanatum"; English = "sanitary"; Meaning = "healing or making sound" },
    @{ Root = "scire"; Bridge = "scientia"; English = "science"; Meaning = "knowing or understanding" },
    @{ Root = "servire"; Bridge = "servitum"; English = "serve"; Meaning = "serving or being subject" },
    @{ Root = "sonare"; Bridge = "sonitum"; English = "sound"; Meaning = "sounding or making noise" },
    @{ Root = "sperare"; Bridge = "speratum"; English = "despair"; Meaning = "hoping or expecting" },
    @{ Root = "stringere"; Bridge = "strictum"; English = "strict"; Meaning = "drawing tight or binding" },
    @{ Root = "sumere"; Bridge = "sumptum"; English = "assume"; Meaning = "taking up or adopting" },
    @{ Root = "timere"; Bridge = "timorem"; English = "timid"; Meaning = "fearing or being afraid" },
    @{ Root = "urgere"; Bridge = "ursum"; English = "urgent"; Meaning = "pressing or driving" },
    @{ Root = "uti"; Bridge = "usum"; English = "use"; Meaning = "using or employing" }
)

if ($families.Count -ne 100) {
    throw "Expected 100 seed families but found $($families.Count)."
}

$outputPath = [System.IO.Path]::GetFullPath($OutputDir)
$repoPath = [System.IO.Path]::GetFullPath((Get-Location).Path)
if (-not $outputPath.StartsWith($repoPath)) {
    throw "refusing to clean seed dir outside repo: $outputPath"
}
if (Test-Path -LiteralPath $OutputDir) {
    Remove-Item -LiteralPath $OutputDir -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$indexLines = @(
    "# Candidate Latin-English Seed Batch",
    "",
    "These 100 seed files are generated candidate inputs. They are not accepted",
    "etymology claims. Each seed queues a Latin base form, a Latin bridge form,",
    "Old French and Middle English candidate stages, a Modern English candidate",
    "derivative, and four candidate relationship edges for source and",
    "relationship review.",
    ""
)

for ($i = 0; $i -lt $families.Count; $i++) {
    $n = $i + 1
    $family = $families[$i]
    $id = "{0:D3}-{1}-{2}" -f $n, $family.Root, $family.English
    $fixtureId = "LEXIS-GEN-CAND-{0:D3}-{1}-{2}" -f $n, $family.Root, $family.English
    $path = Join-Path $OutputDir ("{0}.yaml" -f $id)
    $oldFrench = "old-french-$($family.English)-candidate"
    $middleEnglish = "middle-english-$($family.English)-candidate"

    $yaml = @"
slice_id: candidate-$id
fixture_id: $fixtureId
status: candidate_review
source_state: candidate_review
question: Candidate generated multi-stage graph-facing chain for Latin $($family.Root), Old French, Middle English, and Modern English $($family.English).
sources:
  - id: $sourceId
    state: candidate_review
    may_support_claims: false
forms:
  - form: $($family.Root)
    language: Latin
    source: $sourceId
    meaning: $($family.Meaning)
  - form: $($family.Bridge)
    language: Latin
    source: $sourceId
  - form: $oldFrench
    label: Old French candidate stage for $($family.English)
    language: Old French
    source: $sourceId
  - form: $middleEnglish
    label: Middle English candidate stage for $($family.English)
    language: Middle English
    source: $sourceId
  - form: $($family.English)
    language: Modern English
    source: $sourceId
    meaning: $($family.Meaning)
relationships:
  - kind: descends_from
    source: $($family.Bridge)
    target: $($family.Root)
    claim_type: inference
    uncertainty: source_limited
    support:
      - $sourceId
    review_state: candidate_review
  - kind: borrowed_from
    source: $oldFrench
    target: $($family.Bridge)
    claim_type: inference
    uncertainty: source_limited
    support:
      - $sourceId
    review_state: candidate_review
  - kind: borrowed_from
    source: $middleEnglish
    target: $oldFrench
    claim_type: inference
    uncertainty: source_limited
    support:
      - $sourceId
    review_state: candidate_review
  - kind: descends_from
    source: $($family.English)
    target: $middleEnglish
    claim_type: inference
    uncertainty: source_limited
    support:
      - $sourceId
    review_state: candidate_review
"@

    [System.IO.File]::WriteAllText([System.IO.Path]::GetFullPath($path), $yaml, $utf8NoBom)
    $indexLines += "- $id"
}

$indexPath = Join-Path $OutputDir "README.md"
[System.IO.File]::WriteAllText([System.IO.Path]::GetFullPath($indexPath), ($indexLines -join [Environment]::NewLine), $utf8NoBom)

Write-Output "candidate_seed_batch_generated: $OutputDir"
Write-Output "count: $($families.Count)"
