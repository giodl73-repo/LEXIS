use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match (
        args.next().as_deref(),
        args.next(),
        args.next(),
        args.next(),
        args.next(),
    ) {
        (Some("validate"), Some(path), None, None, None) => {
            match lexis_cli::validate_fixture(PathBuf::from(path).as_path()) {
                Ok(report) => {
                    print!("{report}");
                    if report.has_errors() {
                        ExitCode::from(1)
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(2)
                }
            }
        }
        (Some("slice"), Some(command), Some(seed_path), Some(out_path), None)
            if command == "generate" =>
        {
            match lexis_cli::generate_slice(
                PathBuf::from(seed_path).as_path(),
                PathBuf::from(out_path).as_path(),
            ) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("batch"), Some(command), Some(path), None, None)
            if command == "validate" || command == "summary" =>
        {
            let result = if command == "validate" {
                lexis_cli::batch_validate(PathBuf::from(path).as_path())
            } else {
                lexis_cli::batch_summary(PathBuf::from(path).as_path())
            };
            match result {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("graph"), Some(command), Some(path), Some(flag), Some(format))
            if (command == "emit" || command == "preview") && flag == "--format" =>
        {
            let format = match format.as_str() {
                "json" => lexis_cli::GraphFormat::Json,
                "dot" => lexis_cli::GraphFormat::Dot,
                _ => {
                    eprintln!("error: unsupported graph format '{format}'");
                    return ExitCode::from(2);
                }
            };
            let result = if command == "preview" {
                lexis_cli::preview_graph(PathBuf::from(path).as_path(), format)
            } else {
                lexis_cli::emit_graph(PathBuf::from(path).as_path(), format)
            };
            match result {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("graph"), Some(command), Some(path), None, None)
            if command == "summary" || command == "inspect" =>
        {
            let result = if command == "inspect" {
                lexis_cli::inspect_graph(PathBuf::from(path).as_path())
            } else {
                lexis_cli::summarize_graph(PathBuf::from(path).as_path())
            };
            match result {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("graph"), Some(command), Some(path), Some(start_id), Some(end_id))
            if command == "path" =>
        {
            match lexis_cli::graph_path(PathBuf::from(path).as_path(), &start_id, &end_id) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("graph"), Some(command), Some(path), Some(claim_id), None)
            if command == "explain" =>
        {
            match lexis_cli::explain_claim(PathBuf::from(path).as_path(), &claim_id) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("chronicle"), Some(command), Some(path), None, None) if command == "preview" => {
            match lexis_cli::preview_chronicle(PathBuf::from(path).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("artifact"), Some(command), Some(path), Some(out_dir), None)
            if command == "write" =>
        {
            match lexis_cli::write_preview_artifacts(
                PathBuf::from(path).as_path(),
                PathBuf::from(out_dir).as_path(),
            ) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("artifact"), Some(command), Some(path), Some(out_dir), None)
            if command == "write-batch" =>
        {
            match lexis_cli::write_preview_artifact_batch(
                PathBuf::from(path).as_path(),
                PathBuf::from(out_dir).as_path(),
            ) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("artifact"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_artifacts() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("artifact"), Some(command), Some(path), None, None) if command == "summarize" => {
            match lexis_cli::summarize_artifacts(PathBuf::from(path).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("artifact"), Some(command), Some(path), Some(out_md), None)
            if command == "report" =>
        {
            match lexis_cli::write_artifact_report(
                PathBuf::from(path).as_path(),
                PathBuf::from(out_md).as_path(),
            ) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("source"), Some(command), Some(path), None, None) if command == "status" => {
            match lexis_cli::source_status(PathBuf::from(path).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("source"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_sources() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("source"), Some(command), Some(source_id), None, None) if command == "review" => {
            match lexis_cli::source_review(&source_id) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("correction"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_corrections() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("correction"), Some(command), Some(chain_id), None, None) if command == "review" => {
            match lexis_cli::correction_review(&chain_id) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("correction"), Some(command), Some(chain_id), Some(out_seed), None)
            if command == "seed" =>
        {
            match lexis_cli::generate_correction_seed(&chain_id, PathBuf::from(out_seed).as_path())
            {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("correction"), Some(command), Some(artifact_dir), Some(out_md), None)
            if command == "artifact-report" =>
        {
            match lexis_cli::write_correction_artifact_report(
                PathBuf::from(artifact_dir).as_path(),
                PathBuf::from(out_md).as_path(),
            ) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("acceptance"), Some(command), Some(artifact_dir), Some(out_md), None)
            if command == "ai-report" =>
        {
            match lexis_cli::write_ai_acceptance_report(
                PathBuf::from(artifact_dir).as_path(),
                PathBuf::from(out_md).as_path(),
            ) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("correction"), Some(command), Some(out_dir), None, None) if command == "seed-all" => {
            match lexis_cli::generate_correction_seeds(PathBuf::from(out_dir).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("slice"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_slices() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("slice"), Some(command), Some(slice_id), None, None) if command == "review" => {
            match lexis_cli::slice_review(&slice_id) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("scenario"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_scenarios() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("scenario"), Some(command), Some(scenario_id), None, None) if command == "review" => {
            match lexis_cli::scenario_review(&scenario_id) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("work-package"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_work_packages() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("fixture"), Some(command), Some(path), None, None) if command == "readiness" => {
            match lexis_cli::fixture_readiness(PathBuf::from(path).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("fixture"), Some(command), None, None, None) if command == "list" => {
            match lexis_cli::list_fixtures() {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("fixture"), Some(command), Some(path), None, None) if command == "review" => {
            match lexis_cli::fixture_review(PathBuf::from(path).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("diagnostics"), Some(command), Some(path), None, None) if command == "explain" => {
            match lexis_cli::explain_diagnostics(PathBuf::from(path).as_path()) {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        (Some("trace"), Some(command), Some(path), Some(wordform_id), None)
            if command == "word" || command == "lineage" || command == "neighborhood" =>
        {
            let result = if command == "lineage" {
                lexis_cli::trace_lineage(PathBuf::from(path).as_path(), &wordform_id)
            } else if command == "neighborhood" {
                lexis_cli::trace_neighborhood(PathBuf::from(path).as_path(), &wordform_id)
            } else {
                lexis_cli::trace_word(PathBuf::from(path).as_path(), &wordform_id)
            };
            match result {
                Ok(output) => {
                    print!("{output}");
                    ExitCode::SUCCESS
                }
                Err(err) => {
                    eprintln!("error: {err}");
                    ExitCode::from(1)
                }
            }
        }
        _ => {
            eprintln!("usage: lexis validate <fixture>");
            eprintln!("usage: lexis slice generate <seed.yaml> <fixture.yaml>");
            eprintln!("usage: lexis batch validate <fixture-file-or-dir>");
            eprintln!("usage: lexis batch summary <fixture-file-or-dir>");
            eprintln!("usage: lexis trace word <fixture> <wordform-id>");
            eprintln!("usage: lexis trace lineage <fixture> <wordform-id>");
            eprintln!("usage: lexis trace neighborhood <fixture> <node-id>");
            eprintln!("usage: lexis graph emit <fixture> --format json|dot");
            eprintln!("usage: lexis graph preview <fixture> --format json|dot");
            eprintln!("usage: lexis graph summary <fixture>");
            eprintln!("usage: lexis graph inspect <fixture>");
            eprintln!("usage: lexis graph path <fixture> <start-node-id> <end-node-id>");
            eprintln!("usage: lexis graph explain <fixture> <node-or-edge-id>");
            eprintln!("usage: lexis chronicle preview <fixture>");
            eprintln!("usage: lexis artifact list");
            eprintln!("usage: lexis artifact write <fixture> <out-dir>");
            eprintln!("usage: lexis artifact write-batch <fixture-file-or-dir> <out-dir>");
            eprintln!("usage: lexis artifact summarize <artifact-dir>");
            eprintln!("usage: lexis artifact report <artifact-dir> <out-md>");
            eprintln!("usage: lexis source list");
            eprintln!("usage: lexis source review <source-id>");
            eprintln!("usage: lexis source status <fixture>");
            eprintln!("usage: lexis correction list");
            eprintln!("usage: lexis correction review <chain-id>");
            eprintln!("usage: lexis correction seed <chain-id> <out-seed>");
            eprintln!("usage: lexis correction seed-all <out-dir>");
            eprintln!("usage: lexis correction artifact-report <artifact-dir> <out-md>");
            eprintln!("usage: lexis acceptance ai-report <artifact-dir> <out-md>");
            eprintln!("usage: lexis slice list");
            eprintln!("usage: lexis slice review <slice-id>");
            eprintln!("usage: lexis scenario list");
            eprintln!("usage: lexis scenario review <scenario-id>");
            eprintln!("usage: lexis work-package list");
            eprintln!("usage: lexis fixture list");
            eprintln!("usage: lexis fixture readiness <fixture>");
            eprintln!("usage: lexis fixture review <fixture>");
            eprintln!("usage: lexis diagnostics explain <fixture>");
            ExitCode::from(2)
        }
    }
}
