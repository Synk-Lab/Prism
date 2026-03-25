//! `prism trace` — Replay transaction and output execution trace.

use clap::Args;
use prism_core::types::config::NetworkConfig;
use crate::output::trace_tree;

#[derive(Args)]
pub struct TraceArgs {
    /// Transaction hash to trace.
    pub tx_hash: String,

    /// Output trace to a file instead of stdout.
    #[arg(long, short)]
    pub output_file: Option<String>,
}

pub async fn run(
    args: TraceArgs,
    network: &NetworkConfig,
    output_format: &str
) -> anyhow::Result<()> {
    let progress = indicatif::ProgressBar::new_spinner();
    progress.set_message("Reconstructing state and replaying transaction...");
    progress.enable_steady_tick(std::time::Duration::from_millis(100));

    let trace = prism_core::replay::replay_transaction(&args.tx_hash, network).await?;

    progress.finish_and_clear();

    let output = match output_format {
        "json" => serde_json::to_string_pretty(&trace)?,
        "tree" => {
            // For tree format, we render directly to stdout
            if let Some(path) = &args.output_file {
                // If output file specified, we need to capture the output
                use termcolor::{ BufferWriter, ColorChoice };
                let writer = BufferWriter::stdout(ColorChoice::Never);
                let mut buffer = writer.buffer();
                trace_tree::render_trace_tree(&mut buffer, &trace)?;
                String::from_utf8_lossy(&buffer.into_inner()).to_string()
            } else {
                // Direct rendering to stdout
                trace_tree::print_trace_tree(&trace)?;
                return Ok(());
            }
        }
        _ => format!("{trace:#?}"),
    };

    if let Some(path) = args.output_file {
        std::fs::write(&path, &output)?;
        println!("Trace written to {path}");
    } else {
        println!("{output}");
    }

    Ok(())
}
