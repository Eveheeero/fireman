use crate::{
    core::FirebatCore,
    model::{DecompileRequest, DecompileWithAst, KnownSectionData, OptimizeAstRequest},
};
use std::{sync::mpsc, thread};

pub enum WorkerRequest {
    OpenFile(String),
    AnalyzeSection(String),
    AnalyzeAllSections,
    DecompileSections(DecompileRequest),
    OptimizeAst(OptimizeAstRequest),
}

pub enum WorkerResponse {
    OpenFile(Result<(), String>),
    AnalyzeSection(Result<Vec<KnownSectionData>, String>),
    AnalyzeAllSections(Result<Vec<KnownSectionData>, String>),
    DecompileSections(Result<DecompileWithAst, String>),
    OptimizeAst(Result<DecompileWithAst, String>),
}

pub enum WorkerTryRecv {
    Message(WorkerResponse),
    Empty,
    Disconnected,
}

pub struct FirebatWorker {
    request_tx: mpsc::Sender<WorkerRequest>,
    response_rx: mpsc::Receiver<WorkerResponse>,
}

impl FirebatWorker {
    pub fn spawn() -> Self {
        let (request_tx, request_rx) = mpsc::channel::<WorkerRequest>();
        let (response_tx, response_rx) = mpsc::channel::<WorkerResponse>();
        thread::Builder::new()
            .name("firebat-worker".to_string())
            .spawn(move || {
                let mut core = FirebatCore::default();
                while let Ok(request) = request_rx.recv() {
                    let response = match request {
                        WorkerRequest::OpenFile(path) => {
                            WorkerResponse::OpenFile(core.open_file(&path))
                        }
                        WorkerRequest::AnalyzeSection(address) => {
                            WorkerResponse::AnalyzeSection(core.analyze_section(&address))
                        }
                        WorkerRequest::AnalyzeAllSections => {
                            WorkerResponse::AnalyzeAllSections(core.analyze_all_sections())
                        }
                        WorkerRequest::DecompileSections(request) => {
                            WorkerResponse::DecompileSections(core.decompile_sections(request))
                        }
                        WorkerRequest::OptimizeAst(request) => {
                            WorkerResponse::OptimizeAst(core.optimize_ast(request))
                        }
                    };

                    if response_tx.send(response).is_err() {
                        break;
                    }
                }
            })
            .expect("failed to spawn firebat worker thread");
        Self {
            request_tx,
            response_rx,
        }
    }

    pub fn send(&self, request: WorkerRequest) -> Result<(), String> {
        self.request_tx
            .send(request)
            .map_err(|_| "Background worker is unavailable".to_string())
    }

    pub fn try_recv(&self) -> WorkerTryRecv {
        match self.response_rx.try_recv() {
            Ok(response) => WorkerTryRecv::Message(response),
            Err(mpsc::TryRecvError::Empty) => WorkerTryRecv::Empty,
            Err(mpsc::TryRecvError::Disconnected) => WorkerTryRecv::Disconnected,
        }
    }
}
