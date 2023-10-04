use pty_process::{OwnedReadPty, OwnedWritePty, Pty, ReadPty, Size, WritePty};
use tokio::process::Child;

use super::{ResizablePty, WinSize};

pub struct LocalShell {
    pty: Pty,
    child: Child,
}

impl LocalShell {
    pub fn new(command: &str) -> Result<LocalShell, pty_process::Error> {
        let pty = pty_process::Pty::new()?;
        pty.resize(pty_process::Size::new(24, 80))?;
        let mut cmd = pty_process::Command::new(command);
        let child = cmd.spawn(&pty.pts()?)?;
        Ok(LocalShell {
            pty: pty,
            child: child,
        })
    }

    pub async fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.kill().await
    }
}

impl ResizablePty for LocalShell {
    type BorrowedR<'a> = ReadPty<'a>;
    type BorrowedW<'a> = WritePty<'a>;
    type OwnedR = OwnedReadPty;
    type OwnedW = OwnedWritePty;
    type Error = pty_process::Error;

    fn split(&mut self) -> (ReadPty<'_>, WritePty<'_>) {
        self.pty.split()
    }

    fn into_split(self) -> (OwnedReadPty, OwnedWritePty) {
        self.pty.into_split()
    }

    fn resize(
        &self,
        WinSize {
            row,
            col,
            xpixel,
            ypixel,
        }: super::WinSize,
    ) -> Result<(), pty_process::Error> {
        // TODO: Throw error when value out of range?
        self.pty.resize(Size::new_with_pixel(
            row as u16,
            col as u16,
            xpixel as u16,
            ypixel as u16,
        ))
    }
}
