use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Callback(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Cmd {
    FileSystem(FsApi),
    Notification(Notify),
    SubProcess(Process),
    Window(WindowProxy),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum WindowProxy {
    SetTitle {
        title: String,
        success_callback: String,
        error_callback: String,
    },
    // Maximize {
    //     success_callback: String,
    //     error_callback: String,
    // },

    // Minimize {
    //     success_callback: String,
    //     error_callback: String,
    // },

    // Show {
    //     success_callback: String,
    //     error_callback: String,
    // },

    // Hide {
    //     success_callback: String,
    //     error_callback: String,
    // },

    // SetDecorations {
    //     decorations: bool,
    //     success_callback: String,
    //     error_callback: String,
    // },
    SetFullscreen {
        fullscreen: bool,
        success_callback: String,
        error_callback: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Notify {
    ShowNotification {
        summary: String,
        body: String,
        timeout: i32,
        success_callback: String,
        error_callback: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Process {
    Exec {
        cmd: String,
        cwd: String,
        stream_output: bool,
        success_callback: String,
        error_callback: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FsApi {
    ReadDir {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    ReadTextFile {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    CopyFile {
        from: String,
        to: String,
        success_callback: String,
        error_callback: String,
    },
    RenameFile {
        from: String,
        to: String,
        success_callback: String,
        error_callback: String,
    },
    OpenDialog {
        multiple: bool,
        filter: Option<Vec<String>>,
        success_callback: String,
        error_callback: String,
    },
    OpenFile {
        path: String,
        format: Option<Vec<String>>,
    },
    CreateDir {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    CreateFile {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    RemoveDir {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    RemoveFile {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    OpenMultipleFile {
        path: String,
        format: Option<Vec<String>>,
    },
    OpenDir {
        path: String,
    },
    SelectFolder {
        success_callback: String,
        error_callback: String,
    },
    SaveFile {
        path: String,
        content: Vec<u8>,
        mode: String,
        success_callback: String,
        error_callback: String,
    },
}
