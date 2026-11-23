use super::register_uri_scheme;
use anyhow::{Context as _, Result};
use gpui::{AppContext as _, AsyncApp, Context, PromptLevel, Window, actions};
use release_channel::ReleaseChannel;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use util::ResultExt;
use workspace::notifications::{DetachAndPromptErr, NotificationId};
use workspace::{Toast, Workspace};

actions!(
    cli,
    [
        /// Gram CLI ツールをシステムの PATH にインストールします。
        InstallCliBinary,
    ]
);

async fn install_script(cx: &AsyncApp) -> Result<PathBuf> {
    let cli_path = cx.update(|cx| cx.path_for_auxiliary_executable("cli"))??;
    let link_path = Path::new("/usr/local/bin/gram");
    let bin_dir_path = link_path.parent().unwrap();

    // Don't re-create symlink if it points to the same CLI binary.
    if smol::fs::read_link(link_path).await.ok().as_ref() == Some(&cli_path) {
        return Ok(link_path.into());
    }

    // If the symlink is not there or is outdated, first try replacing it
    // without escalating.
    smol::fs::remove_file(link_path).await.log_err();
    if smol::fs::unix::symlink(&cli_path, link_path).await.log_err().is_some() {
        return Ok(link_path.into());
    }

    // The symlink could not be created, so use osascript with admin privileges
    // to create it.
    let status = smol::process::Command::new("/usr/bin/osascript")
        .args([
            "-e",
            &format!(
                "do shell script \" \
                    mkdir -p \'{}\' && \
                    ln -sf \'{}\' \'{}\' \
                \" with administrator privileges",
                bin_dir_path.to_string_lossy(),
                cli_path.to_string_lossy(),
                link_path.to_string_lossy(),
            ),
        ])
        .stdout(smol::process::Stdio::inherit())
        .stderr(smol::process::Stdio::inherit())
        .output()
        .await?
        .status;
    anyhow::ensure!(status.success(), "osascript の実行に失敗しました");
    Ok(link_path.into())
}

pub fn install_cli_binary(window: &mut Window, cx: &mut Context<Workspace>) {
    const LINUX_PROMPT_DETAIL: &str = "リリースの tarball から Gram をインストールした場合は、~/.local/bin を PATH に追加してください。\n\nパッケージマネージャーなど別の方法でインストールした場合は、エイリアスまたはシンボリックリンクを手動で作成する必要があるかもしれません。\n\nパッケージマネージャーによっては、CLI の名前が gram-editor などになっている場合があります。";

    cx.spawn_in(window, async move |workspace, cx| {
        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            let prompt = cx.prompt(
                PromptLevel::Warning,
                "CLI はインストール済みのはずです",
                Some(LINUX_PROMPT_DETAIL),
                &["OK"],
            );
            cx.background_spawn(prompt).detach();
            return Ok(());
        }
        let path = install_script(cx.deref())
            .await
            .context("CLI のシンボリックリンク作成に失敗しました")?;

        workspace.update_in(cx, |workspace, _, cx| {
            struct InstalledGramCli;

            workspace.show_toast(
                Toast::new(
                    NotificationId::unique::<InstalledGramCli>(),
                    format!(
                        "`gram` を {} にインストールしました。ターミナルから {} を起動できます。",
                        path.to_string_lossy(),
                        ReleaseChannel::global(cx).display_name()
                    ),
                ),
                cx,
            )
        })?;
        register_uri_scheme(cx).await.log_err();
        Ok(())
    })
    .detach_and_prompt_err(
        "gram CLI のインストール中にエラーが発生しました",
        window,
        cx,
        |_, _, _| None,
    );
}
