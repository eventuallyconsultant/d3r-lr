use goose::prelude::*;
use log::LevelFilter;

fn main() -> Result<(), GooseError> {
    let attack = GooseAttack::initialize()?
        // /!\ WARNING : PRODUCTION ENVIRONMENT !!!
        // .set_default(GooseDefault::Host, "https://www.d3v-lr.fr")?
        // OK : Local stuff !
        .set_default(GooseDefault::Host, "http://127.0.0.1:8080")?
        .set_default(GooseDefault::Users, 1000)?
        .set_default(GooseDefault::HatchRate, "20")?
        .set_default(GooseDefault::RunTime, 60*60)?
        .set_default(GooseDefault::LogLevel, LevelFilter::Warn as usize)?
        .set_default(GooseDefault::StatusCodes, true)?
        // .set_default(GooseDefault::LogFile, "goose.logs")?
        // .set_default(GooseDefault::NoResetMetrics, true)?
        ;

    let attack =
        attack.register_taskset(taskset!("LoadtestTasks").register_task(task!(loadtest_index)));

    attack.execute()?.print();

    Ok(())
}

async fn loadtest_index(user: &GooseUser) -> GooseTaskResult {
    let _goose_metrics = user.get("/").await?;
    Ok(())
}
