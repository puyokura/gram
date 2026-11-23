use anyhow::Context as _;

use git::repository::{Remote, RemoteCommandOutput};
use linkify::{LinkFinder, LinkKind};
use ui::SharedString;
use util::ResultExt as _;

#[derive(Clone)]
pub enum RemoteAction {
    Fetch(Option<Remote>),
    Pull(Remote),
    Push(SharedString, Remote),
}

impl RemoteAction {
    pub fn name(&self) -> &'static str {
        match self {
            RemoteAction::Fetch(_) => "fetch",
            RemoteAction::Pull(_) => "pull",
            RemoteAction::Push(_, _) => "push",
        }
    }
}

pub enum SuccessStyle {
    Toast,
    ToastWithLog { output: RemoteCommandOutput },
    PushPrLink { text: String, link: String },
}

pub struct SuccessMessage {
    pub message: String,
    pub style: SuccessStyle,
}

pub fn format_output(action: &RemoteAction, output: RemoteCommandOutput) -> SuccessMessage {
    match action {
        RemoteAction::Fetch(remote) => {
            if output.stderr.is_empty() {
                SuccessMessage {
                    message: "フェッチ: 最新の状態です".into(),
                    style: SuccessStyle::Toast,
                }
            } else {
                let message = match remote {
                    Some(remote) => format!("{} と同期しました", remote.name),
                    None => "リモートと同期しました".into(),
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Pull(remote_ref) => {
            let get_changes = |output: &RemoteCommandOutput| -> anyhow::Result<u32> {
                let last_line = output
                    .stdout
                    .lines()
                    .last()
                    .context("Failed to get last line of output")?
                    .trim();

                let files_changed = last_line
                    .split_whitespace()
                    .next()
                    .context("Failed to get first word of last line")?
                    .parse()?;

                Ok(files_changed)
            };
            if output.stdout.ends_with("Already up to date.\n") {
                SuccessMessage {
                    message: "プル: 最新の状態です".into(),
                    style: SuccessStyle::Toast,
                }
            } else if output.stdout.starts_with("Updating") {
                let files_changed = get_changes(&output).log_err();
                let message = if let Some(files_changed) = files_changed {
                    format!(
                        "{} 件のファイル変更を {} から受信しました",
                        files_changed, remote_ref.name
                    )
                } else {
                    format!("{} からファストフォワードしました", remote_ref.name)
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.starts_with("Merge") {
                let files_changed = get_changes(&output).log_err();
                let message = if let Some(files_changed) = files_changed {
                    format!(
                        "{} 件のファイル変更を {} からマージしました",
                        files_changed, remote_ref.name
                    )
                } else {
                    format!("{} からマージしました", remote_ref.name)
                };
                SuccessMessage {
                    message,
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else if output.stdout.contains("Successfully rebased") {
                SuccessMessage {
                    message: format!("{} からリベースしました", remote_ref.name),
                    style: SuccessStyle::ToastWithLog { output },
                }
            } else {
                SuccessMessage {
                    message: format!("{} からプルしました", remote_ref.name),
                    style: SuccessStyle::ToastWithLog { output },
                }
            }
        }
        RemoteAction::Push(branch_name, remote_ref) => {
            let message = if output.stderr.ends_with("Everything up-to-date\n") {
                "プッシュ: すべて最新です".to_string()
            } else {
                format!("{} を {} にプッシュしました", branch_name, remote_ref.name)
            };

            let style = if output.stderr.ends_with("Everything up-to-date\n") {
                Some(SuccessStyle::Toast)
            } else if output.stderr.contains("\nremote: ") {
                let pr_hints = [
                    ("Create a pull request", "プルリクエストを作成"),    // GitHub
                    ("Create pull request", "プルリクエストを作成"),      // Bitbucket
                    ("create a merge request", "マージリクエストを作成"), // GitLab
                    ("View merge request", "マージリクエストを表示"),     // GitLab
                ];
                pr_hints
                    .iter()
                    .find(|(indicator, _)| output.stderr.contains(indicator))
                    .and_then(|(_, mapped)| {
                        let finder = LinkFinder::new();
                        finder
                            .links(&output.stderr)
                            .filter(|link| *link.kind() == LinkKind::Url)
                            .filter(|link| !link.as_str().contains("www.openssh.org"))
                            .map(|link| link.start()..link.end())
                            .next()
                            .map(|link| SuccessStyle::PushPrLink {
                                text: mapped.to_string(),
                                link: output.stderr[link].to_string(),
                            })
                    })
            } else {
                None
            };
            SuccessMessage {
                message,
                style: style.unwrap_or(SuccessStyle::ToastWithLog { output }),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_push_new_branch_pull_request() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! { "
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: Create a pull request for 'test' on GitHub by visiting:
                remote:      https://example.com/test/test/pull/new/test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { text: hint, link } = &msg.style {
            assert_eq!(hint, "プルリクエストを作成");
            assert_eq!(link, "https://example.com/test/test/pull/new/test");
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! {"
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: To create a merge request for test, visit:
                remote:   https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test
                remote:
                To example.com:test/test.git
                 * [new branch]      test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { text, link } = &msg.style {
            assert_eq!(text, "マージリクエストを作成");
            assert_eq!(
                link,
                "https://example.com/test/test/-/merge_requests/new?merge_request%5Bsource_branch%5D=test"
            );
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_branch_existing_merge_request() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! {"
                Total 0 (delta 0), reused 0 (delta 0), pack-reused 0 (from 0)
                remote:
                remote: View merge request for test:
                remote:    https://example.com/test/test/-/merge_requests/99999
                remote:
                To example.com:test/test.git
                    + 80bd3c83be...e03d499d2e test -> test
                "}
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::PushPrLink { text, link } = &msg.style {
            assert_eq!(text, "マージリクエストを表示");
            assert_eq!(link, "https://example.com/test/test/-/merge_requests/99999");
        } else {
            panic!("Expected PushPrLink variant");
        }
    }

    #[test]
    fn test_push_new_branch_no_link() {
        let action = RemoteAction::Push(
            SharedString::new("test_branch"),
            Remote {
                name: SharedString::new("test_remote"),
            },
        );

        let output = RemoteCommandOutput {
            stdout: String::new(),
            stderr: indoc! { "
                To http://example.com/test/test.git
                 * [new branch]      test -> test
                ",
            }
            .to_string(),
        };

        let msg = format_output(&action, output);

        if let SuccessStyle::ToastWithLog { output } = &msg.style {
            assert_eq!(
                output.stderr,
                "To http://example.com/test/test.git\n * [new branch]      test -> test\n"
            );
        } else {
            panic!("Expected ToastWithLog variant");
        }
    }
}
