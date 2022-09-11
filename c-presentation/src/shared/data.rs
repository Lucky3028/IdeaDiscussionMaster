use crate::shared::global::{GlobalCurrentAgendaId, GlobalRecordId, GlobalVcId};
use c_domain::repository::{AgendaRepository, RecordRepository};
use c_infra::repository::RedmineRepositoryImpl;
use c_usecase::{AgendaUseCase, RecordUseCase};

use derive_new::new;
use std::sync::Arc;

pub struct Repos {
    pub agenda: Arc<dyn AgendaRepository + Sync + Send>,
    pub record: Arc<dyn RecordRepository + Sync + Send>,
}

impl Repos {
    pub fn new(redmine_url: String) -> Self {
        Self {
            agenda: Arc::new(RedmineRepositoryImpl::<c_domain::Agenda>::new(
                redmine_url.clone(),
            )),
            record: Arc::new(RedmineRepositoryImpl::<c_domain::Record>::new(redmine_url)),
        }
    }
}

#[derive(new)]
pub struct UseCases {
    pub agenda: AgendaUseCase,
    pub record: RecordUseCase,
}

pub struct Data {
    pub use_cases: UseCases,
    pub vc_id: GlobalVcId,
    pub record_id: GlobalRecordId,
    pub current_agenda_id: GlobalCurrentAgendaId,
}

impl Data {
    pub fn new(redmine_url: String) -> Self {
        let repos = Repos::new(redmine_url);
        let use_cases = UseCases::new(
            AgendaUseCase::new(repos.agenda),
            RecordUseCase::new(repos.record),
        );

        Self {
            use_cases,
            vc_id: GlobalVcId::new(),
            record_id: GlobalRecordId::new(),
            current_agenda_id: GlobalCurrentAgendaId::new(),
        }
    }
}
