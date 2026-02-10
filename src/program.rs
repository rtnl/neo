use crate::cli::{Cli, CliCommand};
use crate::component::Component;
use crate::executor::Executor;
use crate::node::Node;
use crate::renderer::Renderer;
use crate::request::Request;
use clap::Parser;
use crossterm::cursor::MoveToColumn;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{event, terminal, ExecutableCommand};
use owo_colors::OwoColorize;
use std::env;
use std::io::{stdout, Result, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;
use uuid::Uuid;

const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

pub struct ProgramState {
    components: Vec<Node<Component>>,
    component_root: Uuid,
}

impl ProgramState {
    pub fn new() -> Self {
        let component_root = Node::new(Component::Root {});
        let component_root_id = component_root.get_id();

        Self {
            components: vec![component_root],
            component_root: component_root_id,
        }
    }

    pub fn get_component_root(&self) -> Uuid {
        self.component_root
    }

    pub fn list_component(&self) -> Vec<Uuid> {
        self.components.iter().map(|it| it.get_id()).collect()
    }
}

pub struct Program {
    state: Arc<RwLock<ProgramState>>,
    executor: Arc<Executor>,
    renderer: Arc<Renderer>,
}

impl Program {
    pub async fn new() -> Arc<Self> {
        Arc::new(Self {
            state: Arc::new(RwLock::new(ProgramState::new())),
            executor: Executor::new().await,
            renderer: Renderer::new().await,
        })
    }

    pub async fn launch(self: Arc<Self>) {
        let cli = Cli::parse();

        match cli.get_command().unwrap_or(CliCommand::Interactive) {
            CliCommand::Interactive => {
                self.launch_interactive().await;
            }
            CliCommand::Run { .. } => {}
            CliCommand::Version => {
                self.launch_version().await;
            }
        }
    }

    async fn launch_run(self: Arc<Self>) {}

    async fn launch_version(self: Arc<Self>) {
        eprintln!("version->#{}", &GIT_HASH.unwrap_or("???????")[0..7]);
    }

    async fn launch_interactive(self: Arc<Self>) {
        self.renderer.launch(self.clone()).await;

        loop {
            let mut flag_exit = false;
            let mut input = String::new();

            self.prompt().await;

            terminal::enable_raw_mode().unwrap();

            loop {
                match event::read().unwrap() {
                    Event::Key(KeyEvent {
                        kind,
                        code,
                        modifiers,
                        ..
                    }) => match kind {
                        KeyEventKind::Press => {
                            match (code, modifiers) {
                                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                    // Todo: implement sigint
                                }
                                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                                    flag_exit = true;
                                    break;
                                }
                                (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                                    self.clear().await;
                                    self.reset().await;
                                    break;
                                }
                                (KeyCode::Char(c), KeyModifiers::NONE) => {
                                    input.push(c);
                                    self.clear_line().await;
                                    print!("\r{} {}", "$".bright_black(), input);
                                    stdout().flush().unwrap();
                                }
                                (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                                    let c = c.to_ascii_uppercase();

                                    input.push(c);
                                    self.clear_line().await;
                                    print!("\r{} {}", "$".bright_black(), input);
                                    stdout().flush().unwrap();
                                }
                                (KeyCode::Backspace, KeyModifiers::NONE) => {
                                    if input.pop().is_some() {
                                        self.clear_line().await;
                                        print!("\r{} {}", "$".bright_black(), input);
                                        stdout().flush().unwrap();
                                    }
                                }
                                (KeyCode::Enter, KeyModifiers::NONE) => {
                                    print!("\n");
                                    stdout().flush().unwrap();
                                    self.reset().await;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            terminal::disable_raw_mode().unwrap();

            if flag_exit {
                break;
            }

            if input.is_empty() {
                continue;
            }

            let input = input.trim_end();

            let request = match self.parse_request(input).await {
                Ok(value) => value,
                Err(err) => {
                    println!("error parsing input: {}", err);
                    continue;
                }
            };

            let request = match request {
                Some(value) => value,
                None => continue,
            };

            self.executor.handle_request(request).await;
        }
    }

    async fn clear(&self) {
        let mut stdout = stdout();

        stdout.execute(Clear(ClearType::All)).unwrap();
    }

    async fn clear_line(&self) {
        let mut stdout = stdout();

        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();
    }

    async fn reset(&self) {
        let mut stdout = stdout();

        stdout.execute(MoveToColumn(0)).unwrap();
    }

    async fn prompt(&self) {
        let mut writer = io::BufWriter::new(io::stderr());

        let user = whoami::username().unwrap_or("?".to_string());
        let host = whoami::hostname().unwrap_or("localhost".to_string());
        let path = format!(
            "{}",
            self.reduce_path_home(env::current_dir().unwrap().as_path())
                .unwrap()
                .display()
        );

        let text = format!(
            "{} {} [{}@{}:{}]\n{} ",
            "###".bright_black(),
            "neo".green().bold(),
            user.purple(),
            host.bright_purple(),
            path.yellow(),
            "$".bright_black()
        );

        writer.write_all(text.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
    }

    async fn parse_request(&self, input: &str) -> Result<Option<Request>> {
        let parts: Vec<String> = input
            .split_ascii_whitespace()
            .map(ToString::to_string)
            .collect();

        let key = match parts.first() {
            Some(v) => v.to_string(),
            None => return Ok(None),
        };

        let args: Vec<String> = parts[1..].iter().map(ToString::to_string).collect();

        Ok(Some(Request::new(key, args)))
    }

    fn reduce_path_home(&self, path: &Path) -> Result<PathBuf> {
        let home = match homedir::my_home() {
            Ok(v) => v,
            Err(err) => return Ok(path.to_path_buf()),
        };

        Ok(match home {
            Some(home_path) => match path.strip_prefix(&home_path) {
                Ok(relative) => {
                    let mut result = PathBuf::from("~");
                    result.push(relative);
                    result
                }
                Err(_) => path.to_path_buf(),
            },
            None => path.to_path_buf(),
        })
    }
}
