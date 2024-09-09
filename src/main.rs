use planif::enums::TaskCreationFlags;
use planif::schedule::TaskScheduler;
use planif::schedule_builder::{Action, ScheduleBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task_name = "StartEDR";

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "create" => create_task(task_name)?,
        "delete" => delete_task(task_name)?,
        _ => print_help(),
    }

    Ok(())
}

fn create_task(task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let com_runtime = TaskScheduler::new()?.get_com();
    let scheduler = ScheduleBuilder::new(&com_runtime).unwrap();
    scheduler
        .create_time()
        .author("EDR")?
        .description("Start EDR service")?
        .trigger("time_trigger", true)?
        .action(Action::new(
            "start_edr_service",
            "sc.exe",
            "",
            "start sample",
        ))?
        .start_boundary("1970-01-01T00:00:00")?
        .repetition(
            planif::settings::Duration::default(),
            planif::settings::Duration {
                minutes: Some(1),
                ..Default::default()
            },
            true,
        )?
        .build()?
        .register(task_name, TaskCreationFlags::CreateOrUpdate as i32)?;

    println!("Creating task success");
    Ok(())
}

fn delete_task(task_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    TaskScheduler::delete_task(task_name)?;

    println!("Deleting task success");
    Ok(())
}

fn print_help() {
    println!("win-service.exe <create|delete>")
}
