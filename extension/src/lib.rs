use std::{env, fs};

use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, GithubReleaseOptions, LanguageServerId, Os,
    Result,
};

const SERVER_NAME: &str = "tonel-smalltalk-language-server";
const LSP_REPOSITORY: &str = "mumez/tonel-smalltalk-language-server";

struct TonelSmalltalkExtension;

impl TonelSmalltalkExtension {
    fn installed_binary_path(&self, os: Os) -> &'static str {
        if os == Os::Windows {
            "bin/tonel-smalltalk-language-server.exe"
        } else {
            "bin/tonel-smalltalk-language-server"
        }
    }

    fn release_asset_spec(
        &self,
        os: Os,
        arch: Architecture,
    ) -> Result<(&'static str, DownloadedFileType)> {
        match (os, arch) {
            (Os::Mac, Architecture::Aarch64) => Ok((
                "tonel-smalltalk-language-server-aarch64-apple-darwin.tar.gz",
                DownloadedFileType::GzipTar,
            )),
            (Os::Mac, Architecture::X8664) => Ok((
                "tonel-smalltalk-language-server-x86_64-apple-darwin.tar.gz",
                DownloadedFileType::GzipTar,
            )),
            (Os::Linux, Architecture::Aarch64) => Ok((
                "tonel-smalltalk-language-server-aarch64-unknown-linux-gnu.tar.gz",
                DownloadedFileType::GzipTar,
            )),
            (Os::Linux, Architecture::X8664) => Ok((
                "tonel-smalltalk-language-server-x86_64-unknown-linux-gnu.tar.gz",
                DownloadedFileType::GzipTar,
            )),
            (Os::Windows, Architecture::Aarch64) => Ok((
                "tonel-smalltalk-language-server-aarch64-pc-windows-msvc.zip",
                DownloadedFileType::Zip,
            )),
            (Os::Windows, Architecture::X8664) => Ok((
                "tonel-smalltalk-language-server-x86_64-pc-windows-msvc.zip",
                DownloadedFileType::Zip,
            )),
            _ => Err("current platform is not supported by tonel-smalltalk releases".to_string()),
        }
    }

    fn auto_installed_binary(&self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let (os, arch) = zed::current_platform();
        let binary_path = self.installed_binary_path(os);

        if fs::metadata(binary_path).map_or(false, |stat| stat.is_file()) {
            return Ok(binary_path.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            LSP_REPOSITORY,
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let (asset_name, asset_type) = self.release_asset_spec(os, arch)?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "release {} in {} does not contain required asset '{}'; please publish platform binaries",
                    release.version, LSP_REPOSITORY, asset_name
                )
            })?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(&asset.download_url, "bin", asset_type)?;

        if os != Os::Windows {
            zed::make_file_executable(binary_path)?;
        }

        if !fs::metadata(binary_path).map_or(false, |stat| stat.is_file()) {
            return Err(format!("downloaded asset was not written to {binary_path}"));
        }

        Ok(binary_path.to_string())
    }
}

impl zed::Extension for TonelSmalltalkExtension {
    fn new() -> Self {
        TonelSmalltalkExtension
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = if let Some(binary) = worktree.which(SERVER_NAME) {
            binary
        } else {
            env::current_dir()
                .unwrap()
                .join(self.auto_installed_binary(language_server_id)?)
                .to_string_lossy()
                .to_string()
        };

        Ok(zed::Command {
            command: binary,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TonelSmalltalkExtension);
