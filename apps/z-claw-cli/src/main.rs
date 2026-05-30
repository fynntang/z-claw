use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::mpsc;

use z_claw_agent::{AgentLoop, Harness, HookRegistry, default_system_prompt};
use z_claw_config::load_config;
use z_claw_core::{AgentEvent, NativePlatform};
use z_claw_memory::NoopMemory;
use z_claw_providers::{OpenAiProvider, ProviderChain};
use z_claw_security::{PolicyEngine, SecurityLevel};
use z_claw_tools::builtin_tools;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("z_claw=info")
        .init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "prompt" {
        let prompt = args.get(2).cloned().unwrap_or_default();
        if prompt.is_empty() {
            eprintln!("Usage: z-claw-cli prompt \"<message>\"");
            return;
        }
        run_oneshot(&prompt).await;
    } else {
        run_repl().await;
    }
}

async fn build_agent() -> AgentLoop {
    let platform = NativePlatform;
    let config = load_config(&platform);

    let base_url = config
        .providers
        .first()
        .map(|p| p.base_url.clone())
        .unwrap_or_else(|| "http://localhost:11434/v1".to_string());
    let api_key = config
        .providers
        .first()
        .and_then(|p| z_claw_config::resolve_api_key(p))
        .unwrap_or_else(|| "ollama".to_string());
    let model = config
        .providers
        .first()
        .and_then(|p| p.default_model.clone())
        .or(Some(config.default_model.clone()))
        .unwrap_or_else(|| "llama3".to_string());

    let provider = Arc::new(OpenAiProvider::new(
        "ollama".into(),
        base_url,
        api_key,
        model,
    ));

    let chain = ProviderChain::from_single(provider);
    let harness = Arc::new(Harness {
        providers: chain,
        tools: Arc::new(builtin_tools(None)),
        memory: Arc::new(NoopMemory),
        policy: PolicyEngine::new(
            config.policy.blocked_commands,
            config.policy.allowed_paths,
            SecurityLevel::ConfirmExecute,
        ),
        system_prompt: default_system_prompt(),
        hooks: HookRegistry::new(),
    });

    let session_id = uuid::Uuid::new_v4().to_string();
    AgentLoop::new(harness, session_id)
}

async fn run_oneshot(prompt: &str) {
    let mut agent = build_agent().await;
    let (event_tx, mut event_rx) = mpsc::unbounded_channel::<AgentEvent>();

    let prompt = prompt.to_string();
    let handle = tokio::spawn(async move { agent.run_turn(&prompt, &event_tx).await });

    let mut stdout = io::stdout();
    while let Some(event) = event_rx.recv().await {
        match event {
            AgentEvent::TextDelta { delta, .. } => {
                print!("{delta}");
                stdout.flush().ok();
            }
            AgentEvent::ToolCallStarted { tool_name, .. } => {
                eprintln!("\n[{tool_name}] running...");
            }
            AgentEvent::ToolCallFinished {
                tool_name,
                ok,
                summary,
                ..
            } => {
                let status = if ok { "ok" } else { "error" };
                eprintln!("[{tool_name}] {status}: {summary}");
            }
            AgentEvent::Error { message } => {
                eprintln!("\nError: {message}");
            }
            AgentEvent::StreamingDone { .. } => {
                println!();
                break;
            }
            _ => {}
        }
    }

    match handle.await {
        Ok(Ok(_)) => {}
        Ok(Err(e)) => eprintln!("\nAgent error: {e}"),
        Err(e) => eprintln!("\nTask error: {e}"),
    }
}

async fn run_repl() {
    println!("z-claw CLI — type /help for commands, /quit to exit");

    let mut agent = build_agent().await;

    loop {
        print!("\n> ");
        io::stdout().flush().ok();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        }

        let input = input.trim().to_string();
        if input.is_empty() {
            continue;
        }

        match input.as_str() {
            "/quit" | "/exit" | "/q" => break,
            "/help" | "/h" => {
                println!("Commands:");
                println!("  /quit, /exit, /q  — quit");
                println!("  /help, /h         — this help");
                println!();
                println!("Anything else is sent to the agent.");
                continue;
            }
            s if s.starts_with('/') => {
                println!("Unknown command: {s}");
                continue;
            }
            _ => {}
        }

        let (event_tx, mut event_rx) = mpsc::unbounded_channel::<AgentEvent>();
        let prompt = input.clone();

        let handle = tokio::spawn(async move { agent.run_turn(&prompt, &event_tx).await });

        while let Some(event) = event_rx.recv().await {
            match event {
                AgentEvent::TextDelta { delta, .. } => {
                    print!("{delta}");
                    io::stdout().flush().ok();
                }
                AgentEvent::ToolCallStarted { tool_name, .. } => {
                    println!("\n: [{tool_name}]");
                }
                AgentEvent::ToolCallFinished {
                    tool_name,
                    ok,
                    summary,
                    ..
                } => {
                    let status = if ok { "ok" } else { "ERR" };
                    println!("[{tool_name}] {status}: {summary}");
                }
                AgentEvent::Error { message } => {
                    eprintln!("\n! Error: {message}");
                }
                AgentEvent::StreamingDone { .. } => {
                    println!();
                    break;
                }
                _ => {}
            }
        }

        agent = match handle.await {
            Ok(Ok(_)) => build_agent().await,
            Ok(Err(e)) => {
                eprintln!("\nAgent error: {e}");
                build_agent().await
            }
            Err(e) => {
                eprintln!("\nTask error: {e}");
                build_agent().await
            }
        };
    }

    println!("\nGoodbye.");
}
