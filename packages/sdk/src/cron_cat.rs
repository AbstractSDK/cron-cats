use cosmwasm_std::{Deps, CosmosMsg, StdError};

use abstract_sdk::{base::features::{Identification, AbstractNameService}, Execution, os::objects::{ContractEntry, UncheckedContractEntry}};
use cw_croncat_core::msg::{TaskRequest, ExecuteMsg as CCExecMsg};

pub const CRON_CAT_CONTRACT: &str = "cron-cat:scheduler";

/// Interact with other CronCat on the OS.
pub trait CronCatInterface: Execution + AbstractNameService {
    fn cron_cat<'a>(&'a self, deps: Deps<'a>) -> CronCat<Self> {
        CronCat { base: self, deps }
    }
}

impl<T> CronCatInterface for T where T: Execution + AbstractNameService{}

#[derive(Clone)]
pub struct CronCat<'a, T: CronCatInterface> {
    base: &'a T,
    deps: Deps<'a>,
}



impl<'a, T: CronCatInterface> CronCat<'a, T> {
    /// Send a taks
    pub fn automate(&self, task: TaskRequest) -> Result<CosmosMsg, StdError>{
        let cron_cat_entry = UncheckedContractEntry::try_from(CRON_CAT_CONTRACT.into())?.check();
        let cron_cat_addr = self.base.name_service(self.deps).query(&cron_cat_entry)?;
        
        CCExecMsg::CreateTask { task: tq }
        self.base.executor(self.deps).execute(msgs);

        Ok(())
    }
}