use pty_process::{Pty, ReadPty, WritePty, OwnedReadPty, OwnedWritePty};
use tokio::process::Child;

pub struct LocalShell {
    pty: Pty,
    child: Child,
}

impl LocalShell {
    pub fn new(command: &str) -> Result<LocalShell, pty_process::Error>{
        let mut pty = pty_process::Pty::new()?;
        pty.resize(pty_process::Size::new(24, 80))?;
        let mut cmd = pty_process::Command::new(command);
        let child = cmd.spawn(&pty.pts()?)?;
        Ok(LocalShell {
            pty: pty,
            child: child
        })
    }

    pub fn split(&mut self) -> (ReadPty<'_>, WritePty<'_>){
        self.pty.split()
    }

    pub fn into_split(self) -> (OwnedReadPty, OwnedWritePty){
        self.pty.into_split()
    }
}
