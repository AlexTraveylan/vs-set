use std::fs;
use std::path::Path;
use std::io;

pub trait CopyStrategy {
    fn get_settings(&self) -> &str;
    fn copy_files(&self, target_dir: &Path) -> io::Result<()> {
        let settings_path = target_dir.join("settings.json");
        fs::write(settings_path, self.get_settings())?;
        Ok(())
    }
}

pub struct PythonStrategy;
pub struct TypeScriptStrategy;


impl CopyStrategy for PythonStrategy {
    fn get_settings(&self) -> &str {
        r#"{
  "[python]": {
    "editor.defaultFormatter": "charliermarsh.ruff",
    "editor.formatOnSave": true,
    "editor.codeActionsOnSave": {
      "source.fixAll.ruff": "always",
      "source.organizeImports.ruff": "always",
      "source.unusedImports.ruff": "never"
    }
  },
  "terminal.integrated.env.windows": {
    "PYTHONPATH": "${workspaceFolder}"
  },
  "python.testing.pytestArgs": [
    "tests"
  ],
  "python.testing.unittestEnabled": false,
  "python.testing.pytestEnabled": true,
  "editor.rulers": [
    88,
    90
  ],
  "files.exclude": {
    "**/__pycache__": true,
    "**/*_cache": true
  }
}"#
    }
}

impl CopyStrategy for TypeScriptStrategy {
    fn get_settings(&self) -> &str {
        r#"{
  "typescript.tsdk": "node_modules\\typescript\\lib",
  "typescript.enablePromptUseWorkspaceTsdk": true,
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": "explicit",
    "source.organizeImports": "explicit"
  },
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "prettier.printWidth": 147,
  "editor.rulers": [147],
  "prettier.semi": false
}"#
    }
}